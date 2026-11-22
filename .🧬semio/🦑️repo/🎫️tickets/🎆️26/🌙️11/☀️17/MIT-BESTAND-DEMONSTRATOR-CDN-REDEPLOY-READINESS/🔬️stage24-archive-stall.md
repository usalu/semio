# Stage 24 archive load stall (wd3qizX)

Status: root cause NOT yet proven statically; witness landed so the next browser probe names it.

## What the static trace establishes

- `LAST_MAINTENANCE_STAGE=24` is weak evidence. After stage 24 runs the cursor becomes 25, so in every all-empty pass stage 24 is visited last. It proves only that a live, non-terminal, non-merge-waiting archive load exists (stage 24 quotes no demand otherwise). The earlier `AwaitingInput` fix for terminal/merge-waiting loads is in place, so the stalled load is in a runnable phase.
- The pass result is `Progress(empty)`, so no unit returned `Blocked`/`AwaitingInput`, and stage 14 (replacement), 15 and every other unit also returned empty or had no demand.
- Archive phases that return empty progress: `DecodeParent` (decoder Blocked: work quote > `cx.fuel_remaining()` in `StepTurn::admits`), `HydrateParent` (paged parent not receiver-ready), `AwaitingMembers` with replacement `Initializing`, `FillMember` (grant < page), `AwaitReplacement` (replacement Pending/Progress, or `acknowledge` refused because `terminal_is_empty` is false).
- Replacement states that return empty: any unit whose quote is not funded by the maintenance grant (`plugin_turn_admitted` / `original_lifecycle_step` return `Progress(default)` silently, forever, if the quote exceeds the policy grant on any axis); initializer `Submitted|Idle`; `AwaitingMembers` with `member_ingress` set.
- A replacement in `Complete` that is not `terminal_is_empty` is never retired in live mode (`store_replacement_target` skips Complete, the `Complete` arm returns `Complete(empty)` unless closing), so `AwaitReplacement` would loop on a refused `acknowledge`. No code path found that produces such a state.
- Typed publication (`advance_mounted_genesis_replacement`) returns `Ok(())` without moving while the genesis replacement is Pending/Progress; its driver is stage 14 plus the alternating `mounted_genesis_turn`. Pending effects and mounted-job maintenance wait on `document_blocks_pending_effects`, so any live archive load/replacement that cannot finish also starves `setContributions`/`setActiveExample` publication: both symptoms share one stuck load.
- No fix was applied: relabelling any of these waits as `AwaitingInput` would hide a real deadlock, and a funding shortfall needs the quote vs grant numbers first.

## Witness landed (source)

`[DEBUG] FEM cleanup semantic failure ...` now appends `archive-load: grant=... archive_target=... replacement_target=... waits_for_host=... [archive op state phase target merge terminal awaits_merge replacement members demand funded; replacement op state refusal cancelled committed mounted_genesis member_ingress worker outcome_pending resume_pending terminal_target terminal_empty demand funded]`. `funded=Some(false)` names an unfundable quote; a `Pending` replacement state with `funded=Some(true)` names an unpolled owner.

Files: `plugin/🪜️maintenance-ladder/🦀️.rs` (`LadderRef::document_load_diagnostic`), `plugin/⚛️reactor/🔌️app/🦀️.rs` (trait `PluginApp::document_load_diagnostic`, default empty), `plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs` (impl), `plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs` (debug line).

Evidence: `repo cargo check -p semio-plugin-instance-ladders -p semio-plugin-instance-app -p semio-plugin-runtime-instances --lib` (prototype, shared, -j2) passes. No native reproduction: the fixture laws drive archive loads to completion with fresh grants each turn; a stall needs the browser's grants/turn pacing.

## Next

Rebuild Demonstrator, rerun the wd3qizX probe, read the `archive-load:` field, then fix the named owner with a red native law.
