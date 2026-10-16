# Result W4: carve `semio-framework-ui-render`

Executor: Sonnet 5.5, foreground, no sub-agents. Rules: `📓️WAVE4-BRIEF.md` (including "god objects"), `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `PRODUCER-CRATE-RECIPE.md`. Nothing was committed, staged or stashed. Tools and logs: `🗑️generated/w4ui-tools/*`, `🗑️generated/w4ui-*.log`. Paths below are relative to `🧰️framework/🔨️modules/🖱️ui/🖌️render/` unless absolute.

## 0. Verdict

| Item | Result |
| --- | --- |
| Assets | All 15 embedded fonts (1,083,548 bytes) are gone. They arrive through the new `TextFontSet` port. |
| Crates | 4 new producers; the parent stays as the assembly crate with explicit re-exports. |
| Gate and budget | Every crate is inside 3500 lines, 28 files and 230,000 bytes for lib and test closures. |
| Real compile | Not possible. The launcher still stops with `Finite original maintenance authority required` (start and end of the session). |
| Static verification | Differential oracle clean, UI crate-boundary gate test passes (59/59). |
| God objects | Rule checked: no struct impl spans a crate, none was split. `Measurement::size()` became a free function in layout, not an extension trait. |

## 1. Census (canonical `readRustSourceClosureV1`)

Before: `semio-framework-ui-render` 28 files, 1,461,562 bytes, 7,544 active lines, 15 embedded (1,083,548 B), over the limit (binding: bytes, ratio 5.58). Test closure with `test`: 44 files, 1,726,735 bytes, 10,343 lines.

After (`🗑️generated/w4ui-census-after.log`, test rows in `w4ui-census-test.log` with cfg `test` + `backend-testing`):

| Crate | Lib: files / bytes / lines | Test: files / bytes / lines | Gate |
| --- | --- | --- | --- |
| `semio-framework-ui-render` (assembly) | 5 / 79,122 / 1,377 | 9 / 119,600 / 2,223 | pass; `oversizedDeps: 1` is peer `ui-contract` |
| `…-render-packet` (new) | 5 / 99,824 / 2,017 | 9 / 119,550 / 2,672 | pass, `closureCompilable: true` |
| `…-render-interaction` (new) | 4 / 90,678 / 1,875 | 7 / 139,528 / 2,672 | pass; `oversizedDeps: 1` is peer `ui-contract` |
| `…-render-text` (new) | 1 / 51,940 / 989 | 3 / 71,893 / 1,265 | pass, `closureCompilable: true` |
| `…-render-shader-contract` (new) | 1 / 68,241 / 1,478 | 5 / 212,626 / 1,830 | pass, `closureCompilable: true` |

The shader-contract test closure includes the three backend shader sources its unit test reads with `include_str!` (127 KB); it is the tightest row (212,626 of 230,000).

## 2. Crate graph

```
shader-contract            (no first-party deps; dev: naga)
packet   (resource, scene, tessellate, backend)   deps: geometry, bytemuck; feature backend-testing
interaction (identity, dispatch, schedule)        deps: packet, ui-contract; dev: serde_json
text     (text + Measurement + font port)         deps: packet, parley, swash; dev: serde_json
render   (element, layout, frame, surface)        deps: interaction, packet, shader-contract, text, ui-contract, ui-styling, taffy
```

All edges point downward; no child depends on the parent. Each module file has exactly one owning crate (verified by walking every `#[path]` chain: 0 double owners, 0 missing files, 0 missing `include_str!` targets).

Why this cut. The real module cycles were: `element` and `dispatch` (dispatch needed `ElementId`/`Hitbox`/`Bounds`), `text` and `layout` (text needed `Measurement`), `scene` and `tessellate` (mutual), `element` and `layout` (mutual types). Fixes: the identity vocabulary (`ReconciliationKey`, `ElementId`, `Bounds`, `Hitbox`) moved out of `🧱️element` into the new module `🪪️identity` below dispatch; `Measurement` moved from `📏️layout` into `🖋️text` (layout's private `Measurement::size()` became the free `measurement_size`); `scene`+`tessellate` stay in one crate with `resource` and `backend`; `element`+`layout` stay together in the parent.

Layout on disk (module files did not move; recipe position for single-module crates, glue roots beside the package dir for multi-module crates):
- `✨️shader-contract/📦️packages/🦀️rust` and `🖋️text/📦️packages/🦀️rust`: `[lib] path = "../../🦀️.rs"` (the module file is the lib root).
- `📮️packet/🦀️.rs` and `🕹️interaction/🦀️.rs`: new glue roots mounting the existing module files with `#[path]` and re-exporting every pub item explicitly (the `ReadbackImage` export is gated on `backend-testing`).
- Parent `📦️packages/🦀️rust/🦀️.rs`: mounts element, frame, layout, surface; explicit `pub use` of its own API and of the items consumers or its own code need from the four producers (no globs). No `PointerButton` double export.

## 3. Asset channel (rule 3)

Fonts live unchanged in `🖼️assets/🔤️fonts` (Anta, Kelly Slab, Share Tech Mono, 12 Noto Emoji buckets). New port in `🖋️text/🦀️.rs`:
- `TextFontRole` (15 roles, `ALL`, `asset_path()` relative to the catalog), `TextFontSet<'a>` (`get`, `try_from_loader` for fetch or packaged channels), `TextFontError { role }`.
- `TextFontFiles::read(&Path)` + `.set()` for native hosts (owns the buffers, no leaking), `TextFontReadError`. Both are `cfg(not(target_arch = "wasm32"))` so no guest links `std::fs`/`wasi:filesystem` (the parent manifest's old warning).
- `TextSystem::new(&TextFontSet) -> Result<Self, TextFontError>` and `TextSystem::without_builtin_fonts()`. `impl Default for TextSystem` is removed (no fallback).
- I reused the pattern of the compiler `FontSet` (loader constructor, directory reader, role-named errors) but not its types: the roles differ. `RESULT-W4-CANVAS.md` did not exist yet, so there was no canvas port to share.

Hosts wired. A search of every tracked and untracked `.rs` file found exactly one production construction of `TextSystem`: the layout editor engine (`✏️s/🔌️plugins/📏️layout/…/✏️editor/⚙️engine/🎬️scene/🦀️.rs`). It only shapes with its own host font `layout-sans`, so it now calls `TextSystem::without_builtin_fonts()` and carries none of the text system's font files. Behaviour note: glyphs missing from that font no longer fall back to Anta/Noto; the display list already ignored the fallback font identity (`DisplayGlyph` has no font field), so those glyphs were previously wrong, now `.notdef`. Its `LAYOUT_SANS` `include_bytes!` is the canvas agent's asset and is untouched.

Tests (all load fonts through the new path; `🧪️tests/🔬️text-unit`, native only via `cfg(all(test, not(wasm32)))`): fixture-vs-disk check against the language-neutral `🧪️tests/🧫️fixtures/🔤️text-fonts/🔣️.json` (role, asset path, byte length, sfnt tag, SHA-256; generated independently with Python from the files, and checked byte for byte against the Rust path table), distinct paths, loader order and stop-at-first-failure, missing catalog names the first file, garbage bytes name the role, system without built-ins shapes only a provided font, and a source scan that the module embeds no font bytes. The existing differential test against an independently built `parley` layout now uses the disk-loaded bytes. `frame-unit` uses `without_builtin_fonts()` and `ListenerSet::credited_clone()` (the test-only `Clone` impl does not cross crates).

## 4. Registrations (recipe)

- `🧰️framework/Cargo.toml`: four members after the parent (byte order checked by hand: 📦 < 📮 < 🕹 < 🖋 < ✨ within `🖌️render`, all before `🖥️host`). The list was not globally sorted before; I only placed these correctly.
- Four `Cargo.toml` (recipe template, three boundary keys at the ceilings, `role = "framework"`, ids `ui-render-{packet,interaction,text,shader-contract}`) and four `📋️project.json` (`@semio-tech/<id>-rs`).
- Parent `Cargo.toml`: dropped `semio-framework-geometry`, `bytemuck`, `parley`, `swash`, `serde_json`, `naga` (moved to the producers, none used by the remaining parent sources or tests); added the four path dependencies; `backend-testing` now forwards to the packet; dev-dependency on packet with `backend-testing` (the frame and surface tests call `Scene::finish`, gated on `any(test, backend-testing)`).
- `📋️build-owners.json`: four rows, lexical order (1,744 owners, names unique).
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 24 routes each (4 crates x build/check/test x prototype/release), orders `900.016xx`, `922.046xx`, `903.586xx` verified unused in both files. Both files parse.
- Corpus `🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`: 4 rows (33 total).
- `🔒️dependencies.json`: users of `bytemuck`, `naga`, `parley`, `swash`, `serde_json` re-pointed to the producers' manifests. `workspace:verify -- dependencies literal-external` could not run (see 6).
- Parent `📜️script.ts` `test` now passes the five package names to `-p`.
- Cargo.lock files (framework and root): not refreshed (see 6).

## 5. Verification (real output)

1. Full-resolution differential (`w4ui-tools/oracle.py`; analysis only, `--emit=metadata -o /dev/null`, nightly-2026-07-20; baseline = sandbox copy of the original tree, split = the four producers simulated as modules of one crate with `crate::` and cross-crate names mapped; external-crate globs neutralised identically; `CARGO_MANIFEST_DIR` set to a dummy in both): configs native, `--test`, `backend-testing`, `--test`+`backend-testing`, wasm32-unknown-unknown, wasm32-wasip2, wasip2 `--test`. Final run (`w4ui-oracle-final.log`): only-split 0 and only-baseline 0 in native, `bt`, both wasm targets and wasip2 `--test`. In `--test` and `--test`+`bt` the only difference is +2 `E0433 serde_json` in the new text tests (external dev-dependency, same class as the old `serde_json`/`naga` errors) and the abort counter. No new E0412/E0425/E0422/E0423/E0603/E0616/E0624/E0432 for any first-party name. Limit as documented: external crates (`taffy`, `ui_contract`, `parley`, `swash`, `bytemuck`) are unresolved in both trees, so typeck inside functions using them is not exercised.
2. Oracle sensitivity: dropping `ElementId` from the interaction glue in a throwaway sandbox produced two new E0432 errors (glue and element).
3. `bun test ./🧪️tests/📦️crate-boundaries/🟦️.ts` in `🖱️ui`: 59 pass, 0 fail, 2482 expects. It covers, per new row: TOML parse under two parsers, launch commands present, `[lib] path` = entry, dependencies equal the corpus, budget (4000/262144/32) under macOS cfg, no forbidden tokens, plus the whole-corpus acyclicity (DFS and graphlib).
4. rustfmt (`rustfmt +nightly-2026-07-20 --config-path rustfmt.toml --config skip_children=true`): glue files, text, frame, surface-unit and text-unit formatted; the other touched files are clean except pre-existing diffs I did not touch (leading blank line in four test files, `UiIntent` literals in dispatch).
5. `#[path]` resolution, `include_str!` targets, TOML dependency paths, JSON parse of `launch.json`, the seed (comments stripped), `📋️build-owners.json`, `🔒️dependencies.json`: all fine. Longest new source path 107 bytes after the repo prefix.
6. Consumer scan: 110 files mention `ui_render`; every name they use (96) is exported by the facade; no consumer calls `TextSystem::new` or `::default` except the layout engine I updated. Doc links crossing crates were rewritten (checked with a script, 0 unresolved first segments).
7. Real cargo: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-ui-render` (baseline before edits), `-p …-render-packet` (mid), `-p semio-framework-ui-render` (end), and `-- update --workspace --offline`: all exit 1 with `Finite original maintenance authority required` from `retireInactiveCargoStoreV1`, before any compile (`w4ui-cargo-baseline.log`, `w4ui-cargo-check-packet.log`, `w4ui-cargo-update.log`, `w4ui-cargo-check-end.log`). `workspace:verify -- dependencies literal-external` failed twice with `Original selected owner registry changed`; `workspace:verify -- taxonomy` hit my 7-minute cap. None of the Rust tests were run. Do not read this as "tests pass".

## 6. Residual risks

1. No real compile or test run: the new font-port code, the `Infallible` match in `TextFontFiles::set`, the `-p` test selection and the dev-dependency feature (`backend-testing` through `[dev-dependencies]`) are verified only by the oracle's name resolution plus review.
2. `Cargo.lock` (framework and root) and the per-plugin locks that mention `semio-framework-ui-render` need `update --workspace --offline` once the launcher works; the new packages are absent now.
3. `🔒️dependencies.json` was edited by hand following the file's own format; re-run the literal-external gate when the graph route works.
4. Public API: the parent facade now exports exactly what consumers and its own code use; a peer adding a use of another former `ui_render::X` name (for example `ElementId` families beyond the exported ones) gets a clear unresolved-import error and should import from the producer crate.
5. Layout editor engine: emoji and symbol fallback is gone for stories (see 3); it still embeds `MapLabelSans.ttf` (canvas carve).
6. Peer `ui-runtime` is being carved concurrently in the same tree; I did not touch it.

---

# Addendum: layout engine gets the built-in text fonts again (coordinator request)

This supersedes section 3's "Hosts wired" paragraph and risk 5: the layout editor engine no longer calls `without_builtin_fonts()` as its only mode. Fallback behaviour is the old one whenever the built-in fonts are available to the process.

## Which hosts exist

A manifest scan finds exactly one dependent of `semio-s-artifact-layout-layout-engine`: the layout editor package, whose only production use is the PDF export operation (`⚙️engine/📤️export`, `pdf_engine.get_or_insert_with(LayoutEngine::new)`) plus ~35 test call sites. The editor's wasm-bindgen bridge was deleted earlier (`✏️editor/🌉️wasm/🦀️.rs` is a 4-line tombstone), the interactive display list "never shapes paragraphs and never loads a font", and the wgpu renderer worker does not link the engine. So there is no browser or storybook process that links the engine, and no static route is involved. The hosts are: native processes and tests (disk door), and the plugin component (`wasm32-wasip2`, no file system).

## Wiring (same shape as the compiler fonts)

In `✏️s/🔌️plugins/📏️layout/…/✏️editor/⚙️engine/🎬️scene/🦀️.rs`:
- `install_text_fonts(TextFontSet<'static>)`: process-wide slot, registers the set first so a refused set does not consume it (`TextFontsInstallError::{Unregistrable, WrongFontCount, AlreadyInstalled, Unavailable, Unreadable}`).
- `install_text_font_bytes(Vec<Vec<u8>>)`: fifteen files by value in `TextFontRole::ALL` order, built with `TextFontSet::try_from_loader`, buffers leaked once (the door for a host that fetched the files).
- `install_native_text_fonts(Option<&Path>)` (not on `wasm32`): reads the catalog with `TextFontFiles::read`; directory resolution is the argument, then `SEMIO_TEXT_FONTS`, then the `🖼️assets/🔤️fonts` catalog above the working directory or the executable.
- `LayoutEngine::new()` builds its `TextSystem` from the installed set. A native process that installed nothing resolves the catalog once through the native door at the first engine, so the ~35 existing native and test call sites and the export operation keep the old behaviour with no edits; a failed resolution prints one `[TRACE]` line and falls back to the layout font alone. `LayoutEngine::with_text_fonts(&TextFontSet)` is the explicit constructor.

Tests added to `🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (written, not executed, see below): `an_engine_built_from_a_host_font_set_has_the_built_in_families` (Latin glyph ids non-zero and equal to a plain `TextSystem::new` over the same set, emoji and ZWJ fallback samples equal to that reference, an engine without the set observably differs), `the_layout_font_still_shapes_through_the_engine_with_the_built_in_set_installed`, `new_uses_the_fonts_the_host_installed` (installs a disk-loaded set, then `LayoutEngine::new()` carries the built-in sans), wrong file count and non-font refusal, and a missing native catalog being reported.

## Gap that I could not close

The `wasm32-wasip2` plugin component has no file system and nothing installs bytes into it: `install_text_fonts`/`install_text_font_bytes` exist, but no code runs them in the guest. The platform has a candidate channel, `InstanceOpen.assets` (preloaded package assets, the `read-asset` replacement in `🔌️plugin/🧬️schema/📜️.wit`), but no guest-side reader exists anywhere in the repo (nothing reads `open.assets`; the hosts I found open instances with empty asset lists), and the builder, reactor and guest runtime that would carry it are peer-owned `plugin` crates. Until that channel exists, the PDF export inside the wasip2 guest shapes with the layout font alone (before: with Anta/Kelly/Share Tech/Noto fallback). That is the one place the regression remains; it needs a decision from the plugin owners (declare the fifteen files as package assets and have the layout app's open handler call `install_text_font_bytes`). Native hosts and tests are unchanged.

## Verification run for this addendum

- `bun test ./🧪️tests/📦️crate-boundaries/🟦️.ts` in `🖱️ui`: 59 pass, 0 fail, 2482 expects (re-run after the change).
- TS tests: `🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts` (the one UI TS test that reads ui-render sources): 9 pass, 0 fail, 113 expects. The two layout plugin TS tests (`🎚️renderer-contract`, `🛍️canvas-catalogue`) are vitest files and fail to load under `bun test` (`Explicit Vitest policy required`, `vi.hoisted is not a function`), before and independent of this change; `interactivityMountedPreparedRenderSelfTests` could not run because a peer's taxonomy change breaks `loadCatalogTaxonomy` (`generatorContracts["assets-build"].outputRoots must be unique and lexically ordered`). The frame and scene files it reads are untouched apart from imports.
- rustfmt parses the changed scene file and its tests (remaining diffs are pre-existing style in other lines). The Rust tests, the differential for the plugin crate (every neighbour crate is an unresolved external there) and a real compile were not possible; the launcher failure is unchanged.
- Files changed: `✏️s/🔌️plugins/📏️layout/…/⚙️engine/🎬️scene/🦀️.rs` and its `🧪️tests/🔬️unit/🦀️.rs`.

---

# Addendum 2: the wasm guest gap (package assets through `InstanceOpen.assets`)

Coordinator request: declare the fifteen fonts as package assets of the layout plugin, add the smallest guest-side reader, install them in the layout app's open handler, test both sides. Result: the chain is written end to end for the native Rust hosts and the guest; the browser kernel host and the descriptor regeneration are not done. Nothing here was compiled or executed (launcher still fails with `Finite original maintenance authority required`: `bun repo cargo … check -p semio-framework-os-mcp` at the end, 2 occurrences in `🗑️generated/w4ui-cargo-check-mcp.log`).

## What the survey found

- Precedent for declaring package assets: `PluginBuilder::asset(AssetDeclaration { name, media_type, size_bytes, sha256 })` (example bodies use it); declarations end up in `PackageDescriptor.assets`.
- Host delivery was not there for Rust hosts: both native hosts that open a guest (`🌉️mcp/🏠️workspace/🔖️plugin-binding-and-activation`, `📨️plugin-artifact-channel`) built `assets: Vec::new()`. The browser kernel (`🎠️kernel/🟦️.ts`, `defaultGuestSlimAssetFetcher`) delivers one global asset (`guestslim-typst-fonts`) to every actor.
- No guest-side reader existed: the reactor's `InstanceOpen` arm destructured `{ request, app_id, actor, quotas, .. }` and dropped the assets.

## What was written

1. Declaration. `text_font_asset_declarations()` in the layout engine's `🎬️scene/🦀️.rs` (name = the repository-relative path of the font file, `TEXT_FONT_ASSET_ROOT` + `TextFontRole::asset_path()`; exact size and SHA-256 from a constant table taken from the independently generated fixture; media type `Text`/`Document`, the closest the media model has). Re-exported by the editor crate and applied in `🌎️hub/🧩️compositions/📏️layout/🦀️.rs` `plugin()` with `.asset(…)` for each.
2. Guest reader (plugin crate, kept minimal, in the reactor next to `host()`): `reactor::package_asset(name) -> Option<&'static [u8]>` over a thread-local `PACKAGE_ASSETS`, filled by `admit_package_assets` from the `InstanceOpen` arm in `⚛️reactor/🔄️turn/🦀️.rs` (one added destructured field and one call, before the instance metadata and before `plugin_open_actor_instance`, so the app factory sees them). First delivery of a name wins, bytes are leaked for the process (one copy, no second copy in the font set). Plus the testing hook `admit_package_assets_for_testing` (`cfg(any(test, feature = "artifact-app-testing"))`).
3. Open handler. `LayoutPlayApp` (an `ArtifactEditor`, constructed with `Default` when an instance is created, which happens inside the same open turn) now has a manual `Default` that calls `install_package_text_fonts(package_asset)`: success or `AlreadyInstalled` or `PackageAssetMissing` is silent (a host that delivers nothing keeps the native door or the layout font), any other error prints `[ERROR]`.
4. Host delivery (native Rust hosts). New helper (first written in the mcp crate, moved in addendum 3 to `semio_framework_plugin_host::package_assets`): `package_file_assets(repo_root, &descriptor.assets)` returns the declared assets that are existing files below the repository root, verified against the declared size and SHA-256 (a mismatch is refused by name; names that are no file, absolute or containing `..` are skipped, which leaves example bodies alone). Both mcp open sites now pass the result instead of `Vec::new()` (the channel resolves the root with `find_repo_root()`).
5. Also fixed: the editor crate root had `use …;` before two `#![allow(…)]` inner attributes (a hard error at HEAD, `rustfmt` reported it); the `use` now follows them.

## Tests written (not executed)

- Host side, `📦️package-assets/🧪️tests/🦀️.rs`: declared file assets arrive with their exact bytes in declaration order; non-file, absolute and `..` names are skipped; size and hash mismatches are refused by name.
- Declaration side, scene unit tests: `the_layout_package_declares_every_built_in_text_font_as_an_asset_matching_the_files` (15 declarations, names derive from `asset_path`, size equals the file, SHA-256 equals the fixture's independent value, unique names).
- Guest side: reader laws in `⚛️reactor/🧪️tests/📦️package-assets/🦀️.rs` (delivered name readable, never-delivered name absent, first delivery wins and new names add); `an_engine_shapes_with_real_glyphs_from_the_fonts_the_host_delivered_on_open` and the missing-delivery naming test in the scene tests; and `package_fonts::…` in the editor unit tests, which admits the declared fonts through the testing hook, runs `LayoutPlayApp::default()`, builds `LayoutEngine::new()` and shapes a story with non-zero glyph ids.
- Plugin-host test component (`🧪️testing/🧩️component`, the neutral counter fixture): not applicable. It is a prebuilt `dist` wasm without the layout app, and building a variant that reads assets needs the blocked launcher. So there is no test that drives a real wasm `InstanceOpen` into the layout guest.

## Verification actually run

- `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` on the new mcp helper, native and `--test`: only the external-crate errors (`semio_framework`, `framework_hash`); no local resolution error. Differential of the layout scene file against HEAD: only external-crate differences.
- rustfmt parses every changed or new file.
- UI crate-boundary gate (`bun test --timeout 180000 ./🧪️tests/📦️crate-boundaries/🟦️.ts`): 59 of 59 pass (the first runs timed out at the default 5 s under machine load; the render rows were never among the failures). `♻️physical-job-close`: 4 of 9 pass now, 5 clipboard dispatch tests fail; they read `🧠️runtime` clipboard sources a peer's ui-runtime carve is changing (new staged crates under `🧠️runtime`), not anything in this change.

## Still open (explicit)

1. Browser kernel host: `ActivationRegistry.loadAssets` is per kernel, not per package, and fetches only the guestslim asset. Delivering the layout fonts there needs the manifest entry to carry the declared assets and a static route serving the font catalog (like the compiler fonts route); I did not touch the kernel or the dev servers. In a browser the layout guest therefore still shapes with the layout font alone, which is the one remaining behaviour difference.
2. Descriptors: `🌎️hub/🧩️compositions/📏️layout/🧩️component/🔣️.json` and `🛂️.descriptor.semio` are generated artifacts that still say `assets: []`; they need regenerating (and the `dist` components rebuilt) once the build route works.
3. Other native hosts that open guests (`🏃️run`, the renderer scale bench, tests) still pass empty assets; only the two mcp headless hosts deliver package file assets.
4. Everything above is unexecuted; the first real run should be the three new test files plus a headless open of the layout plugin through `plugin-binding-and-activation`.


---

# Addendum 3: delivery paths (browser kernel, native hosts, descriptors)

## Browser kernel

Contract kept the same as native: the descriptor declares, the kernel verifies size and SHA-256, the bytes reach `instance-open`.

- `🎠️kernel/📦️package-assets/🟦️.ts` (new): `PACKAGE_ASSET_ROUTE` (`/renderer-modules/package-assets`), `PACKAGE_ASSET_DIRECTORIES` (the font catalog), `packageAssetMounts()`/`packageAssetDeliveries()` for servers, `packageAssetUrl(name, base)` (only names below a served directory, no empty/`.`/`..` segments), `fetchPackageAssets(declared, base, fetch, digest)` (fetch each served declared file, reject by name on HTTP failure, wrong size or wrong SHA-256; skips names outside the served directories), `readPackageAssetDeclarations(descriptor)` (admits only well-formed rows). It follows the compiler-fonts pattern (a static route per directory) but exposes only the declared directory, not the repository, because the route embeds the directory path.
- `🎠️kernel/🟦️.ts`: `PluginPackageDescriptor.assets?` filled by `fetchPackageDescriptor` from the served `🔣️.json`; `PluginCatalogTarget.assets?` and `ActivationManifestEntry.assets?` (`registerCatalog` copies them); `ActivationRegistryOptions.fetchPackageAssets` (default: the page's origin through the route, nothing without a page location); `ActivationRegistry.loadPackageAssets` (once per plugin; a refused or failed delivery is reported with `console.error` and the plugin activates with none, like a host that delivers nothing); the three activation sites pass `[...guestslim assets, ...package assets]` to `ShardClient.activate`. The existing browser-bundle worker already splices activation assets into the first `instance-open` (`spliceInstanceOpenAssets`), so no worker change was needed.
- `…/🧊️wgpu/🧩️plugin-modules/🟦️.ts` (`prepareWgpuPluginModules`) copies `descriptor.assets` into the catalog rows the wgpu boot registers; `…/🧊️wgpu/🌐️server/🟦️.ts` mounts the package asset directories next to the runtime asset channels; the os dev builder (`🧑‍💻dev/🏗️builder/🌐️vite`) adds `...packageAssetDeliveries()` after `...runtimeAssetDeliveries()`.
- Test, `🎠️kernel/🧪️tests/📦️package-assets/🟦️.ts` (vitest, added to the kernel config include list): route/mount/URL laws; descriptor row admission; real font fetch through a fake server that reads the repository files, with exact byte equality against disk and the independently generated fixture sizes/hashes; size, hash and HTTP refusals; and the end-to-end chain: `registerCatalog` with declared assets, `ActivationRegistry.activate` posting through the real `ShardClient` to a capturing worker, then the worker's own `spliceInstanceOpenAssets` (extracted from its source text, so the shipped code runs) producing an `instance-open` whose `assets` hold the exact bytes; a refused delivery activates with no assets; a plugin without declarations fetches nothing.
- `🧩️wgpu-module-routes` test updated for the new mounts.

## Native hosts

- The helper now lives in the host crate all native hosts share: `🔌️plugin/🖥️host/🎁️package-assets/🦀️.rs` (`semio_framework_plugin_host::package_assets`): `package_file_assets`, `repository_root_above` (marker `nx.json`), `declared_assets_in_descriptor_json`, `package_assets_beside_module(<dir>/<file>.wasm)` (reads `<dir>/🔣️.json`). Tests in `🎁️package-assets/🧪️tests` (exact bytes and order, skip of non-file/absolute/`..` names, size and hash refusal, a module descriptor beside a wasm). The mcp copy was removed; both mcp open sites call the host crate.
- `🏃️run/🦀️.rs` `open`: new `package_assets_for(plugin_id)` reads the committed descriptor (which already holds `assets` once regenerated) and delivers its file assets.
- Renderer native kernel pool (`📦️kernel-runtime/🏊️kernel-pool` `create_app`): `package_assets_beside_module(&wasm_path)`.
- Scale bench: not changed on purpose. Its `RegistryRecord` rows are synthetic scale fixtures with no module descriptor and declare no assets, so there is nothing to deliver; it keeps `Vec::new()`. Test fixtures that build `InstanceOpen` by hand are unchanged for the same reason.

## Descriptors

`🌎️hub/🧩️compositions/📏️layout/🧩️component/🔣️.json` and `🛂️.descriptor.semio` are produced by describing the built layout component, so they need a rebuilt wasm (cargo) first. Not regenerated. Attempt: `bun repo run @semio-tech/os-plugin-describe-rs:describe` failed before any work with `Active graph readers retain required Nx pool headroom` (`🗑️generated/w4ui-describe.log`); the cargo launcher is still blocked. Exact commands for the next pass, in order: `bun repo run workspace:components -- iterate layout` (rebuilds the layout component, describes it, stages it), or for the whole set `bun repo run workspace:components --all -- describe` after the component builds; then `bun repo run @semio-tech/plugin-registry:generate`. After that the descriptor's `assets` carry the fifteen fonts and every delivery path above activates.

## Verification actually run

- Kernel vitest suite (`vitest run --config 🎠️kernel/🧪️tests/🎚️config/🟦️.ts`, run through the repo's vitest binary with an explicit owner policy): 9 files, 89 tests pass, including the 7 new ones.
- wgpu `wgpu-module-routes` + `wgpu-runtime-assets` (vitest): 2 files, 12 tests pass.
- Rust: rustfmt parses every changed file; `rustc --emit=metadata` on the new host module (native and `--test`) shows only external-crate errors. Nothing compiled or executed with cargo.

## Still open

1. The React renderer's activation path (`🧱️elements/🔌️PluginRuntime/🛰️runtime-singletons`) builds its catalog from the generated registry rows, which carry no descriptor assets, and no server mounts the package route for it; that path still delivers none. The wgpu boot (descriptor-fetch path) and the kernel contract are done.
2. Rust tests and the descriptor regeneration (above) are unexecuted; the first run should be `semio-framework-plugin-host` (`package_assets`), the plugin reactor reader tests, the scene/editor font tests, then a headless open of the regenerated layout component.
