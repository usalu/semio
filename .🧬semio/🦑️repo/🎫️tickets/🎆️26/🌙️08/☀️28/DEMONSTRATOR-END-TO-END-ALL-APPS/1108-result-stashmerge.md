# stashmerge: stash@{0} reject reconciliation (11-11 02:5x)

All 16 `.rej` files removed (`find` shows no `.rej`/`.orig` outside `.venv`).

| File (taxonomy path) | Verdict |
|---|---|
| ticket `1105b-claims.md` | integrated: all 14 stash log lines were missing; inserted after the `01:08 gen3dfinal2 EDITED-unverified instance-dispatch` anchor in stash order, no duplicates |
| value schema `🎛️drive.rs` | obsolete: hunks only add `[DEBUG]` stderr instrumentation |
| time-travel `turn-driving-and-replay-steps` | obsolete: current already has the effect-shaped `drive_original_discard_command(grant,effect)` and `acted`/newer receiving form |
| time-travel `owners-and-retirement` | obsolete: current has `owner.step(child.maximum_copy_bytes,..)`; the `drive_original_discard_command(grant,cx)` call site is superseded by the effect form |
| time-travel `supersede-authoring-and-extension` | obsolete: current uses `retired_poll`/`begun`/`receive_step` (newer port) |
| job-receiving `🏗️steps` | integrated: `born` now returns early only when `output.is_some()&&!admission.settling()` (custody2 production fix) |
| job-receiving `📥️pack`, `🔀️transaction`, `🧾️copies` | obsolete: the constructors now use `built_with` over `admit_original_construction_settled`, which settles itself; no `reset_step` early-return left |
| typed-ops `typing/🦀️.rs`, `typing/🎟️original`, `typing/🏷️tag`, `retirement`, `child-group`, `typed-operation-unit`, `inline-interaction` | obsolete: current paneA ports (`retired`, `begun`, `retired_poll`, `plugin_grant_funds`, `plugin_effect_close`, newer `with_native_receiving`) supersede the stash variants; remaining hunks were type annotations already present |

## Extra fix
`job/🏭️construction/🦀️.rs` defined `admit_original_construction_settled` twice (E0428; present in HEAD, not a stash artifact). Removed the older reserve+overhead copy, kept the `original_construction_settled_quote` version (which funds the full settled quote and settles a born candidate). `settling()` from the stash was already applied.

## Coordinator-listed post-merge reds (edited on disk 03:15, unverified until the labels below finish)
- job-child-vcs `📌️checkpoint/🪆️cascade:20`: `RetainedWorkPoll` now fully qualified (`semio_framework_value::RetainedWorkPoll`).
- composition-publication `🤝️group/🪆️child:81`: `PrivateChildMemberMetadataIssuer::advance` takes a grant; caller now passes its already-clamped local `grant` instead of the effect.
- document-media `🎞️media/💥️fault`: `MediaControlled`/`MediaRetireOwned` removed; the file uses `semio_framework_value::retirement::controlled::ControlledRetirement` and `retirement::RetireOwned` (the RetireOwned design).
- tool-machine `🔁️follow:72`: root cause was `OriginalWindowPreviewEpoch::publish_granted` demanding a `NativeConstructionScope` while every caller (follow, its own tests) holds a `NativeDecodeControl`; signature changed to `&mut NativeDecodeControl<'_>` (only ledger/recipient identity are read).
- Not touched: reactor-context `exports` (reactorturn owns it) and typed-operation-unit:29 (already `::<_,ValueError>` on disk).

## Labels
- `W-stashmerge-1` (native, `-p semio-framework-job -p semio-plugin-job-receiving -p semio-plugin-job`) and `W-stashmerge-hub` (wasm hub, reactor-context + reactor-wit-bridge): queued behind the fleet slots; read `$G/W-stashmerge-*.rc`/`.txt`.
- `W-stashmerge-1` finished rc101 at 03:4x, only in `semio-framework-value-snapshot-projection` (`📽️projection/📦️owned/🦀️.rs:14`, E0599 close_demands/close_work_demand unsatisfied bounds); that file was being edited by a peer at 03:37 (not mine, not a stash file). Errors in framework-job / job-receiving: none reached (blocked upstream). Rerun after that peer finishes: `"$C/1105b-tools/chk.sh" W-stashmerge-2 Cargo.toml native -p semio-framework-job -p semio-plugin-job-receiving -p semio-plugin-job`.
- `W-stashmerge-hub` still waiting at the capacity door (host swapping) when I stopped; rerun as `W-stashmerge-hub2` if no `.rc` exists.

STATUS: BLOCKED capacity queue + peer red in value snapshot-projection owned:14; all 16 `.rej` resolved and removed, edits on disk unverified
