# Result CENSUS4-PKG-20 (prefix `c4p20`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `E` = `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`. `S` = `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`. Tools: `F/c4p20-tools/`. Logs: `F/🗑️generated/c4p20-*` (nothing deleted). Scratch (stages, sandboxes): `scratchpad/c4p20/`.

## 0. Verdict

DONE (static evidence, wave 7). All 8 files of package 20 are split into 34 cohesive children (longest 681 lines), installed on disk and lossless (line multiset, missing 0 for 8 of 8, re-run on the live tree 2026-10-17 07:3x). The scoped patched differential equals the original (rustc resolution differential orig = new in all 21 runs; typeck oracle: lib 307 > 256 and test 709 > 655 non-stub rows with 0 new, the 51 and 54 privacy rows being fixed by the visibility edits of section 2.1). `--check-applied` plugin gate at baseline after the split (exit 0, native 3329 / 3341, `--test` 4252 / 4259, `onlyCombined` 0, admission and shared empty). Not run: a real `bun repo cargo check`, because `📓️FLEET.md` carries no `COMPILE ADMITTED` line (checked 2026-10-17 07:5x); the exact command is in section 7.

## 0.1 Found state (wave 7, 2026-10-17 07:2x)

- The slice was installed and verified before the third restart (all logs `🗑️generated/c4p20-*` dated 06:11 to 07:07). Nothing was half-applied: all 8 parents and 34 children are on disk, no `🦀️.rs` of the editor tree is newer than 06:39 (the last verification), and the `git diff` of the 8 parents matches the split. No rollback was needed.
- Lost with the restart: the scratchpad (`stage/`, sandboxes). The `--check-applied` runs `after` (killed), `after` (launcher `Cannot find module ...policy.json`, a peer's infra edit) and `after2` (`EMFILE` while removing the sandbox) never produced a result. Re-run as `after3` with a new sandbox (section 5).
- Re-verified in wave 7 against the live tree (the staged-byte-equality of `live.py` cannot run any more because the stage is gone; the line-multiset check runs against the live files): `LOSSLESS OK`, missing 0, for 8 of 8; `check.py`: `CHECKS OK` (34 children, longest 681 lines, longest repo-relative path 230 bytes).

## 1. Slice and found state

Census 4 package 20 (plugin-family, rust, 8 files, 11,384 lines), all in the remodeling artifact (crates `semio-s-artifact-remodel-remodeling-{sfm,mesh-surface,mesh,mesh-repair,editor,session,algebra}`). The 8 files are the first column of the table in section 2.

- Found state at the start (session 1): no prior RESULT, no `c4p20-tools/`, no `🗑️generated/c4p20-*`. The 8 files had exactly the census line counts and were unchanged by any peer; nothing to recover or roll back.
- Peer overlap: the `🦀️.rs` of `E/` shows as staged-modified in the index (a peer's earlier edit); I cut the working-tree content as found (pre-edit snapshots in `F/c4p20-tools/orig/`, md5 recorded in section 6). Pkg 8 (same crate family, other files) is installed and untouched by this slice.
- No restart happened during this slice. One of my background `--check-applied` runs had to be killed and restarted (section 5, row `--check-applied`).

## 2. What changed

Method: item-level splitter `c4p20-tools/c4p20.py` (descendant of `c4p8-tools`, `c4p20.py` plus `widen2.py`, `rsitems.py`). Every top-level item keeps its text. Children are `<folder>/🦀️.rs`, wired in the parent by `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `pub use <snake>::*;` (a `pub use` only where the child holds a `pub` item, so every public path is unchanged; otherwise `use <snake>::*;` or nothing). Every child starts with `//! <emoji> ... See [`super`].` whose emoji is the folder emoji (unique among siblings and never the root's), then `use super::*;` (dropped where the child needs nothing from the parent). `use` lines stay in the parent. Private definitions another file of the split names became `pub(super)` (widening planner `widen2.py`, driven by identifier presence in the other children and in the external test companions). Regions (`// #region`) are carried into the children balanced; empty shells in a parent were removed, and every region name lives on in a child.

| File | Lines before | Parent after | Children (lines) | Widened to `pub(super)` |
| --- | ---: | ---: | --- | ---: |
| `E/⚙️engine/📸️sfm/🦀️.rs` | 1,684 | 86 | 206 (`🛰️synthetic-fixtures-and-priors`) + 468 (`🧱️incremental-state`) + 583 (`🧭️seed-and-registration`) + 368 (`🔩️bundle-and-snapshot`) | 70 |
| `E/⚙️engine/📸️sfm/🧪️tests/🔬️unit/🦀️.rs` | 1,115 | 23 | 263 (`🔀️geometry-tests`) + 244 (`🧭️incremental-tests`) + 168 (`🌐️global-and-prior-tests`) + 446 (`🎯️five-point-and-diagnostics`) | 2 |
| `E/⚙️engine/🥽️mesh/🔺️surface/🦀️.rs` | 1,067 | 18 | 219 (`📐️trimesh-and-vectors`) + 164 (`🕸️halfedge-topology`) + 681 (`🧊️iso-surface-extraction`) | 10 |
| `E/⚙️engine/🥽️mesh/🦀️.rs` | 1,405 | 71 | 251 (`📤️interchange-preparations`) + 270 (`✅️bounded-validation`) + 299 (`🪄️bounded-postprocess`) + 533 (`🏭️pipeline`) | 63 |
| `E/⚙️engine/🥽️mesh/🩹️repair/🦀️.rs` | 1,851 | 37 | 633 (`⏱️bounded-preparations`) + 423 (`🧼️clean-and-repair`) + 426 (`🕳️hole-filling`) + 352 (`🧊️voxel-close-and-validate`) | 18 |
| `E/🦀️.rs` | 1,369 | 168 | 138 (`🔌️io-and-commands`) + 192 (`🌉️action-bridge`) + 283 (`🚧️retained-commands`) + 311 (`🎛️artifact-editor`) + 317 (`📜️manifest`) | 13 |
| `E/🧵️reconstruction-session/🦀️.rs` | 1,613 | 84 | 352 (`🧭️run-contract`) + 152 (`🔑️trace-checkpoint-and-ingestion`) + 392 (`🎁️products-and-outbox`) + 568 (`🏃️run-job`) + 125 (`🔍️revalidate-job`) | 84 |
| `S/➕️algebra-internals/🦀️.rs` | 1,280 | 36 | 304 (`🧱️dense-types`) + 169 (`🔗️lattice-sparse-and-errors`) + 268 (`📉️factorizations-and-eigen`) + 302 (`🔁️iterative-and-svd`) + 235 (`🌀️schur-and-roots`) | 0 |

Special cases:
- `📸️sfm/🦀️.rs`: the 1,066-line `impl IncrementalSfm` was cut into three inherent impl blocks at two method boundaries (before `init_pair`, before `refine_registered_poses_pnp`) by a unique-anchor pre-edit (`specs/sfm.py`); the only added lines are 2 x `}` and 2 x `impl IncrementalSfm {`. The first block stays with the type definitions (`🧱️incremental-state`), the seed/registration block and the bundle/snapshot block each sit in their own child.
- `📸️sfm/🧪️tests/🔬️unit/🦀️.rs`: the file ended with the unclosed region `🔖️TwoViewFivePointTests` (opened at line 673, never closed). The pre-edit appends the missing `// #endregion 🔖️TwoViewFivePointTests` (declared as the one added line) so the child is balanced. The 29 `#[test]` names are identical before and after (`testnames.py`: orig 29, new 29). The inline `mod long` moved with the five-point child (its `use super::*;` now resolves through that child's own glob of the parent), and `type CorrByPoint` plus the `use` lines stay in the parent.
- `E/🦀️.rs` (editor lib component): the `app_commands!` invocation and `impl RemodelingCommand` moved together into `🔌️io-and-commands` (which also holds the `pub` io fns, so the child is re-exported with `pub use`, keeping `RemodelingCommand` public). The inline `mod args_bridge` moved into the child `🌉️action-bridge` (the child module is called `action_bridge`, because a child named like the inner module would be shadowed by the parent's own `mod` of that name; checked by `modclash.py`). `pub struct RemodelingPlayApp` and all `use` lines (including the flat payload-module imports the macro addresses) stay in the parent, which is why `use super::*;` resolves the macro's flat payload-module names in the child.
- Session, algebra and sfm: the `#[cfg(test)] #[path = "🧪️tests/..."] mod tests;` mounts stay in the parent (their relative path is unchanged).
- Crate roots (`📦️packages/🦀️rust/🦀️.rs`) and all `Cargo.toml` files are untouched. No file of `🔌️plugin/**` was edited.

### 2.1 Visibility fixes (the one change that is not a pure split)

The typeck oracle showed that all 51 lib-cfg privacy errors (54 in the `--test` cfg) of the family sit in these crates and are carve fallout that predates this slice (the same 51 and 152 total rows are in the `pre` run): `mesh` (bounded validation and pipeline) calls `new`, `advance`, `finish`, `exceeded` and reads `records` of the `Bounded*Preparation` types that live in `mesh-repair`, and `mesh-repair` calls `DisjointSet::{new,find,union}` and `DenseField::index` of `mesh-surface`. All definitions and all call sites are in the 8 files of this package, so I fixed them in the same edit (staged into the specs as `vis_edits`, applied by the splitter, lossless check aware):
- `🔺️surface/📐️trimesh-and-vectors`: `DisjointSet::new`, `find`, `union` are now `pub`.
- `🔺️surface/🧊️iso-surface-extraction`: `DenseField::index` is now `pub`.
- `🩹️repair/⏱️bounded-preparations`: `BoundedCleanPreparation::{new,advance,finish}`, `BoundedEdgePreparation::{new,advance}` and its field `records`, `BoundedRepairPreparation::{new,advance,finish}`, `BoundedOrientPreparation::{new,advance}`, `BoundedHoleFillPreparation::{new,advance,exceeded}` are now `pub`; `BoundedEdgeRecord` is now a `pub struct` with `pub directed` and `pub faces` (mesh reads them through `records`).
Effect on the oracle (section 5): lib privacy 51 > 0, `--test` privacy 152 > 98 (the remaining 98 are in other crates), 0 new rows.

## 3. Data and tables (run-time fixtures)

Nothing to move. Measured over the 8 files and the 5 test companions: no data table (largest literal run is the editor's `REMODELING_RETAINED_TOOL_IDS` list of 43 lines and `REMODELING_PUBLICATION_CONTRACTS` of 46 lines, both `const` registries read in trait consts and therefore compile-time by design; every other file has at most 2 literal-only lines). The long lines (algebra docs, editor action descriptions, session test strings) are prose, not tables. Bytes are identical by construction (line-multiset lossless below). No fixture was created.

## 4. Source-text readers

Searched with `git grep -F` for every moved file path and for `include_str!`/`include_bytes!`/`read_to_string` in the remodel artifact, and read the interactivity-audit (`📚️library/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts`) that names the session folder as a scope: it scans every `.rs` file under the scope directory, so the new children are covered. The other hits are doc comments (`📚️examples/🛰️synthetic-orbit/.../🧩️example/🦀️.rs:147`, `✏️editor/🟦️.ts` line 3), the brep and 3d `🥽️mesh` of other plugins, and `.vscode` ticket paths. No reader of the 8 files exists; nothing to re-point.

## 5. Verification (every command was run; results are the real ones)

Paths relative to `F/c4p20-tools/` unless stated; logs `F/🗑️generated/c4p20-*`.

| Check | Command | Result |
| --- | --- | --- |
| lossless line multiset (live tree) | `python3 live.py` (`c4p20-live-lossless.log`) | 8 of 8 `LOSSLESS OK`, 0 missing lines; each installed file byte-equal to its stage. Extras are only wiring (`#[path]`, `mod`, `use x::*`), child docs, `use super::*;`, carried region markers (balanced: open = close in every file), the 3 declared pre-edit pairs (sfm `}` x2 and `impl IncrementalSfm {` x2, sfmunit `// #endregion`) and the 21 declared `vis_edits` lines (normalised through the declared old/new pairs). |
| test names | `python3 testnames.py sfmunit ...` | 29 `#[test]` in the sfm unit file before and after, identical set; the other 7 files hold no tests |
| typeck oracle (`pos.py`, 21 crates = my 7 + direct dependents, absolute runs `--no-base`, label `pre` before the first install, `post` after install, `post2` after the visibility fixes) | `c4p20-tools/run-oracle.sh pre|post|post2 lib,test` = `POS_PREFIX=c4p20 python3 pos-tools/pos.py run c4p20-tools/families/c4p20.json --cfgs lib,test --no-base --label <L>`; compare with `python3 posdiff-full.py pre post2 lib test` (full non-stub lists; the `-new-<cfg>.txt` files are capped at 400 rows and must not be diffed) | pre: lib 307, test 709 non-stub rows. `post` (split only): lib 307, test 709, **0 new in substance**: the only delta is 4 E0616 rows (`records` of `BoundedEdgePreparation`, a pre-existing foreign privacy error) whose printed type path became `...repair::bounded_preparations::BoundedEdgePreparation`. `post2` (split plus visibility fixes): **lib 256 (51 gone, 0 new), test 655 (54 gone, 0 new)**; privacy lib 0 (from 51), test 98 (from 152); trait-scope 0; resolve 145/418 unchanged; type 110/138 unchanged; canary reached (`E0308 E0616 E0624`) in lib and test in all three runs. Logs `c4p20-pos-{pre,post,post2}.log`, `c4p20-posdiff-full.log`. |
| oracle limit (stub noise) | `c4p20-pos-post.log` line 6 | the stub layer learns that an external name is a trait only from `impl`/bound positions in the same file, so 7 more trait names (`ArtifactEditor`, `ArtifactCommandWork`, `InteractiveJob` in the children of editor and session) are neutralised (4 > 11 neutralised trait paths). That only changes stub-involved rows (total errors 2489 > 2490 lib, 3887 > 3894 test, all `E0407/E0437/E0438/E0277` on `Tr<n>` stubs) and hides one pre-existing borrowck row (`E0382` on `budget`, test cfg) behind a stub-trait method lookup. It is the same noise class as `c4p8` (stub `ArtifactDsl`); real compile will check those bodies. |
| rustc resolution differential (`rustc +nightly-2026-07-20 --edition 2021 --emit=metadata -o /dev/null`, sandbox with the original restored, 7 crates x lib, `--test`, `wasm32-wasip2`; staged and `C4P20_LIVE=1` runs, `oracle.py <group> lib,test,p2`; `c4p20-diff-stage-*.log`, `c4p20-diff-live-*.log`, `c4p20-diff-live2-*.log`) | `python3 oracle.py sfm|surface|repair|mesh|editor|session|algebra lib,test,p2` | errors orig = new in all 21 runs (sfm 68/91/68, surface 8/8/8, repair 40/40/40, mesh 60/92/60, editor 286/286/286, session 65/140/65, algebra 13/14/13), only-new 0, no new warnings. One finding fixed before install: the algebra child `🧱️dense-types` needs nothing from its parent (unused `use super::*;`), now `glob=False`. |
| E0451 scan | `POS_PREFIX=c4p20 python3 rpv-tools/lit.py c4p20-tools/families/c4p20.json pre|post|post2` | 0 candidate sites in all three runs (169 brace structs, 109 then 143 files in the module tree) |
| private declarations used across files | `python3 defscan.py <tag> <live dir>` x8 | 0 leftovers (counts of private declarations scanned: 4, 33, 34, 15, 44, 7, 24, 1) |
| module-name clash | `python3 modclash.py <8 tags>` | OK: no child module name equals an item, import or sibling module name of the parent (the editor `args_bridge` case was found and renamed) |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config skip_children=true` on the 42 files (`c4p20-rustfmt.log`) | 42 of 42 parse |
| paths | `python3 check.py` (`c4p20-check.log`) | longest repo-relative path 230 UTF-8 bytes (<= 240); longest absolute path 182 characters (<= 256); longest child 681 lines; every child mounted exactly once and present; docstring emoji = folder emoji, unique among siblings and never the root's; no `[DEBUG]` |
| duplicate mounts | `python3 ttc-tools/dupmounts.py '✏️s/🔌️plugins/📸️remodel/'` (`c4p20-dupmounts-{before,after}.log`) | 530 > 564 mount targets (+34 children), duplicates 48 > 48 with the identical set (all `📦️packages/🦀️rust` directory targets of the facade chains); 0 duplicate `*.rs` file mounts; 0 new |
| folder names | `bun c4p5-tools/rgi.mjs`, `bun c4p20-tools/tax.ts` (live `semanticDirectoryKindId`) | 34 of 34 RGI emoji + U+FE0F; 0 unresolved (`crate-module-children`, `members-of-tests` for the 4 test folders); every emoji is already a folder prefix in the repo (`rf8-folder-emojis.tsv`); no emoji repeated among siblings (two clashes with existing siblings, `🧵️reconstruction-session` and `📦️packages`, were found and avoided before install) |
| census inside the gate | `c4p20-tools/census.sh before|after` (canonical `readRustSourceClosureV1`; native, wasm32-unknown-unknown, wasm32-wasip2; lib and cfg(test)) | all 7 crates inside 32 files / 262,144 bytes / 4,000 active lines in every configuration, `overLimit` false (0 `over True`). Examples (native lib, before > after, files / bytes / lines): algebra 2/50,753/1,298 > 7/52,294/1,337; editor 2/107,323/1,385 > 7/108,998/1,430; mesh 2/65,216/1,360 > 6/67,030/1,383; mesh-repair 2/84,091/1,870 > 6/85,574/1,894; mesh-surface 2/49,364/1,086 > 5/50,354/1,104; session 2/82,154/1,625 > 7/84,574/1,690; sfm 2/92,261/1,351 > 6/94,612/1,382. Worst ratio after: sfm `--test` closure 11 files / 156,757 bytes / 2,879 lines, ratio 0.720. `c4p20-census-{before,after}.txt` |
| `new crates <pkg> --check-applied --base=f856dff0813` | `c4p20-tools/checkapplied.sh <label>` (= `ulimit -n 65536; bun ./📜️script.ts new crates '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust' --check-applied --base=f856dff0813 --sandbox=<scratch>`) | before (baseline, `c4p20-checkapplied-before.log`): exit 0, crates 47, native base 3329 / combined 3341 / `onlyCombined` 0, `--test` 4252 / 4259 / `onlyCombined` 0, admission and shared empty. After (wave 7, `c4p20-checkapplied-after3.log`, sandbox in the fresh scratchpad; the earlier `after`, `after-run1-killed` and `after2` runs died on a peer infra edit, a kill and `EMFILE`): exit 0, crates 47, native base 3329 / combined 3341 / `onlyCombined` 0, `--test` 4252 / 4259 / `onlyCombined` 0, admission and shared empty; identical to the baseline. `healedBase` additionally lists the five new editor children (new files with no base counterpart; informational, not a gate row). |
| debug logs | `check.py` | none added |

## 6. Snapshots, foreign errors, hand-offs

- Pre-edit snapshots (`F/c4p20-tools/orig/`, md5 of the working tree at the start): sfm `589f9dc3...`, sfmunit `0bc6f8cb...`, surface `caed49fb...`, mesh `f91b9df1...`, repair `e5a5cccc...`, editor `5c2a6808...`, session `d51a7356...`, algebra `689d34fb...`. External test companions used only for the widening scan: `meshunit`, `sessionunit`, `algebraunit`, `algebraexact` (not edited).
- Foreign errors (all pre-existing, identical before and after, none caused by the split): the 98 `--test` privacy rows and the `resolve` / `type` / `other` rows of the family belong to other crates (reconstruct, engine, modes, snapshot tests); the stub-noise in section 5; the `E0382` on `budget` is hidden by the stub and unproven either way. The remaining `Bounded*Preparation` fields that `mesh` reads (`phase`, `cursor`, ...) were not flagged by the oracle after the fixes; real `cargo check` may still show a second layer behind the former E0616/E0624 sites.
- `type CorrByPoint`, the `use std::collections::HashMap;` line and the three `mod tests` / `mod exact_tests` mounts stay in their parents on purpose.

## 7. Real-check commands for the compile queue (not run: compile admission is closed for the fleet)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml' -p semio-s-artifact-remodel-remodeling-algebra -p semio-s-artifact-remodel-remodeling-sfm -p semio-s-artifact-remodel-remodeling-mesh-surface -p semio-s-artifact-remodel-remodeling-mesh-repair -p semio-s-artifact-remodel-remodeling-mesh -p semio-s-artifact-remodel-remodeling-session -p semio-s-artifact-remodel-remodeling-editor --lib --keep-going --message-format=short
```
Run it again with `--tests` instead of `--lib`, and with `--target wasm32-wasip2` (lib). Hold the fleet compile mutex (`🗑️generated/cargo-mutex.lock`) for the call. Expected: the foreign errors above under `--tests`; the former `mesh` / `mesh-repair` E0616 and E0624 should be gone.

## 8. New folders (34, each with a `🦀️.rs`; paths relative to `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

- `✏️editor/⚙️engine/📸️sfm/🛰️synthetic-fixtures-and-priors`
- `✏️editor/⚙️engine/📸️sfm/🧱️incremental-state`
- `✏️editor/⚙️engine/📸️sfm/🧭️seed-and-registration`
- `✏️editor/⚙️engine/📸️sfm/🔩️bundle-and-snapshot`
- `✏️editor/⚙️engine/📸️sfm/🧪️tests/🔬️unit/🔀️geometry-tests`
- `✏️editor/⚙️engine/📸️sfm/🧪️tests/🔬️unit/🧭️incremental-tests`
- `✏️editor/⚙️engine/📸️sfm/🧪️tests/🔬️unit/🌐️global-and-prior-tests`
- `✏️editor/⚙️engine/📸️sfm/🧪️tests/🔬️unit/🎯️five-point-and-diagnostics`
- `✏️editor/⚙️engine/🥽️mesh/🔺️surface/📐️trimesh-and-vectors`
- `✏️editor/⚙️engine/🥽️mesh/🔺️surface/🕸️halfedge-topology`
- `✏️editor/⚙️engine/🥽️mesh/🔺️surface/🧊️iso-surface-extraction`
- `✏️editor/⚙️engine/🥽️mesh/📤️interchange-preparations`
- `✏️editor/⚙️engine/🥽️mesh/✅️bounded-validation`
- `✏️editor/⚙️engine/🥽️mesh/🪄️bounded-postprocess`
- `✏️editor/⚙️engine/🥽️mesh/🏭️pipeline`
- `✏️editor/⚙️engine/🥽️mesh/🩹️repair/⏱️bounded-preparations`
- `✏️editor/⚙️engine/🥽️mesh/🩹️repair/🧼️clean-and-repair`
- `✏️editor/⚙️engine/🥽️mesh/🩹️repair/🕳️hole-filling`
- `✏️editor/⚙️engine/🥽️mesh/🩹️repair/🧊️voxel-close-and-validate`
- `✏️editor/🔌️io-and-commands`
- `✏️editor/🌉️action-bridge`
- `✏️editor/🚧️retained-commands`
- `✏️editor/🎛️artifact-editor`
- `✏️editor/📜️manifest`
- `✏️editor/🧵️reconstruction-session/🧭️run-contract`
- `✏️editor/🧵️reconstruction-session/🔑️trace-checkpoint-and-ingestion`
- `✏️editor/🧵️reconstruction-session/🎁️products-and-outbox`
- `✏️editor/🧵️reconstruction-session/🏃️run-job`
- `✏️editor/🧵️reconstruction-session/🔍️revalidate-job`
- `🧬️schema/➕️algebra-internals/🧱️dense-types`
- `🧬️schema/➕️algebra-internals/🔗️lattice-sparse-and-errors`
- `🧬️schema/➕️algebra-internals/📉️factorizations-and-eigen`
- `🧬️schema/➕️algebra-internals/🔁️iterative-and-svd`
- `🧬️schema/➕️algebra-internals/🌀️schur-and-roots`

## 9. Files

Edited (8): the parent files of section 2, and, through the visibility fixes of section 2.1, three of the new children. Created: the 34 children above. Tools: `F/c4p20-tools/` (`c4p20.py`, `widen2.py`, `rsitems.py`, `defscan.py`, `globuse.py`, `members.py`, `testnames.py`, `modclash.py`, `check.py`, `live.py`, `oracle.py`, `posdiff.py`, `posdiff-full.py`, `run-oracle.sh`, `census.sh`, `census.ts`, `tax.ts`, `rf.sh`, `checkapplied.sh`, `vis_fix.py` (the first, direct form of section 2.1, superseded by `vis_edits` in `specs/repair.py` and `specs/surface.py`), `families/c4p20.json`, `specs/*.py`, `orig/*.rs.txt`). Logs: `F/🗑️generated/c4p20-*`.
