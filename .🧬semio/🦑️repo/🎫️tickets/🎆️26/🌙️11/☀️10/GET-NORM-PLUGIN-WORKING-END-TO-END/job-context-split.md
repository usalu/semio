# job-context source budget split

## Finding
The proposed `source/`, `decode/`, `encoding/` split is not applicable: `decode/` (semio-framework-job-decode) and `encoding/` (semio-framework-job-native-encoding), plus `capture/` and `park/`, already are separate crates. `source/` cannot leave the crate: `StepContext` holds `&OriginalCallerSourceParent`, and the source tree constructs and mutates `StepContext` private state (child turns, borrow_context), so extracting it creates a crate cycle. The decoder owner, weak return, place (held by the native source macro) are fields/members of the same cycle.

## Split done (acyclic)
The original-return header protocol only consumes `StepContext` and sits above it, so it moved to the existing crate `semio-framework-job-return` (continuation/📤️owner):
- `begin/write/with_frozen/accept/close_failure/abandon/cancel_unexposed/finish` header (📤️header, finish_original_header moved there from continuation/source)
- header guard (🛡️guard) and header exposure (📮️exposure)

Source files stay at their domain paths and are mounted by `#[path]` from the new owner file.

Context changes (minimal API): `StepContext::original_return_budget{,_mut}()`, `receive_original_reserved_return`, `OriginalCallerSourceParent::return_source`, `OriginalCallerReturnPlace::qualify_preissued_budget`, `OriginalReturnSourceInput.{source,recipient}` and `post_publisher` made `pub`. The inherent impls on `OriginalReturnSource`/`OriginalCallerSourceParent` became free functions `check_original_header_guard(parent,..)` and `expose_original_header(parent,..)` (cross-crate inherent impls are impossible). No shims; context no longer exports the header API.

## Sizes (lib source closure, bytes, limit 262144)
- semio-framework-job-context: 261876 -> 236276 (20 -> 17 files)
- semio-framework-job-return: about 35000 -> 58651 (8 files)

## Dependents updated
- `semio-framework-job-return` Cargo.toml: deps job-return-escrow-publication, job-return-budget, value-control-contract.
- actor retained-turn context (`🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs`): calls `semio_framework_job::check_original_header_guard/expose_original_header`.
- All other users go through the `semio_framework_job` facade, which re-exports job-return (`pub use ...::*`).
- No `maxSourceBytes` value touched (budget unchanged).

## Verification
`check -p job-context -p job-return -p job --all-targets` with testkit features: exit 0. `check -p semio-plugin-instance-app --lib`: see result in chat. Pre-existing and unrelated: `semio-framework-job-testing-caller` lib tests (🧪️tests/🪪️qualification) do not compile at HEAD (private `cx.operation`, missing `original_failure_test_support`/value deps); not touched.
