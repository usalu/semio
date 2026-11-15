# N2 Result: The 256 Character Path Envelope As A Static, Root-Independent Law

## Verdicts

| Question | Verdict |
|---|---|
| `repo run @semio-tech/framework-os-mcp-rs:build` passes | **NO, not verified.** Both runs on the live tree (12:40 and 12:44, 134 s and 8 min) ended with `The disk is full: partition / has 92 GiB free, so the resident service could not finish writing`: the graph image `.cache/b/g/m` is 100 % full (12 GiB, 44 MiB free; `inference/selections` holds 11.7 GB). `repo cache cas gc` freed 102 MiB of the CAS only. This is the graph volume (N4/AC), the path envelope was never reached. N3 reported the same image to N4. |
| The rename `…-retained-field-preparation` → `…-field-prep` is complete and consistent | **Yes, statically.** 0 leftovers of either spelling outside tickets; manifest, dependent manifest, `📋️build-owners.json`, 109 `Cargo.lock` (entry present, position between its sorted neighbours, no dangling reference to it); `cargo metadata --no-deps` resolves the crate and its dependent in `🧰️framework`. Not verified by a build (see above). |
| The staging schema fault (`semio-framework-os-mcp` vs `semio-os-mcp`) | **Already fixed** by the worker: schema (`package`, `binary`, `sourcesRecord`), crate, lock and `MCP_CARGO_PACKAGE` all say `semio-os-mcp`. Not rerun (graph volume). |
| Static root-independent law exists and passes | **Yes.** Schema `Budget` in `📏️path-envelope/🧬️schema/🔣️.json`, code `📏️path-envelope/📐️budget/🟦️.ts`, 6 tests, all pass live (43 s). |
| Offenders found and fixed | **Found 727 packages (old model, 199 even at the worker root) / 37 (corrected model); fixed all 37 by rename; 0 remain.** 42 over-long ticket files archived. |
| Worst remaining Cargo output path | **255 characters** (`wasm32-wasip2`, `prototype`, macOS root with a 16 character account, `semio-s-artifact-energy-model-mut-lighting-gain-emission`): **headroom 1 character** there; host 249 (headroom 7), browser 250 (6), Linux worst 244, Windows worst 224. |
| Non-cargo bootstrap paths | **Fit** (`.cache` and `dist`: 210 characters, 250 below the macOS root, headroom 6). Other generated trees exceed (hub test artifacts, hub state copies, ticket outputs, the Playwright browser bundle): listed below, not fixed. |

## What The Fault Was (Two Faults, Not One)

1. **The model put the codegen temporaries in the wrong place and called them shorter than they are.** `cargoCompilerIntermediatePathsV1` modelled `…/build/<pkg>/<hash>/out/<crate>-<hash>.<25>.<7>.rcgu.o`. Measured with the repo toolchain (`nightly-2026-07-20`, rustc 1.99; and stable 1.95): an incremental compilation (profile `prototype`) writes `<crate>-<hash>.<25>.<7>.rcgu.{o,bc,no-opt.bc}` (the model), but a non-incremental one (`release`, 16 codegen units) writes `<crate>-<hash>.<crate>.<16 hex>-cgu.<N>.rcgu.{o,bc,thin-lto-after-rename.bc}` with the crate name **three times**; rustc shortens any such file name to **143 characters** (measured: longest name 143, front of the crate name cut). So `release` units of every crate with a name over about 45 characters exceeded 256 at every root and were simply never checked.
2. **The check was calibrated on the shortest root.** 27 characters (`/Users/ueli/Documents/semio`) ran 199 packages over; the canonical worst root is 39.

## The Design Change (Layout, For All Crates)

| Change | Where | Gain |
|---|---|---|
| rustc writes all codegen temporaries to a short directory of the lane: `-Z temps-dir=<lane>/z` | `compilerTemporaryArgumentsV1` (`⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts`), called by the wrapper (`📜️script.ts`); layout key `compilerTemporaries: "z"` | Their path is now `<lane>/z/<≤143 characters>` = **root + 171** (210 below the macOS root) for every crate, instead of `unit dir + 2 × name`. For the incident crate (54 characters) 262 → 210 in the old model, 293 → 210 with real release names: **52 to 83 characters**. Proven with the real wrapper and the repo nightly toolchain in a scratch lane: 3 files in `<lane>/z` (longest name 143), 0 `*.rcgu.*` below the unit directories, build `Finished`. Only compilations whose arguments already carry an unstable flag (the repository's `-Z threads=0`) get it, probes and a stable toolchain get nothing (the 26 unit-cache tests build with stable Cargo and found this the hard way: 9 failed first). |
| Product lane identifier `q` + 11 digits → `q` + 6 digits | `pool.product.digestDigits` 11 → 6 (unit-cache schema); the receipt still refuses a different full key behind the same short name (`identity collision`, test updated: 58 `b`) | **5 characters** on every Cargo path (the lane is a segment of every path). |

Not done (and why): the profile name `prototype`, the Cargo hash directory and `build/<pkg>/<hash>/out` are Cargo's or the repository's public names; no other segment of the layout is wasteful.

## The Law (One Schema-Declared Place)

`📏️path-envelope/🧬️schema/🔣️.json`, definition `Budget` (plus the new `Policy` keys `temporaryDirectory`, `temporaryFileCharacters`, `wholeCodegenHashCharacters`, `wholeCodegenIndexCharacters`, `incrementalCrateCharacters`, `incrementalTimestampCharacters`, `incrementalSessionCharacters`):

- **Limit 256** (UTF-16 units, the existing `Policy.maximumCharacters`).
- **Canonical worst roots** (account name budget **16 characters**): macOS `/Users/<16>/Documents/semio` = **39**, Linux `/home/<16>/semio` = 28, Windows `C:\semio` = 8.
- **Cache layout**: `.cache/b/r/m/u` + `l` + lane of 7 characters (`q` + 6).
- **Builds** (the repository modes only, `readBuildMode`): profiles `prototype`, `release`; targets `host` (every unit of every package: library, build script, binaries, tests, examples), `wasm32-wasip2` (every package in the dependency closure of a `cdylib` package), `wasm32-unknown-unknown` (closure of the packages below `📺️renderer`).
- **Formula** is the existing `cargoCompilerIntermediatePathsV1` (derived from the code that builds the paths): unit directory outputs, `t/<recipe>` outputs, `incremental/<crate>-<13>/s-<10>-<7>-<25>/<25>.o`, and the temporaries in `<lane>/z`.
- **Packages**: all local packages of the tracked manifests (library name from `[lib]`, `[[bin]]`, `[[test]]`, `[[example]]`, `[[bench]]`, `tests/*.rs`, `src/bin/*.rs`, `build.rs`) and all locked registry packages (library and build script), 5 317 packages in total.
- **Source paths**: every tracked or trackable file (`git ls-files --cached --others --exclude-standard`, existing on disk) below the longest root.
- **Generated trees**: `.cache` and `dist` as measured on the machine (800 000 entries, 9 s).

Tests (`📐️budget/🧪️tests/🟦️.ts`, run: `bun test`, 43 s): the roots are 39/28/8; every Cargo package fits for every root × target × profile (a failure prints the 64 longest offenders as `length root target profile subject`); every tracked file fits; **the worst path per root, target and profile equals an independent pathlib computation** (`🐍️.py`, same inputs, 18 numbers compared); temporaries do not grow with the name (two names of 80 and 85 characters give the same lengths, equal to `lane + 3 + 143`); generated trees fit. Unit-cache side (`🧵️lane/🧪️tests/🟦️.ts`): the wrapper arguments (compilation, stable toolchain, probe), `compilerTemporaries` equals the budget's `temporaryDirectory`, `q` + digits equals `laneCharacters`, `.cargo/config.toml` build dir starts with `<cacheDirectory>/<lanes>/`. The existing path-envelope fixtures (12 cases) were regenerated (boundary rows kept at exactly 256 / 257 / UTF-16 257) and still equal the pathlib oracle.

## Offenders And Fixes

- **Corrected model, `release` and `prototype`, all roots**: 37 packages over (94 root × target × profile rows), all names of 57 to 85 characters, most of them `semio-s-artifact-norm-…-mut-…`, stdio and energy mutation crates and test crates carved from long labels. All renamed by hand (`N2/rename-map.json`, 37 pairs, new names 45 to 54 characters, none colliding with an existing name) with `N2/rename-crates.py`: one pass over 254 files in 28 seconds (13:13), word-boundary matching of both spellings (dash and underscore, so `…-imperfections-more` and `…-imperfections` stay distinct), `Cargo.lock` blocks moved to their sorted position and touched dependency lists re-sorted, sorted `[dependencies]` tables kept sorted, `Component crate of P: label` descriptions updated. Afterwards: 0 leftovers of an old name; `cargo metadata --no-deps` resolves all 35 changed workspaces and sees all 37 new names; `cargoBudgetV1` finds 0 offenders afterwards.
- **Examples of the sizes**: `…-trinity-rewriting-tests-narrows-the-lhs-pattern-to-a-shaft-neighbour` (85) → `…-trinity-rewriting-tests-narrows-lhs` (52); `…-wfc-grid2d-tests-cuts-the-pinned-corner-out-of-the-problem` (75) → `…-wfc-grid2d-tests-cuts-pinned-corner` (52).
- **Source paths**: 42 tracked files of the old ticket `🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN` (backup copies of build sources, 257 to 264 characters below the macOS root, 0 of them in `HEAD`, 13 `.absent` markers) archived in one file `N2/ticket-backups-over-256.tar.gz` (42 members, extracted and compared byte for byte) and removed with single `rm`. Longest remaining tracked path 216 characters (256 with the root).
- **Headroom after the fix**: 11 `wasm32-wasip2` libraries sit at 254–255 (names of 56 characters), 77 within 5; host and browser have none within 5. Any new crate of 57 characters or more fails the test, which is the point. Renaming the 11 would buy 2 more characters; not done (another 250-file burst on a live tree for 2 characters).
- **The generator still makes long names**: the crate-tree drafts (`🏗️authoring/📦️crate-tree/🗺️plan/🧮️pack/🏗️crate-drafts`, `name = ${lib.name}-${entry.label}`) take labels from test names. The next carve can produce a 85 character crate again; the budget test then fails with the name. Capping the label there is the follow-up (not changed: its test suite is not in this package).

## Non-Cargo Paths (Task 3)

Measured with `find` over the whole tree of this Mac (2 390 177 entries, relative lengths; add 20 for the canonical root of this server tree (19 → 39), add 12 for a worker (27 → 39)):

| Generated by | Longest relative path | With the macOS root (39 + 1) | Verdict |
|---|---|---|---|
| Dependency stores `.cache/b/d`, `.cache/b/t` (`node_modules`), staging dirs, `.cache` in total | 209 | 249 | fits (headroom 7) |
| `dist` (`repo-coordinator/next/standalone/…` copies tracked sources below a 48 character prefix) | 210 | 250 | fits (headroom 6); a tracked source path of more than 162 characters in that closure would break it |
| `.venv` | 205 | 245 | fits |
| Tracked sources (non-ticket) | 210 | 250 | fits |
| Hub test fixtures `🌎️hub/📦️packages/🦀️rust/🗑️generated/test-artifacts/os-hub-test-*-<uuid>/db/artifact-cas/v1/<64 hex>/manifest/<64 hex>…` (generator: `🌎️hub/🧪️tests/🗂️artifact-root/🦀️.rs`, 18 467 leftover directories, 1.4 GB, 112 366 files over 216) | 298 | 338 | **exceeds, not fixed**: left-over test output (the bootstrap does not run hub tests); intrinsic part below the artifact root is about 245 (two 64 hex digests, `plugin-modules`), so only a shorter root helps (`SEMIO_TEST_ARTIFACT_DIR` under `.cache`, 14 characters, instead of 55) and cleaning them |
| Hub state copies `.🧬semio/🌐hub/s14-*` (agent scratch overlays of `✏️s/…`) | 299 | 339 | **exceeds, not fixed**: hand-made copies of tracked sources, not a generator |
| Ticket outputs `🗑️generated/…` of old tickets | 310 | 350 | **exceeds, not fixed**: ignored files of finished tickets |
| Playwright Chromium `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/…/Google Chrome for Testing Helper (Aperitif Renderer).app/…` (tool setup of N1) | 326 | 366 | **exceeds, cannot be fixed by us**: the macOS app bundle alone is about 278 characters without any root; it is a third-party archive and macOS has no 256 limit. It needs a declared exemption in the tool registry, not a rename |

## Not Verified

- The real build `repo run @semio-tech/framework-os-mcp-rs:build` (graph image full). After the graph volume is freed (N4) run it once; expectation: the `Selected Cargo compiler path exceeds 256 characters` refusal is gone (0 offenders), the wrapper passes `-Z temps-dir`.
- No cargo compilation of the 37 renamed crates (only `cargo metadata --no-deps`, lock position and reference checks).
- The new `-Z temps-dir` was proven in a scratch lane with a one-crate workspace and the repo nightly, not through the build driver (`SEMIO_UNIT_CACHE_LANE_ROOT` of the broker lane `g` is assumed to be the build directory the envelope checks; the unit-cache lane normally is).
- Windows: `C:\semio` numbers come from the same formula with `win32` path semantics (oracle agrees); no Windows machine was used.
- The top-level `native-build/🧪️tests/🟦️.ts` needs `SEMIO_TEST_ARTIFACT_DIR` from the test harness and was not run.

## What The Other Packages Must Know

- N4/AC: `.cache/b/g/m` full (`inference/selections` 11.7 GB) blocks every `repo run` on the server; `repo cache cas gc` does not touch it.
- Everyone: a new Cargo package name over 56 characters (plugin libraries in a `cdylib` closure), a new tracked path over 216 characters or a new generated tree path over 216 characters fails `📐️budget` before any worker meets it.
- The files of this package are listed in `FLEET-ONE-MACHINE-…/landings.md` (landing 12:36).
