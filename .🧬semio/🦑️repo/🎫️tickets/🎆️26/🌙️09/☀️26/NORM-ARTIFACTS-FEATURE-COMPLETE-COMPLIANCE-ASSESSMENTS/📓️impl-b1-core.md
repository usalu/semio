# Impl B1 — Core Assessment Model

Owner: Wave B agent B1. Families (Wave C) code against this API.

## Location

- Rust: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs` (re-exported as `semio_s_artifact_norm_contract::document`)
- Schema facets: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- TS export: `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/🟦️.ts` → `compliance_check_report`
- Fixture: `✏️s/🔌️plugins/📕️norm/⚖️compliance/🎫️fixtures/🔬️check-report/🔣️.json`

## Public API (exact)

```rust
pub enum CheckStatus { Pass, Warning, Fail, NotApplicable }

pub struct LocalizedCopy { pub en: String, pub de: String }
impl LocalizedCopy {
    pub fn new(en: impl Into<String>, de: impl Into<String>) -> Self;
    pub fn resolve(&self, locale: &protocol::Locale) -> &str; // protocol::Locale == ViewModel.locale
}

pub struct SubjectRef {
    pub entity_id: String,
    pub path: String,
    pub label: LocalizedCopy,
}
impl SubjectRef {
    pub fn new(entity_id: impl Into<String>, path: impl Into<String>, label: LocalizedCopy) -> Self;
    pub fn whole(label: LocalizedCopy) -> Self; // entity_id="", path=""
}

pub enum RemedyBound { AtLeast, AtMost, Exactly, OneOf }

pub struct Remedy {
    pub target: SubjectRef,
    pub current: Quantity,
    pub required: Quantity,
    pub bound: RemedyBound,
    pub options: Vec<String>,
    pub action: LocalizedCopy,
    pub applicable: bool,
}
impl Remedy {
    pub fn at_least(target: SubjectRef, current: Quantity, required: Quantity, action: LocalizedCopy) -> Self;
    pub fn at_most(target: SubjectRef, current: Quantity, required: Quantity, action: LocalizedCopy) -> Self;
    pub fn exactly(target: SubjectRef, current: Quantity, required: Quantity, action: LocalizedCopy) -> Self;
    pub fn one_of(target: SubjectRef, options: Vec<String>, action: LocalizedCopy) -> Self; // applicable=false
}

pub struct CheckResult {
    pub id: String,
    pub part: String,
    pub clause: ClauseId,
    pub subject: SubjectRef,
    pub status: CheckStatus,
    pub title: LocalizedCopy,
    pub explanation: LocalizedCopy,
    pub computed: Quantity,
    pub limit: Quantity,
    pub utilization: f64,
    pub annex: AnnexChoice,
    pub remedies: Vec<Remedy>,
}
impl CheckResult {
    pub fn assess(
        id: impl Into<String>,
        part: impl Into<String>,
        clause: ClauseId,
        subject: SubjectRef,
        title: LocalizedCopy,
    ) -> CheckBuilder;
}

pub struct CheckBuilder { /* opaque */ }
impl CheckBuilder {
    pub fn utilization(self, computed: Quantity, limit: Quantity) -> Self; // status from computed ≤ limit; u = computed/limit
    pub fn minimum(self, computed: Quantity, minimum: Quantity) -> Self;   // status from computed ≥ minimum; u normalized ≤1 when pass
    pub fn status(self, status: CheckStatus) -> Self;
    pub fn warn_above(self, threshold: f64) -> Self; // Pass with u > threshold → Warning
    pub fn annex(self, annex: AnnexChoice) -> Self;
    pub fn explanation(self, explanation: LocalizedCopy) -> Self;
    pub fn remedy(self, remedy: Remedy) -> Self; // repeatable
    pub fn not_applicable(self, reason: LocalizedCopy) -> Self;
    pub fn build(self) -> CheckResult; // debug_assert!: Fail ⇒ !remedies.is_empty()
}

pub struct PartVerdict {
    pub part: String,
    pub pass: u32,
    pub warning: u32,
    pub fail: u32,
    pub not_applicable: u32,
    pub worst_utilization: f64,
    pub complies: bool,
}

pub struct CheckReportSummary {
    pub total: u32,
    pub pass: u32,
    pub warning: u32,
    pub fail: u32,
    pub not_applicable: u32,
    pub worst_utilization: f64,
    pub complies: bool, // false iff any Fail
    pub parts: Vec<PartVerdict>,
}

pub struct CheckReport {
    pub summary: CheckReportSummary,
    pub checks: Vec<CheckResult>,
}
impl CheckReport {
    pub fn push(&mut self, check: CheckResult);
    pub fn extend(&mut self, checks: impl IntoIterator<Item = CheckResult>);
    pub fn complies(&self) -> bool;
    pub fn worst_utilization(&self) -> f64;
    pub fn failing(&self) -> impl Iterator<Item = &CheckResult>;
    pub fn by_part(&self) -> Vec<(&str, Vec<&CheckResult>)>;
}
```

Removed: `message: String`, `all_pass()`, `CheckResult::pass` / `fail` / `from_utilization` / `from_minimum`.

`NormFamily::evaluate` unchanged.

## Status rules

- `utilization(computed, limit)`: Pass iff `u = computed/limit ≤ 1` (limit≈0 → u=0).
- `minimum(computed, minimum)`: Pass iff `computed ≥ minimum`; utilization is `minimum/computed` when pass, else `computed/minimum`.
- `warn_above(t)`: after Pass, if `u > t` → Warning (still complies).
- `not_applicable(reason)`: status NA, explanation = reason, u = 0.
- Warning never breaks `complies()`; only Fail does.
- Fail must carry ≥1 remedy (`debug_assert!`).

## Conventions

- Derives: `Clone, Debug, PartialEq, value_derive::ToValue/FromValue`; serde behind `cfg(any(test, feature = "compliance-testing"))` with `rename_all = "camelCase"` on new multi-word records.
- `DslRecord` on `LocalizedCopy` / `SubjectRef`; `DslScalar` on `RemedyBound`; no `DslRecord` on types embedding `Quantity`.

## Tests run

```bash
CARGO_TARGET_DIR=/tmp/semio-norm-nfc-target cargo test -p semio-s-artifact-norm-contract --lib
# test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

bun test ./✏️s/🔌️plugins/📕️norm/⚖️compliance/🧪️tests/🔬️check-report-fixture/🟦️.ts
```

## Notes for Wave C

Construct every check via `CheckResult::assess(...).utilization|minimum|status|... .remedy(...).build()`. On Fail, always attach at least one `Remedy::{at_least,at_most,exactly,one_of}`.

## Notes for B2

Minimal compile shims were applied in `🖥️app-surface` (`message` → `title.en`, `all_pass` → `complies`, unit helpers → builder). Full Results UI redesign remains B2.
