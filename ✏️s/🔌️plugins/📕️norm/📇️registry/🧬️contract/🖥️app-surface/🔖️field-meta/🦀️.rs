//! 🔖️ Field specifications of the norm app surface: typed unit-aware [`FieldSpec`] and the wildcard-aware path-template resolution. See [`super`].

use super::*;

//#region 🔖️ValuePath
pub use semio_s_plugin_norm_compliance::{Measure, UnitError, UnitFamily, UnitId, UnitSystem};

/// 🏷️ One enum/select option — wire `value` plus localized display labels families MUST supply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormFieldChoice {
    pub value: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
}

/// 🌐️ Const-constructible bilingual label; [`FieldLabel::localized`] lifts it into the framework label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLabel {
    pub en: &'static str,
    pub de: &'static str,
}

/// 📑️ Clause pointer that opens the clause panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClauseRef {
    pub family: &'static str,
    pub part: &'static str,
    pub section: &'static str,
}

/// 🏳️ National-annex parameter badge; the NA-DE value is the default and reset target. `id` is the registry id (`ndp.en1993.gamma-m0`) that binds the field to its row of the inspector's annex parameters section, which then offers a reset to the active annex value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NdpRef {
    pub id: Option<&'static str>,
    pub symbol: &'static str,
    pub clause: ClauseRef,
    pub default: Option<f64>,
}

/// 📏️ Closed numeric range in stored units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldRange {
    pub min: f64,
    pub max: f64,
}

/// 🪜️ Inspector disclosure tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Basic,
    Advanced,
    Expert,
}

impl Tier {
    /// 🪜️ Disclosure depth: a field shows when its rank is at most the window's.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Basic => 0,
            Self::Advanced => 1,
            Self::Expert => 2,
        }
    }
}

/// ⚙️ Comparison of a [`Predicate`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PredicateOp {
    Equals,
    NotEquals,
}

/// 👁️ Visibility rule: the sibling field at `path` compares to the wire `value`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Predicate {
    pub path: &'static str,
    pub op: PredicateOp,
    pub value: &'static str,
}

/// 🧭️ Coordinate axis of a handle hint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HintAxis {
    X,
    Y,
    Z,
}

/// 🕹️ How a field can be edited directly in the viewport.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandleHint {
    LengthAlong(HintAxis),
    Offset(HintAxis),
    Count,
    Radius,
    Angle,
    Thickness(HintAxis),
    Spacing(HintAxis),
    PlacementOrigin,
    DimensionLine,
}

/// 🔗️ Which checks a field edit invalidates; feeds the incremental planner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Affects {
    #[default]
    AllChecks,
    Entities(&'static [&'static str]),
}

/// 🧩️ Control kind of a field. `Angle` is stored in radians and shown in degrees, `Percent` is stored in percent, `Ratio` is a plain fraction. `Quantity` stores in `stored` and edits in `display` (empty `units` offers every unit of the family).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldKind {
    Inferred,
    Text,
    Int,
    Bool,
    Angle,
    Ratio,
    Percent,
    Choice(&'static [NormFieldChoice]),
    EntityRef { to: &'static str },
    Quantity { stored: UnitId, display: UnitId, units: &'static [UnitId] },
}

/// 🏷️ Typed, unit-aware, localized specification of one document field; path-template lookup via [`lookup_norm_field_meta`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldSpec {
    pub label: FieldLabel,
    pub help: Option<FieldLabel>,
    pub clause: Option<ClauseRef>,
    pub kind: FieldKind,
    pub read_only: bool,
    pub can_insert: bool,
    pub can_remove: bool,
    pub range: Option<FieldRange>,
    pub soft_range: Option<FieldRange>,
    pub step: Option<f64>,
    pub precision: u8,
    pub group: &'static str,
    pub order: u16,
    pub tier: Tier,
    pub visible_if: Option<Predicate>,
    pub ndp: Option<NdpRef>,
    pub handle: Option<HandleHint>,
    pub affects: Affects,
}

impl FieldLabel {
    pub const fn new(en: &'static str, de: &'static str) -> Self {
        Self { en, de }
    }

    pub fn localized(&self) -> LocalizedLabel {
        LocalizedLabel::native(self.en, self.de)
    }

    pub fn resolve(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => self.en,
            Locale::De => self.de,
        }
    }
}

impl FieldRange {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

impl FieldSpec {
    /// 🕳️ Neutral base for struct update: empty label, control inferred from the value.
    pub const EMPTY: Self = Self {
        label: FieldLabel::new("", ""),
        help: None,
        clause: None,
        kind: FieldKind::Inferred,
        read_only: false,
        can_insert: false,
        can_remove: false,
        range: None,
        soft_range: None,
        step: None,
        precision: 3,
        group: "",
        order: 0,
        tier: Tier::Basic,
        visible_if: None,
        ndp: None,
        handle: None,
        affects: Affects::AllChecks,
    };

    /// 🏷️ Labelled field whose control is inferred from the value.
    pub const fn new(en: &'static str, de: &'static str) -> Self {
        Self { label: FieldLabel::new(en, de), ..Self::EMPTY }
    }

    /// 📐️ Labelled quantity stored and edited in `unit`.
    pub const fn quantity(en: &'static str, de: &'static str, unit: UnitId) -> Self {
        Self { kind: FieldKind::Quantity { stored: unit, display: unit, units: &[] }, ..Self::new(en, de) }
    }

    /// 🔽️ Labelled choice field.
    pub const fn choice(en: &'static str, de: &'static str, choices: &'static [NormFieldChoice]) -> Self {
        Self { kind: FieldKind::Choice(choices), ..Self::new(en, de) }
    }

    pub fn label(&self, locale: Locale) -> &'static str {
        self.label.resolve(locale)
    }

    /// 🔤️ Stored unit symbol of a quantity field.
    pub fn unit(&self) -> Option<&'static str> {
        match self.kind {
            FieldKind::Quantity { stored, .. } => Some(stored.symbol()),
            FieldKind::Angle => Some(UnitId::Radian.symbol()),
            FieldKind::Percent => Some(UnitId::Percent.symbol()),
            _ => None,
        }
    }

    pub fn choices(&self) -> Option<&'static [NormFieldChoice]> {
        match self.kind {
            FieldKind::Choice(choices) => Some(choices),
            _ => None,
        }
    }

    /// 🎛️ Units the field offers: the explicit list or every unit of the stored family.
    pub fn units(&self) -> Vec<UnitId> {
        match self.kind {
            FieldKind::Quantity { units, stored, .. } if units.is_empty() => stored.compatible().collect(),
            FieldKind::Quantity { units, .. } => units.to_vec(),
            FieldKind::Angle => UnitId::Radian.compatible().collect(),
            _ => Vec::new(),
        }
    }

    /// 🖥️ Unit the field shows under `system`, if it is a quantity.
    pub fn display_unit(&self, system: UnitSystem) -> Option<UnitId> {
        match self.kind {
            FieldKind::Quantity { stored, display, .. } => Some(if system == UnitSystem::Si { display } else { stored.in_system(system) }),
            FieldKind::Angle => Some(UnitId::Degree),
            _ => None,
        }
    }

    /// ⬅️ Stored value to display value in `display`.
    pub fn to_display(&self, stored: f64, display: UnitId) -> Result<f64, UnitError> {
        match self.kind {
            FieldKind::Quantity { stored: unit, .. } => unit.convert(stored, display),
            FieldKind::Angle => UnitId::Radian.convert(stored, display),
            _ => Ok(stored),
        }
    }

    /// ➡️ Display value in `display` to the stored value, the only place a unit conversion happens before `setField`.
    pub fn to_stored(&self, shown: f64, display: UnitId) -> Result<f64, UnitError> {
        match self.kind {
            FieldKind::Quantity { stored, .. } => display.convert(shown, stored),
            FieldKind::Angle => display.convert(shown, UnitId::Radian),
            _ => Ok(shown),
        }
    }

    /// 📖️ Parses user text (bare numbers take `display`) into the stored value; callers check [`FieldSpec::accepts`].
    pub fn parse(&self, text: &str, display: UnitId) -> Result<f64, UnitError> {
        let measure = Measure::parse(text, display)?;
        self.to_stored(measure.value, measure.unit)
    }

    /// 🖨️ Formats a stored value in `display` with the field precision for `locale`.
    pub fn format(&self, stored: f64, display: UnitId, locale: Locale) -> Result<String, UnitError> {
        Ok(Measure { value: self.to_display(stored, display)?, unit: display }.format(self.precision, locale))
    }

    /// ✅️ True when the stored value is inside the hard range (or no range is declared).
    pub fn accepts(&self, stored: f64) -> bool {
        self.range.is_none_or(|range| range.contains(stored))
    }
}

/// 🛡️ Combines descriptive metadata with the family's native setter admission while preserving intentional read-only fields.
pub fn admit_field_meta(meta: Option<FieldSpec>, can_set: bool) -> Option<FieldSpec> {
    let spec = meta.unwrap_or(FieldSpec::EMPTY);
    Some(FieldSpec { read_only: spec.read_only || !can_set, ..spec })
}

/// 🧺️ List admission is independent of the containing field’s replacement capability.
pub fn admit_list_meta(meta: Option<FieldSpec>, can_insert: bool, can_remove: bool) -> Option<FieldSpec> {
    Some(FieldSpec { can_insert, can_remove, ..meta.unwrap_or(FieldSpec::EMPTY) })
}

/// 🏷️ Lookup hook consumed by [`resolve_field_meta`] (families usually wrap [`lookup_norm_field_meta`]).
pub type NormFieldMetaFn = fn(&str) -> Option<FieldSpec>;

/// 🏷️ Empty metadata table hook — wire `Some(empty_field_meta)` until a family fills its table.
pub fn empty_field_meta(_path: &str) -> Option<FieldSpec> {
    None
}

/// 🧩 Shared adapter for family metadata tables.
///
/// # Table format
///
/// Each entry is `(key, FieldSpec)` where `key` is a camelCase value-tree path template:
/// - Exact keys: `annex`, `site.sApR`
/// - List wildcards: replace every `[index]` with `[]`, e.g. `buildings[].storeys[].massKg`
/// - Nested list keys may nest wildcards: `footings[].loadCases[].verticalPermanent`
///
/// `FieldKind::Choice` holds a slice of [`NormFieldChoice`]: wire `value` plus `label_en`/`label_de`.
/// Families MUST supply localized choice labels — raw codes (or humanized codes) as the select label
/// alone are not acceptable.
///
/// Lookup order for a concrete path such as `buildings[0].storeys[id=S2].massKg`:
/// 1. Exact key match on the concrete path
/// 2. Exact key match on the `[]`-normalized template (`[index]` and `[id=…]` both become `[]`)
/// 3. Longest prefix at a `.` segment boundary (trying template form, then concrete), so a table
///    entry `buildings[].storeys[]` still labels `buildings[0].storeys[id=S2].massKg` when the leaf
///    key is absent
pub fn lookup_norm_field_meta(table: &[(&str, FieldSpec)], path: &str) -> Option<FieldSpec> {
    if let Some((_, meta)) = table.iter().find(|(key, _)| *key == path) {
        return Some(*meta);
    }
    let templated = index_wildcards(path);
    if let Some((_, meta)) = table.iter().find(|(key, _)| *key == templated.as_str()) {
        return Some(*meta);
    }
    longest_prefix_meta(table, &templated).or_else(|| longest_prefix_meta(table, path))
}

fn index_wildcards(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' {
            out.push_str("[]");
            for x in chars.by_ref() {
                if x == ']' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn longest_prefix_meta(table: &[(&str, FieldSpec)], path: &str) -> Option<FieldSpec> {
    let mut best: Option<(usize, FieldSpec)> = None;
    for (key, meta) in table {
        if path == *key || path.starts_with(&format!("{key}.")) || path.starts_with(&format!("{key}[")) {
            let rank = key.len();
            if best.as_ref().map(|(len, _)| rank > *len).unwrap_or(true) {
                best = Some((rank, *meta));
            }
        }
    }
    best.map(|(_, meta)| meta)
}

/// 🏷️ Resolves metadata for `path` via the family hook, applying `[]` normalization + longest prefix.
pub fn resolve_field_meta(path: &str, meta_fn: Option<NormFieldMetaFn>) -> Option<FieldSpec> {
    let lookup = meta_fn?;
    if let Some(meta) = lookup(path) {
        return Some(meta);
    }
    let templated = index_wildcards(path);
    if templated != path {
        if let Some(meta) = lookup(&templated) {
            return Some(meta);
        }
    }
    let mut prefix = templated.as_str();
    while let Some(cut) = prefix.rfind(['.', '[']) {
        prefix = &prefix[..cut];
        if prefix.is_empty() {
            break;
        }
        if let Some(meta) = lookup(prefix) {
            return Some(meta);
        }
    }
    None
}

pub fn field_label(path: &str, meta: Option<FieldSpec>, locale: Locale) -> String {
    if let Some(meta) = meta.filter(|meta| !meta.label(locale).is_empty()) {
        return meta.label(locale).to_owned();
    }
    path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']').to_string()
}
//#endregion 🔖️ValuePath

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
