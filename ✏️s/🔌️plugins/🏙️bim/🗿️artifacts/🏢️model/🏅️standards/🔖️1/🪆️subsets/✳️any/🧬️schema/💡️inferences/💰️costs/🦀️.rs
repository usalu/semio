//! 💰️ `costs`: the money of the model. Authored [`CostItem`]s (a priced basis) are linked to element types by [`TypeCostLink`]s (a factor); every element of a linked type
//! is priced with `quantity * unit_cost * factor`, where the quantity is the field of the already inferred quantity row that the item's basis names. Nothing is stored.
//!
//! The product of the canonical decimal inputs is exact and rounded once, half up, to the precision of the item; sums are exact decimals per currency, never converted.
//! A missing item, an invalid item, link or quantity and an overflow are [`CostDiagnostic`]s and publish no amount.
//!
//! Related: the TypeScript twin `../🟦️.ts`, <https://en.wikipedia.org/wiki/Rounding#Rounding_half_up>.

use std::collections::BTreeMap;

//#region 🔖️Schema
/// 📏️ The quantity field an item is priced by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CostBasis {
    Count,
    Length,
    GrossArea,
    NetArea,
    SurfaceArea,
    GrossSideArea,
    NetSideArea,
    GrossVolume,
    NetVolume,
    Mass,
}

impl CostBasis {
    /// 🔤️ The basis named by its schema string.
    pub fn parse(name: &str) -> Option<CostBasis> {
        Some(match name {
            "Count" => CostBasis::Count,
            "Length" => CostBasis::Length,
            "GrossArea" => CostBasis::GrossArea,
            "NetArea" => CostBasis::NetArea,
            "SurfaceArea" => CostBasis::SurfaceArea,
            "GrossSideArea" => CostBasis::GrossSideArea,
            "NetSideArea" => CostBasis::NetSideArea,
            "GrossVolume" => CostBasis::GrossVolume,
            "NetVolume" => CostBasis::NetVolume,
            "Mass" => CostBasis::Mass,
            _ => return None,
        })
    }
}

/// 🏷️ A priced basis in one currency.
#[derive(Clone, Debug, PartialEq)]
pub struct CostItem {
    pub name: String,
    pub currency: String,
    pub basis: Option<CostBasis>,
    pub unit_cost: f64,
    pub precision: f64,
}

/// 🔗️ The price of an item for every element of one type, scaled by a factor.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeCostLink {
    pub type_id: String,
    pub item: String,
    pub factor: f64,
}

/// 🧮️ The fields of one inferred quantity row that costing consumes.
#[derive(Clone, Debug, PartialEq)]
pub struct CostQuantity {
    pub storey: String,
    pub type_id: String,
    pub measures: BTreeMap<CostBasis, f64>,
}

/// 🧾️ A derived cost line with enough authored and inferred inputs to explain its amount.
#[derive(Clone, Debug, PartialEq)]
pub struct CostLine {
    pub element: String,
    pub link: String,
    pub item: String,
    pub type_id: String,
    pub storey: String,
    pub currency: String,
    pub quantity: f64,
    pub unit_cost: f64,
    pub factor: f64,
    pub precision: f64,
    pub total: f64,
}

/// 🚫️ A failed cost input; consumers localize the machine code.
#[derive(Clone, Debug, PartialEq)]
pub struct CostDiagnostic {
    pub code: &'static str,
    pub link: String,
    pub item: String,
    pub element: String,
}

/// 💰️ Derived amounts grouped by scope and currency, without currency conversion or authored totals.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelCosts {
    pub lines: Vec<CostLine>,
    pub elements: BTreeMap<String, BTreeMap<String, f64>>,
    pub types: BTreeMap<String, BTreeMap<String, f64>>,
    pub storeys: BTreeMap<String, BTreeMap<String, f64>>,
    pub items: BTreeMap<String, BTreeMap<String, f64>>,
    pub project: BTreeMap<String, f64>,
    pub diagnostics: Vec<CostDiagnostic>,
}
//#endregion 🔖️Schema

//#region 🔖️Decimal
/// 🔟️ A nonnegative exact decimal `digits * 10^exponent`, digits least significant first, no leading zeros.
#[derive(Clone, Debug, PartialEq)]
struct Decimal {
    digits: Vec<u8>,
    exponent: i32,
}

impl Decimal {
    fn zero() -> Decimal {
        Decimal { digits: Vec::new(), exponent: 0 }
    }

    fn trimmed(mut self) -> Decimal {
        while self.digits.last() == Some(&0) {
            self.digits.pop();
        }
        self
    }

    fn of(value: f64) -> Decimal {
        let text = format!("{:e}", value.abs());
        let (mantissa, exponent) = text.split_once('e').unwrap_or((&text, "0"));
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let digits = whole.bytes().chain(fraction.bytes()).rev().map(|digit| digit - b'0').collect();
        Decimal { digits, exponent: exponent.parse::<i32>().unwrap_or(0) - fraction.len() as i32 }.trimmed()
    }

    fn shifted(&self, places: usize) -> Vec<u8> {
        if self.digits.is_empty() { Vec::new() } else { std::iter::repeat_n(0, places).chain(self.digits.iter().copied()).collect() }
    }

    fn times(&self, other: &Decimal) -> Decimal {
        let mut digits = vec![0u32; self.digits.len() + other.digits.len() + 1];
        for (at, left) in self.digits.iter().enumerate() {
            for (to, right) in other.digits.iter().enumerate() {
                digits[at + to] += u32::from(*left) * u32::from(*right);
            }
        }
        let mut carry = 0;
        let digits = digits.into_iter().map(|digit| {
            let total = digit + carry;
            carry = total / 10;
            (total % 10) as u8
        });
        Decimal { digits: digits.collect(), exponent: self.exponent + other.exponent }.trimmed()
    }

    fn plus(&self, other: &Decimal) -> Decimal {
        let exponent = self.exponent.min(other.exponent);
        let (left, right) = (self.shifted((self.exponent - exponent) as usize), other.shifted((other.exponent - exponent) as usize));
        let mut carry = 0;
        let digits = (0..left.len().max(right.len()) + 1).map(|at| {
            let total = left.get(at).copied().unwrap_or(0) + right.get(at).copied().unwrap_or(0) + carry;
            carry = total / 10;
            total % 10
        });
        Decimal { digits: digits.collect(), exponent }.trimmed()
    }

    fn rounded(self, precision: i32) -> Decimal {
        let shift = self.exponent + precision;
        if shift >= 0 {
            return Decimal { digits: self.shifted(shift as usize), exponent: -precision };
        }
        let dropped = (-shift) as usize;
        let up = self.digits.get(dropped - 1).is_some_and(|digit| *digit >= 5);
        let kept = Decimal { digits: self.digits.iter().skip(dropped).copied().collect(), exponent: -precision };
        if up { kept.plus(&Decimal { digits: vec![1], exponent: -precision }) } else { kept }
    }

    fn number(&self) -> f64 {
        let digits: String = self.digits.iter().rev().map(|digit| char::from(b'0' + digit)).collect();
        format!("{}e{}", if digits.is_empty() { "0" } else { &digits }, self.exponent).parse().unwrap_or(f64::INFINITY)
    }
}

fn product(terms: [f64; 3], precision: i32) -> Decimal {
    terms.iter().map(|term| Decimal::of(*term)).reduce(|left, right| left.times(&right)).unwrap_or_else(Decimal::zero).rounded(precision)
}
//#endregion 🔖️Decimal

//#region 🔖️Inference
fn nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn valid_item(item: &CostItem) -> bool {
    !item.name.trim().is_empty() && !item.currency.trim().is_empty() && item.basis.is_some() && nonnegative(item.unit_cost) && item.precision.fract() == 0.0 && (0.0..=9.0).contains(&item.precision)
}

type Totals = BTreeMap<String, BTreeMap<String, Decimal>>;

fn present(totals: &Totals) -> BTreeMap<String, BTreeMap<String, f64>> {
    totals.iter().map(|(id, currencies)| (id.clone(), currencies.iter().map(|(currency, value)| (currency.clone(), value.number())).collect())).collect()
}

/// 💡️ Costs existing inferred quantity rows using authored items and links; no model or inference state is held.
pub fn infer_costs(items: &BTreeMap<String, CostItem>, links: &BTreeMap<String, TypeCostLink>, elements: &BTreeMap<String, CostQuantity>) -> ModelCosts {
    let mut result = ModelCosts::default();
    let mut scopes: [Totals; 5] = Default::default();
    let mut by_type: BTreeMap<&str, Vec<(&String, &TypeCostLink, &CostItem)>> = BTreeMap::new();
    for (id, link) in links {
        let item = items.get(&link.item);
        let code = if link.type_id.trim().is_empty() || !nonnegative(link.factor) {
            Some("cost.link-invalid")
        } else if item.is_none() {
            Some("cost.item-missing")
        } else if !item.is_some_and(valid_item) {
            Some("cost.item-invalid")
        } else {
            None
        };
        match (code, item) {
            (Some(code), _) => result.diagnostics.push(CostDiagnostic { code, link: id.clone(), item: link.item.clone(), element: String::new() }),
            (None, Some(item)) => by_type.entry(&link.type_id).or_default().push((id, link, item)),
            (None, None) => {}
        }
    }
    for (element, measured) in elements {
        for (id, link, item) in by_type.get(measured.type_id.as_str()).into_iter().flatten() {
            let quantity = item.basis.and_then(|basis| measured.measures.get(&basis).copied()).unwrap_or(0.0);
            let diagnostic = |code| CostDiagnostic { code, link: (*id).clone(), item: link.item.clone(), element: element.clone() };
            if !nonnegative(quantity) {
                result.diagnostics.push(diagnostic("cost.quantity-invalid"));
                continue;
            }
            let amount = product([quantity, item.unit_cost, link.factor], item.precision as i32);
            let keys = [element.as_str(), measured.type_id.as_str(), measured.storey.as_str(), link.item.as_str(), ""];
            let updates: Vec<Decimal> = scopes.iter().zip(keys).map(|(scope, key)| scope.get(key).and_then(|currencies| currencies.get(&item.currency)).unwrap_or(&Decimal::zero()).plus(&amount)).collect();
            if !amount.number().is_finite() || updates.iter().any(|value| !value.number().is_finite()) {
                result.diagnostics.push(diagnostic("cost.overflow"));
                continue;
            }
            for ((scope, key), update) in scopes.iter_mut().zip(keys).zip(updates) {
                scope.entry(key.to_string()).or_default().insert(item.currency.clone(), update);
            }
            result.lines.push(CostLine { element: element.clone(), link: (*id).clone(), item: link.item.clone(), type_id: measured.type_id.clone(), storey: measured.storey.clone(), currency: item.currency.clone(), quantity, unit_cost: item.unit_cost, factor: link.factor, precision: item.precision, total: amount.number() });
        }
    }
    [result.elements, result.types, result.storeys, result.items] = [present(&scopes[0]), present(&scopes[1]), present(&scopes[2]), present(&scopes[3])];
    result.project = present(&scopes[4]).remove("").unwrap_or_default();
    result
}
//#endregion 🔖️Inference
