# 🧪️ Test-Closure Census of the First-Party Crates (2026-10-16)

Read-only audit by a Haiku auditor, foreground, no sub-agents, no compile, no git write, no source edit.
Tool: `tcc-tools/tcc-census.ts` (Bun, reuses `readRustSourceClosureV1` and `rustCfgMatchesV1` from `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`).
Raw outputs: `🗑️generated/tcc-*` (`tcc-units.tsv` 156,379 rows, `tcc-crates.tsv`, `tcc-overlimit-test-units.tsv`, `tcc-overlimit-devdeps.tsv`, `tcc-overlimit-files.tsv`, `tcc-errors.tsv`, `tcc-validation.tsv`, `tcc-summary.json`, `tcc-run.log`).

## 0. Verdict

- 727 first-party crates measured (every manifest in `🗑️generated/crate-census.csv`). 552 have tests.
- **28 native test units exceed the gate** across **27 crates**. 27 of them exceed a hard limit (4000 active lines, 32 files or 262,144 bytes); `semio-framework-io-base64` exceeds only its own declared budget (4 files, 32,768 bytes, 1,024 lines). Units over on any of the three labels: 82.
- **Oversized first-party dev-dependencies**: `semio-framework-ui` (80 native rows, 66 direct) and `semio-framework-plugin` (10 native rows, 3 direct). 87 crates depend on one of them through a dev edge.
- **11 native test units in 11 crates cannot be measured** (missing `include_str!` JSON fixtures or a missing test module). Each is one unit; the counter refuses it, so its size is unknown. The refusal is not a sizing result.
- The hub workspace (`🌎️hub/Cargo.toml`) is **not** measured: its manifest is not in `crate-census.csv`, and the task only includes it when listed.

## 1. Method

For each crate, three cfg snapshots (native host, `wasm32-unknown-unknown`, `wasm32-wasip2`), computed from `rustc --print cfg` for each target.

Test compilation units measured, one closure per unit through the canonical counter:
- lib under `cfg(test)`, entry `[lib] path` or `src/lib.rs` (kind `lib-test`);
- bin targets, with `cfg(test)` (kind `bin-test`);
- integration targets from `tests/*.rs`, `tests/*/main.rs` and `[[test]]` (kind `integration`), `required-features` honored;
- examples (kind `example`, no `cfg(test)`), listed separately because `cargo test` builds them and `--all-targets` does too.

Dependency graph: a feature-aware fixpoint from the crate root. Normal and build edges everywhere, dev edges only from the root. Optional deps and `dep/feat` tokens are resolved with the same closure rule the canonical census uses. `[target.'cfg(...)'…]` tables are evaluated per label with `rustCfgMatchesV1`, not over-approximated. Feature requests from dependents' edges are unified, as cargo does. A dev-dependency's lib closure is measured under the feature set the test build actually gives it.

Limit per unit: `min(hard, rust-source-boundary metadata)`, as in `census.ts`. Over means the closure exceeds one of its three limits.

"Test-only" = files in the lib-test closure that are not in the crate's normal lib closure (same label).

Wasm: all three labels are measured for every crate. The wasm numbers are upper bounds; no build was attempted, so build-ability for wasm is not verified.

## 2. Validation

- `crate-census.csv` was written at 11:23. 592 of its 727 native lib rows still match the live counter exactly. 135 differ, because the source tree changed after 11:23: peers edited many files and several crates were split (for example `semio-framework-os-services` 5 to 15 files, `semio-framework-os-mcp-http-transport` 2 to 8 files). Spot check: `semio-framework-value` differs from the CSV (25 / 186,338 / 2,784 vs 25 / 186,249 / 2,783), and the canonical `okwp0-tools/closure.ts` run now returns 25 / 186,249 / 2,783, identical to this census. So the CSV is stale, not the measurement.
- The test-closure rule was exercised on the three smoke crates before the full run; error units were examined one by one (section 6).

## 3. Over-limit test units (native), grouped by area and owning slice

Slice owner is taken from `📓️FLEET.md` and the RESULT files. Where no FLEET entry names a crate, the row says so and gives the nearest area.

Columns: files / bytes / active lines of the test unit; `normal` = crate's own lib closure; `test-only` = files not in the normal lib closure; `embedded` = `include_str!`/`include_bytes!` inputs counted toward the bytes budget.

### 3.1 OS kernel and store (slice: OS-KERNEL-ENGINE and OS-KERNEL-SQLITE-HOST, wave 5)

| Crate (unit) | Test unit files / bytes / lines | Normal lib | Test-only (files / bytes / lines) | Companion split |
| --- | --- | --- | --- | --- |
| `semio-framework-os-kernel` (lib-test, ratio 12.07) | 241 / 3,163,369 / 34,332 | 5 / 116,802 / 1,623 (under limit) | 236 / 3,046,567 / 32,667; embedded 142 files, 548,316 B | Move `🏪️store/🧪️tests/**` (and `🔄️sync/🧪️tests`) to an `os-kernel-tests` companion crate. Split the single 1,032,246-byte `🏪️store/🧪️tests/🔬️unit/🦀️.rs` (12,625 lines), which alone exceeds 262,144 bytes. Move the embedded JSON fixtures out of the compiled closure. |
| `semio-framework-os-kernel` (integration `sqlite_snapshot_native_admission`) | 36 / 313,259 / 2,396 | n/a | n/a | Move this integration target into the companion crate. |
| `semio-framework-os-kernel-dsl-derive` (lib-test) | 28 / 316,675 / 2,389 | 2 / 103,743 / 1,481 | 26 / 212,932 / 881; embedded 14 files, 160,276 B | Move the test modules to a `-tests` companion; move the fixtures out. The DSL derive is not named by any FLEET slice. |
| `semio-framework-io-sqlite-snapshot` (lib-test) | 40 / 383,713 / 2,190 | 12 / 239,072 / 1,438 | 28 / 144,641 / 585 | Move test modules to a `-tests` companion. Not named by any FLEET slice. |
| `semio-framework-os-channel` (lib-test) | 26 / 319,607 / 4,889 | 10 / 185,750 / 3,450 | 16 / 133,857 / 1,435 | Move test modules to a `-tests` companion. Named in `RESULT-OS-KERNEL-SQLITE-HOST.md`. |
| `semio-framework-os-pack` (lib-test) | 17 / 312,390 / 4,919 | 6 / 183,645 / 2,825 | 11 / 128,745 / 2,083 | Move test modules to a `-tests` companion. Named in `RESULT-DUPLICATE-MOUNTS-2.md`. |

### 3.2 OS services, MCP, directory (slice: OS-SERVICES-MCP-SPLITS, WAVE7 WP-A)

| Crate (unit) | Test unit | Normal lib | Test-only | Companion split |
| --- | --- | --- | --- | --- |
| `semio-framework-os-services` (lib-test, lines over) | 27 / 318,145 / 5,591 | 15 / 199,306 / 3,659 (under) | 12 / 118,839 / 1,928 | Move the 12 test-only files to the `-tests` companion crate that `RESULT-OS-SERVICES-MCP-SPLITS.md` §6 already describes. Its own result states the test closure still needs this carve. |
| `semio-framework-os-directory` (lib-test) | 45 / 555,060 / 4,781 | 20 / 187,086 / 3,980 | 25 / 367,974 / 793; embedded 17 files, 316,858 B | Move the test modules and the fixtures to a `os-directory-tests` companion. Named in `RESULT-OS-SERVICES-MCP-SPLITS.md`, which did not split it. |
| `semio-framework-os-directory-client` (lib-test) | 23 / 413,608 / 3,957 | 10 / 185,092 / 2,513 | 13 / 228,516 / 1,438; embedded 10 files, 139,256 B | Move test modules and fixtures to a companion. Named in `RESULT-OS-SERVICES-MCP-SPLITS.md`. |
| `semio-framework-os-command` (lib-test, files over) | 38 / 175,255 / 2,773 | 2 / 57,264 / 1,033 | 36 / 117,991 / 1,655 | Move the 36 test-only files to a companion. Not named by any FLEET slice (`📡️spr`). |
| `semio-framework-os-history-record-decode` (lib-test) | 41 / 354,287 / 1,339 | 14 / 207,125 / 972 | 27 / 147,162 / 352 | Move test modules and fixtures to a companion. Named in `RESULT-DUPLICATE-MOUNTS-2.md`. |

### 3.3 Plugin composition (PLUGIN-* slices)

| Crate (unit) | Test unit | Normal lib | Test-only | Companion split |
| --- | --- | --- | --- | --- |
| `semio-framework-plugin-publication-mounted` (lib-test) | 26 / 283,026 / 2,381 | 10 / 154,856 / 1,600 | 16 / 128,170 / 763; embedded 9 files, 23,537 B | Move test modules to a `-tests` companion. Not named by a single PLUGIN result. |

### 3.4 Framework core, UI host, configuration, and pack (no single FLEET owner; areas named where known)

| Crate (unit) | Test unit | Normal lib | Test-only | Companion split and owner |
| --- | --- | --- | --- | --- |
| `semio-framework-print` (lib-test, ratio 9.83) | 52 / 2,575,613 / 1,733 | 22 / 139,410 / 1,212 | 30 / 2,436,203 / 520; embedded 27 files, 2,358,705 B | Move the 1.6 MB `🧬️schema/🔣️.json` and the 497 KB and 106 KB fixtures out of the test closure (runtime read or data companion). Named in `RESULT-CARVE-PRINT-SERVER.md`. |
| `semio-framework-os-config` (lib-test, ratio 6.97) | 223 / 385,627 / 5,646 | 29 / 108,385 / 2,289 | 194 / 277,242 / 3,315; embedded 152 files, 38,955 B | Move the 194 test-only files to a companion. Named in `RESULT-RELOCATE-PLUGIN-SDK.md`. |
| `semio-framework-pixels` (lib-test) | 48 / 735,478 / 4,351 | 13 / 202,134 / 3,210 | 35 / 533,344 / 1,129; embedded 23 files, 431,208 B | Move the JPEG/PNG/2D fixtures out of the closure; move the test modules to a companion. Named in `RESULT-TILED-MAP.md`. |
| `semio-framework-artifact-space-collection` (lib-test) | 35 / 636,836 / 2,127 | 21 / 105,555 / 1,440 | 14 / 531,281 / 682; embedded 7 files, 436,481 B | Move fixtures out; move test modules to a companion. No FLEET slice names it. |
| `semio-framework-value` (lib-test) | 77 / 538,035 / 5,347 | 25 / 186,249 / 2,783 | 52 / 351,786 / 2,544; embedded 36 files, 108,365 B | Move `♻️retirement/🧪️tests` and `🧬️retained-clone` tests to a `value-tests` companion; move the JSON fixtures out. Named in `RESULT-COMPILE-PASS-1.md`. |
| `semio-framework-async` (lib-test) | 55 / 343,284 / 6,299 | 24 / 161,170 / 3,341 | 31 / 182,114 / 2,931; embedded 13 files, 25,577 B | Move test modules to a companion. Named in `RESULT-RENDERER-WP2.md` (as a dependency). |
| `semio-framework-tool-run` (lib-test) | 40 / 460,530 / 4,295 | 13 / 225,190 / 3,156 | 27 / 235,340 / 1,126; embedded 16 files, 100,857 B | Move test modules to a companion. Named in `RESULT-PLUGIN-WP5.md`. |
| `semio-framework-tool-machine` (lib-test) | 11 / 421,588 / 3,631 | 2 / 96,827 / 1,973 | 9 / 324,761 / 1,655; embedded 6 files, 237,878 B | Move the embedded fixtures out (one 237 KB file set) and the test modules to a companion. No FLEET slice names it. |
| `semio-framework-time-travel` (lib-test) | 9 / 470,603 / 1,957 | 5 / 73,053 / 1,307 | 4 / 397,550 / 647; embedded 2 files, 337,944 B | Move the two embedded files out of the closure (they are 338 KB of the 471 KB). Named in `RESULT-PLUGIN-WP5.md`. |
| `semio-framework-pack-json` (lib-test) | 42 / 477,267 / 5,758 | 9 / 228,704 / 3,396 | 33 / 248,563 / 2,332; embedded 22 files, 37,589 B | Move test modules to a companion. Named in `RESULT-SINGLE-MOUNT-AND-GATE.md`. |
| `semio-framework-mesh-engine` (lib-test) | 25 / 369,219 / 2,718 | 7 / 147,752 / 1,941 | 18 / 221,467 / 764; embedded 10 files, 170,058 B | Move fixtures out; test modules to a companion. No FLEET slice names it. |
| `semio-framework-quiz` (lib-test, lines over) | 20 / 408,357 / 6,695 | 11 / 137,241 / 2,648 | 9 / 271,116 / 4,038 | Move the test modules to a companion. Named in `RESULT-CARVE-QUIZ-GRAPHQL.md`. |
| `semio-framework-server` (lib-test, lines over) | 20 / 353,837 / 6,915 | 14 / 165,989 / 3,448 | 6 / 187,848 / 3,461 | Move test modules to a companion. Named in `RESULT-CARVE-PRINT-SERVER.md`. |
| `semio-framework-io-base64` (lib-test, over its declared budget only) | 7 / 47,936 / 755 (declared 4 / 32,768 / 1,024) | 3 / 18,436 / 341 | 4 / 29,500 / 412 | Move test modules to a companion, or raise the declared budget only if the owner wants it (no change proposed here). Named in `RESULT-W4-TRACE-TESTKIT.md`. |
| `semio-framework-ui-host` (lib-test, lines over) | 27 / 263,615 / 4,610 | 6 / 129,849 / 2,619 | 21 / 133,766 / 1,920; embedded 10 files, 28,075 B | Move test modules to a companion. UI family (W4 UI slices by area); no single slice names it. |
| `semio-framework-artifact-workflow-run` (lib-test, files over) | 60 / 202,547 / 2,363 | 29 / 97,110 / 1,429 | 31 / 105,437 / 922 | Move the test-only files to a companion. No FLEET slice names it. |

Count check: 28 native units = 26 hard-limit crates (os-kernel has two units) + io-base64 declared-only, across 27 crates. Ratio ranking of the hard-limit units is in §8.

## 4. Oversized first-party dev-dependencies

| Dev-dependency | Own lib (root) | As a dev-dependency | Over | Dependents with a native dev over-row |
| --- | --- | --- | --- | --- |
| `semio-framework-ui` | 2 / 1,550 / 32 | 31 / 1,109,244 / 19,372 (feature `wgpu-engine`, from `🧊️wgpu` renderer manifests) and 48 / 647,769 / 14,164 for other feature sets | yes, lines and bytes (ratio up to 4.87) | 80 crates, 66 direct: essentially every `🧊️wgpu` shell and renderer test crate under `📺️renderer`, plus `semio-framework-os-renderer-camera`, `semio-framework-os-renderer-wgpu-canvas2d-host`, `semio-framework-repo-dashboard-labels` (transitive) |
| `semio-framework-plugin` | 128 / 2,073,206 / 27,863 (its own lib is over the hard limit) | 138 / 2,446,561 / 32,855 (ratio 9.33) | yes | 10 crates: `semio-framework-os-mcp`, `-mcp-dispatch`, `-mcp-http-transport`, `-mcp-tools-inference`, `-mcp-workspace` (transitive), `semio-framework-os-flow-wasm` (transitive), `semio-framework-os-host-media-raster` (direct), `semio-framework-os-flow-extensions-laws`, `semio-framework-os-renderer-wgpu` (transitive), `semio-framework-plugin-host-test-component` (direct) |

Companion suggestion, dev-dependency:
- `semio-framework-ui`: the renderer shells depend on it with `features = ["wgpu-engine"]`, which pulls the full GPU engine closure into every test build. Split a `ui-wgpu-test-support` companion that exposes only the types the shell tests use, or move the `wgpu-engine` dev-feature into the one crate that needs it. FLEET.md (wave 5 ownership rulings) keeps `🖱️ui/…/🧊️wgpu` files with the peer's UI GPU cut, so the change is not for this slice.
- `semio-framework-plugin`: the crate's own lib is over the hard limit (128 files, 2.07 MB), so the `plugin` slice (PLUGIN-WP0 to WP7 and PLUGIN-INTEGRATION-1) must carve it before these dependents can compile tests.

## 5. Other over-limit observations

- Native lib closures over the hard limit: only `semio-framework-plugin` (128 files, 2,073,206 B, 27,863 lines). Every other crate's normal lib is under the limit now, including `semio-framework-os-kernel` (5 files, 116,802 B), so the kernel carve already holds for the normal build; its problem is the test side.
- Over-limit units per label: 28 native, 27 wasm32-unknown-unknown, 27 wasm32-wasip2 (82 in total). The wasm figures are upper bounds because no wasm build was attempted.

## 6. Units that could not be measured (11 native, 32 across labels)

The canonical counter refuses these closures. Each refusal is a real missing input, not a sizing result:

- `Missing Rust source input` for `include_str!`/`include_bytes!` JSON fixtures under `🌲️built/🧪️tests/🧫️fixtures/` (`semio-framework-plugin` lib-test, `semio-framework-ui-runtime-reconcile` and `-primitives` lib-test), `✏️TextEditor/🧫️fixtures/…` (`semio-framework-os-renderer-wgpu` lib-test), `🎯️targets/…/📡️EventFeedHost/🧫️fixtures/…` (`semio-framework-os-renderer-wgpu-scene-feeds`), `🖱️ui/🧫️fixtures/…` (`semio-framework-os-renderer-wgpu-scene-tables`, `-scene-text-vfs`), `🧪️tests/🥣️laws-companion/…/🎚️field-set/🔣️.json` (`semio-framework-os-spr-laws-tests`). The `🌲️built/🧪️tests/🧫️fixtures` directory named by the `ui-runtime`/`plugin` refusals does not exist on disk (checked for that one path).
- `Missing Rust source input` for `🧪️tests/🧪️aggregate-admission/🦀️.rs` (`semio-framework-plugin-reactor-lifecycle` lib-test).
- `Missing or ambiguous Rust module: sqlite_snapshot_tests` (`semio-framework-plugin-host-test-component` lib-test).
- `Missing Rust source input` for an `os-kernel` integration target that includes `replication` fixture JSON (`artifact_command_retirement`).

These units are unmeasured. Their sizes are unknown, so they are not counted as over. A crate with a test error may still be over on another unit.

## 7. Caveats

- Feature sets follow the canonical counter's rule (default features plus unified dependent requests, with the head-name quirk kept). Feature-gated test modules are measured only when enabled by the test build.
- The canonical gate census (`census.ts`) includes every `[target.…]` dependency table; this census evaluates them per label. The two can differ for dependency closures, not for a crate's own source.
- Embedded `include_str!`/`include_bytes!` inputs count toward the bytes budget in the canonical counter, so most "test-only" bytes here are fixtures, not code.
- The hub workspace is not measured (section 0).

## 8. Top 10 over-limit test units (native, by ratio)

| # | Crate | Unit | Files | Bytes | Lines | Ratio |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| 1 | `semio-framework-os-kernel` | lib-test | 241 | 3,163,369 | 34,332 | 12.07 |
| 2 | `semio-framework-print` | lib-test | 52 | 2,575,613 | 1,733 | 9.83 |
| 3 | `semio-framework-os-config` | lib-test | 223 | 385,627 | 5,646 | 6.97 |
| 4 | `semio-framework-pixels` | lib-test | 48 | 735,478 | 4,351 | 2.81 |
| 5 | `semio-framework-artifact-space-collection` | lib-test | 35 | 636,836 | 2,127 | 2.43 |
| 6 | `semio-framework-value` | lib-test | 77 | 538,035 | 5,347 | 2.41 |
| 7 | `semio-framework-os-directory` | lib-test | 45 | 555,060 | 4,781 | 2.12 |
| 8 | `semio-framework-pack-json` | lib-test | 42 | 477,267 | 5,758 | 1.82 |
| 9 | `semio-framework-time-travel` | lib-test | 9 | 470,603 | 1,957 | 1.80 |
| 10 | `semio-framework-tool-run` | lib-test | 40 | 460,530 | 4,295 | 1.76 |

The full list of 28 native units, with per-file rows for companion planning, is in `🗑️generated/tcc-overlimit-test-units.tsv` and `🗑️generated/tcc-overlimit-files.tsv`.
