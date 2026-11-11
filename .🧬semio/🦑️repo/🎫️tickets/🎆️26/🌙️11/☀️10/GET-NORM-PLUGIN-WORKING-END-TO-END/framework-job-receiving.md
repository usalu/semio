# semio-plugin-job-receiving migration

Crate: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving` (modules 📥️pack, 🧾️copies, 🔀️transaction, 🏗️steps, ⚠️failure). The brief's other module names (request, fields, lifted, media-export, registry) live in plugin decl-app, not here.

## Result
`repo cargo ... check -p semio-plugin-job-receiving --lib` passes with no errors and no warnings in this crate (log `🗑️generated/job-receiving-6.log`).

## Changes
- `🏗️steps`: now the single effect-based helper layer: `built`/`built_with` over `admit_original_construction_settled` (fresh admission, reports actual capacity), `retired` (effect closure, `close_original_retained_effect_step`), `retired_poll`, `begun` (`RetainedCloneClose::begin_granted`), `funded` (forward effect step with admission check and receipt posting), `replied` (reply-slot cursor calls), `receiver_close`, `receiving`, `receiving_primary`, `fixed`/`construction`/`receipt` (deduplicated from the three modules). Signature change of `retired` (grant closure to effect closure) affects its re-exporters `plugin/👷️job/🦀️.rs` and `🏛️reserved/🪆️child-vcs`.
- 🧾️copies: admission via `funded`; cursor via `advance_into`/`take`/`close_into`; close through `begun`/`step_granted`.
- 🔀️transaction: helpers and decoder identity type (`NativeAllocationIdentity`); close through effect steps.
- 📥️pack: `RetainedIntrinsicBody::new` needs an effect, so the original bytes stay in the boxed owner (`bytes`, `limits`, `maximum`) until a funded creation phase (11 for typed, 5 for intrinsic); receiving uses `with_native_receiving` + `source_witness` + `receive_original_receiving_work` (resumable `units`), `take` and receiver close through effects, parser steps through `funded`/`retired_poll`.
- Cargo: new dependency `semio-framework-action-bus` (for `admit_original_construction_settled`).
- Tests (`🧪️tests`) updated to the new retirement helpers; pack test asserts byte pointer identity on `bytes` after admit.

## Shared modules
None changed. Cargo.lock needs the new action-bus edge (refreshed by the tooling).

## Open
Pending (non-settling) replies of cursor/receiver closes are treated as errors or zero-progress retries (same limitation as the store record precedent).
