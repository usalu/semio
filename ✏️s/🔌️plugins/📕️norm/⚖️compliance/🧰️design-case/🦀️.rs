//! 🗂️ Design cases: the domain-neutral bridge from a catalogue entry to an editable, evaluated document entity. An artifact declares a [`CaseCatalogue`] (entries, typed input vocabulary, evaluator); its snapshot stores [`DesignCase`] rows; this module searches the catalogue, seeds and validates cases and turns every case into [`CheckResult`]s with clause, utilisation, quantities and remedies. See [`super`] and `./🧬️schema/🔣️.json`.
use super::*;
use std::collections::BTreeMap;

//#region 🔖️Vocabulary
/// 🔽️ One selectable value of a choice input with its bilingual label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputChoice {
    pub value: &'static str,
    pub en: &'static str,
    pub de: &'static str,
}
impl InputChoice {
    pub const fn new(value: &'static str, en: &'static str, de: &'static str) -> Self {
        Self { value, en, de }
    }
}
/// 🧩 How an input is stored and edited; a vector input is stored as the number rows `key.0`, `key.1`, … and a series input as the numbered rows `h1`, `h2`, … (the key ends in the first number), both in the stored unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputKind {
    Number,
    Vector,
    Series,
    Choice,
    Text,
}
/// 🏷️ Typed definition of one case input: the stored unit of a number, the bilingual label, the bounds and the choices.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputSpec {
    pub key: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub scope: &'static str,
    pub kind: InputKind,
    pub unit: UnitId,
    pub range: Option<(f64, f64)>,
    pub lever: bool,
    pub example: Option<f64>,
    pub example_text: Option<&'static str>,
    pub choices: &'static [InputChoice],
}
impl InputSpec {
    const fn base(key: &'static str, en: &'static str, de: &'static str, kind: InputKind, unit: UnitId, choices: &'static [InputChoice]) -> Self {
        Self { key, en, de, scope: "", kind, unit, range: None, lever: false, example: None, example_text: None, choices }
    }
    pub const fn number(key: &'static str, en: &'static str, de: &'static str, unit: UnitId) -> Self { Self::base(key, en, de, InputKind::Number, unit, &[]) }
    pub const fn vector(key: &'static str, en: &'static str, de: &'static str, unit: UnitId) -> Self { Self::base(key, en, de, InputKind::Vector, unit, &[]) }
    pub const fn series(key: &'static str, en: &'static str, de: &'static str, unit: UnitId) -> Self { Self::base(key, en, de, InputKind::Series, unit, &[]) }
    pub const fn choice(key: &'static str, en: &'static str, de: &'static str, choices: &'static [InputChoice]) -> Self { Self::base(key, en, de, InputKind::Choice, UnitId::One, choices) }
    pub const fn text(key: &'static str, en: &'static str, de: &'static str) -> Self { Self::base(key, en, de, InputKind::Text, UnitId::One, &[]) }
    pub const fn range(mut self, min: f64, max: f64) -> Self { self.range = Some((min, max)); self }
    pub const fn lever(mut self) -> Self { self.lever = true; self }
    pub const fn example(mut self, value: f64) -> Self { self.example = Some(value); self }
    pub const fn example_text(mut self, value: &'static str) -> Self { self.example_text = Some(value); self }
    pub const fn scope(mut self, entries: &'static str) -> Self { self.scope = entries; self }
    /// 🎯 True when the row applies to the catalogue entry `entry_id` (an empty scope applies to every entry).
    pub fn applies_to(&self, entry_id: &str) -> bool { self.scope.is_empty() || self.scope.split(',').any(|id| id == entry_id) }
    pub fn label(&self, locale: &semio_framework_ui_locale::Locale) -> &'static str {
        match locale {
            semio_framework_ui_locale::Locale::En => self.en,
            semio_framework_ui_locale::Locale::De => self.de,
        }
    }
    pub fn is_number(&self) -> bool { matches!(self.kind, InputKind::Number | InputKind::Vector | InputKind::Series) }
    /// 🧭 True for inputs stored as several number rows.
    pub fn is_sequence(&self) -> bool { matches!(self.kind, InputKind::Vector | InputKind::Series) }
    /// 🧭 The number-row key of element `index` (zero-based) of a vector or series input.
    pub fn element_key(&self, index: usize) -> String {
        if self.kind == InputKind::Series { series_key(self.key, index) } else { element_key(self.key, index) }
    }
    /// 🧭 The zero-based element index when `element` is a number-row key of this vector or series input.
    pub fn element_index(&self, element: &str) -> Option<usize> {
        match self.kind {
            InputKind::Vector => element_index(self.key, element),
            InputKind::Series => series_index(self.key, element),
            _ => None,
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Entries
/// 🧊 What a case draws in 3D; the form fixes which solids stand for the case and how the declared inputs size them (see the projection kit `design_case::forms`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaseForm {
    /// 🧱 One solid of width × length × height (soil block, footing, building volume).
    Block,
    /// 📏 A horizontal member on two supports: length along X, width across, height up.
    Member,
    /// 🏛 A vertical member: width and depth in plan, height up.
    Column,
    /// 🟦 A thin plate or slab: width × length in plan, thickness up.
    Plate,
    /// 🧱 A wall: length along X, thickness across, height up.
    Wall,
    /// 🛢 A cylinder: width is the diameter, height up (silo, chimney, bar, specimen).
    Cylinder,
    /// 🔩 Two lapped plates with a fastener through them.
    Connection,
    /// ⬇ A loaded patch of width × length under a load bar.
    Load,
    /// 🎚 An abstract verification: a gauge pillar scaled by the load input against its reference mark.
    Gauge,
    /// 🔣 A criteria sign on a plinth for a pure rule or checklist case without any number to draw.
    Symbol,
    /// 🔩 A bolt, screw or nail group: two lapped plates (width × length × thickness) pierced by `count` fasteners of `diameter` at `spacing`.
    Fastener,
    /// 🔥 A fillet weld: a plate (width) with a stiffener on it joined by a bead of length `length` and `throat`.
    Weld,
    /// ⚓ A post-installed or cast-in anchor: a concrete block (width × length × height) with a headed anchor of `diameter` embedded `embed`.
    Anchor,
    /// 🕳 A slab (width × length × thickness) with a rectangular opening `open_w` × `open_l`.
    Opening,
    /// 🦶 A foundation pad (width × length × thickness) at `embed` below ground on soil `layers` with a groundwater table at `water`.
    Footing,
    /// 🏗 A pile (width = diameter, height = length) or, with `rows`, `columns` and `spacing`, a pile group under a cap, in soil `layers` and `water`.
    Pile,
    /// 🧗 A gravity or cantilever retaining wall: stem (width thickness, height) on a base (`base`, `base_t`, `toe`) retaining backfill with surcharge and `water`.
    RetainingWall,
    /// 📍 An embedded sheet wall (width thickness, height retained) with `embed` below the excavation level, a `prop` and `water`.
    SheetWall,
    /// ⛰ A slope or embankment profile (height, `angle` or `ratio`, `crest`, extrusion length) on a `soft` layer with `water`.
    Slope,
    /// 🌍 A soil block (width × length × depth) with stacked `layers`, a groundwater table `water` and an optional test `rod`.
    SoilBlock,
    /// ⚓ A ground anchor: bore of diameter width, fixed length `length`, `free` length, inclined by `angle`, behind a wall.
    GroundAnchor,
    /// 💧 A seepage path of length `length` through soil with a head difference `height` between two water levels, a cut-off of `embed` and a soil `layer`.
    Seepage,
    /// 🚰 A pumped well of radius `radius` in an aquifer (height) with the water level `level` in the well and the radius of influence (width).
    Well,
}
/// 🔗 One auxiliary driver of a [`CaseGeometry`]: a role of the form and the input key that sizes it; an empty key draws the role at its nominal size.
pub type AuxRow = (&'static str, &'static str);
/// 📐 What a case declares to draw: its form and the input keys that drive the dimensions (width, length, height; a length input of its vocabulary row each), the load bar (any number input) and the auxiliary roles of its form (`aux`); an empty key leaves the dimension to the nominal size of the form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaseGeometry {
    pub form: CaseForm,
    pub width: &'static str,
    pub length: &'static str,
    pub height: &'static str,
    pub load: &'static str,
    pub aux: &'static [AuxRow],
}
impl CaseGeometry {
    /// 🔎 The input key declared for the auxiliary `role`; `None` when the form does not draw the role, `Some("")` when it draws it nominally.
    pub fn aux_key(&self, role: &str) -> Option<&'static str> { self.aux.iter().find(|(name, _)| *name == role).map(|(_, key)| *key) }
    /// 🔑 Every input key the declaration names (width, length, height, load, auxiliary), without the empty ones.
    pub fn keys(&self) -> Vec<&'static str> { [self.width, self.length, self.height, self.load].into_iter().chain(self.aux.iter().map(|(_, key)| *key)).filter(|key| !key.is_empty()).collect() }
}
/// 📐 The geometry declaration of one catalogue entry, the schema-first row a generated table lists per artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeometryRow {
    pub entry: &'static str,
    pub geometry: CaseGeometry,
}
/// 🔎 The geometry declared for `entry_id` in `rows`.
pub fn geometry_in(rows: &[GeometryRow], entry_id: &str) -> Option<CaseGeometry> { rows.iter().find(|row| row.entry == entry_id).map(|row| row.geometry) }
/// 🗂️ One catalogue entry as the bridge sees it: identity, clause, bilingual title, declared input keys and optional geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaseEntry {
    pub id: &'static str,
    pub part: &'static str,
    pub clause: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub inputs: &'static [&'static str],
    pub geometry: Option<CaseGeometry>,
}
impl CaseEntry {
    pub fn title(&self, locale: &semio_framework_ui_locale::Locale) -> &'static str {
        match locale {
            semio_framework_ui_locale::Locale::En => self.en,
            semio_framework_ui_locale::Locale::De => self.de,
        }
    }
}
fn fold(text: &str) -> String { text.chars().filter(|c| !matches!(c, ' ' | '-' | '_' | '.' | '§')).flat_map(char::to_lowercase).collect() }
/// 🔎 Entries matching every whitespace-separated token of `query` in id, part, clause or the English or German title; exact id and clause prefixes rank first, the catalogue order is kept otherwise.
pub fn search(entries: &[CaseEntry], query: &str) -> Vec<CaseEntry> {
    let tokens: Vec<String> = query.split_whitespace().map(fold).filter(|t| !t.is_empty()).collect();
    let rank = |entry: &CaseEntry| {
        let id = fold(entry.id);
        let clause = fold(entry.clause);
        if tokens.iter().any(|t| *t == id) {
            0
        } else if tokens.iter().any(|t| clause.starts_with(t.as_str())) {
            1
        } else {
            2
        }
    };
    let mut hits: Vec<(usize, usize, CaseEntry)> = entries
        .iter()
        .filter(|entry| {
            let fields = [fold(entry.id), fold(entry.part), fold(entry.clause), fold(entry.en), fold(entry.de)];
            tokens.iter().all(|token| fields.iter().any(|field| field.contains(token.as_str())))
        })
        .enumerate()
        .map(|(index, entry)| (rank(entry), index, *entry))
        .collect();
    hits.sort_by_key(|(rank, index, _)| (*rank, *index));
    hits.into_iter().map(|(_, _, entry)| entry).collect()
}
//#endregion 🔖️Entries

//#region 🔖️Cases
/// 🧾 Typed input values of one case: numbers in the stored unit of their [`InputSpec`], texts as wire values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CaseValues {
    pub numbers: BTreeMap<String, f64>,
    pub texts: BTreeMap<String, String>,
}
impl CaseValues {
    pub fn with_number(mut self, key: &str, value: f64) -> Self { self.numbers.insert(key.into(), value); self }
    pub fn with_text(mut self, key: &str, value: &str) -> Self { self.texts.insert(key.into(), value.into()); self }
    pub fn with_vector(mut self, key: &str, values: &[f64]) -> Self { self.numbers.extend(values.iter().enumerate().map(|(index, value)| (element_key(key, index), *value))); self }
    pub fn with_series(mut self, key: &str, values: &[f64]) -> Self { self.numbers.extend(values.iter().enumerate().map(|(index, value)| (series_key(key, index), *value))); self }
    /// 📈 The vector input `key`: the number rows `key.0`, `key.1`, … up to the first gap; `None` when `key.0` is absent.
    pub fn vector(&self, key: &str) -> Option<Vec<f64>> {
        let values: Vec<f64> = (0..).map_while(|index| self.numbers.get(&element_key(key, index)).copied()).collect();
        (!values.is_empty()).then_some(values)
    }
    /// 📈 The series input `key` (`h1`): the rows `h1`, `h2`, … up to the first gap; `None` when the first row is absent.
    pub fn series(&self, key: &str) -> Option<Vec<f64>> {
        let values: Vec<f64> = (0..).map_while(|index| self.numbers.get(&series_key(key, index)).copied()).collect();
        (!values.is_empty()).then_some(values)
    }
}
fn digit_suffix(key: &str) -> (&str, Option<usize>) {
    let cut = key.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    (&key[..cut], key[cut..].parse().ok())
}
/// 🧭 The dotted number-row key of element `index` (zero-based) of the vector input `key`: `profile.0`, `profile.1`, ….
pub fn element_key(key: &str, index: usize) -> String {
    format!("{key}.{index}")
}
/// 🧭 The zero-based element index when `element` is a dotted number-row key of the vector input `key`.
pub fn element_index(key: &str, element: &str) -> Option<usize> { element.strip_prefix(key)?.strip_prefix('.')?.parse().ok() }
/// 🧭 The numbered number-row key of element `index` (zero-based) of the series input whose key ends in its first number: `h1` → `h1`, `h2`, ….
pub fn series_key(key: &str, index: usize) -> String {
    let (prefix, start) = digit_suffix(key);
    format!("{prefix}{}", start.unwrap_or(1) + index)
}
/// 🧭 The zero-based element index when `element` is a numbered number-row key of the series input `key`.
pub fn series_index(key: &str, element: &str) -> Option<usize> {
    let (prefix, start) = digit_suffix(key);
    let rest = element.strip_prefix(prefix)?;
    (!rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())).then(|| rest.parse::<usize>().ok()).flatten().and_then(|n| n.checked_sub(start.unwrap_or(1)))
}
/// 📇 One design case of a document: identity, bilingual label, the catalogue entry it verifies and its input values.
#[derive(Clone, Debug, PartialEq)]
pub struct DesignCase {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub entry_id: String,
    pub values: CaseValues,
}
/// 📚 The contract an artifact fulfils to expose its catalogue as design cases.
pub trait CaseCatalogue {
    fn entries() -> Vec<CaseEntry>;
    fn vocabulary() -> &'static [InputSpec];
    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult>;
}
/// 🔎 The catalogue entry with `id`.
pub fn entry<C: CaseCatalogue>(id: &str) -> Option<CaseEntry> { C::entries().into_iter().find(|entry| entry.id == id) }
/// 🏷️ The input definition of `key` for `entry_id`: a row scoped to the entry wins over the unscoped row.
pub fn spec<C: CaseCatalogue>(entry_id: &str, key: &str) -> Option<&'static InputSpec> {
    let rows = |name: &str| C::vocabulary().iter().filter(|row| row.key == name && row.applies_to(entry_id)).collect::<Vec<_>>();
    let pick = |name: &str| {
        let found = rows(name);
        found.iter().find(|row| !row.scope.is_empty()).or_else(|| found.first()).copied()
    };
    pick(key).filter(|row| !row.is_sequence()).or_else(|| C::vocabulary().iter().find(|row| row.is_sequence() && row.applies_to(entry_id) && row.element_index(key).is_some()))
}
/// 📝 The inputs an entry declares, in declaration order; keys without a vocabulary row are skipped.
pub fn declared<C: CaseCatalogue>(entry: &CaseEntry) -> Vec<&'static InputSpec> { entry.inputs.iter().filter_map(|key| spec::<C>(entry.id, key)).collect() }
/// 🌱 A new case of `entry` with the example values of its declared inputs and the entry title as label.
pub fn seed<C: CaseCatalogue>(entry: &CaseEntry, id: &str) -> DesignCase {
    let mut values = CaseValues::default();
    for row in declared::<C>(entry) {
        match (row.kind, row.example, row.example_text) {
            (InputKind::Number, Some(value), _) => {
                values.numbers.insert(row.key.into(), value);
            }
            (_, _, Some(text)) => {
                values.texts.insert(row.key.into(), text.into());
            }
            _ => {}
        }
    }
    DesignCase { id: id.into(), label_en: entry.en.into(), label_de: entry.de.into(), entry_id: entry.id.into(), values }
}
/// 📐 Width and height in metres of a case whose entry declares both keys, converted from the stored unit of each input; `None` without geometry, an undeclared or missing input or a non-positive size.
pub fn geometry_m<C: CaseCatalogue>(entry: &CaseEntry, values: &CaseValues) -> Option<(f64, f64)> {
    let geometry = entry.geometry?;
    let read = |key: &str| {
        let unit = spec::<C>(entry.id, key)?.unit;
        unit.convert(*values.numbers.get(key)?, UnitId::Metre).ok().filter(|metres| *metres > 0.0)
    };
    Some((read(geometry.width)?, read(geometry.height)?))
}
/// 🧭 Value-tree path of one case input row.
pub fn input_path(case_id: &str, key: &str, number: bool) -> String {
    format!("cases[id={case_id}].{}[id={key}].value", if number { "numbers" } else { "texts" })
}
//#endregion 🔖️Cases

//#region 🔖️Validation
/// ⚠️ Why an input of a case is not acceptable.
#[derive(Clone, Debug, PartialEq)]
pub enum CaseIssue {
    UnknownInput { key: String },
    WrongKind { key: String },
    OutOfRange { key: String, value: f64, min: f64, max: f64 },
    NotAChoice { key: String, value: String },
}
/// 🧪 Every issue of `values` against the vocabulary of `entry`; a key that is not a vocabulary row at all is unknown, a number stored as text (or the reverse) is of the wrong kind.
pub fn issues<C: CaseCatalogue>(entry: &CaseEntry, values: &CaseValues) -> Vec<CaseIssue> {
    let mut out = Vec::new();
    for (key, value) in &values.numbers {
        match spec::<C>(entry.id, key) {
            None => out.push(CaseIssue::UnknownInput { key: key.clone() }),
            Some(row) if !row.is_number() => out.push(CaseIssue::WrongKind { key: key.clone() }),
            Some(row) => {
                if let Some((min, max)) = row.range.filter(|(min, max)| value < min || value > max) {
                    out.push(CaseIssue::OutOfRange { key: key.clone(), value: *value, min, max });
                }
            }
        }
    }
    for (key, value) in &values.texts {
        match spec::<C>(entry.id, key) {
            None => out.push(CaseIssue::UnknownInput { key: key.clone() }),
            Some(row) if row.is_number() => out.push(CaseIssue::WrongKind { key: key.clone() }),
            Some(row) => {
                if row.kind == InputKind::Choice && !row.choices.iter().any(|choice| choice.value == value) {
                    out.push(CaseIssue::NotAChoice { key: key.clone(), value: value.clone() });
                }
            }
        }
    }
    out
}
/// 🧮 A remedy quantity of a case input: the value in the stored unit of the input (kind dimensionless), because `applyRemedy` writes `required` unchanged into the stored number row.
fn quantity(value: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, value) }
fn loc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }
/// ➖ The list row of the input a value path belongs to (`cases[id=c].numbers[id=k]`), the target of a row removal.
fn row_target(value: &SubjectRef) -> SubjectRef {
    SubjectRef::new(&value.entity_id, value.path.strip_suffix(".value").unwrap_or(&value.path), value.label.clone())
}
fn issue_result<C: CaseCatalogue>(entry: &CaseEntry, case: &DesignCase, annex: AnnexChoice, issue: &CaseIssue) -> CheckResult {
    let (key, number) = match issue {
        CaseIssue::UnknownInput { key } | CaseIssue::WrongKind { key } => (key.as_str(), case.values.numbers.contains_key(key)),
        CaseIssue::OutOfRange { key, .. } => (key.as_str(), true),
        CaseIssue::NotAChoice { key, .. } => (key.as_str(), false),
    };
    let row = spec::<C>(entry.id, key);
    let target = SubjectRef::new(&case.id, input_path(&case.id, key, number), loc(row.map_or(key, |r| r.en), row.map_or(key, |r| r.de)));
    let builder = CheckResult::assess(
        format!("case.{}.input.{}.{key}", entry.id.to_ascii_lowercase(), case.id),
        entry.part,
        ClauseId::new(entry.part, format!("§{}", entry.clause), entry.clause),
        target.clone(),
        loc(&format!("{} – input {key}", entry.en), &format!("{} – Eingabe {key}", entry.de)),
    )
    .annex(annex)
    .status(CheckStatus::Fail);
    match issue {
        CaseIssue::UnknownInput { .. } => builder
            .explanation(loc(&format!("Input {key} is not defined by the catalogue."), &format!("Die Eingabe {key} ist im Katalog nicht definiert.")))
            .remedy(Remedy::remove(row_target(&target), loc(&format!("Remove the input {key}; the catalogue does not define it."), &format!("Die Eingabe {key} entfernen; der Katalog definiert sie nicht."))))
            .build(),
        CaseIssue::WrongKind { .. } => builder
            .explanation(loc(&format!("Input {key} is stored with the wrong kind."), &format!("Die Eingabe {key} ist mit falscher Art gespeichert.")))
            .remedy(Remedy::remove(row_target(&target), loc(&format!("Remove the wrongly typed input {key} and enter it again with the kind the catalogue declares."), &format!("Die falsch typisierte Eingabe {key} entfernen und mit der im Katalog deklarierten Art neu eingeben."))))
            .build(),
        CaseIssue::OutOfRange { value, min, max, .. } => {
            let unit = row.map_or(UnitId::One, |r| r.unit);
            let below = value < min;
            let bound = if below { *min } else { *max };
            let action = loc(&format!("Set {key} to {bound} {}.", unit.symbol()), &format!("{key} auf {bound} {} setzen.", unit.symbol()));
            let remedy = if below { Remedy::at_least(target, quantity(*value), quantity(bound), action) } else { Remedy::at_most(target, quantity(*value), quantity(bound), action) };
            builder
                .explanation(loc(&format!("Input {key} = {value} is outside {min} … {max} {}.", unit.symbol()), &format!("Die Eingabe {key} = {value} liegt außerhalb {min} … {max} {}.", unit.symbol())))
                .remedy(remedy)
                .build()
        }
        CaseIssue::NotAChoice { value, .. } => {
            let options: Vec<String> = row.map(|r| r.choices.iter().map(|choice| choice.value.to_string()).collect()).unwrap_or_default();
            builder
                .explanation(loc(&format!("'{value}' is not an option of {key}."), &format!("'{value}' ist keine Option von {key}.")))
                .remedy(Remedy::one_of(target, options, loc(&format!("Select an option for {key}."), &format!("Option für {key} wählen."))))
                .build()
        }
    }
}
fn lever_remedy<C: CaseCatalogue>(entry: &CaseEntry, case: &DesignCase, utilization: f64) -> Option<Remedy> {
    declared::<C>(entry).into_iter().filter(|row| row.lever).find_map(|row| {
        let current = *case.values.numbers.get(row.key).filter(|v| **v > 0.0)?;
        let required = current * utilization;
        Some(Remedy::at_least(
            SubjectRef::new(&case.id, input_path(&case.id, row.key, true), loc(row.en, row.de)),
            quantity(current),
            quantity(required),
            loc(&format!("Increase {} to at least ≈ {required:.0} {} (first-order estimate, utilisation scales linearly).", row.en, row.unit.symbol()), &format!("{} auf mindestens ≈ {required:.0} {} erhöhen (Näherung erster Ordnung, Ausnutzung linear skaliert).", row.de, row.unit.symbol())),
        ))
    })
}
/// 🩹 The advisory remedy of a failing catalogue result no lever input could size: it names the declared inputs to revise and is not applicable as a one-click write.
fn revise_inputs_notice<C: CaseCatalogue>(entry: &CaseEntry, result: &CheckResult) -> Remedy {
    let keys: Vec<String> = declared::<C>(entry).iter().map(|row| row.key.to_owned()).collect();
    let list = keys.join(", ");
    let action = loc(&format!("Revise the inputs of this case ({list}) until the check passes."), &format!("Die Eingaben dieses Falls ({list}) überarbeiten, bis der Nachweis erfüllt ist."));
    Remedy { applicable: false, ..Remedy::one_of(result.subject.clone(), keys, action) }
}
//#endregion 🔖️Validation

//#region 🔖️Evaluation
/// 🧮 Evaluates one design case: input issues first, then the catalogue results re-addressed to the case row; failing results gain a sizing remedy on the first lever input that is set.
pub fn evaluate_case<C: CaseCatalogue>(case: &DesignCase, annex: AnnexChoice) -> Vec<CheckResult> {
    let label = if case.label_en.is_empty() && case.label_de.is_empty() { loc(&case.id, &case.id) } else { loc(&case.label_en, &case.label_de) };
    let subject = SubjectRef::new(&case.id, format!("cases[id={}]", case.id), label);
    let Some(entry) = entry::<C>(&case.entry_id) else {
        return vec![CheckResult::assess(format!("case.unknown.{}", case.id), "Design case", ClauseId::new("Design case", "§0", "0"), subject, loc("Unknown catalogue entry", "Unbekannter Katalogeintrag"))
            .annex(annex)
            .not_applicable(loc(&format!("No catalogue entry {}.", case.entry_id), &format!("Kein Katalogeintrag {}.", case.entry_id)))
            .build()];
    };
    let mut out: Vec<CheckResult> = issues::<C>(&entry, &case.values).iter().map(|issue| issue_result::<C>(&entry, case, annex, issue)).collect();
    for mut result in C::evaluate(&entry, annex, case) {
        result.subject = subject.clone();
        if result.status == CheckStatus::Fail && result.utilization > 1.0 {
            result.remedies.extend(lever_remedy::<C>(&entry, case, result.utilization));
        }
        if result.status == CheckStatus::Fail && result.remedies.is_empty() {
            result.remedies.push(revise_inputs_notice::<C>(&entry, &result));
        }
        out.push(result);
    }
    out
}
//#endregion 🔖️Evaluation
#[path = "📇️rows/🦀️.rs"]
mod rows;
pub use rows::*;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
