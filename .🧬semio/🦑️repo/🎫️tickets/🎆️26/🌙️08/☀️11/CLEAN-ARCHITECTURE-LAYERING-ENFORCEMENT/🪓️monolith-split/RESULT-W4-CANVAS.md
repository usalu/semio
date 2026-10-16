# 🖼️ Wave 4 Result: `semio-framework-canvas` (icons out, five producers)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool (the ticket is left to the coordinator).
Brief and rules: `📓️WAVE4-BRIEF.md`, `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `PRODUCER-CRATE-RECIPE.md`, `RESULT-CARVE-COMPILER.md`, `RESULT-CARVE-PETS.md`, `RESULT-CARVE-PRINT-SERVER.md`.
`C` = `🧰️framework/🔨️modules/🖼️canvas`, `A` = `🧰️framework/🔨️modules/🖼️assets`, `W` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu`.
Tools and logs: `canvas-carve-tools/` (`carve.py`, `probe.py`, `compare.py`, `union.py`, `lossless.py`, `verify.ts`, `census-test.ts`, `assets-check.ts`) and `🗑️generated/canvas-*.log`.

## 0. Verdict

No cargo check or test could run (the launcher is down, section 6.9), so everything below is analysis-only evidence. Within that limit: the embedded icon set is gone from the crate, one general runtime asset port replaces the compiler font port and carries the icon pack too, every host that paints icons is wired, and the former 5,681-line crate is five crates that all sit inside the target (3500 lines, 28 files, 230,000 bytes) for the lib and for the test closure.

| Crate | Lib: files / bytes / active lines | Test closure (cfg `test`, `testkit`) | Embedded |
| --- | --- | --- | --- |
| `semio-framework-canvas` before | 318 / 1,996,242 / 5,676 (over: files) | 342 / 2,159,597 / 7,005 | 311 files, 1,653,610 B |
| `semio-framework-canvas` (facade) | 1 / 2,208 / 40 | 4 / 15,927 / 191 | 0 |
| `semio-framework-canvas-assets` (new) | 1 / 12,024 / 256 | 4 / 21,443 / 401 | 0 (test fixture 2,080 B) |
| `semio-framework-canvas-camera` (new) | 1 / 10,544 / 235 | 4 / 30,613 / 397 | 0 |
| `semio-framework-canvas-icons` (new) | 4 / 107,957 / 2,561 | 6 / 170,489 / 2,683 | 0 |
| `semio-framework-canvas-scene` (new) | 7 / 122,703 / 2,528 | 24 / 189,021 / 3,413 | 1 file, 1,454 B (section 2.4) |

Counter: `readRustSourceClosureV1` through `census.ts` (lib) and `canvas-carve-tools/census-test.ts` (test selection). All rows `ok`, `closureCompilable: true`, `oversizedDeps: 0` (`🗑️generated/canvas-census-before.log`, `canvas-census-after.log`, `canvas-census-test-after.log`). The old crate was over on files (binding), bytes (ratio 9.9) and lines.

## 1. What the "embedded icon set" was

The 311 embedded files were not one thing:

- 279 SVG sources (29 metabolism + 250 catalog), pulled in by `include_str!` from the generated `🔤️shortcodes/🦀️.rs` (2,184 lines, one match arm per code) — 853,838 bytes of SVG once packed.
- 29 normalized metabolism SVG copies under `🌱️metabolism/🔣️icons/🤖️generated/🌱️metabolism_svgs`, pulled in by `MetabolismIconName::svg()`, which nothing in the repository calls.
- `😀️NotoColorEmoji-subset.ttf` (152,308 B), byte-identical to the compiler's emoji font (`cmp` equal); `🔤️MapLabelSans.ttf` (26,772 B); the 1,454-byte label-shapes schema.

How icons are resolved: `icon_codec::resolve_icon_kind(encoded, themed_lookup)` decodes the wire string into `Icon`, then shortcode, catalog and metabolism icons were looked up in the generated match, math/emoji/text icons go through the compiler fonts, inline/data icons are decoded in place. The result (`ResolvedIcon`) is parsed by `SvgDocument::parse_icons` (`usvg`, emoji face from `usvg_options_icons`) inside `os-infinite`'s `IconPaintCache::get_or_build` and cached as a vello `Scene`; map and editor labels are shaped with `usvg_options_map_labels` (label face). There is no separate icon atlas in Rust: the cache is the atlas. All painting is host/browser only; `wasm32-wasip2` guests only measure.

## 2. The runtime asset channel

### 2.1 One port, not two

The font port from `RESULT-CARVE-COMPILER.md` is generalised instead of adding a second mechanism next to it. New crate `semio-framework-canvas-assets` (`C/🔌️runtime-assets`):

| Item | Role |
| --- | --- |
| `RuntimeAsset::{CompilerFont(FontRole), LabelFont, IconPack}`, `ALL` (7), `file_name()`, `repository_directory()` | stable identity of the seven assets and where each lives (fonts stay in `📚️compiler/🌍️world/🔤️fonts`, the label face in `C/🖼️assets`, the pack in `A/🔣️icons/🤖️generated/🔤️shortcodes`) |
| `RuntimeAssets<'a> { fonts: FontSet, label_font, icon_pack }`, `try_from_loader`, `from_root(&Path)`, `validate()` | host-neutral constructor; `from_root` reads `<root>/<repository_directory>/<file_name>` and leaks once; `validate` parses without installing |
| `install_runtime_assets(RuntimeAssets<'static>)`, `install_runtime_asset_bytes(7 × Vec<u8>)` | parse first (fonts, pack), then one `OnceLock`; errors `UnparseableFont { role }`, `UnparseableIconPack { cause }`, `AlreadyInstalled` |
| `compiler_fonts()`, `emoji_font()`, `label_font()`, `icon_pack()` | accessors for the consumers; `compiler_fonts()` keeps the wasip2 estimate-only arm |
| `IconPack::{parse, themed, plain}` (wire version 1) | reads the pack document with `serde`; refuses another version or unknown fields |
| `semioSetRuntimeAssets(7 × Uint8Array)` (wasm-bindgen, browser only) | **one door exported by every wasm module that links Canvas**; an already installed module keeps its first install |
| `testkit` feature (`🧪️tests/🧰️testkit`) | `install_repository_assets()` reads the repository once per process and installs it |

The Noto subset is no longer a Canvas asset: `usvg_options_icons` reads the compiler's installed emoji face (`emoji_font()`). The label face is the only new font channel.

### 2.2 The icon pack

`A/🔣️icons/🏗️builder/📽️projection/🟦️.ts` (`renderShortcodes`) now emits, next to the TS projection, `🔤️shortcodes/🔣️.json` (`{version, themed, plain}`, the raw SVG sources exactly as `include_str!` read them) and `🦀️.rs` holds only the generated emoji table `icon_shortcode_emoji` (1,891 arms, byte-identical arms to the former file; kept compiled in so wasip2 guests still size emoji shortcodes). Checked against the previous output (old builder from `git archive HEAD` in a sandbox): 279 of 279 arms equal their pack entry, 1,891 of 1,891 emoji arms equal. `renderRustMetabolism` lost `svg()` and the `🌱️metabolism_svgs` mirror; `bun assets-check.ts publish` regenerated and pruned (check: fresh). Resolution order is unchanged: compiled emoji, then themed, then plain.

`🤖️generated` is gitignored, so the pack, like the generated Rust the crate already needed, exists after the assets build (`@semio-tech/assets:build`).

### 2.3 Hosts wired (all in this change)

| Host | Delivery | Door | Where |
| --- | --- | --- | --- |
| Native renderer, headless smoke | repository-shaped root: `SEMIO_RUNTIME_ASSETS` (the runner sets it to the repo) or the repository found above the executable / plugin module root / cwd | `runtime_assets::install_native_runtime_assets()` | `W/🔤️runtime-assets/🦀️.rs` (renamed from `🔤️compiler-fonts`), called by `run_native` and `run_smoke_with_axes` in `W/🧊️renderer/🔬️native-socket-probe/🦀️.rs`; `⌨️native-entrypoint/📜️script.ts` sets the variable |
| Browser frame Worker | three static routes `/renderer-modules/{compiler-fonts,canvas-fonts,icon-pack}` | `installRuntimeAssets(loaded, …)` → `semioSetRuntimeAssets`, before `semioWgpuWorkerBootstrap`, failure reported as `runtime-assets-fault` and boot continues | `W/🎞️frame-worker/🟦️.ts`, mounts in `W/🌐️server/🟦️.ts` (`RUNTIME_ASSET_CHANNELS`; `fontRoot` config key removed) |
| Flow browser wasm | embedder option `runtimeAssets` (7 byte arrays) or `installPageRuntimeAssets(core)` | `semioSetRuntimeAssets` (re-exported by `🌊️flow/🫀️core/📦️packages/🦀️rust/🟦️.ts`; `flowSetCompilerFonts` removed) | `🌊️flow/🕸️wasm/🌐️browser/{🏃️runtime/🟨️.js,📝️declaration/📤️projection/🟦️.ts}` |
| React lane wasm sessions (surface, node-graph, editor, flow) | same routes, same-origin fetch | `installPageRuntimeAssets(module)` after `module.default()` | `🧱️elements/🪪️WasmSessionLoader/🟦️.tsx` |
| Puzzle 2d board wasm | same | same | `✏️s/…/◻️2d/…/✏️editor/🌉️wasm/🟦️.ts` |
| Storybook | `staticDirs` in `.storybook/main.ts`, scope row in `📖️stories/🧭️coordination/🟦️.ts` | `singleFlightWasmInit` installs after init | `.storybook/preview.tsx` |
| Dev servers (React lane, wgpu serve, play) | static-dir deliveries from `runtimeAssetDeliveries()` | — | `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` (now for every renderer, not only wgpu), `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts` |

The single TS declaration of the channels is `C/🔌️runtime-assets/🟦️.ts` (`RUNTIME_ASSET_CHANNELS`, `RUNTIME_ASSET_FILES`, `runtimeAssetDeliveries`, `installRuntimeAssets`, `installPageRuntimeAssets`). The compiler's own `🌍️world/🟦️.ts` keeps only `COMPILER_FONTS_DIRECTORY` and `COMPILER_FONT_FILES`.

I found the React lane and puzzle hosts by a dependency scan the compiler carve had not done (it listed the wgpu renderer and Flow only): `surface`, `editor`, `node-graph` and the puzzle board are `wasm-bindgen` crates that link Canvas and paint icons and labels too.

### 2.4 What stays embedded

`🏷️text` keeps `include_str!` of `🧬️schema/🏷️label-shapes/🔣️.json` (1,454 bytes): it is the schema whose `const`s are the shaped-label cache bounds ("their only source", schema-first), read on first cache use, and a runtime channel would make the cache depend on host delivery. The crate passes the gate with it; the ownership test pins the count to exactly one `include_str!` and zero `include_bytes!` in the scene tree.

## 3. Crate graph (downward only; the facade is the only parent)

```
semio-framework-compiler ─┐
                          ├─> canvas-assets ──> canvas-scene ──┐
geometry, ui-styling ─────┤                 └─> canvas-icons ──┼─> semio-framework-canvas (facade)
                          └─> canvas-camera ───────────────────┘
```

| Crate | Node (lib root `<node>/🦀️.rs`, package `<node>/📦️packages/🦀️rust`) | Content | Depends on (first-party) |
| --- | --- | --- | --- |
| `…-canvas-assets` | `C/🔌️runtime-assets` | the port above | compiler |
| `…-canvas-scene` | `C/🎬️scene` | root (re-exports, theme, raster, render, canvas_content, native_render, `CanvasError`, retirements), `🖌️renderer` (1,237 lines: paint types, `Scene`, retirement registry, `draw_list`, `SvgDocument`), `🖼️svg-icon`, `🏷️text`, `🖥️gpu-session` (browser) | assets, geometry, ui-styling, value, async |
| `…-canvas-camera` | `C/📷️camera` | `camera`, `lod` | geometry, ui-styling |
| `…-canvas-icons` | `C/🪪️icon-codec` | `Icon` union and codec, resolver, `MetabolismIconName`, generated emoji table | assets, compiler, icon-contract, value, value-derive, base64, ui-styling |
| `semio-framework-canvas` | `C` (lib root `C/🦀️.rs`, package `C/📦️packages/🦀️rust`) | explicit `pub use` of every former root item, `CanvasExtension`/`CanvasEngine`, the draw-list law | the four above |

Why scene keeps text and svg_icon together: `usvg_options_*`, `render_svg_tree_*` and `svg_icon_content_bounds` take `usvg` types and are `pub(super)`; the ownership test (and the rule against exporting third-party types) forbids widening them, so the `usvg` boundary stays inside one crate. The old glue `📦️packages/🦀️rust/🦀️.rs` (`mod component; pub use component::*`) is deleted; the facade manifest points `[lib] path` at `C/🦀️.rs`. Every file under `C` has exactly one owning crate (`verify.ts`).

## 4. Files changed

Created (all start with an emoji docstring): `C/🔌️runtime-assets/{🦀️.rs,🟦️.ts,🧪️tests/🦀️.rs,🧪️tests/🧫️fixtures/🔣️.json,🧪️tests/🧰️testkit/🦀️.rs,📦️packages/🦀️rust/{Cargo.toml,📋️project.json}}`; `C/🎬️scene/{🦀️.rs,🖌️renderer,🖼️svg-icon,🏷️text,🖥️gpu-session,📦️packages/🦀️rust/…}`; `C/📷️camera/{🦀️.rs,📦️packages/…}`; `C/🪪️icon-codec/{🦀️.rs,📦️packages/…}`; `C/🧪️tests/🧭️camera-unit/🦀️.rs`; `W/🔤️runtime-assets/{🦀️.rs,🧪️tests/🦀️.rs}`; `W/…/🧪️tests/🔤️wgpu-runtime-assets` (engine tests dir); `🌊️flow/🕸️wasm/🧪️tests/🔌️runtime-assets`; `canvas-carve-tools/*`.

Deleted: `C/📦️packages/🦀️rust/🦀️.rs` (glue); `C/🖼️assets/😀️NotoColorEmoji-subset.ttf`; `W/🔤️compiler-fonts` and the two compiler-font TS test dirs (replaced); the 29-file `🌱️metabolism_svgs` mirror (generated, gitignored).

Moved without change of content (only dedent, `#[path]`/`include!` prefixes, and the intended edits in section 2): the 3,167-line `C/🦀️.rs` into the new roots and module files (`lossless.py`: every original code line is present, the only differences are the font port, the shortcode resolution, the two `usvg_options_*` functions and mounts; `🗑️generated/canvas-lossless.log`).

Edited: `C/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts,🧪️execution/🔣️.json}` (facade manifest, test route runs all five packages bottom up), `C/🧪️tests/{🔬️unit,🏷️label-shapes,🔬️icon-codec-unit,🧱️ownership}`, `C/♻️retirement/🎬️scene/🧪️tests/🦀️.rs` (imports the scene crate), `C/🌉️icon-name-value/🦀️.rs` (doc), `A/🔣️icons/🏗️builder/📽️projection/🟦️.ts`, `A/🌱️metabolism/🏗️builder/📽️projection/🟦️.ts`, `A/README.md` (generated line), `📚️compiler/🌍️world/🟦️.ts`, `🔣️taxonomy.json` (assets-build output root added in UTF-16 order, `🌱️metabolism_svgs` member removed), `📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts` (Rust icon-binding law), `W/🧊️renderer/{🦀️.rs,🔬️native-socket-probe/🦀️.rs}`, `W/🎞️frame-worker/🟦️.ts`, `W/🌐️server/{🟦️.ts,🎚️config/🟦️.ts}`, `W/⌨️native-entrypoint/{📜️script.ts,🧪️tests/📇️catalog/🟦️.ts}`, `W/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`, `W/🧪️tests/🎚️config/🟦️.ts`, `🧑‍🎨engine/🧪️tests/{🧩️wgpu-module-routes,🧫️fixtures/🧩️wgpu-module-routes/🔣️.json}`, `🌊️flow/{🫀️core/…/🟦️.ts,🕸️wasm/🖼️surface-presentation/🦀️.rs,🕸️wasm/🌐️browser/…}`, `🪪️WasmSessionLoader/🟦️.tsx`, `✏️s/…/🌉️wasm/🟦️.ts`, `.storybook/{main.ts,preview.tsx,📖️stories/🧭️coordination/🟦️.ts}`, the two vite builders, `✍️editor` (dev-dependency feature + two test lines), the puzzle 2d icon test and its `Cargo.toml` dev-dependency.

Registrations (recipe steps 6, 8, 9, 10): `🧰️framework/Cargo.toml` (4 members, byte-sorted locally; the list is not globally sorted because of peer rows), `📋️build-owners.json` (4 rows `@semio-tech/framework-canvas-{assets,camera,icons,scene}-rs`), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (24 routes each, groups `4_build`/`4_gate`/`4_test`, orders `900.01501…`, `922.04501…`, `903.58501…`, none reused). Boundary corpus (step 11): not applicable (UI-only). Facade feature `testkit` forwards to the port's.

Consumers re-pointed: none of the ~25 Rust consumers of `semio_framework_canvas` needed an edit; the facade keeps every public path (`canvas::icon_codec`, `::text`, `::camera`, `::draw_list`, `::gpu_session`, …). The font-port consumers (`install_compiler_fonts`, `FontRole`, `FontSet`, `flowSetCompilerFonts`, `semioWgpuSetCompilerFonts`, `SEMIO_COMPILER_FONTS`, route `/renderer-modules/compiler-fonts` as the only mount) were all rewritten; `git grep` finds none of the old names left.

## 5. Tests

Rust (written, type-analysed with stubs, not executed — section 6): assets `the_asset_catalog_matches_the_shared_fixture_and_the_files_on_disk`, `compiler_font_roles_keep_their_file_names_inside_the_catalog`, `try_from_loader_asks_every_asset_in_order_and_stops_at_the_first_failure`, `from_root_names_the_first_missing_asset_file`, `an_icon_pack_of_another_version_or_shape_is_refused`, `unparseable_assets_are_refused_by_validation_before_anything_is_installed`, `installed_assets_serve_every_consumer_and_a_second_install_is_refused` (reads the fixture samples against the SVG files on disk); icons `shortcode_resolution_matches_the_assets_snapshot_oracle` (the former oracle over the 1,891-entry snapshot, now through the pack) and the codec tests installing the assets; scene label/theme/unit tests install them; wgpu `icons_resolve_after_the_native_install` (math, emoji, text, `:plus:`, `:capsule_J:`); editor and puzzle tests use `canvas::testkit`.

Language-neutral: `C/🔌️runtime-assets/🧪️tests/🧫️fixtures/🔣️.json` (channels, seven assets with bytes and sfnt tags, pack samples with source paths) is read by the Rust tests and by TS.

TS (run): `bun test C/🧪️tests/🧱️ownership/🟦️.ts` 9 pass, 0 fail, 134 expects — facade has no glob re-export and embeds nothing, the producer graph never touches the facade, the TS channels equal the fixture, the Rust catalog and the files on disk, the pack equals every vendored SVG byte for byte (independent `JSON.parse` against `readCatalogSources`) and equals the builder's own artifact, the installer fetches the seven routes and refuses a partial set. Vitest (`W/🧪️tests/🎚️config/🟦️.ts`, `🔤️wgpu-runtime-assets` + `wgpu-module-routes`) 12 pass; Flow `🔌️runtime-assets` 3 pass; native-entrypoint catalog 1 pass (8 expects, asserts `SEMIO_RUNTIME_ASSETS`); `path-emoji-statutes -t "Rust icon bindings"` 1 pass (it was already failing on `renderRust` before my change; fixed for both renderers).

## 6. Verification (commands and actual results)

1. **Census** — section 0.
2. **Per-crate full-resolution differential** (`probe.py` + `compare.py`; `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` from each real lib root; sandboxes with `extern crate` lines neutralised identically; baseline built from `git archive HEAD` plus the old builder run in a sandbox). Native, `--test`, `wasm32-unknown-unknown` (+test), `wasm32-wasip2` (+test): baseline 200 / 293 / 202 / 289 / 112 / 140 errors versus the sum over the five crates 212 / 312 / 218 / 312 / 120 / 154. Every difference is an unresolved external crate (`compiler`, `geometry`, `ui_styling`, `serde`, `serde_json`, `wasm_bindgen`, `semio_framework_intrinsic_size`) or one of the four new crate names; no E0412/E0425/E0422/E0423/E0603/E0432 for an in-crate item (`🗑️generated/canvas-oracle-per-crate.log`).
3. **Union oracle** (`union.py`): the five crates mounted as modules of one crate with crate-name paths rewritten, so cross-crate privacy and the facade's re-export list are resolved: the same six configurations differ from baseline only in external-crate imports (`🗑️generated/canvas-oracle-union.log`). Seeded sensitivity: making `install_runtime_assets` private gave E0603 on the facade, renaming `canvas_content` gave E0432, making `camera::screen_to_world` private gave E0603 in the `--test` run — all caught; the seeded copy was deleted.
4. **Type analysis of the new code against hand-written stubs** (`rustc -o /dev/null`, full analysis, success shown by the only remaining message being the refusal to create `/dev/rmeta…`; a type error in a negative control is reported): assets lib (native and wasip2), assets lib + testkit + tests (`--test`), the icons shortcode resolution snippet, and the wgpu `runtime-assets` module with its tests: all analysis-clean. The stubs stand in for `compiler`, `serde_json`, and the canvas facade, so this proves my code's own types and lifetimes, not the real crates' signatures; the `wasm_bindgen` door is a copy of the existing door pattern and was not analysed.
5. **Definition scan**: no `pub(crate)`/`pub(super)` item crosses a crate edge (the scene's `pub(crate)` and `pub(super)` items are used only inside the scene tree; checked by grep and by the union oracle). Camera has one private helper (`view_bounds`), used internally.
6. **Static** (`verify.ts`): five manifests parse (Bun TOML), `package.workspace` resolves to `🧰️framework`, lib paths and every dependency path resolve, boundary keys exact, members present and byte-sorted, owner rows, project files, 24 launch commands present once with the exact shape in both JSON files, no order key reused, all 37 `#[path]`/`include*!` literals resolve, longest path 109 bytes. `ALL STATIC CHECKS PASSED`. `launch.json`, the seed (JSONC), `📋️build-owners.json`, `🔣️taxonomy.json` and the fixtures parse. `tsc --strict --noUncheckedIndexedAccess` on the new TS module: clean. rustfmt (repo config, `skip_children`): new files parse; `🦀️.rs` (facade), the port and its tests were formatted; the verbatim moved files keep their pre-existing style diffs.
7. **Region markers**: every one of the original 20 region names is present, and every region is balanced in every new file; four names are new (`🔖️Reexports`, `🧪️Tests`, `🔖️IconPack`, `🔖️Installation`). `🏷️IconAssets` now delimits the port's asset identity and install error types.
8. **Taxonomy**: `verify taxonomy report --scope C` first refused the registry (`outputRoots must be unique and lexically ordered`: my new `🔣️.json` row had to precede `🟦️.ts` in UTF-16 order); fixed, then the registry loads and the 40-minute report finished: `clean=false errors=124` for the whole canvas scope, which was already unclean (the module root `🖼️canvas` and its folders `♻️retirement/*`, `🌉️icon-name-value`, `🎨️react-renderer` have no registered semantic kind; the root script and `🧪️execution/🔣️.json` are rejected; 75 repository-wide `rust-path-join-unproven` references). My new folders add the same single class: `directory-kind-unresolved` for `🎬️scene` (+4 children), `📷️camera`, `🔌️runtime-assets`, `🪪️icon-codec` (the existing sibling folders get the identical message), and two `reference-preimage-unreadable` because I added docstrings to those files while the run was in flight. No new error class. The sibling-emoji statute (`pathEmojiStatuteFindings` over the three trees I touched) flagged one real duplicate of mine, `🧪️tests/📷️camera-unit` next to `📷️camera-fit`; renamed to `🧭️camera-unit`. The remaining duplicates (`🏷️`, `🔬️` test folders) predate this change. Log: `🗑️generated/canvas-taxonomy-verify.log`.
9. **Launcher / real compile**: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-canvas --lib` at the start: `Finite original maintenance authority required` (`retireInactiveCargoStoreOwnedV1`); mid-run a peer's `🧹️retention/📥️physical/🟦️.ts` had a duplicate `material` declaration (TS syntax error); at the end the first error was back. `NX_FORCE_REUSE_CACHED_GRAPH=true bun repo run @semio-tech/framework-canvas-assets-rs:check --configuration=prototype --parallel=1 -- --lib --offline`: `Active graph readers retain required Nx pool headroom`. Logs: `🗑️generated/canvas-cargo-check-{start,mid,end}.log`, `canvas-nx-check-assets.log`. No cargo `check`, `test`, `update` ran, so no lock file was refreshed (section 7).

## 7. Residual risks and open items

1. **Nothing was compiled or executed.** Moved code is verbatim and resolves; new code is analysed against stubs only. The first real `check`/`test` of the five crates is the actual verdict. Likeliest surprises: the wasm-bindgen export of `semioSetRuntimeAssets` from a dependency rlib (precedent in this repo: `initialize_browser_clock` from `⏱️clock`, re-exported by Flow's TS facade), `usvg::Options` struct-update syntax in the two fallbacks, and the stubs' fidelity in section 6.4.
2. **Lock files**: `Cargo.lock` (root, framework, hub, teaching workspaces) must be refreshed with `update --workspace --offline` once the launcher works: four new packages and the facade's new edges; the canvas package loses `vello`, `vello_svg`, `kurbo`, `peniko`, `image`, `compiler` edges to the children.
3. **Production packaging**: closed by the addendum below (staged trees, both pipelines, bundle-content test). Open there: the stage target and the new release job have never run end to end through `bun repo run` (the graph build failed twice on `Original selected owner registry changed` while peers edited `📋️build-owners.json`).
4. **Native packaged installs**: closed by the addendum below (a `runtime-assets` folder beside the executable is the default; `SEMIO_RUNTIME_ASSETS` stays an override). The pack is a generated, gitignored file: the assets build must have run before staging.
5. **wasip2 guests**: closed by the addendum below for the seven hub plugins whose closure reaches Canvas; the descriptors and component builds still have to be regenerated and rebuilt before any host delivers anything.
6. **Peer interference seen while working** (not mine, not touched): (a) `⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs` carries an uncommitted `pub(in crate::m_geometry)` on `RequestedAllocator`, `observe`, `observed_requested_bytes`, `observe_backing`; it is mounted by ~30 crates (the scene crate mounts it for the retirement tests), and every crate whose root is not `m_geometry` fails in `--test` until it is reverted or widened (my test sandbox reverts it); (b) the fleet's `🧩️icon-embeds/🦀️.rs` generator (`ICON_SVGS` via `include_str!` of all 250 catalog SVGs, RESULT-ICON-RENDER-GENERATED.md) lives in the same builder function and the assets publication/project files; the coordinator decided to keep it (82 KB, fits the crate budget, the renderer crate is partitioned separately) and it is untouched (the addendum records the short detour); it broke the statutes law until `escapeRustString` was added to the definitions the law evaluates (done); (c) `semio-framework-surface` was split into per-family crates meanwhile (the canvas ownership test now reads the paint crate's manifest); (d) launcher/Nx infrastructure above.
7. The label-shapes schema stays embedded (section 2.4). The `Icon` enum keeps its serde and value derives in the icons crate; `IconName` stays in `semio-framework-icon-contract`.
8. Facade tests: only the draw-list law remains in the facade (it needs camera and scene); `original_scene_retirement` is now the scene package's integration test.

---

# Addendum: closing the three regressions (coordinator follow-up)

## A. Item 3, the `icon-embeds` generator: left as is

I had converted it to the runtime pack (an ordered `plain_at` accessor, `IconRenderHost` reading the installed pack, generator and output roots dropped) before the coordinator's correction arrived ("leave it as is; do not convert it"). The conversion is **fully reverted**: the builder emits `🧩️icon-embeds/🦀️.rs` again, `assetOutputRoots` and the assets `📋️project.json` list it again, `IconRenderHost` mounts it with `#[path]` and `ICON_SVGS` again (its diff against HEAD is the fleet's original 5/1 lines), the `IconPack` is back to two `HashMap`s, the facade exports are back, and the extra renderer dev-dependency and unit test are gone. `assets-check.ts publish` regenerated the file and `check` reports the tree fresh (263 outputs). Consumer inspection for the record: `IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs` (the atlas), compiled into the wgpu renderer crate, so it is gated Rust; it stays an embed by decision.

## B. Item 1: runtime assets staged for every production and play bundle

- **Staging** (`🖼️canvas/🔌️runtime-assets/🚚️staging/🟦️.ts`, Node side; the browser file stays Node-free). `runtimeAssetStageTrees()` yields six provenance trees below `🖼️canvas/📦️packages/🦀️rust/dist`: `port-assets/<channel>` (all seven port files, owner `canvas:runtime-assets`, destination = the channel route without its slash, e.g. `renderer-modules/compiler-fonts`) and `guest-assets/<channel>` (the three guest files, owner `canvas:package-assets`, destination `renderer-modules/package-assets/<repository directory>`, the route `🎠️kernel/📦️package-assets` fetches). `canvasRuntimeAssetSources(workspace)` turns them into `BrowserArtifactSource` rows. No staged root contains "runtime" or "activation" (the production test forbids both).
- **Nx target** `semio-framework-canvas:stage-runtime-assets` (`🖼️canvas/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`, the existing canvas script gained the `stage-runtime-assets` command): cached, `dependsOn @semio-tech/assets:build` (the pack is generated), inputs the two TS files, the two font directories and the pack output (`dependentTasksOutputFiles`), outputs `dist/port-assets` and `dist/guest-assets`; it stages through `stageRepositoryArtifacts` like `infinite:fonts`. Registered in `.vscode/launch.json` and the seed (`📦️canvas-runtime-assets🏗️`, group `2_build`, order 213, beside `canvas-font-assets`).
- **os production**: `productionBrowserSources` appends `canvasRuntimeAssetSources(workspace)` after the infinite font pack (six extra rows before the components); the Nx inference (`🏗️build/🕸️graph/📥️inference/🟨️.mjs`) adds `semio-framework-canvas:stage-runtime-assets` to `build-<variant>-react-release` (and so `build-<variant>-react`). The production-browser-artifacts fixture lists the six destinations; the test's fake-staging loop now names fakes by `shimDirectory` instead of source index so it survives sources between the fonts and the components.
- **play**: `playRuntimeAssetSources` appends the same rows; `playDevStaticDirMounts` filters them out (the dev serve mounts the repository directories through `runtimeAssetDeliveries()`, and mounting the staged copies on the same route would be the second claim `staticDirMountVitePlugins` refuses); the play dev server also mounts `packageAssetDeliveries()` now (its browser kernel fetches guest assets). The release orchestrator in `🏢️semio-tech/🎡️play/📋️project.json` gained the prerequisite `@semio-tech/assets:build` and a `canvas-runtime-assets` job (after `support`, before `pages`'s `after` list closes), mirrored in the `nx-contract` fixture.
- **Native packaged installs**: `🧊️wgpu/🔤️runtime-assets/🦀️.rs` resolves, in order: `SEMIO_RUNTIME_ASSETS` (an override, repository-shaped, kept because this design already overrides by environment, e.g. `SEMIO_PLUGIN_MODULES`), a `runtime-assets` folder beside the executable laid out as `<channel>/<file>` (exactly the `port-assets` stage output), the repository above the anchors. `RuntimeAsset::channel()` and `RuntimeAssets::from_staged_root` are new in the port.
- **Bundle-content check** (`🔌️runtime-assets/🧪️tests/📦️bundle-staging/🟦️.ts`, bun): stages the six trees into a temporary workspace from the real repository files, runs the real `copyBrowserArtifacts`, and asserts every port asset sits at `renderer-modules/<channel>/<file>` and every guest asset at `renderer-modules/package-assets/<directory>/<file>` with SHA-256 equal to the catalog (the fixture's `shasum` digests for the six fonts, the assets builder's pack digest for the pack), exactly ten files, no provenance marker copied, `packageAssetUrl` mapping each guest name to the route, and a tree staged under a foreign owner refused. 3 tests pass. The same staging against the real repository (not a temporary one) produced the ten-file bundle (longest path 108 characters), then the staged `dist` was removed.

## C. Item 2: wasip2 guests get the icon pack and fonts through package assets

- **What a guest needs**: `RuntimeAsset::GUEST` = emoji face, label face, icon pack (1.03 MB; the compiler estimates text without glyphs on wasip2, so the other four fonts stay out). `package_asset_name()` is the repository-relative path; `guest_asset_declarations()` returns name, size and SHA-256 per asset. The two font digests are constants, tested against the fixture and the files; the pack digest is written by the assets builder next to the pack as `🔤️shortcodes/🔏️digest/{🔣️.json,🦀️.rs}` and mounted with `#[path]` (no `include_str!`, the ownership law still counts zero embeds in the port crate).
- **Install on open, without touching the reactor**: `register_guest_asset_reader(reader)` stores the guest's `reactor::package_asset`; every port accessor (`icon_pack`, `emoji_font`, `label_font`) installs the three assets lazily on first use once the host has delivered all of them (estimate-only compiler fonts, so `compiler_fonts()` keeps its wasip2 meaning), answers `None` before a complete delivery exactly like a host that delivers nothing, and refuses an unparseable pack once with an `[ERROR]` line. An explicit install still wins on hosts that make one.
- **Declaration**: `PluginBuilder::canvas_runtime_assets()` (feature `canvas-runtime-assets` of `semio-framework-plugin`, optional dependency on the port crate, so no other guest pulls the compiler) declares the three assets (media type text document, like the layout fonts) and registers the reader. Applied in `plugin()` of the seven hub plugins whose dependency closure reaches Canvas (computed from the Cargo manifests): `dag`, `flow`, `forms`, `reasoning`, `sequence`, `space`, `writer`; each enables the feature on its `semio-framework-plugin` dependency.
- **Delivery**: native hosts already deliver declared repository files (`package_file_assets`, names are repository-relative, so the three resolve). The browser kernel's `PACKAGE_ASSET_DIRECTORIES` now also lists the three canvas directories (derived from `RUNTIME_ASSET_CHANNELS`), so the dev mounts and production staging above serve them. Closing ui-render's open React-lane gap: `fetchDescriptorManifest` now remembers the descriptor's declared assets (`descriptorPackageAssets(manifest)`, a side table keyed by the manifest object, malformed declarations refused with `plugin.descriptor-invalid`), and the React `module-loader` passes them to `registerManifest`, so the React lane delivers package assets too (the layout fonts included).
- **Tests**: Rust `🧪️tests/📥️guest-delivery` (declarations against fixture, files, hash crate and the builder's digest; lazy install after partial and complete delivery) and `🧪️tests/🚫️guest-refusal` (unparseable pack), registered as `[[test]]` `guest_delivery` and `guest_refusal`; kernel vitest (canvas guest assets fetched, verified and byte-equal through the route; descriptor assets handed to the loader).

## D. Verification actually run

| Check | Result |
| --- | --- |
| Kernel vitest (all) | 9 files, 91 tests pass (new: canvas guest assets, descriptor assets) |
| wgpu `wgpu-module-routes` + `wgpu-runtime-assets` (vitest) | 12 pass (package mounts derive from the kernel list) |
| Canvas bun: ownership + bundle-staging | 12 pass (50+ bundle assertions, hashes against fixture and builder digest) |
| Play `playdevmounts` (vitest in-source) | pass |
| `assets-check.ts` freshness | fresh, 263 outputs |
| Static checks (`verify.ts`), census | all pass; canvas-assets lib 2 files / 360 lines, tests 5 files |
| rustc oracle on the port crate (native, test, wasip2) | only external-crate errors (`serde`, `serde_json`), no local resolution error |
| Stub harness: guest delivery region of the port (verbatim) | full analysis on native, wasip2 and wasm32-unknown-unknown; seeded type error is caught |
| Stub harness: wgpu host root resolution + tests (verbatim logic, stubbed canvas) | compiles and **runs**: 3 pass (explicit root, repository root, staged folder beside the executable wins, override wins, fallback) |
| rustfmt | the port crate and the guest tests are formatted |

Not run, with the reason: any cargo `check`/`test` (the launcher gate; the Rust unit and integration tests above are written, not executed), `bun repo run semio-framework-canvas:stage-runtime-assets` (failed twice in graph construction on `Original selected owner registry changed`, a peer rewriting the owner registry; the same staging code ran directly against the real repository instead), the full `testProductionBrowserArtifacts` (it stops at its first assertion, `project.targets.build.dependsOn` is `["build-s-react"]` but the fixture expects `build-s-react-release`, which predates this work; my destination order was checked by reading), the path-emoji statutes file (7 of 37 fail on a peer's stdio/graph/normalization areas; the icon bindings law passes).

## E. Still open

1. Cargo lock refresh (four canvas crates plus the plugin crate's optional edge to the port crate; hub lock files) and the first real `check`/`test` of everything Rust above.
2. The seven plugins' `🧩️component/🔣️.json` and `🛂️.descriptor.semio` still say `assets: []` until the components are rebuilt and described (`bun repo run workspace:components --all -- describe`, then `@semio-tech/plugin-registry:generate`); no host delivers the canvas assets before that.
3. The workspace `release` orchestrator (jobs `describe-tool` … `support`) does not stage the canvas trees; production builds get them through the Nx dependency, play through its new job. Layout's fifteen text fonts (`🖼️assets/🔤️fonts`) are served in dev but have no production staging tree yet (same mechanism, not requested).
4. macOS app bundles keep assets under `Contents/Resources`, not beside the executable; the default only knows the sibling folder (the override covers the rest).
