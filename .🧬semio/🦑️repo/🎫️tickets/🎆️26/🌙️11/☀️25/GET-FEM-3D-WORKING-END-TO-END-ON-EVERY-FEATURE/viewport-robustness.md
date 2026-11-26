# FEM 3D viewport robustness

Driver: own `vr-driver.mjs` (port 6250, `vr-d.sh`, helpers `vr-pre.js`, `vr-mem.js`), page `http://127.0.0.1:6087/`. Probe scripts in `🗑️generated/vr/*.js`. Laws: `vr-vitest.config.mjs` (3 files, 33 tests green). TypeScript fixes are served live by vite; the item-2 Rust fix needs the coordinator's wasm rebuild.

## 1. Pick tolerance (FIXED, browser verified)
Root cause: the 3 px padded screen pick works, but it runs on pointerup; the following click goes to r3f `onPointerMissed` (`handleEmptyClick`), because the real raycast does not hit a 3 px member at 2+ px. That handler found no hovered target and dispatched an empty `worldPick`/`interactionSelect`, erasing the pick just made. Only 0/+-1 px survived (raycast still hit).
Fix: a successful padded pick marks the following click as consumed (`wasMarqueeDragRef`, `👆️pointer-and-drop/🔸️handle-pointer-up`). Tolerance is now 4 CSS px; `resolveClickInstanceIdFromProjected` ranks boxes containing the click first, then nearest box edge (`world3dProjectedAabbGap`), then depth (`🎯️marquee-and-bridges`).
Browser (column x=540): dx 0, +-1, +-2, +-3, +-4, +-5 all resolve `col_20_l`; +6 misses.
Law: `🧪️tests/🌍️world3d-picking-and-previews/🔶️raycasts-z-0-ground` (gap, edge, nearest-wins asserts).

## 2. Stale window authority (FIXED: Rust root cause plus TS coalescing)
Cause: `setCamera` was only debounced, so several window-config commands were in flight at once; the second captured a root the first then advanced, and `admit_begin` (`🪟️window/🎚️config/🗄️store-owners/🦀️.rs`, `matches_committed`) turned that into a fatal `InvalidValue` refusal that killed the canvases.
Rust fix (needs the coordinator's wasm rebuild): the stale begin is now `stale_window_authority()` = `ValueRefusalKind::Canceled`; the begin site in `typed-operation-unit` maps exactly that kind to the shared `cancel_stale_publication` (extracted from the line-148 block, so both paths cancel identically as the benign `interactive-job.stale` terminal fault). TS: camera sync is single-flight, latest wins (`🧭️fit-and-presence/🔸️meshes-ref`), which also avoids the cancel in the first place; 4 rounds of orbit + 10 rapid wheel ticks kept both canvases.
Laws: `store_owners::stale_authority_law::a_superseded_window_root_is_a_cancellation_never_an_invalid_value` passes (`vr-law.sh`: `cargo test -p semio-plugin-window-config --lib`, private `C:	`, target = build dir, detached; `C:	rb` hit `os error 206`, the shorter dir passed). `cargo check -p semio-plugin-instance-app --lib` (`vr-check.sh`) compiles the `typed-operation-unit` change. A fuller law (`original_window_constructor_cancellation...` in `🪟️retained-window-config`, asserting kind, message and that inputs and actor stay untouched) is written but cannot run: the `semio-plugin-tests-19` crate does not compile at baseline (about 145 errors in `🪝️mounts/🌱️wp1-foundation/🧪️test-hosts`: missing `semio_framework_value_persistent` dependency, private `registry` crate, missing `CapturedWindowMutationIssuer`).

## 3. Undo refused `interactive-job.stale` (TS retry kept, not reproduced in the browser)
The item-2 root fix does NOT make this retry unnecessary: an undo's `interactive-job.stale` comes from the document-root freshness check (`typed-operation-unit` line 148, `typed_operation_document_is_fresh`), which was already the benign cancel and is what the user sees as a refused undo. The item-2 change only touches the window-config begin path. Removing the retry needs a Rust rebase of history moves onto the live root, which is not done; until then the retry is the only thing that makes an undo during a solve apply.
`retrySupersededHistoryAction` (`🚀️program-dispatch`): an undo/redo whose typed operation is cancelled with `interactive-job.stale` is re-issued (5 attempts, 25 ms backoff) against the new root in both the program and browser-actor routes. Law `🧪️tests/🔁️history-supersede` (3 tests). Assumes the stale fault is thrown to the dispatcher; confirm with an undo during a solve.

## 4. Renderer crash and memory (no leak found, crash not reproduced)
Boot + 20 interactions (pick, orbit, wheel), GC forced before each sample:
- JS heap 79 -> 85 MB (flat). DOM nodes 2059 -> 2047-2073, listeners 823 -> 832.
- Wasm linear memory (shard worker hook): 167 -> 170 MB, one instance only; of 4 shard workers just one loads `core.wasm` (140 MB). No growth.
- Renderer working set 970-1000 MB at boot, 1018-1114 MB during the run (plateau, +-50 MB). Workers constant (1 backbone + 4 shard).
So one tab holds about 1 GB (146 MB module, compiled code, 170 MB linear memory). With 5+ headless agent browsers of about 1 GB each on the box, `Target crashed` is most likely renderer OOM/pressure, not a leak. Mitigation if needed: lazy-compile or share the module, start idle shard workers on demand.

## Files
`🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx`, `🌐️World3dHost/👆️pointer-and-drop/🔸️handle-pointer-up/🟦️.ts`, `🌐️World3dHost/🧭️fit-and-presence/🔸️meshes-ref/🟦️.ts`, `🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🚀️program-dispatch/🟦️.ts`, Rust `🪟️window/🎚️config/🗄️store-owners/🦀️.rs`, `🧾️typed-operations/.../📬️typed-operation-unit/🦀️.rs`, `🪟️window/🎚️config/🧪️tests/🪟️retained-window-config/🦀️.rs`, tests `🌍️world3d-picking-and-previews/🔶️raycasts-z-0-ground`, `🔁️history-supersede` (new).
