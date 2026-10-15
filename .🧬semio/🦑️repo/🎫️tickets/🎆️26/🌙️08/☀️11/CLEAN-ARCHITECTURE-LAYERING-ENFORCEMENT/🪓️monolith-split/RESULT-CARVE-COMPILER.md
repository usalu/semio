# 🔤️ Carve Result: `semio-framework-compiler` (fonts out of the crate)

Executor: Sonnet, foreground, no sub-agents. Rules followed: `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md` (rule 3, embedded assets), `📓️RESOLUTION-ORACLE.md`.

## Outcome

- The five `include_bytes!` fonts (1,469,864 bytes) are gone from the crate. `semio-framework-compiler` now passes the compiler-ingress source boundary and is `closureCompilable` (tier-0 leaf unlocked).
- Fonts are delivered by the host through an explicit port: a `FontSet` of byte slices, parsed once into `CompilerFonts` and passed to every `compile_*_to_svg` call as the first parameter.
- The font files did not move and are byte-identical (`git status` on `🌍️world/🔤️fonts` is empty). Reason: `🔲️pixels`, `🚪️io/🔢️binary/📥️source` and the taxonomy already reference those paths, and the `🖼️assets/🔤️fonts` catalog is a Google-Fonts TrueType catalog (`ttf/woff/woff2` encodings, generated per-subset Rust crates that themselves `include_bytes!`) that cannot describe CFF `.otf` or the CBDT emoji subset.
- No sub-agent or ticket tool was used. The ticket is left to the coordinator.

## Design of the font port

All code is in `🧰️framework/🔨️modules/📚️compiler/`.

### `🌍️world/🦀️.rs` (port, std only)

| Item | Role |
| --- | --- |
| `FontRole::{Math, Serif, SerifItalic, Mono, Emoji}` | stable role keys (unchanged) |
| `FontRole::ALL`, `FontRole::asset_file_name()` | the asset identity a host resolves through its own channel (for example `🧮️LibertinusMath-Regular.otf`) |
| `FontSet<'a>` | borrowed `&'a [u8]` per role; was `&'static [u8]` from `embedded_fonts()` (removed) |
| `FontSet::get(role)` | unchanged |
| `FontSet::try_from_loader(\|role\| -> Result<&[u8], E>)` | host-neutral constructor: file system, fetch, packaged asset, wasm `read-asset` seam |
| `FontSet::<'static>::from_directory(&Path)` | native delivery; reads the five files, leaks the buffers once for process lifetime, errors name the missing file |

### `🦀️.rs` (facade)

| Item | Role |
| --- | --- |
| `CompilerFonts<'a>` | parsed Math/Serif/Mono/Emoji faces (replaces the private `OnceLock<Fonts>` + `embedded_fonts()`); built by `CompilerFonts::parse(&FontSet<'a>) -> Result<_, FontParseError>` |
| `FontParseError { role }` | names the role whose bytes are not a font |
| `compile_{snippet,text,emoji,code}_to_svg(fonts: &CompilerFonts<'_>, ..)` | explicit parameter, first position, same output |
| wasm32-wasip2 arm | `CompilerFonts` is a `PhantomData` unit with `parse` (always `Ok`) and `const fn estimate_only()`; the four functions take `_fonts` and keep the character-count estimate. Same API on every target. |

No global state remains in the compiler. Two independently parsed `CompilerFonts` over the same set produce identical SVG (test below).

### `🖼️canvas/🦀️.rs` (the only caller of the compiler)

`canvas` is the single crate that depends on `semio-framework-compiler` (Cargo.toml scan over all tracked manifests). Its public icon entry `icon_codec::resolve_icon_kind(encoded, themed_lookup)` is called from `os-infinite` (`IconPaintCache::get_or_build` and the DAG `preview_media_natural_size`), from canvas's own tests and from a puzzle test, and `os-infinite` is a protected active crate. Changing that signature would force edits through every `IconPaintCache::new()` construction site (four in the infinite tree) and the board state above them. So canvas receives the fonts through a host port of its own:

- `icon_codec::install_compiler_fonts(FontSet<'static>) -> Result<(), CompilerFontsInstallError>`: parses first (a refused set does not consume the slot), then stores once. Errors: `Unparseable { role }`, `AlreadyInstalled`. On wasm32-wasip2 it is a no-op `Ok(())`.
- Re-exports `FontSet`, `FontRole`, `CompilerFontsInstallError`, `install_compiler_fonts` (explicit `pub use`) so hosts never name the compiler crate.
- `resolve_math_src`, `resolve_emoji_body`, `resolve_text_body` read the installed fonts. Until a host installs them these three icon kinds resolve to `ResolvedIcon::None` (the pre-existing graceful-degradation value for bad notation). On wasip2 the estimate arm needs no fonts and keeps working with no install.

## Files changed

Compiler (`🧰️framework/🔨️modules/📚️compiler/`):
- `🌍️world/🦀️.rs` (rewritten: port, no `include_bytes!`, `embedded_fonts()` removed)
- `🦀️.rs` (facade: `CompilerFonts`, `FontParseError`, parameterised `compile_*`)
- `🌍️world/🧪️tests/🔬️unit/🦀️.rs` (rewritten; also hosts `pub(crate) fn test_fonts()`)
- `🌍️world/🧪️tests/🧫️fixtures/🔣️.json` (new, language-neutral)
- `🧪️tests/🔬️unit/🦀️.rs`, `📤️svg/🧪️tests/🔬️unit/🦀️.rs`, `🔤️text/🧪️tests/🔬️unit/🦀️.rs`, `🧮️math/🧪️tests/🔬️unit/🦀️.rs` (load fonts through `crate::world::tests::test_fonts()`)

Callers:
- `🧰️framework/🔨️modules/🖼️canvas/🦀️.rs` (port + re-exports)
- `🧰️framework/🔨️modules/🖼️canvas/🧪️tests/🔬️icon-codec-unit/🦀️.rs` (install helper, 3 new tests, 2 tests install first)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🧪️tests/🔬️unit/🦀️.rs` (`typst:$x^2$` and `emoji:` tests install the fonts first)

Other: root `Cargo.lock` was refreshed twice with `bun repo cargo … -- update --workspace --offline` (rule 6) so the launcher could run. The diff only adds peers' new members (`semio-framework-server-*`, `semio-framework-pets-*`); no compiler or canvas entry changed. No manifest, launch route, owner row or corpus row changed (no new crate). The five font files, license texts, `Cargo.toml` and TS/wasm bindings are untouched (no TS or wasm binding references `embedded_fonts`, `FontSet` of the compiler, or `compiler_world`; searched with `git grep`).

Tests added or changed, all loading the fonts from disk through the new path (`FontSet::from_directory(CARGO_MANIFEST_DIR/…/🌍️world/🔤️fonts)`, inside `#[cfg(test)]`, excluded from the gate):
- world: `font_assets_loaded_through_the_port_match_the_shared_fixture` (role, file name, byte length, sfnt tag, bytes equal disk), `font_set_get_matches_field_by_role`, `font_roles_name_distinct_asset_files`, `try_from_loader_asks_every_role_in_order`, `try_from_loader_stops_at_the_first_failure`, `from_directory_names_the_first_missing_asset_file`, `compiler_sources_embed_no_font_bytes` (scans all non-test `.rs` of the module for the byte-embedding macro).
- facade: `independently_parsed_font_sets_compile_identical_svg`, `parse_names_the_role_whose_bytes_are_not_a_font`; existing compile tests now pass `fonts()`.
- canvas: `icon_codec_resolves_math_to_svg_through_the_installed_fonts`, `icon_codec_refuses_unparseable_compiler_fonts_and_a_second_install`.
- Third-party validation: `rustybuzz` parses every font and the existing metric/glyph/MATH-table/CBDT tests (text, math, svg) run over the disk-loaded bytes. The fixture row values (bytes, sfnt tag) were additionally checked independently with Python against the files (all 5 match). Output cannot change: same bytes, same parser, same layout code.

## Census (canonical counter `readRustSourceClosureV1`, `ONLY=… bun census.ts`)

| Crate | Files | Bytes | Active lines | Embedded | Over limit |
| --- | --- | --- | --- | --- | --- |
| compiler before | 16 | 1,580,756 | 2,411 | 5 files, 1,469,864 B | yes (bytes, ratio 6.03) |
| compiler after | 11 | 113,431 | 2,450 | 0 | no, `closureCompilable: true` |
| canvas before | 318 | 1,992,623 | 5,619 | 311 files, 1,653,610 B | yes (files) |
| canvas after | 318 | 1,995,609 | 5,674 | 311 files, 1,653,610 B | yes (files, unchanged) |

Compiler is within the carving target (at most 3500 lines, 28 files, 230,000 bytes). Canvas stays over for its own reasons (311 embedded icon files, lines); my change adds 55 lines and 2,986 bytes there and removes nothing that helps. Its own carve (icons out, then lines) is still a separate job. Logs: `🗑️generated/carve-compiler-census-{before,after}.log`.

## Verification (real results)

Static, analysis-only (`rustc +nightly-2026-07-20 --emit=metadata`, output to a scratch dir outside the repo, no cargo workspace built):

1. Full-resolution differential, compiler crate, original tree (copy of the module before my edits) versus current, run from `📦️packages/🦀️rust/🦀️.rs`, four configurations:

| Config | Error multiset |
| --- | --- |
| native | identical (22 = 22) |
| `--test` | only +2 `semio_framework_pack_json` unresolved (external dev-dependency, same class the old syntax tests already show) |
| wasm32-wasip2 | identical (15 = 15) |
| wasm32-wasip2 `--test` | only +2 `semio_framework_pack_json` |

   No new E0412, E0425, E0422, E0423, E0603, E0616, E0624 or E0432 against first-party items.
2. Same differential for canvas (sandbox with the HEAD versions of the two canvas files restored, sibling modules symlinked), native, `--test`, wasip2, wasip2 `--test`: the only difference in each is +4 `E0433 cannot find … crate compiler` (the four new `compiler::` paths; external crate, expected).
3. Because unresolved external crates stop typeck early, I also type-checked the new code against real signatures with harness crates (stubs for `rustybuzz`-dependent modules and `pack_json`, the real `🌍️world/🦀️.rs` and `🦀️.rs` mounted by `#[path]`): compiler native, `--test`, wasm32-wasip2, wasip2 `--test`, wasm32-unknown-unknown, and `--test` for unknown-unknown: clean. The canvas hunk (port, resolvers, new tests, with the harness compiler as `--extern`) and the puzzle test helper type-check on native, native `--test`, wasip2, wasip2 `--test`. Limitation: these stubs have no field invariance, but the real run below covers that.
4. rustfmt (repo `rustfmt.toml`, `skip_children`): all files I wrote or changed parse; world, world tests, facade and facade tests are `--check` clean. The canvas test file only shows two pre-existing diffs elsewhere in the file.
5. Path resolution: 24 `#[path]`/`include_*!` literals in the compiler module and those of the two canvas files resolve, 0 missing. Fonts: `git status` empty.

Real cargo, via the launcher (`bun repo cargo --store=shared --configuration=prototype`):
- At the start the launcher failed with "Live Original process issuance required" (baseline, before my edits). Later it got past that and stopped on a stale lock; after the offline workspace update it worked:
- `check -p semio-framework-compiler --message-format=short`: exit 0, `Finished prototype profile in 2m 38s`, real `rustybuzz`, `base64` and `semio_framework_*` dependencies, zero warnings or errors in the compiler crate. This type-checks the whole new non-test port on the native target (log `carve-compiler-cargo-check.log`).
- `check -p semio-framework-compiler --tests` compiled the lib-test target for real: 8 errors, all `unresolved import/crate semio_framework_pack_json`, in the old syntax tests (`📖️syntax/🦀️rust/📁️paths`, `📤️generation`) and in my new world test. The cause is the launcher invoking the root workspace, where the compiler is an `exclude`d path-only dependency, so cargo ignores its `[dev-dependencies]`; this fails identically for the pre-existing tests and is not a new regression. Nothing else in that run errored. It reported three `unnecessary qualification` warnings in my world test (`std::fs::…` where `fs` was already in scope); I fixed them afterwards (`use std::{fs, io, …}`) and re-checked with the harness (no warnings, native and `--test`), but that last edit was not re-run through cargo.
- `test -p semio-framework-compiler [--lib]` and `check --tests --manifest-path 🧰️framework/Cargo.toml` (the framework workspace, where the dev-dependency resolves) could not run: from then on every launcher call stops with `Finite original maintenance authority required` in `retireInactiveCargoStoreV1` before compiling (about 20 attempts over 30 minutes; the peer-edited `🏃️process/🧭️routing/📥️invocation` files are modified in the working tree). This is launcher infrastructure, not the crate. Logs: `carve-compiler-cargo-{test,test2,test-fw,check-tests,check-tests-fw}.log`.

Not verified by running: the Rust unit tests (compiler, canvas, puzzle) have not been executed in this session. Everything above is type-check evidence plus the independent fixture check. Do not read it as "tests pass".

## Open items (explicit, for the next owner)

1. Host wiring (not done). No production host installs the fonts yet, so math, emoji and text icons resolve to `ResolvedIcon::None` in native and browser hosts until they call `semio_framework_canvas::install_compiler_fonts(FontSet<'static>)` once at boot (`os-renderer-wgpu` is protected and mid-split, so I did not edit it):
   - native entry `run_native_entrypoint` in `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`: `FontSet::from_directory` over the shipped `🌍️world/🔤️fonts` directory;
   - browser: the page fetches the five files named by `FontRole::asset_file_name()` and hands them to a new `semioWgpuSetCompilerFonts` export next to `semioWgpuSetHostLocale` (build with `FontSet::try_from_loader`, leak the `Vec`s).
   The icon paint path is `infinite` → `IconPaintCache::get_or_build` → `resolve_icon_kind`.
2. Canvas still embeds its own `😀️NotoColorEmoji-subset.ttf` (byte-identical to the compiler's, 152,308 bytes) and `🔤️MapLabelSans.ttf` through `icon_assets`; that is the canvas carve, which can reuse this port.
3. The launcher `Finite original maintenance authority required` failure blocks `cargo test` for everyone; re-run `bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-compiler --lib --manifest-path 🧰️framework/Cargo.toml` once it clears, plus `test -p semio-framework-canvas` once canvas fits the gate.

---

# Addendum: host wiring (coordinator request, carving rule 7)

The "Open items 1" gap is closed: every host that runs non-wasip2 icon code now installs the compiler fonts before the first icon resolves. Which hosts exist was established by dependency scan: `semio_framework_canvas::icon_codec::resolve_icon_kind` is reached only through `os-infinite` (`IconPaintCache::get_or_build`), so the processes that can resolve a math, emoji or text icon are the wgpu renderer (native winit host, headless smoke mode, browser frame Worker) and the Flow browser wasm. Plugin workers and plugin components are `wasm32-wasip2`, whose arm estimates sizes without fonts (nothing to install); `print`, `os-mcp`, `server` and the React renderer do not link canvas.

## Design

One port, three doors, no global default:

| Host | Delivery channel (existing) | Door | Where it is called |
| --- | --- | --- | --- |
| Native renderer (`run_native`, `run_smoke_with_axes`) | files on disk: the runner sets `SEMIO_COMPILER_FONTS` next to `SEMIO_PLUGIN_MODULES` and `SEMIO_ASSET_BASE_URL`; without it the repository directory is found above the executable, the module root or the working directory | `compiler_fonts::install_native_compiler_fonts()` | first statement of `run_native` and of `run_smoke_with_axes`, before the temporal formatter, the event loop and plugin load |
| Browser renderer (frame Worker) | the dev servers' static-dir asset route `/renderer-modules/compiler-fonts` | wasm export `semioWgpuSetCompilerFonts` | `boot()` in the frame Worker: after the renderer wasm instantiates, before `semioWgpuWorkerBootstrap` |
| Flow browser wasm | the embedding host's own channel | wasm export `flowSetCompilerFonts`, SDK option `compilerFonts` of `createFlowBrowserRuntime` | before the first session opens |

Single sources: `FontRole::asset_file_name` (Rust) is mirrored by `COMPILER_FONT_FILES` in `📚️compiler/🌍️world/🟦️.ts`, guarded by a test that compares both against the shared fixture and the Rust source text.

## Files changed for the wiring

Rust:
- new `…/🧊️wgpu/🔤️compiler-fonts/🦀️.rs` and `🔤️compiler-fonts/🧪️tests/🦀️.rs` (native resolution and install, the wasm export, tests)
- `…/🧊️wgpu/🧊️renderer/🦀️.rs` (one `#[path]` mount) and `…/🧊️renderer/🔬️native-socket-probe/🦀️.rs` (two one-line calls)
- `🖼️canvas/🦀️.rs` (`install_compiler_font_bytes`, re-exported) — leaks buffers received by value
- `🌊️flow/🕸️wasm/🦀️.rs` (`flowSetCompilerFonts`)
- `🌊️flow/🫀️core/📦️packages/🦀️rust/🟦️.ts` (export list)

TypeScript:
- new `📚️compiler/🌍️world/🟦️.ts` (route, directory, file list, `installCompilerFonts`)
- `…/🧊️wgpu/🎞️frame-worker/🟦️.ts` (import, binding type, one boot step; a failed fetch is reported as a `compiler-fonts-fault` progress message and does not abort the boot)
- `…/🧊️wgpu/🌐️server/🟦️.ts`, `🌐️server/🎚️config/🟦️.ts` (`fontRoot` mount `/renderer-modules/compiler-fonts`)
- `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` (wgpu dev mount), `.storybook/📖️stories/🧭️coordination/🟦️.ts` (static-dir row)
- `…/⌨️native-entrypoint/📜️script.ts` (`SEMIO_COMPILER_FONTS`)
- `🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.js` and `📝️declaration/📤️projection/🟦️.ts` (option `compilerFonts`)
- taxonomy `🔣️taxonomy.json` (one line in the `wgpu-frame-worker` `sourceModulePaths` allow-list) and `🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` (one Nx input)

Tests and fixtures:
- new `🧑‍🎨engine/🧪️tests/🔤️wgpu-compiler-fonts/🟦️.ts`, registered in `🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` and in `test-browser-worker`
- new `🌊️flow/🕸️wasm/🧪️tests/🔤️compiler-fonts/🟦️.ts`
- `🧪️tests/🧫️fixtures/🧩️wgpu-module-routes/🔣️.json` (new mount row), `⌨️native-entrypoint/🧪️tests/📇️catalog/🟦️.ts` (env assertion), `🦑️repo/…/🧊️browser-serving/🟦️.ts` (the config literal gains `fontRoot`)

## Verification of the wiring (real results)

Rule applied: no `.rmeta` stubs or written outputs; the `/tmp`-stub harness from the first pass was discarded by the coordinator and none of its results are used below.

TypeScript (run, passing):
- `vitest run --config 🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` on `🔤️wgpu-compiler-fonts` and `🧩️wgpu-module-routes`: 2 files, 11 tests passed (first run 9, after adding the storybook and fault-tolerance oracles 11). The font test proves: the TS file list equals the Rust fixture rows, the Rust `asset_file_name` source text and the files on disk (sizes); `installCompilerFonts` fetches the five URLs (`/renderer-modules/compiler-fonts/<encoded file>`) and calls the door once, in role order; no door means no fetch; one 404 rejects and installs nothing; and the boot-sequence oracle on the frame Worker source orders `instantiateRendererWasm` < `installCompilerFonts(loaded, …)` < `semioWgpuWorkerBootstrap`. The mounts test passes with the new `fontRoot` mount row.
- `bun test` of `⌨️native-entrypoint/🧪️tests/📇️catalog/🟦️.ts`: 1 pass, now also asserting that the native run launches the binary with `SEMIO_COMPILER_FONTS` set to the repository font directory.
- `bun test` of `🌊️flow/🕸️wasm/🧪️tests/🔤️compiler-fonts/🟦️.ts`: 3 pass (install once in role order before any session, refuse fonts without the door, no door call when no fonts offered).

Rust, native "icon resolves after installation" test: `🔤️compiler-fonts/🧪️tests/🦀️.rs::math_emoji_and_text_icons_resolve_after_the_native_install` installs from the repository directory the resolver found above the crate manifest and asserts `math:x^2`, `emoji:🚀`, `text:Hi` resolve to `SvgPlain` with `<path`/`<image`. Two more tests cover directory resolution (found above the crate holding all five roles; explicit directory wins, a bad explicit one is refused, not replaced). They are written and format-clean but NOT executed: the renderer crate is gate-refused and the launcher stopped (below). The canvas twin (`icon_codec_resolves_*`, `…refuses_unparseable_compiler_fonts_and_a_second_install`) has the same status as before.

Rust differential (`rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`, from the real lib roots, sandbox = the same tree with only my three Rust edits reverted so peers' concurrent changes cancel; absolute prefixes normalized):

| Crate / config | Result |
| --- | --- |
| os-renderer-wgpu native | 380 → 381: only `E0432 unresolved import semio_framework_canvas` in the new module (external crate, expected) |
| os-renderer-wgpu `--test` | 1616 → 1618: the same external import in the new module and its test file |
| os-renderer-wgpu wasm32-unknown-unknown, and `--test` | identical (334 = 334, 464 = 464) |
| canvas native, `--test`, wasip2, wasip2 `--test` (re-run with `/dev/null`, after `install_compiler_font_bytes`) | only the same +4 `compiler` crate references as before |
| compiler native / `--test` / wasip2 / wasip2 `--test` (re-run with `/dev/null`) | identical / +2 `pack_json` / identical / +2 `pack_json` |

No new E0412, E0425, E0422, E0423, E0603, E0616, E0624 or non-external E0432. As before, unresolved externals end analysis before type checking, so these runs prove resolution (module mount, `compiler_fonts::` paths, imports) and not types. Types for the compiler crate are covered by the real launcher `check` (exit 0); the renderer, canvas and flow additions have no type-check evidence.

Launcher: `bun repo cargo … check -p semio-framework-os-renderer-wgpu` stopped with `Finite original maintenance authority required` (same infrastructure fault as before, and the crate is gate-refused regardless). `bun repo run @semio-tech/framework-renderer-wgpu:test-browser-worker` stopped earlier with `Canonical native dependency owner required: …/◻️2d/⚙️engine/📦️packages/🦀️rust` (a peer's missing owner), so the TS tests above were run directly with vitest/bun.

Pre-existing faults found, not caused by this change:
- `🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js` fails at start (`🕸️wasm/🧫️fixtures/🚀️browser-startup/🔣️.json` missing); my flow test is a separate file for that reason.
- Bundling the frame Worker in memory (`renderWgpuBrowserBundles`) fails with 48 `import is not schema-owned` errors from `🌱️value/📦️packages/🟦️typescript/🗂️ordered/🔢️numeric` (a moved value package); none mention the compiler module, so the allow-list addition could not be exercised end to end.
- The first-pass items about launcher/maintenance faults remain.

Not run: `🦑️repo/…/🧊️browser-serving/🟦️.ts` (needs the Nx/Vite serving harness; its config literal gained `fontRoot` so it stays consistent with `createWgpuBrowserConfig`), the Rust renderer/flow/canvas/puzzle tests, and any real browser or native window boot. Nothing here has been seen rendering an icon end to end.

## Remaining caveats

1. A packaged native install has no repository above the executable; its runner must set `SEMIO_COMPILER_FONTS` (the dev runner does). Without either, the host boots and prints `[TRACE] native compiler fonts unavailable: …; math, emoji and text icons will not render`.
2. A browser embedding that serves the wgpu renderer but not `/renderer-modules/compiler-fonts` boots without those icons and reports `compiler-fonts-fault`. Served today by: wgpu serve (`🌐️server`), os dev with `SEMIO_RENDERER=wgpu`, storybook coordination. The Flow SDK has no in-repo production consumer; embedders pass `compilerFonts` themselves.
3. Production (`build`) bundles of the wgpu renderer were not changed: the wgpu production path is the Nx wasm target, which is outside the servers edited here.
