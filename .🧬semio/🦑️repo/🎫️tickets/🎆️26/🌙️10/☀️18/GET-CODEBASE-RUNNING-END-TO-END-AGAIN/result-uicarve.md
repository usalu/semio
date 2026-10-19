# Result uicarve (executor `uicarve`): `semio-framework-ui` carved, every feature closure inside the source gate

Sonnet 5.5, foreground, no sub-agents, no cargo/`bun repo`/nx, no modifying git command. `C` = this ticket folder, tools in `C/uicarve-tools/` (`PLAN.md` = working notes, `ledger.json` + `orig/` = first-touch backups of every file the tools modified, `lists/` = rule tables, reports and censuses). Nothing was compiled: evidence is static (canonical closure counter, typeck/privacy oracle, rustfmt parse, manifest/TOML/JSON parse, path existence, acyclicity).

## 0. Verdict

- The real gate for every crate is **4,000 active lines / 32 files / 262,144 bytes** (`Math.min(policy, crate metadata)`, so the 12,000-line override of the ui crate never applied). Before: the all-features closure of `semio-framework-ui` was 258 files / 56,932 lines / 3.5 MB (lib 98 files / 33,833 lines). After: `semio-framework-ui` is a 2-file crate (28 lines) and **32 new crates** hold the former tui, wgpu-light and wgpu-engine trees. **Worst closure of any crate over all feature sets, lib and test: 0.892** (`ui-tui-core` test closure; worst lib closure 0.840, `ui-wgpu-events`). Full table in section 2.
- The three feature trees are no longer features: `tui`, `tui-terminal`, `tui-bindgen`, `wgpu`, `wgpu-engine`, `testkit`, `typegen` are gone from `semio-framework-ui`; `native`, `bindgen` and `testkit` live on the crates that own the gated code. **No facade, no compatibility re-export**: all 242 dependency edges onto `semio-framework-ui` (222 manifests, 489 source files, 3,580 `ui_wgpu::wgpu::` paths) were re-pointed to the owning crates; dependents now import e.g. `semio_framework_ui_wgpu_input::InputState`, `semio_framework_ui_declaration::ui::UiNode`, `ui_scene::math::Mesh3dFault`.
- Oracle (`rustc +nightly-2026-07-20 --emit=metadata`, base git `7880cc36f78`): tui lib NEW 0 / test NEW 0 non-stub; wgpu lib NEW 0 privacy/resolve/trait-scope, 1 type row that exists identically in the base test configuration (stub-typed `SELECT_SIDE_OFFSET`); wgpu test NEW 0 privacy/resolve/trait-scope/other and 2 type rows that are stub-typed calls into crates outside the family (same call sites compile at base).

## 1. Architecture

Crate roots are the existing module files (`[lib] path = "../../🦀️.rs"`, manifest in the sibling `📦️packages/🦀️rust`) or a group root in a new taxonomy folder that mounts sibling module trees with `#[path]`. All new folders resolve through `semanticDirectoryKindId` (0 unresolved), emojis are RGI, no sibling emoji clash.

| tree | crate (`semio-framework-ui-…`) | content |
| --- | --- | --- |
| tui | `tui-core` | geometry, theme, event, text, cell, rows, signal (`WidgetSignal`, `CursorShape`, `CursorSpec` lifted out of `widget` to break the vt/widget cycle) |
| tui | `tui-vt` | virtual terminal (parser, screen, pane, history, encoders) |
| tui | `tui-terminal` | ANSI codec, backend, pty, windows ABI; feature `native` (= former `tui-terminal`, owns `libc`) |
| tui | `tui-widgets` | `widget` + 18 element widgets |
| tui | `tui-frame` | chrome, layout, scene, footer, navbar, window |
| tui | `tui-engine`, `tui-host` | engine; wasm host (feature `bindgen`) |
| tui laws | `tui-laws`, `tui-unit-laws`, `tui-vt-laws` | cross-crate behaviour laws, the unit corpus, the vt laws (companions, `#![cfg(test)]`) |
| wgpu light | `wgpu-light`, `wgpu-input` | minimap, tree window, presence bar, tabs; input state (retained hit registration peeled out) |
| wgpu engine | `wgpu-layout` | chrome + layout + flex |
| wgpu engine | `wgpu-mounted-layout` | mounted layout + new `hit` module (retained hit registration, was `#[cfg(feature = "wgpu-engine")]` in `input`); feature `testkit` |
| wgpu engine | `wgpu-widget-kit` | immediate-mode context, interaction maps, widget node model, text drawing (peeled out of `widgets`; `constrain_number_input` moved down from `events`) |
| wgpu engine | `wgpu-controls` | select, slider, stepper, input, button, toggle, key value, ring, icon selector (`push_stepper_*` moved down from `paint`) |
| wgpu engine | `wgpu-events`, `wgpu-paint` | event router; retained paint |
| wgpu engine | `wgpu-paint-reference` | the 33 `#[cfg(test)]` immediate-mode reference paint functions of `paint` (golden oracle of the paint and engine laws) |
| wgpu engine | `wgpu-widgets` | `measure_widget` / `render_widget` dispatch + the tree element |
| wgpu engine | `wgpu-hosting` | accessibility, cursor, shell, host, scene slots; feature `testkit` |
| wgpu engine | `wgpu-engine-surface`, `wgpu-engine` | the 79 types/functions in front of `Ui` (1,260 lines, peeled out of `engine`) and the `Ui` facade (`impl Ui` stays whole: inherent impls cannot cross a crate); `testkit` forwards mounted-layout, hosting, engine-surface, retained-tree and the optional paint-reference |
| wgpu laws | `wgpu-input-laws`, `wgpu-layout-laws`, `wgpu-mounted-layout-laws`, `wgpu-events-laws`, `wgpu-events-control-laws`, `wgpu-paint-laws`, `wgpu-engine-laws`, `wgpu-engine-scene-laws`, `wgpu-laws` | companions holding the test roots that exceeded or cycled the owner (events 2,198-line unit root, paint 1,679, engine 3,318 plus scene/physical-close laws, input/layout/mounted tests that need higher crates, and the five root-level cross-crate tests, plus the two former `[[test]]` targets of the ui crate: `original_prepared_raster_refusal`, `original_clipboard_ingress`) |

Dependency graph is acyclic including dev-dependencies (`cyc.py`, `devcyc.py`: 32 crates, 0 cycles, 0 dev cycles). Embedded JSON fixtures of the oversized companions are read at run time (`fixture_text!`), the `physical_job_close` allocator witness is its own module mounted by both binaries that measure.

## 2. Closure census (canonical `readRustSourceClosureV1`, per crate over default + every single feature + all features, lib and cfg(test))

| crate | worst lib | worst test | binding closure (files / bytes / lines) |
| --- | ---: | ---: | --- |
| `semio-framework-ui-tui-core` | 0.664 | 0.892 | default: 14 / 192869 / 3568 |
| `semio-framework-ui-tui-vt` | 0.725 | 0.725 | default: 13 / 108867 / 2899 |
| `semio-framework-ui-tui-terminal` | 0.639 | 0.806 | native: 21 / 143891 / 3223 |
| `semio-framework-ui-tui-widgets` | 0.707 | 0.707 | default: 20 / 117575 / 2828 |
| `semio-framework-ui-tui-frame` | 0.566 | 0.566 | default: 7 / 95799 / 2263 |
| `semio-framework-ui-tui-engine` | 0.273 | 0.273 | default: 6 / 44778 / 1092 |
| `semio-framework-ui-tui-laws` | 0.031 | 0.574 | default: 10 / 137579 / 2294 |
| `semio-framework-ui-tui-unit-laws` | 0.031 | 0.405 | native: 8 / 78687 / 1621 |
| `semio-framework-ui-tui-vt-laws` | 0.031 | 0.547 | default: 14 / 143512 / 1233 |
| `semio-framework-ui-tui-host` | 0.031 | 0.031 | default: 1 / 3279 / 58 |
| `semio-framework-ui-wgpu-light` | 0.197 | 0.442 | default: 10 / 115750 / 1177 |
| `semio-framework-ui-wgpu-input` | 0.193 | 0.195 | default: 1 / 29403 / 779 |
| `semio-framework-ui-wgpu-layout` | 0.462 | 0.633 | default: 10 / 144595 / 2533 |
| `semio-framework-ui-wgpu-mounted-layout` | 0.444 | 0.649 | default: 7 / 157694 / 2595 |
| `semio-framework-ui-wgpu-widget-kit` | 0.150 | 0.189 | default: 4 / 42686 / 754 |
| `semio-framework-ui-wgpu-controls` | 0.313 | 0.313 | default: 10 / 42053 / 623 |
| `semio-framework-ui-wgpu-events` | 0.840 | 0.840 | default: 1 / 187058 / 3361 |
| `semio-framework-ui-wgpu-paint` | 0.778 | 0.778 | default: 1 / 173011 / 3112 |
| `semio-framework-ui-wgpu-widgets` | 0.170 | 0.214 | default: 4 / 51724 / 855 |
| `semio-framework-ui-wgpu-hosting` | 0.364 | 0.637 | default: 17 / 166995 / 2465 |
| `semio-framework-ui-wgpu-engine-surface` | 0.315 | 0.315 | default: 1 / 59368 / 1261 |
| `semio-framework-ui-wgpu-paint-reference` | 0.195 | 0.195 | default: 1 / 51183 / 750 |
| `semio-framework-ui-wgpu-events-laws` | 0.031 | 0.565 | default: 10 / 145696 / 2259 |
| `semio-framework-ui-wgpu-events-control-laws` | 0.031 | 0.197 | default: 3 / 45827 / 787 |
| `semio-framework-ui-wgpu-engine-scene-laws` | 0.031 | 0.283 | default: 7 / 74230 / 1008 |
| `semio-framework-ui-wgpu-paint-laws` | 0.031 | 0.435 | default: 9 / 101000 / 1741 |
| `semio-framework-ui-wgpu-engine-laws` | 0.031 | 0.858 | default: 12 / 214070 / 3430 |
| `semio-framework-ui-wgpu-input-laws` | 0.031 | 0.188 | default: 6 / 20869 / 310 |
| `semio-framework-ui-wgpu-layout-laws` | 0.031 | 0.236 | default: 7 / 61827 / 577 |
| `semio-framework-ui-wgpu-mounted-layout-laws` | 0.031 | 0.145 | default: 3 / 38016 / 481 |
| `semio-framework-ui-wgpu-laws` | 0.031 | 0.523 | default: 7 / 137019 / 2077 |
| `semio-framework-ui-wgpu-engine` | 0.714 | 0.712 | testkit: 1 / 168214 / 2857 |
| `semio-framework-ui` | 0.063 | 0.194 | default: 5 / 50963 / 208 |

## 3. Registration and gates

- 32 members in `🧰️framework/Cargo.toml`, 32 owner rows in `📋️build-owners.json` (`@semio-tech/<id>-rs` with `📋️project.json`), 192 launch routes (6 per crate, orders `900/922/903.96xxx`). The seed was not changed (same practice as the earlier carves).
- `ui` package script (`📜️script.ts`): the default test, `wgpu-engine` test, the three native gates and the wasm checks now select the carved packages (`UI_TEST_PACKAGES`, `UI_WGPU_PACKAGES`, `UI_TUI_WASM_PACKAGES`, ... exported constants); `native-command` corpus, oracle and fixture re-based on them. Gate filters in `launch.json` (`test-wgpu-engine -E ...`, `test-physical-close-native`, ...) work unchanged.
- Removed: law `🧊️feature-ownership` (test, fixture, project target, launch/seed routes, taxonomy kind, tsconfig include, script branch): it audited feature gating that no longer exists. Removed `pub use ui_wgpu::wgpu::*;` from `os/🖥️host/🦀️.rs` (a facade glob with no user found by grep).
- Text readers re-pointed: `📦️crate-boundaries` (producer consumers are the carved crates, the old wgpu root must not exist), `🔬️interactivity-mounted-layout-text` and the interactivity audit run (engine = root + `🪟️surfaces`), `🔬️targets-wgpu-mounted-layout-unit` (worker-close function now in `🪟️surfaces`), hub `unit-census` fixture (8 stale `semio-framework-ui` feature rows dropped, `wgpu-mounted-layout/testkit` added). `bun test` cannot run here (dependencies image not mounted): the edits are static.
- Workspace tables: the `[workspace.dependencies] semio-framework-ui|ui_wgpu` entries of 9 workspaces (root, composition, cad, procedural foundation/generation2d/generation3d, wfc 2d/grid2d/grid3d) were removed (no member inherits them any more).

## 4. Verification actually run

- `census.ts` (all 33 crates, 6 feature sets, lib and test): section 2; before the carve `census.ts` of the ui crate reported lib 33,833 / test 56,932 lines.
- Oracle families `families/tui.json`, `families/wgpu-x.json` (`iter.py`: run, widen what privacy errors demand, repeat; final runs FINAL2): see section 0. Widening is `pub` (no `doc(hidden)`) and tool-driven (`pos2widen`, `widen.py`); `cfg(test)`-only items used by moved tests became `any(test, feature = "testkit")` (`cfgtest_scan.py`).
- Probe crate (temporary, removed): `use` line for each of the 962 rule targets compiled in the oracle: no unresolved target in a carved crate (remaining rows are cfg-gated items, private helpers that no consumer can name, and stubbed prepared/theme).
- `rustfmt +nightly --check` parse of the 600 modified or created `.rs` files: 0 parse errors. 244 manifests parse (Bun TOML), `📋️build-owners.json` and `launch.json` parse, every `path =` dependency in the touched manifests exists, no dangling `#[path]`/`include!` in the ui and renderer trees (`lawcheck.py`; the two unmounted case dirs `targets-wgpu-prepared-laws`, `...icon-name...` are crate roots and were so before).
- Latent defect found and fixed on the way: after the census-6 split, `tui-terminal` native builds did not import `process_environment` / `osc52_payload_check` (the second was gated `cfg(test)`); fixed in `⌨️tui/🔌️backend`.

## 5. Open risks (nothing compiled)

1. **Lock refresh**: the 32 new packages and ~240 new dependency edges are in no `Cargo.lock`. Workspaces to refresh online: framework, root, `🌎️hub`, `✏️s/🧑‍💻dev/🧩️composition`, plugins cad, equation, flow, playbook, dag, wfc 2d/grid2d/grid3d, procedural foundation/generation2d/generation3d.
2. **Feature parity of dependents**: `wgpu-engine` used to switch on `semio-framework-ui-prepared/browser-presenter`; the rewrite adds the feature only to dependents that now name the prepared crate (`specs/featmap.json`). A dependent that relied on it without importing prepared names needs it explicitly. Dependents with `testkit` get `wgpu-engine/testkit`.
3. **Windows-only code** moved with `tui-terminal` (`windows_abi`, native-windows backend, windows pty) and host `cfg(not(target_os = "wasi"))`: only text/path/oracle evidence, no `--target x86_64-pc-windows-msvc` compile.
4. **Pre-existing defects not touched**: `EngineCanvas/…/✍️text-editor` names `ui_wgpu::wgpu::draft_changes_json` (defined nowhere) and `♾️infinite/🌍️world/♻️resources/🧪️tests` names `MeshGpuTable` from a missing path: both stay unrewritten.
5. **Cargo dependency detail**: dev-dependency features were generated from usage; `semio-framework-ui-wgpu-engine` makes `paint-reference` optional (feature `testkit`) and a dev-dependency; consumers of `testkit` must enable the owner crates' `testkit` themselves.
6. `Cargo.lock` free check of `required-features` is gone with the two ui `[[test]]` targets (now plain `[[test]]` of `wgpu-laws`).
7. Docs under `.cursor/plans` and ticket folders still mention the old paths (not touched).

## 6. Crates to compile (`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests`, after the lock refresh)

New (32, list in `C/uicarve-tools/lists/crates-new.txt`): `semio-framework-ui-tui-core`, `semio-framework-ui-tui-engine`, `semio-framework-ui-tui-frame`, `semio-framework-ui-tui-host`, `semio-framework-ui-tui-laws`, `semio-framework-ui-tui-terminal`, `semio-framework-ui-tui-unit-laws`, `semio-framework-ui-tui-vt`, `semio-framework-ui-tui-vt-laws`, `semio-framework-ui-tui-widgets`, `semio-framework-ui-wgpu-controls`, `semio-framework-ui-wgpu-engine`, `semio-framework-ui-wgpu-engine-laws`, `semio-framework-ui-wgpu-engine-scene-laws`, `semio-framework-ui-wgpu-engine-surface`, `semio-framework-ui-wgpu-events`, `semio-framework-ui-wgpu-events-control-laws`, `semio-framework-ui-wgpu-events-laws`, `semio-framework-ui-wgpu-hosting`, `semio-framework-ui-wgpu-input`, `semio-framework-ui-wgpu-input-laws`, `semio-framework-ui-wgpu-laws`, `semio-framework-ui-wgpu-layout`, `semio-framework-ui-wgpu-layout-laws`, `semio-framework-ui-wgpu-light`, `semio-framework-ui-wgpu-mounted-layout`, `semio-framework-ui-wgpu-mounted-layout-laws`, `semio-framework-ui-wgpu-paint`, `semio-framework-ui-wgpu-paint-laws`, `semio-framework-ui-wgpu-paint-reference`, `semio-framework-ui-wgpu-widget-kit`, `semio-framework-ui-wgpu-widgets`.
Feature checks: `ui-tui-terminal --features native`, `ui-tui-unit-laws --features native`, `ui-tui-host --features bindgen`, `ui-wgpu-engine --features testkit`, `ui-wgpu-mounted-layout --features testkit`, `ui-wgpu-paint --features testkit`, `ui-wgpu-hosting --features testkit`; wasm: `ui-tui-{core,vt,terminal,widgets,frame,engine}` and `ui-wgpu-engine` for `wasm32-unknown-unknown`, `ui-wgpu-light` + `ui-wgpu-input` for `wasm32-wasip2`.
Residual `semio-framework-ui` (`--lib --tests`) and the 200 dependents whose manifests changed: `C/uicarve-tools/lists/crates-consumers.txt` (they are the 257 root members the source gate used to refuse, plus the framework products/hub/plugin workspaces).
Test runs: `@semio-tech/ui-rs:test`, `:test-wgpu-engine`, `:test-physical-close-native`, `:test-mounted-admission-native`, `:test-original-presented-source-native`, then the TS laws `native-command`, `crate-boundaries`, `interactivity-mounted-layout-text`, `targets-wgpu-mounted-layout-unit`, hub `unit-census`.
