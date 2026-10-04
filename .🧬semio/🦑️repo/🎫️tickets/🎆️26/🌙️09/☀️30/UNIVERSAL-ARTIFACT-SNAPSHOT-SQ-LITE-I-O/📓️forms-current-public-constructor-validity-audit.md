# Forms Current Public Constructor Validity Audit

Read-only source refresh, 2026-10-03. No edits to production/tests, Cargo, build or runtime execution. The unchanged thirteen-test Native selection remains Root-owned and pending. This report does not claim its assertions ran.

`R` means `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms`; `F` means `R/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## Concrete Public Construction Gap

`R/🦀️.rs:361–364` still derives children from arbitrary steps, clones the definition, and returns FormsSnapshot without invoking any validator. Wrong schema, empty/duplicate step/question IDs, blank question kinds, nonfinite min/max/step/vector values, reversed ranges, nonpositive increments, duplicate option/vector keys and non-object params can therefore be returned by the named public construction API even though persistence rejects them. This is genuine unchecked constructor creation, distinct from the fact that public fields permit struct literals.

No existing fallible snapshot constructor was found. Existing domain authorities are `F/🧬️schema/📝️definition/🦀️.rs:11–26` (`validate()->Result<(),String>`), snapshot `🧬️schema/📸️snapshot/🦀️.rs:71–81` (`validate()->Result<(),String>`), validated FromValue at lines45–66, and typed fallible native/SQL persistence. `schema::configured_dictionary` at `F/🧬️schema/🦀️.rs:107` returns `Result<FormDictionary,ValueError>` but only validates dictionary uniqueness, not the supplied full snapshot definition. Dictionary admission cannot substitute for snapshot construction admission.

## Full Signature Pair And Caller Treatment

Current:

```rust
pub fn forms_snapshot_with_state(schema: String, id: String, version: String, title: Option<String>, steps: &[FormStep]) -> FormsSnapshot
```

Recommended replacement, preserving the complete current argument set and typed refusal:

```rust
pub fn forms_snapshot_with_state(schema: String, id: String, version: String, title: Option<String>, steps: &[FormStep]) -> Result<FormsSnapshot, semio_framework_value::ValueError>
```

Build and validate FormsDefinition before deriving structure/results; map domain failure to InvalidValue; construct the full snapshot and invoke its existing validator before Ok. Do not retain an unchecked compatibility helper, silent empty replacement, or old signature. Production Default/empty-template callsites use deliberate `expect` with the fixed authored template invariant; callers supplied dynamic input propagate Result. Test fixtures unwrap only their known-valid authored input; rejection laws must assert the typed refusal rather than unwrapping.

## Actual Validated Domains

Definition step IDs are nonempty and unique; question IDs are globally nonempty/unique and kinds reject trimmed-empty text. Option value and vector field key identities reject empty/exact duplicate values. Identity checks do not trim names, so whitespace identities remain literal valid identities unless an explicit future domain decision changes this consistently. Numeric min/max/step are finite, bounds ordered, increment positive; vector optional numbers are finite. Absent optionals remain valid. Finite signed zero, subnormals and maximum finite values remain valid where ordering/positive-increment constraints permit them. Intrinsic default/params values retain full nine-kind words/octet/member-order semantics; params require an Object outer kind, not finite nested values.

Snapshot requires exactly `forms.form`, validates definition/responses and both owned child coordinate dialects. Snapshot id/version currently have no nonempty or trim predicate, and title has no additional content predicate. Do not invent these restrictions while closing the constructor gap. FormsResponse separately requires nonempty response ID/definition_version, timestamp <= 9007199254740991 and nonempty unique answer IDs (`F/🧬️schema/📨️response/🦀️.rs:25–31`).

FormDictionary validates only exact duplicate question identities, retaining arbitrary literal identities including whitespace and exact full intrinsic values. Its entry PartialEq now uses numeric variant/raw-word equality rather than Number's broad numeric equality. Recent PlaybookValues/FormDictionary changes do not close the snapshot constructor gap and must not motivate trimming or blanket String restrictions.

## Meaningful Test-First Laws

Mount a separate constructor boundary law without altering the unchanged13 selection. Hand-author invalidSteps cases covering each existing definition predicate, wrong schema, every nonfinite widget/vector branch, reversed bounds, zero/negative increment and duplicate identities. Assert current unchecked behavior as the authentic baseline, then require InvalidValue after replacing the signature. Pair constructor-refused definitions with existing native Text/Binary/SQL refusal; use the actual owner, not a synthetic validator-only object. Valid owner laws must compare constructor-produced full to_value trees/raw IEEE words and child coordinates after Native/SQL roundtrip. Include allowed whitespace IDs, NUL/Unicode strings, optional absence, finite signed zero/subnormal/maximum values, plus full intrinsic UInt64/Int64/NaN/infinity/octets/order/duplicate-member defaults and object params. This preserves the existing business guards while proving valid literal fidelity.

## Exact Current Caller Roster

Repository-wide rg found seventeen calls plus the single declaration; no TypeScript or outside-artifact calls. Three production callsites:

- `F/🧬️schema/🦀️.rs:61` — FormsArtifact Default, fixed empty authored state.
- `F/🧬️schema/🦀️.rs:112` — empty_forms_snapshot, fixed Inputs step.
- `F/🧬️schema/📸️snapshot/🦀️.rs:87` — FormsSnapshot Default, fixed empty authored state.

Fourteen test calls:

- `R/🧪️tests/🔬️unit/🦀️.rs:45`.
- `F/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs:48`.
- `F/👁️viewer/🎭️modes/👁️view/🪟️windows/▶️try/🧪️tests/🔬️unit/🦀️.rs:23`.
- `F/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs:32` — helper returning FormsSnapshot.
- `F/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs:15`.
- `F/✏️editor/🎮️commands/✅️submit/🧪️tests/🦀️.rs:9,46`.
- `F/✏️editor/❓️questions/📍️placement/🧪️tests/🦀️.rs:34,59`.
- `F/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️authoring/🦀️.rs:20`.
- `F/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🦀️.rs:208` — multiline helper returning FormsSnapshot.
- `F/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:42` — helper returning FormsSnapshot.
- `F/✏️editor/🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🧪️tests/🔬️unit/🦀️.rs:10`.

## Adjacent Result API Qualification

`prepare_response(definition:&FormsDefinition, values:&PlaybookValues, id:String, submitted_at:u64, definition_version:String)->Result<FormsResponse,Vec<FormsAnswerError>>` (`F/🧬️schema/📨️response/🦀️.rs:36–52`) validates visible answers but does not invoke definition.validate or final response.validate. Empty response ID/version or oversized timestamp can still be returned as Ok. Its real submit caller is `F/✏️editor/🎮️commands/✅️submit/🦀️.rs:30`; the other current caller is response events test:33. This is a separate creation-boundary concern, not evidence that existing Result APIs already guarantee full owner validity. If addressed, use a typed response-preparation error separating domain refusal from answer errors and update the actual submit match explicitly; do not disguise domain failure as empty answer errors.
