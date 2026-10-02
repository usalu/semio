# Raw Positive Failure And Closed Documentation Oracles

Read-only source diagnosis; no scanner execution, tests or compiler run. Root reports successful composite native execution and all original14scope rows succeeding, with all three new raw positives refused.

## Exact Common Cause

All three new raw fixture sources place their finite macro invocation inside println!'s token tree. `ordinary-template-raw-invocation` directly prints load!(...) and r#load!(...); `raw-template-and-builtin` does the same; `raw-metavariable-binding` prints load!(...). The scanner's deliberate opaque invocation containment check at discovery/🟦️.ts:6667 rejects a candidate whenever an enclosing paired delimiter follows `!`. That applies equally to println and unknown helper macros. Successful rustc expansion does not supply a closed scanner-owned println expansion contract.

The raw tokenizer at6408 is reached after raw-string prefix probing confirms no quote, and canonical identity is used throughout binding. No source-reading evidence suggests raw tokenization caused these three failures. Preserve opaque wrapper refusal. Hand-author the positive sources with plain `let first = load!(...); let second = r#load!(...); println!("{} {}", first, second);` and the corresponding single let for raw metavariables. Keep all expected inputs/output and hand-adjust exact authored offsets. Do not add a println exemption or expand a generic helper contract solely to make these fixtures pass. A separate native-success/refused println-wrapped raw call should retain the hostile boundary.

## Closed Documentation Oracle Shape

The authored references array and active rustc dependency inputs represent different facts. Add a closed independent nativeInputs string array to relevant fixture rows, or a separate closed documentation row owner with source, files, references, nativeInputs, native outcome and typed problems. Never derive nativeInputs from references for dormant cfg_attr. Every key should be required/closed according to the selected row variant; keep original core rows exact.

| Row | Authored gate fact | Native dep-info |
| --- | --- | --- |
| outer doc include_str | doc.txt | doc.txt |
| inner doc include_str | doc.txt | doc.txt |
| doc concat around include_str | doc.txt | doc.txt |
| active cfg_attr(all(),doc=...) | doc.txt | doc.txt |
| dormant cfg_attr(any(),doc=...) | other.txt | no other.txt |
| nested dormant cfg_attr doc | other.txt | no other.txt |
| doc literal spelling include_str! | none | none |
| comment spelling include_str! | none | none |

Materialize both doc.txt and other.txt in the positive rows; compare exact source offsets and relative definition-file origin. Add physical-deletion siblings: active doc deletion is native failure plus gate missing input; dormant doc deletion is native success plus all-config gate missing input. These are independent expected outcomes, not exceptions generated from row identifiers.

Opaque rows must assert typed unsupported-expression rather than claim a guessed dependency:

- Outer inactive `#[cfg_attr(any(), opaque(include_str!("absent.txt")))] pub struct Subject;` is native success, gate refusal.
- Inner inactive `#![cfg_attr(any(), opaque(include_str!("absent.txt")))] pub struct Subject;` must yield the same refusal. It currently bypasses the outer-only attribute guard.
- A doc wrapper provided by an owned tiny macro that consumes its include-looking argument and returns a string must be native success with no input dependency and gate refusal until its expansion identity is owned. Supply that actual macro/helper provider; do not merely spell an unknown active proc attribute and expect native success.

Keep the actual UI builder's two doc concat compile-fence inputs as real-owner regressions, preserving their fixture bytes. No fullpass claim is made here.
