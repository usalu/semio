# FEM window-config ownership and playback (editor crates, 2D + 3D)

## 1. window_config_ownership: FIXED (3D and 2D)

Root cause (found by a temporary [DEBUG] print of the wire head, removed again): the persisted history rows of a window-config store are `OpBinary` bytes (`dsl::variants_binary::encode_op`, first bytes `01 00 00 01 ...`), but the FEM declaration installed `ArtifactOperationWire::canonical().with_original_receiving()`, i.e. the JSON receiver that demands the `op` + canonical JSON header (`OriginalJsonReceiving`, header `b"op"`). Hence "original JSON operation wire header differs" (copied_bytes 3 = header 2 + phase 1). Energy only passes because it uses `original_pack_value_operation_receiving!`; puzzle windows and gisterrain declare `canonical().with_original_binary_receiving()` for exactly this encoding.

Fix: `with_original_binary_receiving()` in `native_operation_wire` of both 3D owners
(`🧊️3d/.../✏️editor/🎭️modes/✏️edit/🪟️windows/{🧱️model,📊️results}/🎚️config/🦀️.rs`).
2D had no native receivers at all (law failed earlier with "must declare its original native Pack receiver"); ported the 3D declaration to
`◻️2d/.../🪟️windows/{🧱️model,📊️results}/🎚️config/🦀️.rs` and `🧬️schema/🦀️.rs`: `OriginalArtifactPackReceiving` (via `store::os_store::codec_registry::native_snapshot`, no new deps), `ArtifactOperationTargets`, `native_operation_wire` (binary receiving + target), `native_pair_demands`/`admit_native_pair`, `DslReceiving` on the state, `NativeReceive` on patch/update/mutation/FemResultSourceChange.

Results: 3D `--lib window_config_ownership`: 3 passed. 2D `editor_app_2` `window_config_ownership`: runtime law + document-admission law pass (2 passed).

## 2. playback_presses_are_one_config_edit_and_never_a_history_row: NOT FIXED

Both 3D and 2D stop identically in `settle_registered_typed_operation` ("settlement stopped after 14862 turns", maintenance not empty, nothing pending, close owner `Begin`):
- 3D: demand copy 20456 / capacity 19328, remaining grant copy 20278
- 2D: demand copy 20136 / capacity 19008, remaining grant copy 19867

Findings:
- The stock grant is not a constant in the harness: it is `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` (copy 32768 per turn, `fixture_mounted_policy`) intersected with the context's remaining grant. 20278 = 32768 - 12490 (3D), 19867 = 32768 - 12901 (2D): the number is what is left of the 32 KiB turn after the preceding spend, evaluated by the refusal diagnostic, so the "grant" in the message is not an independent budget.
- Replacing the fixture renewal grant `cx.retained_grant()` by `cx.original_turn_ceiling().0`, even with 4x copy, did not change the numbers (the renewal goes through `renew_pinned_turn`, which clamps to `original_source_total_grant()`); reverted, no harness change left.
- The demand did not grow legitimately by a specific FEM change that I could identify: it is a ladder maintenance unit (`maintenance_ladder_demands`, `PluginAppDelegate`) of ~19-20 KB capacity that never completes while 12-13 KB of copy are spent each turn; it looks like a livelock/stage that cannot finish within one pinned return-source budget, not a fudgeable grant.
- Native header lifecycle gap (`"terminal ticket has no explicit native header lifecycle"`, `factory_ticket_demands` in `🌱️value/♻️retirement/🎟️tickets/🦀️.rs`): reached only with a larger grant (earlier round). Candidate owners lacking `native_header_release` (default `None`): `TimeTravelDiscardStoreEffects` (⏪️time-travel/🎮️decision/🗑️discard) and `CheckpointOutputRetirement` (🧠️interpreter/📸️checkpoint-codec), among ~40 impls without it. I could not reproduce the second error with the current grant to identify the actual type (a backtrace print in `factory_ticket_demands` is the way: `scripts fwc-dbg.py on|off` toggles it; currently OFF). Sibling pattern to copy: `fn native_header_release(&self)->Option<RetirementHeaderRelease>{self.terminal_is_empty().then(||unsafe{RetirementHeaderRelease::native(self,0,0)})}` (transient publication, interaction retirement).
- Next step: print the ladder unit that quotes 20456 B (stage `u64::MAX`, `maintenance_unit_demand`) and why its step makes no progress; then handle the header lifecycle for the owner that is then reached.

## Files changed
- 3D: `...🧱️model/🎚️config/🦀️.rs`, `...📊️results/🎚️config/🦀️.rs` (binary receiving).
- 2D: `...🧱️model/🎚️config/{🦀️.rs,🧬️schema/🦀️.rs}`, `...📊️results/🎚️config/{🦀️.rs,🧬️schema/🦀️.rs}` (port, written by `fwc-port-2d.py`).
- Framework: none left (temporary debug edits in `🎒️pack/🔤️json/📥️decode/🧳️receiving/🦀️.rs`, `♻️retirement/🎟️tickets/🦀️.rs` and `artifact-app-laws` settle file were reverted; verified no `[DEBUG]` left).

## Env notes
`C:\t\3d` and `C:\t\tmp` had disappeared mid-session (link error 1104); recreated and rebuilt cold. 2D ran in `C:\t\2dh`/`2dhb` per the coordinator. Logs: `🗑️generated/fwc-*.log`, `fwc2d-*.log`. Scripts: `fwc-3d.sh`, `fwc-2d.sh`, `fwc-2d-run.sh`, `fwc-dbg.py`, `fwc-port-2d.py`.
