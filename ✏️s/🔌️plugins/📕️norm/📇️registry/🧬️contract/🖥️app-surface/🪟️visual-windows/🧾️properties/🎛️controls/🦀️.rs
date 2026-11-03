//! 🎛️ Typed controls of the norm inspector: unit tables, locale-aware number parsing and one control per field kind. See [`super`].

use crate::*;

//#region 📏️Units
/// 📏️ A stored unit shown in a presentation unit of its family: `display = stored * factor`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitDisplay {
    pub stored: UnitId,
    pub si: String,
    pub symbol: String,
    pub factor: f64,
}

impl UnitDisplay {
    /// 📏️ `stored` shown in `shown`; units of another family or with an offset (°C) stay in the stored unit.
    pub fn between(stored: UnitId, shown: UnitId) -> Self {
        let shown = if shown.family() == stored.family() && shown.offset() == 0.0 && stored.offset() == 0.0 { shown } else { stored };
        Self { stored, si: stored.symbol().to_owned(), symbol: if shown == UnitId::One { String::new() } else { shown.symbol().to_owned() }, factor: stored.factor() / shown.factor() }
    }

    /// 📏️ A dimensionless value: no symbol, no scaling.
    pub fn plain() -> Self {
        Self::between(UnitId::One, UnitId::One)
    }

    /// 📏️ The unit a field spec shows under `system`: quantities, angles (degrees), percent and ratios; `None` for non-numeric kinds.
    pub fn of_spec(spec: &FieldSpec, system: UnitSystem) -> Option<Self> {
        match spec.kind {
            FieldKind::Quantity { stored, .. } => Some(Self::between(stored, spec.display_unit(system).unwrap_or(stored))),
            FieldKind::Angle => Some(Self::between(UnitId::Radian, UnitId::Degree)),
            FieldKind::Percent => Some(Self::between(UnitId::Percent, UnitId::Percent)),
            FieldKind::Ratio => Some(Self::plain()),
            _ => None,
        }
    }
}
//#endregion 📏️Units

//#region 🔢️Numbers
fn separators(locale: Locale) -> (char, char) {
    match locale {
        Locale::En => ('.', ','),
        Locale::De => (',', '.'),
    }
}

/// 🔢️ Fixed-precision number text in the locale's decimal separator, trailing zeros trimmed.
pub fn format_number(value: f64, precision: u16, locale: Locale) -> String {
    let fixed = format!("{value:.*}", precision as usize);
    let trimmed = if fixed.contains('.') { fixed.trim_end_matches('0').trim_end_matches('.').to_string() } else { fixed };
    let trimmed = if trimmed == "-0" { "0".into() } else { trimmed };
    trimmed.replace('.', &separators(locale).0.to_string())
}

/// 🔢️ Parses locale number text: with both separators the last is the decimal mark; a repeated one groups; a lone one decimals unless it is the locale's grouping mark in a `1,234` pattern.
pub fn parse_number(text: &str, locale: Locale) -> Option<f64> {
    let (decimal, group) = separators(locale);
    let clean: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let dot = clean.matches('.').count();
    let comma = clean.matches(',').count();
    let decimal_mark = match (dot, comma) {
        (0, 0) => None,
        (_, 0) if dot == 1 => Some('.'),
        (0, _) if comma == 1 => Some(','),
        (_, _) if dot > 0 && comma > 0 => clean.rfind(['.', ',']).and_then(|index| clean[index..].chars().next()),
        _ => None,
    };
    let decimal_mark = decimal_mark.filter(|mark| {
        let index = clean.rfind(*mark).unwrap_or(0);
        let grouping = *mark == group && *mark != decimal && dot + comma == 1 && clean.len() - index - 1 == 3 && index <= 3;
        !grouping
    });
    let digits: String = clean
        .chars()
        .filter_map(|c| match c {
            '.' | ',' => None,
            other => Some(other),
        })
        .collect();
    let parsed = match decimal_mark {
        Some(mark) => {
            let index = clean.rfind(mark)?;
            let tail: String = clean[index + 1..].chars().filter(|c| *c != '.' && *c != ',').collect();
            let head: String = clean[..index].chars().filter(|c| *c != '.' && *c != ',').collect();
            format!("{head}.{tail}")
        }
        None => digits,
    };
    parsed.parse::<f64>().ok().filter(|value| value.is_finite())
}

/// ⚠️ Why a typed quantity could not be taken.
#[derive(Clone, Debug, PartialEq)]
pub enum QuantityIssue {
    Empty,
    NotANumber,
    UnknownUnit(String),
}

/// 📥️ Parses `1,5 mm` / `1.5` in `display`'s unit family to a SI value; a bare number is read in the display unit.
pub fn parse_quantity(text: &str, display: &UnitDisplay, locale: Locale) -> Result<f64, QuantityIssue> {
    let text = text.trim();
    if text.is_empty() {
        return Err(QuantityIssue::Empty);
    }
    let split = text.char_indices().find(|(_, c)| !(c.is_ascii_digit() || matches!(c, '.' | ',' | '+' | '-' | 'e' | 'E' | ' ' | '\u{a0}'))).map(|(index, _)| index).unwrap_or(text.len());
    let (number, unit) = text.split_at(split);
    let unit = unit.trim();
    let value = parse_number(number, locale).ok_or(QuantityIssue::NotANumber)?;
    if unit.is_empty() || unit == display.symbol {
        return Ok(value / display.factor);
    }
    if unit == display.si {
        return Ok(value);
    }
    let typed = UnitId::lookup(unit).ok_or_else(|| QuantityIssue::UnknownUnit(unit.into()))?;
    typed.convert(value, display.stored).map_err(|_| QuantityIssue::UnknownUnit(unit.into()))
}

/// 📤️ `1,5 mm` for a SI value in `display`'s unit and the locale's decimal separator.
pub fn format_quantity_text(si_value: f64, display: &UnitDisplay, precision: u16, locale: Locale) -> String {
    let number = format_number(si_value * display.factor, precision, locale);
    if display.symbol.is_empty() {
        number
    } else {
        format!("{number} {}", display.symbol)
    }
}
//#endregion 🔢️Numbers

//#region 🧾️FieldView
/// 🧾️ One selectable option of a choice or entity-reference field.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
}

/// 🧾️ What a field is, which decides its control.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldControl {
    Quantity { unit: UnitDisplay },
    Int,
    Toggle,
    Choice(Vec<FieldOption>),
    EntityRef { to: String, options: Vec<FieldOption> },
    Text,
}

/// 🏷️ National-annex parameter badge: the NDP symbol, its clause and the annex default in the stored unit.
#[derive(Clone, Debug, PartialEq)]
pub struct NdpBadge {
    pub symbol: String,
    pub clause: ClauseRef,
    pub default: Option<f64>,
}

/// 🗃️ A field edited over several rows at once: the collection, the selected row ids and the field name; its controls raise one `setFields` bundle instead of one `setField`.
#[derive(Clone, Debug, PartialEq)]
pub struct BulkTarget {
    pub collection: String,
    pub ids: Vec<String>,
    pub field: String,
}

/// 🧾️ Everything the inspector needs to draw and validate one field.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldView {
    pub path: String,
    pub label: String,
    pub help: Option<String>,
    pub control: FieldControl,
    pub range: Option<(f64, f64)>,
    pub soft_range: Option<(f64, f64)>,
    pub step: Option<f64>,
    pub precision: u16,
    pub ndp: Option<NdpBadge>,
    pub clause: Option<ClauseRef>,
    pub bulk: Option<BulkTarget>,
}

impl FieldView {
    pub fn new(path: impl Into<String>, label: impl Into<String>, control: FieldControl) -> Self {
        Self { path: path.into(), label: label.into(), help: None, control, range: None, soft_range: None, step: None, precision: 3, ndp: None, clause: None, bulk: None }
    }

    /// 🎛️ The action the controls of this field raise: `setField` for one row, `setFields` for a bulk edit.
    pub fn verb(&self) -> &'static str {
        if self.bulk.is_some() { "setFields" } else { "setField" }
    }

    /// 🎛️ The fixed arguments of [`Self::verb`]; the control adds the new `value`. A bulk edit names the collection, the row ids (one per line) and the field, which the command bridge expands to one write per row.
    pub fn args(&self) -> UiAssemblyResult<UiValue> {
        match &self.bulk {
            None => set_args(&self.path),
            Some(target) => action_args_map(vec![("collection", ui_value_text(&target.collection)?), ("ids", ui_value_text(target.ids.join("\n"))?), ("field", ui_value_text(&target.field)?)]),
        }
    }

    /// 🚧️ Localized refusal naming the hard range in display units.
    pub fn refusal(&self, locale: Locale) -> Option<String> {
        let (low, high) = self.range?;
        let unit = match &self.control {
            FieldControl::Quantity { unit } => unit.clone(),
            _ => UnitDisplay::plain(),
        };
        let bounds = format!("{} … {}", format_quantity_text(low, &unit, self.precision, locale), format_quantity_text(high, &unit, self.precision, locale));
        Some(format!("{} {bounds}", chrome("Out of range, allowed:", "Außerhalb des Bereichs, zulässig:", locale)))
    }

    /// 🚧️ The refusal when `si` leaves the hard range.
    pub fn violation(&self, si: f64, locale: Locale) -> Option<String> {
        let (low, high) = self.range?;
        (si < low || si > high).then(|| self.refusal(locale)).flatten()
    }

    /// 🏷️ Label with its unit suffix: the accessible name of the control.
    pub fn labelled(&self) -> String {
        match &self.control {
            FieldControl::Quantity { unit } if !unit.symbol.is_empty() => format!("{} ({})", self.label, unit.symbol),
            _ => self.label.clone(),
        }
    }
}
//#endregion 🧾️FieldView

//#region 🎛️Controls
fn limits_of(field: &FieldView, locale: Locale) -> Option<ui::UiNumberLimits> {
    let (low, high) = field.range?;
    let refusal = field.refusal(locale).and_then(|text| norm_ui_label(text).ok());
    Some(ui::UiNumberLimits {
        min: Some(ui::UiNumberBound { value: low, exclusive: false, refusal: refusal.clone() }),
        max: Some(ui::UiNumberBound { value: high, exclusive: false, refusal }),
    })
}

fn built(node: impl Into<BuiltNode>) -> BuiltNode {
    node.into()
}

fn finish<B: HasBase + Into<BuiltNode>>(builder: B, name: &str) -> UiAssemblyResult<BuiltNode> {
    let builder = builder.try_label(name).map_err(|_| ui_error("ui.node.label"))?;
    Ok(built(builder))
}

fn set_args(path: &str) -> UiAssemblyResult<UiValue> {
    action_args_map(vec![("path", ui_value_text(path)?)])
}

fn number_input(field: &FieldView, si: Option<f64>, unit: &UnitDisplay, row_id: &str, controller_id: &'static str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let mut input = ui::input(InputKind::Number).try_id(format!("{row_id}.input")).map_err(|_| ui_error("ui.node.id"))?.display_factor(unit.factor).precision(field.precision).placeholder(norm_ui_label(chrome("empty", "leer", locale))?);
    if let Some(step) = field.step {
        input = input.step(step);
    }
    if let Some((low, high)) = field.range {
        input = input.min(low).max(high);
    }
    if let Some(limits) = limits_of(field, locale) {
        input = input.limits(limits);
    }
    if let Some(value) = si {
        input = input.number(value);
    }
    let input = input.commit(ui_text("blur")?).draft_target(ui_text(&field.path)?);
    finish(bind_on(input, Trigger::Commit, controller_id, field.verb(), field.args()?)?, &field.labelled())
}

fn slider(field: &FieldView, si: f64, unit: &UnitDisplay, row_id: &str, controller_id: &'static str, locale: Locale) -> UiAssemblyResult<Option<BuiltNode>> {
    let Some((low, high)) = field.soft_range else { return Ok(None) };
    let step = field.step.unwrap_or((high - low) / 100.0);
    let mut slider = ui::slider(si).min(low).max(high).step(step).precision(field.precision).display_factor(unit.factor).display_unit(ui_text(&unit.symbol)?).unit(ui_text(&unit.si)?);
    if let Some(limits) = limits_of(field, locale) {
        slider = slider.limits(limits);
    }
    for index in 0..16 {
        let snap = low + index as f64 * step * 10.0;
        if snap > high {
            break;
        }
        slider = match slider.try_snap(snap) {
            Ok(next) => next,
            Err(same) => {
                slider = same;
                break;
            }
        };
    }
    let slider = slider.try_id(format!("{row_id}.slider")).map_err(|_| ui_error("ui.node.id"))?;
    finish(bind_change(slider, controller_id, field.verb(), field.args()?)?, &field.labelled()).map(Some)
}

fn select_of(value: &str, options: &[FieldOption], row_id: &str, name: &str, field: &FieldView, controller_id: &'static str) -> UiAssemblyResult<BuiltNode> {
    let mut select = ui::select(ui_text(value)?).try_id(format!("{row_id}.select")).map_err(|_| ui_error("ui.node.id"))?;
    let mut seen = false;
    for option in options.iter().take(31) {
        seen |= option.value == value;
        select = select.try_item(ui_text(&option.value)?, norm_ui_label(&option.label)?).map_err(|_| ui_error("ui.select.item"))?;
    }
    if !seen && !value.is_empty() {
        select = select.try_item(ui_text(value)?, norm_ui_label(value)?).map_err(|_| ui_error("ui.select.item"))?;
    }
    finish(bind_change(select, controller_id, field.verb(), field.args()?)?, name)
}

/// 🎛️ The controls of one field row for its current document value (several for a slider-backed quantity).
pub fn field_controls(field: &FieldView, value: &semio_framework_value::DslValue, row_id: &str, controller_id: &'static str, locale: Locale) -> UiAssemblyResult<Vec<BuiltNode>> {
    use semio_framework_value::DslValue;
    let name = field.labelled();
    let mut nodes = Vec::new();
    match (&field.control, value) {
        (FieldControl::Quantity { unit }, value) => {
            let si = match value {
                DslValue::Number(number) => Some(number.as_f64()),
                _ => None,
            };
            if let Some(value) = si {
                if let Some(slider) = slider(field, value, unit, row_id, controller_id, locale)? {
                    nodes.push(slider);
                }
            }
            nodes.push(number_input(field, si, unit, row_id, controller_id, locale)?);
        }
        (FieldControl::Int, value) => {
            let current = if let DslValue::Number(number) = value { number.as_f64() } else { 0.0 };
            let mut stepper = ui::number_stepper(current).step(field.step.unwrap_or(1.0)).precision(0);
            if let Some((low, high)) = field.range {
                stepper = stepper.min(low).max(high);
            }
            let stepper = stepper.try_id(format!("{row_id}.stepper")).map_err(|_| ui_error("ui.node.id"))?;
            nodes.push(finish(bind_change(stepper, controller_id, field.verb(), field.args()?)?, &name)?);
        }
        (FieldControl::Toggle, value) => {
            let on = matches!(value, DslValue::Bool(true));
            let toggle = ui::toggle(on).appearance(ui::ToggleAppearance::Checkbox).text(norm_ui_label(&field.label)?).try_id(format!("{row_id}.toggle")).map_err(|_| ui_error("ui.node.id"))?;
            nodes.push(finish(bind_change(toggle, controller_id, field.verb(), field.args()?)?, &name)?);
        }
        (FieldControl::Choice(options), value) | (FieldControl::EntityRef { options, .. }, value) => {
            let current = if let DslValue::String(raw) = value { raw.as_str() } else { "" };
            nodes.push(select_of(current, options, row_id, &name, field, controller_id)?);
        }
        (FieldControl::Text, value) => {
            let current = if let DslValue::String(raw) = value { raw.as_str() } else { "" };
            let input = ui::input(InputKind::Text).value(ui_text(current)?).placeholder(norm_ui_label(chrome("empty", "leer", locale))?).commit(ui_text("blur")?).try_id(format!("{row_id}.input")).map_err(|_| ui_error("ui.node.id"))?;
            nodes.push(finish(bind_on(input, Trigger::Commit, controller_id, field.verb(), field.args()?)?, &name)?);
        }
    }
    Ok(nodes)
}
//#endregion 🎛️Controls
