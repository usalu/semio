# Spec — Core Assessment Model (binding contract for Wave B and Wave C)

Owner of implementation: Wave B agent. Every Wave C family agent codes against exactly this API. If Wave B must deviate (compile constraints of `dsl`/`value_derive`), it updates THIS file first and states the change under "Changelog".

Location: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs` (re-exported through `semio_s_artifact_norm_contract::document`).

## Types

```rust
pub enum CheckStatus { Pass, Warning, Fail, NotApplicable }   // Warning = complies but advisory (e.g. u in (0.95,1.0], recommended-not-mandatory rule); never breaks compliance

pub struct LocalizedCopy { pub en: String, pub de: String }  // both mandatory, no default language
impl LocalizedCopy {
    pub fn new(en: impl Into<String>, de: impl Into<String>) -> Self;
    pub fn resolve(&self, locale: &<framework locale type>) -> &str; // exact match; en/de only languages supported by norm
}

pub struct SubjectRef {
    pub entity_id: String,   // stable id of the subject entity (e.g. "wall-north", "member-B12", "product-4711"); "" = whole subject
    pub path: String,        // camelCase snapshot path, dotted + [index], e.g. "elements[2].layers[1].thicknessM"
    pub label: LocalizedCopy // human label of the entity, e.g. "North wall / Nordwand"
}

pub enum RemedyBound { AtLeast, AtMost, Exactly, OneOf }

pub struct Remedy {
    pub target: SubjectRef,          // which subject field to change
    pub current: Quantity,           // current value of that field (SI, kind-tagged)
    pub required: Quantity,          // computed value that makes the check pass (analytic inversion or search), SI
    pub bound: RemedyBound,
    pub options: Vec<String>,        // for OneOf / discrete choices (e.g. "HEB 240", "C30/37", "U_w ≤ 1.1 glazing") — empty otherwise
    pub action: LocalizedCopy,       // full sentence incl. numbers + units, e.g. "Increase insulation of layer 2 from 80 mm to at least 124 mm." / "Dämmstärke von Schicht 2 von 80 mm auf mindestens 124 mm erhöhen."
    pub applicable: bool,            // true if `required` can be written straight into `target.path` by `applyRemedy`
}

pub struct CheckResult {
    pub id: String,                  // stable, unique within report, deterministic from subject+clause, e.g. "en1992.6.2.2.vrdc.member-B12"
    pub part: String,                // grouping key, e.g. "DIN EN 1992-1-1", "DIN 4108-2", "VDI 3805 Blatt 6"
    pub clause: ClauseId,
    pub subject: SubjectRef,
    pub status: CheckStatus,
    pub title: LocalizedCopy,        // what is verified, e.g. "Shear resistance without shear reinforcement"
    pub explanation: LocalizedCopy,  // why it passes/fails with the key numbers & formula reference
    pub computed: Quantity,
    pub limit: Quantity,
    pub utilization: f64,            // computed/limit normalized so that ≤ 1.0 complies (for minima: limit/computed)
    pub annex: AnnexChoice,
    pub remedies: Vec<Remedy>,       // MUST be non-empty when status == Fail (at least one concrete way to comply); MAY be non-empty for Warning
}

pub struct PartVerdict { pub part: String, pub pass: u32, pub warning: u32, pub fail: u32, pub not_applicable: u32, pub worst_utilization: f64, pub complies: bool }
pub struct CheckReportSummary { pub total: u32, pub pass: u32, pub warning: u32, pub fail: u32, pub not_applicable: u32, pub worst_utilization: f64, pub complies: bool, pub parts: Vec<PartVerdict> }

pub struct CheckReport { pub summary: CheckReportSummary, pub checks: Vec<CheckResult> }
impl CheckReport {
    pub fn push(&mut self, check: CheckResult);          // keeps summary in sync
    pub fn extend(&mut self, checks: impl IntoIterator<Item = CheckResult>);
    pub fn complies(&self) -> bool;                      // no Fail
    pub fn worst_utilization(&self) -> f64;
    pub fn failing(&self) -> impl Iterator<Item = &CheckResult>;
    pub fn by_part(&self) -> Vec<(&str, Vec<&CheckResult>)>; // stable part order of first appearance
}
```

All derive `Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue` (+ `dsl::DslRecord`/`DslScalar` where needed for export) and `serde` only behind `cfg(any(test, feature = "compliance-testing"))` — identical conventions to the existing types. `message: String` is REMOVED. `all_pass()` is REMOVED (use `complies()`).

## Builder (the only way families construct checks)

```rust
CheckResult::assess(id, part, clause, subject, title)   // -> CheckBuilder
    .utilization(computed, limit)       // status from computed ≤ limit
    .minimum(computed, minimum)         // status from computed ≥ minimum
    .status(CheckStatus)                // explicit (qualitative/data-conformance checks); computed/limit may be Quantity::new(Dimensionless, …)
    .warn_above(0.95)                   // optional: Pass with u > threshold becomes Warning
    .annex(AnnexChoice)
    .explanation(LocalizedCopy)
    .remedy(Remedy)                     // repeatable
    .not_applicable(reason: LocalizedCopy)
    .build() -> CheckResult             // debug_assert!: Fail ⇒ !remedies.is_empty()
```

`Remedy::at_least(target, current, required, action)`, `Remedy::at_most(…)`, `Remedy::one_of(target, options, action)` convenience constructors.

## Family contract additions

```rust
pub trait NormFamily {
    type Document; type Mutation;
    fn family_id() -> NormFamilyId;
    fn evaluate(document: &Self::Document) -> CheckReport;
}
```

Unchanged signature. Remedy application and field editing are GENERIC in `🖥️app-surface` (see below), so families need no extra trait methods: the app surface projects the document to its camelCase value tree, writes the value at `path`, decodes it back, and commits the family's existing `XMutation::from_snapshot` bundle.

## App surface (Wave B)

- New retained verbs (added to `NORM_RETAINED_TOOL_IDS`, bridge macro, publication contracts): `setField {path, value}`, `insertItem {path, index, value?}`, `removeItem {path, index}`, `applyRemedy {checkId, remedyIndex}`, `setLocale`-free (locale comes from `ViewModel`).
- Payload/artifact size bounds raised so a complete subject (hundreds of entities) fits; `setSnapshot` stays for bulk import.
- Inputs window = structured, schema-driven, localized property editor over the document value tree (sections for records, rows for list items, editable scalar leaves, add/remove for lists) — not JSON.
- Results window = grouped by part; per part a verdict header; per check: status chip (localized), title, computed vs limit with units, utilization; failing checks list remedies with an "apply" affordance when `applicable`.
- Inspection panel = full check card (subject label+path, explanation, quantities, remedies).
- Document panel = localized summary headline from `summary`.
- Viewer table = columns Part · Clause · Subject · Status · Utilization · Title · Remedy (first action), localized.
- All render helpers take the locale from the `ViewModel`.

## Changelog

- 2026-09-26 v1 coordinator.
- 2026-09-26 v1.1 agent B1:
  - `LocalizedCopy::resolve` takes `&protocol::Locale` (`semio_framework_os_kernel::Locale`, same axis as `ViewModel.locale`).
  - `Remedy::exactly(...)` added beside `at_least` / `at_most` / `one_of`.
  - `SubjectRef::new` / `SubjectRef::whole` constructors added.
  - `DslRecord` only on string-shaped types (`LocalizedCopy`, `SubjectRef`); `Remedy` / `CheckResult` / report rollups stay `ToValue`/`FromValue` only because `Quantity` is not a `DslField`. `RemedyBound` is `DslScalar`.
  - Serde (cfg test / `compliance-testing`) uses `rename_all = "camelCase"` on the new multi-word records.
- 2026-09-26 v1.2 coordinator — **path selectors (binding for all families + app surface)**:
  - A path segment may address a list element by position `[<index>]` OR by stable id `[id=<id>]` (e.g. `members[id=B1].actions[id=ULS-1].nEd`). Ids are the element's `id` field; they must be unique within the list and must not contain `]`, `.` or `=`.
  - Families SHOULD use `[id=…]` for every entity list in `SubjectRef.path` and `Remedy.target.path` (stable across `insertItem`/`removeItem` — event-sourced replays stay valid). `[<index>]` remains valid for anonymous value lists.
  - The app-surface path resolver (`parse_path` / `set_value_at_path` / field-meta lookup) MUST accept both forms; field-metadata wildcard `[]` matches both.
