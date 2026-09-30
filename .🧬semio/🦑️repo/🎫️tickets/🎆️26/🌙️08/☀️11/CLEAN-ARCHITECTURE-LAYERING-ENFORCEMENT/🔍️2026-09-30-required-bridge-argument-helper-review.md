# Required Bridge Argument Helper Review

This read-only review inspects the settled private `declared_bridge_arguments` helper in the framework plugin root, its neutral fixture, the Rust law, and the AJV oracle. It edits only this report. Native framework and app execution belong to the implementation owner and parent.

## Concrete Findings And Repairs Verified By Read-Only Review

1. The initial review found integer `2` and `3` expected payloads for typed `f64` number schemas, which would disagree with exact Rust `serde_json::Value` equality after serialization as `2.0` and `3.0`. The implementation owner corrected only those expected literals to `2.0` and `3.0`. A fresh fixture read confirms the corrected float encodings; exact equality and mathematical values are preserved.

2. The initial 12-case fixture omitted nullable scalar/default-null, required `Any` with a null default, required record with an object default, and optional-default coverage. The implementation owner authored five explicit cases, and the Rust adapter now applies each fixture's `nullable` flag. The reviewed final fixture has **17 cases: 13 supported and four explicit unsupported**. Its AJV oracle asserts **38** independent payload validations (13 positive representatives and 25 invalid payloads). Read-only recount agrees exactly. The new cases exercise nullable required Boolean with a non-null representative, nullable required Boolean with a declared null default, required `Any` with a null default, required record with `{}` default, and optional string with a declared default.

No remaining blocking source or fixture issue was found in this scope. The implementation owner's final framework native run is now independently verified below; this review does not infer runtime success from source inspection or the authored oracle count.

## Schema And Candidate Semantics

The helper processes only required arguments without declared defaults. A single-option string uses that declared option; other supported schema families reuse the existing generic `declared_argument_alternatives`. Every candidate is serialized and validated against `ActionArgDef::json_schema` by the existing first-party `OwnedJsonSchemaValidator`; only an admitted candidate is staged. The canonical schema projection carries option membership, string length/pattern, numeric hard bounds and exclusive bounds, vector length/component bounds, array item/length constraints, reference identifier type, and nullable unions. Candidate generation is intentionally narrower than every possible schema-valid value; if its two representatives violate the schema, the helper fails by argument name rather than invoking the action with invalid data.

Pattern `^q+$` and an exclusive `(2,3)` numeric range reject the available generic representatives as authored. A closed single-option choice now has a valid representative even though the existing pair generator cannot return two distinct values. Required records and `Any` without declared defaults return an explicit named error. Nullable unsupported types likewise do not gain an invented null representative: the unsupported case remains explicit until a declared default or supported representative exists.

`effective_action_args` merges all declared defaults after representative staging. Declared defaults, including null, are preserved without additional validation in this helper; their admission remains a declaration/schema responsibility. Nullable scalar schemas wrap their JSON schema in `anyOf` with null, while the generic non-null scalar candidate still must validate against the projected schema. Required `Any` with a declared null default is preserved; required records with declared object defaults are preserved. Optional arguments receive no invented representative and retain any declared default. These branches now have explicit neutral vectors and validator assertions; their final native execution is verified below.

## Ownership And API Scope

The new helper is a private function in the existing `artifact_app_laws` module, which is gated by `cfg(any(test, feature = "artifact-app-testing"))`. Its existing public assertion signature and callers remain unchanged. It adds no runtime dependency or public product type. The helper contains no app, geometry, kernel, action-id, domain-id, or widget-id specialization; `p8-alpha`/`p8-beta` are existing generic sample values. It adds no action skips. The caller retains the preexisting recognized framework-owned route exclusions and still requires every exercised app action to decode through `command_from_action` and round-trip `command_id`. A staging error causes an explicit assertion failure, not omission from the action roster.

## Independent Oracle Structure

The Rust fixture law compares the staged payload exactly with its authored expected object, validates that payload against its independently authored neutral JSON schema, and requires each authored invalid payload to be rejected. Its four unsupported rows require a named helper error. The AJV oracle validates the independent expected payloads and all negative payloads for the 13 supported rows, and asserts a hard count of 38. It does not imitate the helper algorithm or generate its expected outputs from the implementation. Unsupported helper policies have no successful payload for an AJV admission check and remain covered by the Rust error assertions. Both plugin test and canonical routes invoke this oracle.

The implementation owner reported an earlier native RED for required texts: actual `{}` versus the three-string expected payload. The settled helper addresses that specific missing-required-input trigger. The final receipt group `🗑️generated/declared-bridge-final/exact-cargo-laws-hNUuHD/00` was independently read: build and native list exited zero; the new fixture law executed with `--exact`, exited zero, and reported one passed, zero failed, zero ignored. All 12 individual native law outputs report one passed, zero failed, zero ignored. The executable receipt SHA-256 is `78bd314ca3a669b5bf7744145ba263a4e7501da47a313ec45a2431c5ceb4e96a`. The owner records an uncached canonical Nx run in 43.2 seconds with 38 AJV assertions and the existing extension-retirement 13/source-freshness 17 oracle cases in `🛠️2026-09-30-required-bridge-arguments.md`.

This review verifies the final 12-law framework native receipt and does not claim the concrete 275-law app group or full 355-law aggregate has passed. The later document-to-mesh app failure requires its own supplied geometry boundary repair.

## Final Reviewed Snapshot Paths And Digests

| Path | SHA-256 |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `373016404d5606a4735a36f8276e2a29db1d114b617fc7c0f117b24b4cf09350` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🎮️declared-bridge-arguments.json` | `f00b44f1debd2a80d39ffabfe3e247aac36787d27b63428debb5662b91c0e092` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🦀️.rs` | `ef073df600fac7f86848cf461e4d9f31c3752baae10e83105305dba316aadbf0` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts` | `dddb1b4a50ef727ca63bb82c33239c626ad2f5e3a426f5ccb7897e233b8c7a61` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` | `27e555ac06dfcf64f237ab11df8f890fb9af45247c45f8d340070a576dcbf898` |
