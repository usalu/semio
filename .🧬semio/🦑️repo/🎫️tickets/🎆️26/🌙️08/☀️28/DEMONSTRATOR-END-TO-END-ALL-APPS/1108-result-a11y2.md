# 1108 a11y2 result

## Verdicts so far (11/10 00:12)
- check.py rerun: `python3 -I 1107-a11y/check.py` -> violations: 0.
- Zero compile errors mention LocalizedLabel/Locale/MediaPortSpec/terminology in any run below: no a11y-caused red found. All reds are the 733 effect-contract / output-slot sqlite migration debt.
- W-a11y2-3 (semio-plugin-app-builder, wasm) rc1: `semio-plugin-decl-views` 237 errors (RetireOwned/RetirementCursor trait shape: `plugin/declaration/view/{child/**,tool-run,operation-contexts,presence,transient}`), `semio-plugin-window-config-original` 7 errors (epoch + capture, effect shape). Not a11y. extA claimed only decl `{job, view/lifted/job}`; nobody claims the rest of decl-views: coordinator call needed (237-error port).
- W-a11y2-4 (decl-surface) rc1: same decl-views 237 + `semio-plugin-window-transient` 4.
- W-a11y2-5 (host-workflow) and W-a11y2-6 (semio-os) rc1: `semio-framework-artifact-space-collection` (50 errors: body-receiving OwnedFieldReceiving shape, sqlite snapshot host old 4/5/6-arg trait shape, `artifact_persisted_binary_operation` missing in kernel) and `space-space` (12). Same output-slot sqlite port I did for workflow-snapshot (`ArtifactSqliteSnapshotHost`: pre-checks `NotStarted`, `decode/encode_sqlite_snapshot_record_native(.., output)`, drop validate overrides). Unclaimed; not a11y.
- Pane crates W-a11y2-9/10/11: invalid runs (the predecessor's run4.sh used root `Cargo.toml -p <pane>`, which does not contain the panes; each pane has its own `.../📦️packages/🦀️rust/Cargo.toml`). Rerun correctly via `1108-a11y2-run.sh`: W-a11y2-18 norm app-surface, W-a11y2-19 wgpu shell-session (navbar locale change), W-a11y2-20 hub-space-app, W-a11y2-21..28 the 8 panes in order (generation3d, cad, puzzle-3d, energy-model, sourcing, process3d, gis2d, fem-3d). Results in `$G/W-a11y2-<n>.rc/.txt`, log `$G/a11y2-chain.log`. These were queued behind capacity at the time of writing.
- Process notes: killed the predecessor's stale `W-a11y-ts-1` (3.6 h stuck in graph "Verifying input closures"). My own TS run crawled the same way (800/2815 closures in 20 min under load 20+); I cancelled it to free the capacity queue. Rerun when the machine is calmer:
  `"$C/1105b-tools/slot.sh" W-a11y2-ts-2 env SEMIO_LANGUAGE=en repo run @semio-tech/mit-bestand-demonstrator:test`
  renderer suite: `repo run @semio-tech/framework-renderer-react:test` (vitest config includes `engineSuite("📋️renderer-types-and-forms")`, args pass through to vitest).

## Later results (11/10 00:51)
- W-a11y2-19 (wgpu shell-session): rc1, all errors upstream: `semio-plugin-window-transient` 31, `semio-plugin-decl-views` 69, `semio-ui-wgpu-engine` 5, `semio-os-infinite-board-dag-host` 6, `semio-renderer-wgpu-host-base` 28 (unresolved `wasm_bindgen`/`web_sys`/`js_sys`/`semio_framework_async`: dependency edges missing in its manifest/lock, not a11y). The shell-session crate itself (navbar `self.prefs.active_locale()`) was never reached, still uncompiled.
- W-a11y2-21 (generation3d pane): rc1 on decl-views (now 3 errors), window-transient 31, infinite-board-dag-host 6. Panes 22..28 follow the same chain; not waited for.
- W-a11y2-18 (norm app-surface): rc1: decl-views 3, window-transient 31, decl-staged-form 16.
- decl-views dropped 237 -> 69 -> 3 during this session (peers are porting it). Remaining gate for a11y re-verification: window-transient (31), decl-staged-form (16), board-dag-host (6), ui-wgpu-engine (5), host-base deps, space-collection/space-space.

## Open
1. W-a11y2-18..28 results (norm, wgpu shell session incl. navbar `self.prefs.active_locale()`, hub, 8 panes).
2. Two TS tests not run.
3. Unclaimed non-a11y reds blocking the closure: decl-views (237), window-config-original (7), window-transient (4), space-collection (50) + space-space (12). a11y re-verification of builder/decl-surface/host-workflow/semio-os/hub is impossible until these are green.

STATUS: BLOCKED a11y closure (builder, decl-surface, norm, wgpu shell, panes) behind non-a11y plugin-tier reds (window-transient 31, decl-staged-form 16, decl-views 3, board-dag-host 6) and host-workflow/semio-os behind space-collection (sqlite port); no a11y-caused red seen; hub W-a11y2-20 and panes 22..28 unread; TS tests not run; check.py 0 violations
