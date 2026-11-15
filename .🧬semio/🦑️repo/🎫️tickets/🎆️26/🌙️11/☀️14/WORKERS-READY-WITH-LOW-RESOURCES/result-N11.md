# N11 Result: Proof Of The N2 Crate Renames, Target Run, Headroom, Index Case Variant (PARTIAL)

Status: package stopped unfinished. From about 13:45 every shell call of this session hung (even `echo hi` did not return in 180 s) while my own `repo cargo` check chain was running, so steps 2 and 3 were not done and nothing was landed. No lock was taken, no live file was edited by N11, no `landings.md` entry exists.

## Verdicts

| Question | Verdict |
|---|---|
| All 38 renamed packages resolve in `cargo metadata` (full, with dependencies, `--locked`) in the 17 workspaces that contain them | YES. 17/17 rc=0 through `repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --locked`; all 38 new names present, 0 old names (workspaces: equation, grid2d, bitmap, flow, shooting, en1991, en1992, en1993, en1997, en1998, en1999, vdi3805, energy model, trinity rewriting, raster, stdio gltf, `🧰️framework`). |
| Stale old names (hyphen and underscore, all 38) anywhere | NONE in the tracked tree (rg over the worktree without tickets, caches, `dist`, `.venv`: 0 hits; covers manifests, `build-owners.json`, `project.json`, schemas, fixtures, sources, docs). Two git-IGNORED generated locks still held the old grid2d/bitmap name: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` and the same under `🖼️bitmap`. A `cargo metadata --offline` through `repo cargo` regenerated both (rc=0, 0 old names afterwards). Pre-change copies are in the scratchpad `N11/bridge-lock-before-*.lock`. |
| Every renamed crate compiles | NOT PROVEN. Native `check` ran for grid2d (whole workspace, 1011 units, thousands of warnings so type-checking really ran) and for the equation workspace. Both stopped on faults that are NOT the rename: (a) `semio-s-wfc-sqlite-bitmap`: `cannot find pack in semio_os_kernel` (`✏️s/🔌️plugins/🀄️wfc/🪶️sqlite/🎨️bitmap/…/🧪️tests/🦀️.rs:4`), a peer kernel change; (b) `semio-s-artifact-mathematical-equation-examples`: `use semio_s_artifact_mathematical_equation_editor::…` has no dependency entry in its `Cargo.toml` (identical to `HEAD`, last written 2026-11-11). `…-update-graph` (renamed) depends on `-examples`, so it cannot be checked until (b) is fixed; the remaining 16 workspaces were queued (`N11/runchecks2.sh`, renamed set and direct dependants per workspace) and the first seven results were rc=101 within 30 to 60 s each; I could not read their errors before the shell hung. None of the 38 renamed crates has wasm-gated code (grep `wasm32|wasi` in their directories: 0 files); only `semio-framework-replication-retained-field-prep` needed a wasm check (queued last, not run). |
| `repo run @semio-tech/framework-os-mcp-rs:build` | NOT RUN. |
| Headroom raised to 8 | NOT DONE. Measured and designed, see below. |
| Index case variant | DONE (read only), see below. |

## Headroom Analysis (nothing changed)

Worst Cargo output paths now (live budget model): macOS 255 (`wasm32-wasip2`, prototype, `semio-s-artifact-energy-model-mut-lighting-gain-emission`), 253 release, browser 250/248, host 249/247, Linux worst 244, Windows 224. Limit with headroom 8 is 248. Only the macOS root with a 16 character account is affected: **113 packages (190 library units) exceed 248, 0 on Linux/Windows**.
Each package name counts twice in the path (directory `build/<pkg>` and file `lib<ident>-<hash>`), so one character of crate name gains 2.
Layout candidates, measured with the budget function (packages still over 248): gain 0: 113; gain 2: 77; gain 3: 38; gain 5: 11; gain 7: 0.
- Product lane digits 6 to 4 (gain 2) or 3 (gain 3): NOT recommended. `pool.maximumLiveLanes` is 32 and the receipt refuses a colliding key, so 4 digits give about 0.8 % and 3 digits about 12 % chance of a refused product build. 5 digits (gain 1) is possible.
- Dropping the `l` lane directory (gain 2): needs `layout.lanes` merged into the unit-cache root and a new lane-id rule (single-letter ids clash with `o e k s q …`); touches `.cargo/config.toml` and the pool.
- `cargo/home` to a short name (gain 9) is the only fix for the generated-tree test, see below, and is a storage-ledger change.
- Profile name `prototype` (9) is Cargo's profile directory; renaming helps prototype only.
So no safe layout change reaches gain 7; the remaining route is renaming 113 packages by 1 to 4 characters. I generated a collision-free map (all 113 new names unique, none existing, each at least as short as required): scratchpad `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/N11/rename-map-2.json` (inputs `over248.json`, probe scripts `probe*.ts` beside it; will be lost when the scratchpad is wiped). It applies with N2's `N2/rename-crates.py <root> <map> --apply` in about 30 s. Names use abbreviations (`found`, `bldg`, `restr`, `ents`) and dropped filler tokens; review them before landing. Planned law change: `Budget.minimumHeadroom: 8` in `📏️path-envelope/🧬️schema/🔣️.json`, `cargoBudgetV1` offenders at `maximumCharacters - minimumHeadroom`, a 7th test that the table rows are at most 248. Do this only after the 38 N2 renames compile, otherwise two unproven bursts stack.
Tracked sources have 0 headroom (longest 216 + 40 = 256; 61 files over 248: about 30 `10/18` ticket backup files that N2's archive missed and about 30 `📸️snapshot/⬅️before/🔣️.json` fixtures of gis, bim, architect, energy, puzzle).

## Existing Failure Found In N2's Test Set

`path-envelope/📐️budget/🧪️tests/🟦️.ts` (run 13:30): 5 pass, 1 fail. The generated-tree test fails: `.cache/b/r/m/cargo/home/registry/src/index.crates.io-1949cf8c6b5b557f/android-activity-0.6.1/…/android_native_app_glue.h` is 261 characters below the 39 character macOS root (third-party registry source, new since N2's measurement). Fix options: shorter `cargo/home` (gain 9, 252), or an exemption list for the registry source of crates that no supported target builds.

## Index Case Variant (task 4)

Wrong index entries (`git --no-optional-locks ls-files -s`, `core.ignorecase=true` on this Mac, blobs identical to the disk files, checked with `git hash-object`): 10 files, lowercase `ff` in the index, `FF` on disk. Directory (all below `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`):
- `📚️examples/🏛️bestest-600ff/{🟦️.ts,🦀️.rs,🧪️tests/🧩️example/🟦️.ts,🧪️tests/🧩️example/🦀️.rs}` and the same four under `🏛️bestest-900ff`
- `🖼️assets/🏛️bestest-600ff/🗣️.dsl.semio` and `🖼️assets/🏛️bestest-900ff/🗣️.dsl.semio`
The index entries under `🧪️tests/🧫️fixtures/🏛️bestest-600FF` and `…900FF` (3 each) are already correct.

Instruction for the archivist machine (run once, `core.ignorecase=false` is required or git matches the old entries and renames nothing):
1. `git -c core.ignorecase=false ls-files -z | grep -zE '/🏛️bestest-(600|900)ff/' | xargs -0 git -c core.ignorecase=false rm --cached -q --`
2. `git -c core.ignorecase=false add -- "$P/📚️examples/🏛️bestest-600FF" "$P/📚️examples/🏛️bestest-900FF" "$P/🖼️assets/🏛️bestest-600FF" "$P/🖼️assets/🏛️bestest-900FF"` with `P='✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any'`, then commit (the ticket files `🧫️bestest-600FF-semio-model.json` and `900FF` under `🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END` are consistent and need nothing).

## Addendum: Errors Of The Queued Checks (read later, shell recovered)

Workspaces 1 to 4 (equation, grid2d, bitmap, flow), all rc=101, none a name error (no `unresolved import` of an old or new crate name except the missing `-editor` dependency in equation-examples): grid2d `wfc-sqlite-bitmap` (`semio_os_kernel::pack` missing); bitmap `…wfc-bitmap-snapshot` (49 errors: `no method begin_close`, `close_step` on retained types) and `…-transient` (`SnapshotProjection` needs `unsafe impl`, `OwnedFieldProjection::begin` missing); flow `…flow-command-retirement-tests` (29 errors: methods now take 2 arguments). These are framework API drift from peers' in-flight retained/close changes, so the renamed crates `grid2d-tests-cuts-pinned-corner`, `wfc-bitmap-tests-shrinks-output-pin` and `flow-command-retirement-tests` could not be proven; workspace 4 dependants and 5 to 17 never produced a result.

## Not Verified

Compile of all 38 renamed crates, the target `framework-os-mcp-rs:build`, headroom 8, the wasm check of field-prep, the six budget tests after any change, whether the machine hang came from my check chain (the last `repo cargo` of the chain, workspace 4 dependants, may still be running or queued in the capacity door; it was started by me and is not killed).
