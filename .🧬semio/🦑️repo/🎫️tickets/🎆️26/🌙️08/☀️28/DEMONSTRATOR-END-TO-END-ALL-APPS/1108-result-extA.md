# Lane extA result
Crates: plugin-abi, ownership, original-job-ownership, operation, tool-run-operations, decl-tool-contract, interaction (wasm via flow list extension manifest).
Rerun: chk.sh W-extA-<n> "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🧩️component/📦️packages/🦀️rust/Cargo.toml" wasm -p <the 7 crates>; then 1105b-tools/value/ext.sh <round>.

## Baseline W-extA-1 (rc1): abi 20, original-job-ownership 2, operation 3, decl-tool-contract 14, interaction 24; ownership + tool-run-operations green on wasm.

## Edits (verified W-extA-8 rc0, wasm, all 7 crates 0 errors)
- operation: 📬️completion/♻️effects + ♻️vacant: inherent close_step(body,effect) + native_retirement_reply!, next_computational_work; vacant records its 1 work unit and reports Progress(released_bytes). ♻️cancellation/🎟️issuer: claim_close uses ControlledRetirement::step(body,effect) directly as the poll.
- original-job-ownership: 🔗️lease -> ArcLeaseRetirement (SharedControlledRetirement::lease removed).
- decl-tool-contract (lifted/👷️job + 👷️job/♻️alias): `built` replaced by OriginalStoreBirth::built (admit_original_empty takes the birth; capture caller updated); inference sessions, close_granted and alias_return_step effect-shaped; reserve_one_funded under cx.with_original_effect; alias cursor = inherent close_step + native_retirement_reply!, RetireOwned via retirement_seed.
- interaction: stray `effect:` parameters removed, SharedOriginalStorage from semio_framework_value::retirement::arc_custody::original.
- abi (🚪️io/🛂️authority): HostAllocation implements NativeAllocationAuthorityPort over the host operation_allocation import (Admit/Quote forwarded once per request, Post/Cancel settle without a host call, optional shared ledger); all four with_*_operation_authority run inside NativeAllocationAuthority::with_send_loan. 🪶️snapshot: authority persisted in SnapshotOperation (rebind checks its identity), sql control gets its own authority on the shared ledger, snapshot_close runs in an OriginalRetirementPump turn (owner-less close), Pending/denied keep receipts.
- LOCK NEEDED: abi Cargo.toml +semio-framework-job (logged).

## Open / debts
- ext.sh 1113 (stopped after 2/21): bim and brep rc1, both 'Rust crate Source refused: semio-plugin-window-config-original (Unbalanced Rust source delimiter)', i.e. extB2's in-flight window-config edit, no error in extA crates. Rerun after extB2 releases: ext.sh <round>.
- abi Cargo.toml +semio-framework-job: lock still to be run by the coordinator (the check passed with the current lock state).
- Debts: Quote/Admit forwarded to the host once per request (Post/Cancel settle locally); no wasm tests run.
## Round 2 (coordinator fixes), W-extA-10 wasm rc0 (abi, interaction, decl-tool-contract; ownership/operation/original-job/tool-run unchanged since W-extA-8)
- No leak: an authority with an open transaction at release is parked (thread-local, never dropped) and retried by `settle_parked()` at the start of the next forwarded operation, snapshot start and snapshot close; the finished operation's result is not turned into an error.
- No u64::MAX: `snapshot_close(ticket,grant,SnapshotCloseBudget{fuel,deadline_us,clock_binding})`; the pump's clock is `pure::operation_clock_us(clock_binding)`. Component export passes its WIT `actor-context-budget` (fuel, deadline_us, input.work.clock_binding). Owned export has no budget arguments, so `SnapshotCloseInput` gained `budget:Option<SnapshotCloseBudget>` (None for component/actor-input); owned close without it is refused (invariant). Host snapshot literals updated: owned passes Some(fuel, budget.deadline_us, budget.input.work.clock_binding) -- UNVERIFIED host compile, guestrt owns that file (claim logged).
- Macro (📣️macros/⚛️wp4-reactor) edited; verified only through the ext component checks, which are still blocked by extB2.

STATUS: FIXED-verified W-extA-10 (7 crates wasm rc0); extension reruns BLOCKED on extB2 window-config-original unbalanced delimiter
