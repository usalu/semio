//! 🧮️ EN 1990 combination generator: Eq. 6.10, 6.10a/b, 6.11b, 6.12b and 6.14b-6.16b with leading and accompanying variable actions and mutually exclusive action groups. See [`super`].

use super::*;

//#region 🔖️ActionModel
/// 🏷️ Action classes of EN 1990 4.1.1 that decide the combination role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionClass {
    Permanent,
    Prestress,
    Variable,
    Accidental,
    Seismic,
}

impl ActionClass {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "G" => Some(Self::Permanent),
            "P" => Some(Self::Prestress),
            "Q" => Some(Self::Variable),
            "A" => Some(Self::Accidental),
            "E" => Some(Self::Seismic),
            _ => None,
        }
    }
}

/// 🎚️ Project overrides of the annex factors of one action (EN 1990 A1: the National Annex values are replaced only where the annex allows it): γ (γ_G,sup of a permanent action, γ_Q of a variable one) and ψ₀, ψ₁, ψ₂; `None` keeps the annex value.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ActionOverride {
    pub gamma: Option<f64>,
    pub psi: [Option<f64>; 3],
}

impl ActionOverride {
    pub fn is_empty(&self) -> bool {
        self.gamma.is_none() && self.psi.iter().all(Option::is_none)
    }

    fn applied(&self, row: PsiRow) -> PsiRow {
        PsiRow { psi_0: self.psi[0].unwrap_or(row.psi_0), psi_1: self.psi[1].unwrap_or(row.psi_1), psi_2: self.psi[2].unwrap_or(row.psi_2) }
    }
}

/// ⚡️ One action reduced to its effect on the verified point: `value` is the signed characteristic or design effect (the sign decides whether it is adverse or relieving in a direction), `group` a mutually exclusive group id ("" = independent).
#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    pub id: String,
    pub class: ActionClass,
    pub value: f64,
    pub category: String,
    pub group: String,
    pub psi_scale: f64,
    pub cap: Option<f64>,
    pub overrides: ActionOverride,
}

impl Action {
    pub fn new(id: impl Into<String>, class: ActionClass, value: f64) -> Self {
        Self { id: id.into(), class, value, category: String::new(), group: String::new(), psi_scale: 1.0, cap: None, overrides: ActionOverride::default() }
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    /// 🧢️ Upper bound of |value| while a traffic load group acts together with this action (F*_W of Annex A2.2.2); `None` = uncapped.
    pub fn cap(mut self, cap: f64) -> Self {
        self.cap = Some(cap);
        self
    }

    /// 🎚️ Project overrides of γ and ψ of this action (see [`ActionOverride`]).
    pub fn overrides(mut self, overrides: ActionOverride) -> Self {
        self.overrides = overrides;
        self
    }

    /// 🏛️ Scale φ of ψ_2 in the seismic combination 6.12b (ψ_E,i = φ·ψ_2,i, EN 1998-1 4.2.4); 1.0 elsewhere.
    pub fn psi_scale(mut self, scale: f64) -> Self {
        self.psi_scale = scale;
        self
    }

    /// 🔤️ Parses `id|class|value|category|group|psi_scale|cap` rows; the last four parts are optional, `-` means empty (scale 1, no cap).
    pub fn parse(row: &str) -> Result<Self, String> {
        let parts: Vec<&str> = row.split('|').map(str::trim).collect();
        if parts.len() < 3 {
            return Err(format!("action row {row:?} needs id|class|value"));
        }
        let class = ActionClass::parse(parts[1]).ok_or_else(|| format!("unknown action class {:?} in {row:?}", parts[1]))?;
        let value: f64 = parts[2].parse().map_err(|_| format!("bad action value {:?} in {row:?}", parts[2]))?;
        let part = |index: usize| parts.get(index).copied().filter(|text| *text != "-").unwrap_or("");
        Ok(Self { id: parts[0].into(), class, value, category: part(3).into(), group: part(4).into(), psi_scale: part(5).parse().unwrap_or(1.0), cap: part(6).parse().ok(), overrides: ActionOverride::default() })
    }

    /// 🔤️ Parses a `;`-separated list of action rows.
    pub fn parse_list(text: &str) -> Result<Vec<Self>, String> {
        text.split(';').map(str::trim).filter(|row| !row.is_empty()).map(Self::parse).collect()
    }
}
//#endregion 🔖️ActionModel

//#region 🔖️Equations
/// 🔢️ The EN 1990 equations the generator evaluates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Equation {
    E610,
    E610a,
    E610b,
    E611b,
    E612b,
    Characteristic,
    Frequent,
    QuasiPermanent,
}

impl Equation {
    pub fn label(self) -> &'static str {
        match self {
            Self::E610 => "6.10",
            Self::E610a => "6.10a",
            Self::E610b => "6.10b",
            Self::E611b => "6.11b",
            Self::E612b => "6.12b",
            Self::Characteristic => "6.14b",
            Self::Frequent => "6.15b",
            Self::QuasiPermanent => "6.16b",
        }
    }

    pub fn title(self) -> (&'static str, &'static str) {
        match self {
            Self::E610 => ("Fundamental combination", "Grundkombination"),
            Self::E610a => ("Fundamental combination (a)", "Grundkombination (a)"),
            Self::E610b => ("Fundamental combination (b)", "Grundkombination (b)"),
            Self::E611b => ("Accidental combination", "Außergewöhnliche Kombination"),
            Self::E612b => ("Seismic combination", "Erdbebenkombination"),
            Self::Characteristic => ("SLS characteristic combination", "GZG charakteristische Kombination"),
            Self::Frequent => ("SLS frequent combination", "GZG häufige Kombination"),
            Self::QuasiPermanent => ("SLS quasi-permanent combination", "GZG quasi-ständige Kombination"),
        }
    }

    pub fn ultimate(self) -> bool {
        matches!(self, Self::E610 | Self::E610a | Self::E610b)
    }
}

/// 🧭️ Everything the generator needs besides the actions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Context {
    pub annex: AnnexChoice,
    pub domain: Domain,
    pub set: FactorSet,
    pub k_fi: f64,
    pub accidental_leading_psi1: bool,
    pub transient: bool,
    pub wind_thermal_exclusive: bool,
    pub xi: Option<f64>,
    pub k_fi_overridden: bool,
}

impl Context {
    pub fn new(annex: AnnexChoice, domain: Domain, set: FactorSet, k_fi: f64) -> Self {
        Self { annex, domain, set, k_fi, accidental_leading_psi1: true, transient: false, wind_thermal_exclusive: domain == Domain::Bridge && ndp(annex, "ndp.en1990.a2.wind_thermal.simultaneous") < 0.5, xi: None, k_fi_overridden: false }
    }

    /// 🎚️ Project override of the reduction factor ξ of Eq. 6.10b.
    pub fn with_xi(mut self, xi: f64) -> Self {
        self.xi = Some(xi);
        self
    }

    /// 🎚️ Marks the K_FI of this context as a project override (the value itself is the `k_fi` argument of [`Self::new`]).
    pub fn with_k_fi_override(mut self) -> Self {
        self.k_fi_overridden = true;
        self
    }

    /// 🛠️ The transient (execution) situation: only execution actions act as variable actions, the persistent situation excludes them.
    pub fn transient(mut self) -> Self {
        self.transient = true;
        self
    }
}

/// 🧾️ One contribution to a combination: the action, its total factor and its signed contribution.
#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    pub action: String,
    pub factor: f64,
    pub contribution: f64,
    pub deviates: bool,
}

/// 🧮️ One generated combination: the unfavourable and favourable sums with the leading unit and the exclusive group that was active.
#[derive(Clone, Debug, PartialEq)]
pub struct Combination {
    pub equation: Equation,
    pub direction: f64,
    pub leading: String,
    pub group: String,
    pub excluded: String,
    pub unfavourable: f64,
    pub favourable: f64,
    pub terms: Vec<Term>,
}

impl Combination {
    /// ➕️ The signed design effect in the combination's direction: adverse contributions plus relieving ones (relieving ones carry the opposite sign).
    pub fn total(&self) -> f64 {
        self.unfavourable + self.favourable
    }

    /// ➖️ The net destabilising effect E_d,dst − E_d,stb (EQU, direction +1): identical to [`Self::total`] because stabilising contributions are negative.
    pub fn net(&self) -> f64 {
        self.total()
    }

    /// 🛡️ The stabilising effect E_d,stb as a positive number.
    pub fn stabilising(&self) -> f64 {
        -self.favourable
    }

    /// 🎚️ Whether any factor of this combination is a project override that deviates from the National Annex.
    pub fn deviates(&self) -> bool {
        self.terms.iter().any(|term| term.deviates)
    }

    pub fn describe(&self) -> String {
        let mut text = format!("{} leading={}", self.equation.label(), if self.leading.is_empty() { "-" } else { &self.leading });
        if !self.group.is_empty() {
            text.push_str(&format!(" group={}", self.group));
        }
        if !self.excluded.is_empty() {
            text.push_str(&format!(" without={}", self.excluded));
        }
        for term in &self.terms {
            text.push_str(&format!(" {}*{:.4}{}", term.action, term.factor, if term.deviates { "!" } else { "" }));
        }
        text
    }
}
//#endregion 🔖️Equations

//#region 🔖️Generator
struct Unit<'a> {
    id: String,
    members: Vec<&'a Action>,
}

fn variable_psi(ctx: &Context, action: &Action) -> Result<PsiRow, String> {
    psi_lookup(ctx.annex, &action.category).map(|row| action.overrides.applied(row)).ok_or_else(|| format!("variable action {} has unknown ψ category {:?}", action.id, action.category))
}

/// 🚧️ One variable-action scenario: the active traffic group, the family excluded by the non-simultaneity rule of Annex A2.2.2 and the units that act.
struct Scenario<'a> {
    group: String,
    excluded: &'static str,
    units: Vec<Unit<'a>>,
}

fn family_of<'a>(unit: &Unit<'a>) -> Option<&'static str> {
    unit.members.iter().find_map(|action| non_simultaneous_family(&action.category))
}

/// 🌬️ Wind and thermal actions never act together (Annex A2.2.2, registry `a2.wind_thermal.simultaneous`): a scenario holding both splits into one without thermal and one without wind.
fn split_exclusive<'a>(ctx: &Context, group: &str, units: Vec<Unit<'a>>) -> Vec<Scenario<'a>> {
    let has = |family: &str| units.iter().any(|unit| family_of(unit) == Some(family));
    if !ctx.wind_thermal_exclusive || !has("wind") || !has("thermal") {
        return vec![Scenario { group: group.into(), excluded: "", units }];
    }
    ["thermal", "wind"]
        .into_iter()
        .map(|excluded| Scenario { group: group.into(), excluded, units: units.iter().filter(|unit| family_of(unit) != Some(excluded)).map(|unit| Unit { id: unit.id.clone(), members: unit.members.clone() }).collect() })
        .collect()
}

fn scenarios<'a>(ctx: &Context, variables: &[&'a Action]) -> Vec<Scenario<'a>> {
    let mut groups: Vec<&str> = variables.iter().map(|action| action.group.as_str()).filter(|group| !group.is_empty()).collect();
    groups.sort_unstable();
    groups.dedup();
    let independent: Vec<Unit<'a>> = variables.iter().filter(|action| action.group.is_empty()).map(|action| Unit { id: action.id.clone(), members: vec![*action] }).collect();
    if groups.is_empty() {
        return split_exclusive(ctx, "", independent);
    }
    groups
        .into_iter()
        .flat_map(|group| {
            let mut units: Vec<Unit<'a>> = independent.iter().map(|unit| Unit { id: unit.id.clone(), members: unit.members.clone() }).collect();
            units.push(Unit { id: group.into(), members: variables.iter().copied().filter(|action| action.group == group).collect() });
            split_exclusive(ctx, group, units)
        })
        .collect()
}

/// 🧢️ The value an action contributes in a scenario: capped at F*_W while a traffic load acts in the same scenario.
fn scenario_value(action: &Action, units: &[Unit<'_>]) -> f64 {
    match action.cap {
        Some(cap) if units.iter().flat_map(|unit| unit.members.iter()).any(|other| traffic_group(&other.category).is_some()) => action.value.signum() * action.value.abs().min(cap),
        _ => action.value,
    }
}

fn push(terms: &mut Vec<Term>, unfavourable: &mut f64, favourable: &mut f64, action: &Action, value: f64, (factor, deviates): (f64, bool), fav_side: bool) {
    let contribution = factor * value;
    terms.push(Term { action: action.id.clone(), factor, contribution, deviates });
    if fav_side {
        *favourable += contribution;
    } else {
        *unfavourable += contribution;
    }
}

/// 🧮️ The maximum-direction combinations of one equation (see [`combinations_dir`]).
pub fn combinations(ctx: &Context, actions: &[Action], equation: Equation) -> Result<Vec<Combination>, String> {
    combinations_dir(ctx, actions, equation, 1.0)
}

/// 🧮️ Generates every combination of one equation in one direction (+1 maximises, −1 minimises the effect): one per (active exclusive group, leading variable unit); accidental and seismic equations additionally branch per A_d / A_Ed. A permanent action takes γG,sup when its effect adds in the direction and γG,inf when it relieves; a variable action whose effect relieves is never counted.
pub fn combinations_dir(ctx: &Context, actions: &[Action], equation: Equation, direction: f64) -> Result<Vec<Combination>, String> {
    let permanents: Vec<&Action> = actions.iter().filter(|action| action.class == ActionClass::Permanent).collect();
    let prestress: Vec<&Action> = actions.iter().filter(|action| action.class == ActionClass::Prestress).collect();
    let variables: Vec<&Action> = actions.iter().filter(|action| action.class == ActionClass::Variable && action.value * direction > 0.0 && is_execution_category(&action.category) == ctx.transient).collect();
    let design: Vec<&Action> = match equation {
        Equation::E611b => actions.iter().filter(|action| action.class == ActionClass::Accidental).collect(),
        Equation::E612b => actions.iter().filter(|action| action.class == ActionClass::Seismic).collect(),
        _ => Vec::new(),
    };
    if matches!(equation, Equation::E611b | Equation::E612b) && design.is_empty() {
        return Ok(Vec::new());
    }
    for variable in &variables {
        variable_psi(ctx, variable)?;
    }
    let (g_sup, g_inf) = gamma_g(ctx.annex, ctx.domain, ctx.set);
    let ultimate = equation.ultimate();
    let permanent_factor = |action: &Action, relieving: bool| match (ultimate, relieving) {
        (true, false) => {
            let reduces = equation == Equation::E610b;
            let xi_value = if reduces { ctx.xi.unwrap_or_else(|| xi(ctx.annex, ctx.domain)) } else { 1.0 };
            (ctx.k_fi * action.overrides.gamma.unwrap_or(g_sup) * xi_value, action.overrides.gamma.is_some() || ctx.k_fi_overridden || (reduces && ctx.xi.is_some()))
        }
        (true, true) => (g_inf, false),
        _ => (1.0, false),
    };
    let variable_scenarios = scenarios(ctx, &variables);
    let mut out = Vec::new();
    let branches: Vec<Option<&Action>> = if design.is_empty() { vec![None] } else { design.iter().copied().map(Some).collect() };
    for branch in branches {
        for Scenario { group, excluded, units } in &variable_scenarios {
            let leads: Vec<Option<&Unit<'_>>> = if units.is_empty() || equation == Equation::QuasiPermanent || equation == Equation::E612b { vec![None] } else { units.iter().map(Some).collect() };
            for lead in leads {
                let (mut unfavourable, mut favourable, mut terms) = (0.0, 0.0, Vec::new());
                for action in &permanents {
                    let relieving = action.value * direction < 0.0;
                    push(&mut terms, &mut unfavourable, &mut favourable, action, action.value, permanent_factor(action, relieving), relieving && ultimate);
                }
                for action in &prestress {
                    let relieving = action.value * direction < 0.0;
                    push(&mut terms, &mut unfavourable, &mut favourable, action, action.value, (if ultimate { gamma_p(ctx.annex, relieving) } else { 1.0 }, false), relieving && ultimate);
                }
                if let Some(design_action) = branch {
                    push(&mut terms, &mut unfavourable, &mut favourable, design_action, design_action.value, (1.0, false), false);
                }
                for unit in units {
                    let is_lead = lead.is_some_and(|candidate| candidate.id == unit.id);
                    for action in &unit.members {
                        let row = variable_psi(ctx, action)?;
                        let gq = ctx.k_fi * action.overrides.gamma.unwrap_or_else(|| gamma_q(ctx.annex, ctx.domain, ctx.set, &action.category));
                        let ovr = &action.overrides;
                        let uses_gamma = equation.ultimate();
                        let psi_slot = match (equation, is_lead) {
                            (Equation::E610 | Equation::E610b, true) | (Equation::Characteristic, true) => None,
                            (Equation::E610 | Equation::E610a | Equation::E610b, _) | (Equation::Characteristic, false) => Some(0),
                            (Equation::E611b, true) => Some(if ctx.accidental_leading_psi1 { 1 } else { 2 }),
                            (Equation::Frequent, true) => Some(1),
                            _ => Some(2),
                        };
                        let deviates = (uses_gamma && (ovr.gamma.is_some() || ctx.k_fi_overridden)) || psi_slot.is_some_and(|slot| ovr.psi[slot].is_some());
                        let factor = match (equation, is_lead) {
                            (Equation::E610 | Equation::E610b, true) => gq,
                            (Equation::E610a, true) => gq * row.psi_0,
                            (Equation::E610 | Equation::E610a | Equation::E610b, false) => gq * row.psi_0,
                            (Equation::E611b, true) => if ctx.accidental_leading_psi1 { row.psi_1 } else { row.psi_2 },
                            (Equation::E611b, false) | (Equation::QuasiPermanent, _) => row.psi_2,
                            (Equation::E612b, _) => row.psi_2 * action.psi_scale,
                            (Equation::Characteristic, true) => 1.0,
                            (Equation::Characteristic, false) => row.psi_0,
                            (Equation::Frequent, true) => row.psi_1,
                            (Equation::Frequent, false) => row.psi_2,
                        };
                        push(&mut terms, &mut unfavourable, &mut favourable, action, scenario_value(action, units), (factor, deviates), false);
                    }
                }
                let leading = lead.map(|unit| unit.id.clone()).unwrap_or_default();
                let leading = match (branch, leading.is_empty()) {
                    (Some(design_action), true) => design_action.id.clone(),
                    (Some(design_action), false) => format!("{}+{}", design_action.id, leading),
                    (None, _) => leading,
                };
                out.push(Combination { equation, direction, leading, group: group.clone(), excluded: (*excluded).into(), unfavourable, favourable, terms });
            }
        }
    }
    Ok(out)
}

/// 📈️ The governing combination of a list: the largest total effect (net destabilising effect for EQU).
pub fn governing(combinations: &[Combination], net: bool) -> Option<&Combination> {
    let key = |combination: &Combination| if net { combination.net() } else { combination.total() };
    combinations.iter().max_by(|left, right| key(left).total_cmp(&key(right)))
}

/// 📉️ The minimum-direction governing combination: the most negative total.
pub fn governing_min(combinations: &[Combination]) -> Option<&Combination> {
    combinations.iter().min_by(|left, right| left.total().total_cmp(&right.total()))
}

/// 🔢️ Whether Eq. 6.10a/b may replace Eq. 6.10: the governing value of the pair is the larger of both.
pub fn governing_610ab(a: &[Combination], b: &[Combination]) -> Option<f64> {
    let left = governing(a, false).map(Combination::total)?;
    let right = governing(b, false).map(Combination::total)?;
    Some(left.max(right))
}
//#endregion 🔖️Generator
