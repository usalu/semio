# 🎛️ Result RENDERER-WP5: Shell use-case and view crates as borrowed-bundle impls (staged, spilled to budget, consumers rewritten)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool, no cargo compile, no stub harness. Plan: `PLAN-RENDERER-WGPU-CRATES.md` (7.2, 7.4, 7.5, 10.1, 10.4, Appendix B/C/D/F), hand-off `RESULT-RENDERER-WP4.md` (section 2 interface, section 3). Tools: `rwp5-tools/` (this ticket folder, re-runnable pipeline). Logs: `🗑️generated/rwp5-*`, working copies `🗑️generated/rwp5/{orig-shell,new-shell,new-ext,apply-state}`. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `SH` = `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu`, `STG` = `SH/📥️staging`.

## 0. Verdict

The eight WP5 crates (`shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`) are converted **inside the monolith, in their staging trees**: every use-case member of the 272 that WP4 left as `impl ShellState` is now a method of the crate's borrowed bundle (`SessionCtx`, `WindowActionsCtx`, `ActionsCtx`, `PointerCtx`, `ChromeViewCtx`, `OverlaysViewCtx`, `PanelsViewCtx`; `shell-view-paint` has no methods and no bundle), calls into lower use-case crates go through explicit bridges, all three over-budget crates were re-cut to at most 3,470 lines, and the 772 consumer call sites (tests of the Shell, Dock, IconRenderHost and root, plus five WP3 renderer files and `browser-worker`) use the new API. The patched full-resolution differential shows **0 differences** in native lib and wasm32 lib (section 6). **Nothing is compiled** (no rustc type check exists for these crates: the ingress gate refuses the frontier and the coordinator forbids stubs), so types, borrowck and privacy of methods rest on the static checkers of section 6, not on rustc.

**The crate flip is not done, and cannot be done yet.** The WP5 crates need `ShellState` and the fifteen component structs as crates; those are still staging modules inside the monolith (only `shell-vocab` was carved by WP4). A WP5 crate root that mounted its files today would name types that exist in no dependency, and the hub line `extern crate … as …;` would turn the whole Shell into an empty module in every peer's sandbox. The flip is therefore the mechanical last step of section 8 (it needs WP4's tier-1 crates first). Same finding as WP3 and WP4.

## 1. Design

1. **Bundles (plan 7.2.3).** Each bundle is `pub struct XCtx<'a> { pub chrome: &'a mut ChromeState, …, pub commands: &'a mut CommandState }` with the 15 composer fields (every WP5 crate touches or bridges to all 15 components, measured: `rwp5-bundles.json`; a narrower split is impossible because every higher crate calls `shell-session`), `XCtx::of(shell: &'a mut ShellState)` and one `as_<crate>(&mut self) -> LowerCtx<'_>` bridge per lower crate it actually calls (15 bridges, 270 bridged call sites). Bundles live in `STG/<crate>/🧺️bundle/🦀️.rs`. `impl ShellState` blocks became `impl<'ctx> XCtx<'ctx>` (the first build used `'a` and collided with the explicit `<'a>` of the 13 `ShellTurn<'a, _>` methods, E0496, caught by the oracle).
2. **No upward call, no extension trait, no wrapper.** The direction graph (`deps.py`: 625 nodes, methods and free items, impls folded into their types) has 0 references from a lower to a higher crate before and after the spill; the bridges only point down (asserted at generation and by `verify_static5.py`).
3. **`&self` use cases that bridge became `&mut self`** (a bridge needs `&mut`): `anchor_at_point`, `candidate_input_geometry`, `dock_tab_drop_index`, `open_floating_panel_rects`, `surface_overlay_controls` (view-chrome), `chrome_accessibility_nodes`, `footer_chrome_layout`, `footer_tab_row_rect`, `palette_accessibility_nodes` (view-panels).
4. **Self-less associated functions became free functions** (28: `same_session`, `panel_json`, `pointer_hit_owner`, `window_actions_rect`, `chrome_dialog_paint_ops`, …), callers rewritten (109 sites: `Self::f(` and `ShellState::f(` to `f(`).
5. **Free functions that took the whole `ShellState`** (the knot of plan 7.6): the four tutorial snapshot functions take `&mut WindowActionsCtx<'_>`; `chrome_tour_layout` and `chrome_tour_anchor_rect` take `&OverlaysViewCtx<'_>`; `render_tutorial_gesture_overlay_node` takes the four components it reads (`&ChromeState, &SurfaceState, &ShellDockState, &TutorialState`) because `shell-view-panels` calls it from above.
6. **Persisted dock load** (decision, WP4 residue): `ShellState::new` ended with `state.load_persisted_dock()`, an upward call into `shell-session`. The call is removed from the composer's `new` and is the first statement of `ActionsCtx::boot`. Both production paths (`browser-worker` step 5 to step 6, `native-socket-probe`) call `boot()` right after `new()`, and the dock store is keyed by the session app, which is absent at both points, so the observable order is unchanged for them; a test that calls `new()` and never `boot()` no longer sees a persisted dock (none does: no test reads `load_persisted_dock`).
7. **Borrow consequence of bridges** (the one behavioural-looking change): `self.as_x().m(args)` creates the receiver before the arguments, so an argument that borrows `self` conflicts where `self.m(args)` used two-phase borrows. The generator hoists a single argument that names `self` into a block (2 sites); 4 sites where a `&self.docs.session` borrow was alive across a bridged call take the controller id by value first (`detach_sync_backbone_internal`, `apply_ops_inner`, `handle_shell_hit`, `activate_find_item`); 6 test sites hoist screen sizes, body rects or a `pane_overlay_hits.clone()`. `borrowscan.py` lists what is left: 3 false positives (owned locals).

## 2. Size spill (plan 7.4; WP4 hand-off asked for it)

Moves are caller-closed (a moved member is called by no member left behind) and checked by the direction graph; they keep the enclosing `//#region` pairs balanced and move `impl` blocks of types together with the type. Plan in `rwp5-tools/moves.py`, 118 moves:

* `🎓️tutorial-runtime` slug (whole file, 520 lines) from `shell-session` to `shell-window-actions`.
* `shell-actions`: 19 members (the keyboard-and-palette group and `pump_sync_events`, …) up to `shell-pointer`; 50 free items (types, constants, pure helpers) down to `shell-window-actions`.
* `shell-view-chrome`: 48 paint free items (plus their `impl`s) down to `shell-view-paint`, which therefore is no longer empty.

| crate | files before | lines before | bytes before | methods before | files after | lines after | bytes after | methods after | free items after |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `shell-session` | 21 | 3628 | 218795 | 72 | 21 | 3159 | 190883 | 54 | 90 |
| `shell-window-actions` | 12 | 1518 | 98541 | 20 | 18 | 2713 | 165546 | 24 | 95 |
| `shell-actions` | 18 | 4639 | 294141 | 69 | 14 | 3377 | 217163 | 53 | 22 |
| `shell-pointer` | 8 | 2191 | 132221 | 22 | 14 | 3119 | 186670 | 41 | 33 |
| `shell-view-paint` | 0 | 0 | 0 | 0 | 9 | 493 | 26988 | 0 | 48 |
| `shell-view-chrome` | 11 | 3503 | 212599 | 44 | 9 | 3199 | 193429 | 46 | 43 |
| `shell-view-overlays` | 8 | 2360 | 148932 | 24 | 9 | 2453 | 151588 | 23 | 47 |
| `shell-view-panels` | 11 | 1755 | 99432 | 20 | 12 | 1867 | 104754 | 24 | 11 |

Method counts after include the bundle functions (`of`, 15 bridges); the 28 self-less functions are free items. All eight rows are inside 3,500 lines, 230,000 bytes and 27 files (the crate root and test stub are two more files; the largest is `shell-actions` at 3,377 lines). Counted by file length (every line, comments and blanks), `rwp5-tools/measure.py`; the gate's cfg-selected closure of the real crate roots cannot be measured before the flip, so the test closure (the `cfg(test)` mod and the companion bins) is not counted.

## 3. Consumers

* **Tests of the Shell element (744 call sites in 40 files)**, including the WP4-owned `docs-*` and `windows-*` bins' tests: `shell.m(…)` became `XCtx::of(&mut shell).m(…)`. Receivers are found as in WP4 (variables declared `ShellState`, helper results, `*.shell` fields); `&mut *x` is used for parameters typed `&mut ShellState`.
* **Other packages' files (28 sites in 7 files + 1 hand edit)**: WP3 renderer `mailbox-impl`, `native-socket-probe`, `frame-presentation`, `app-runtime`, the root test `wgpu-frame-job-unit`, `IconRenderHost` test `export-batch`, `Dock` test `wgpu-unit`, and `browser-worker` (`ActionsCtx::of(self.shell.as_mut().expect(…)).boot()`), written through `apply5.py` with a baseline guard; the qualified path `crate::shell::XCtx` is used outside the Shell tree. `Interpreter/🎯️chrome-ledger` calls methods of `UiEngine`, not of the shell (false positive of the name scan).
* The WP3 hand-off "re-point the 16 `winit_app::dispatch_normalized_event` calls of `🔬️wgpu-shell-input`" was already applied by the WP4 apply (16 calls use `crate::dispatch_normalized_event`, 0 old); WP3 may remove its `pub(crate) use` line in `winit-app`.
* Test bins: the tests are not moved (the owner mounts them at the flip). **Finding for WP0**: at least 15 of the tests Appendix D gives to the WP4 bins (`windows-1`, `windows-2`, `docs-1`, `docs-2`, `docs-3`: for example `wgpu-panel-anchor-model` 50 uses, `wgpu-time-travel` 83, `wgpu-window-pane-chrome` 27) call WP5 use cases, but those bins' frozen dependency sets stop at `shell-commands`/`shell-state`. At the flip either add the eight WP5 crates to those five bins (companions may depend on anything below them; `verify_static.ts` then needs the table updated) or move the tests to the `chrome-*`/`input-*` bins. The WP5 bins (`input-1..3`, `chrome-1..6`) are unchanged skeletons.

## 4. Files

Changed: 106 files in `SH` (staging of the eight crates, the parent `SH/🦀️.rs` mounts and `SHELL_SOURCE` for the eight dirs, 40 test files, one line in `STG/🐙️state/🧩️session-and-topology`), 27 files and 7 folders added (7 bundles, the `view-paint` files, moved slugs), 8 external files. Not touched: crate roots, manifests, registries, hub `🎛️wp5-shell-views` (still only the `settle_pump_tests` mount), `Cargo.lock`. New folders carry a taxonomy-unregistered emoji folder `🧺️bundle` per crate (same status as WP4's `🧬️component`).

## 5. Verification (commands run, real results)

| check | result |
|---|---|
| `verify_static5.py`: no leftover `impl ShellState` or `&ShellState` in WP5 files, every `self.x()` resolves to the crate's own method or an `as_` bridge pointing down, no unknown `self.field`, bundles carry 15 fields, `of` covers 15, every bridge used and declared, every staging file mounted exactly once and in `SHELL_SOURCE`, no mount of a missing file, regions balanced, docstrings, path length ≤ 240 bytes, budgets, `rustfmt +nightly --emit stdout` (skip_children) on 106 files | `FAILURES 0`, rustfmt failures 0 |
| `deps.py` direction graph over methods and free items | 0 upward references |
| `lossless5.py` (normalized line multiset of the eight staging trees before and after) | only the intended edits are lost (impl headers, bridge insertions, the 7 free-function signatures, the 6 borrow rewrites) |
| definition scan (`private fn`/`struct` referenced from another file) | 0 cross-file private definitions (WP4 widened; moves created none) |
| orphan scan (files without items, empty impls, empty regions) | 8 emptied files and 3 empty impls removed by `cleanup.py`; none left |
| patched full-resolution differential (`oracle5.py`, A = live tree with the new Shell element and the consumer overlay, B = live tree; extern crates empty, external globs neutralized) native lib | A 3647, B 3647, **onlyNew 0, onlyOld 0** (first run found 27: bare ctx names in 7 production files and the E0496 lifetime clash, both fixed) |
| same, `wasm32-unknown-unknown` lib | A 3455, B 3455, **0 / 0** |
| same, `--cfg test` and wasm32 `--cfg test` | see section 5.1 |
| `ts5.py` renderer readers | see section 5.1 |

### 5.1 Late results

(filled in below by the last run)

## 6. What this evidence does not cover

* **rustc never type-checked any of this.** Unchecked: argument types after `&mut self.<comp>` became `&mut &mut Comp` (relies on deref coercion at concrete parameters; a generic parameter would break), method-existence inside bodies the truncated typeck skips, E0502/E0499 where an argument holds a borrow of `self` across a bridge (`borrowscan.py` heuristics found six, fixed), temporaries of tail expressions in the two hoisted blocks (correct in edition 2021, wrong in 2024), and boxed `ShellTurn` futures that are stored instead of awaited in the same statement (scan: all 48 bridged `ShellTurn` calls are awaited in the same statement).
* The differential only cancels name resolution; it cannot see privacy of methods.
* Seeded sensitivity: see 5.1.

## 7. Residual risks and hand-offs

1. **Flip** (section 8) waits for the tier-1 crates. The `Cargo.toml` of each WP5 crate still lists the WP0 dependency set; the non-Shell edges the code uses at module level are in `🗑️generated/rwp5/needs5.json` (for example `shell-pointer`: dock, engine_canvas, interpreter, program_bridge, scenes; `shell-view-chrome`: agent_chat_panel, agent_presence, element_contracts, interpreter, scenes; root items `boot_*`, `spawn_app_task`, `renderer_worker_pool`, `action_args_json!` to `host-base`). Adding them changes the frozen table `verify_static.ts` checks; not done.
2. **Ports P1/P2** are still not inversions (WP4 lifted callers instead); `dispatch_action` (620 lines) and `refresh_ui_rendered` live in `shell-actions` and `shell-window-actions` as use cases and are reached through bridges.
3. **`ShellTurn` and `dispatch_action` recursion**: the boxed futures borrow the bundle (`'a`); callers must await in the statement that creates the bridge (true for all 48 sites today).
4. Edition-2021 tail-expression temporaries (two hoisted blocks) and `&mut &mut T` coercions are the first things a real compile may flag.
5. WP3: nothing outstanding on Shell views; `mailbox-impl`, `app-runtime`, `frame-presentation`, `native-socket-probe`, `browser-worker` and the root test now call `XCtx::of(&mut shell)`. `AppInteractionState.shell` is unchanged.

## 8. Flip procedure (mechanical, for the integration agent once tier-1 crates exist)

1. For each crate: lib root `T/<folder>/🦀️.rs` = docstring, explicit imports computed with `needs5.py` plus the tier-1 crates' re-exports, one `#[path = "../../🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/<dir>/<slug>/🦀️.rs"]` mount per slug (the files stay in place, WP1 precedent) with `pub use <module>::*;` where the staging mount has one, the test stub.
2. Hub `🎛️wp5-shell-views`: `extern crate semio_framework_os_renderer_wgpu_<suffix> as <old module>;` is not needed because the old module is the Shell parent; instead delete the eight `STG/<dir>` mount blocks from `SH/🦀️.rs` and `pub use` the crates explicitly (plan 10.4.4 item 3).
3. Add the lower WP5 and non-Shell edges to the manifests (`needs5.json`), update the frozen table, run `verify_static.ts`.
4. Move the test files to their bins and add the WP5 crates to the WP4 bins (section 3).

Reproduce: `python3 rwp5-tools/build.py` (snapshot to working copy), `verify_static5.py`, `oracle5.py <tag> --configs lib,wasm`, `apply5.py` (guarded against peer edits; applied once, state in `🗑️generated/rwp5/apply-state`).
