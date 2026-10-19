# audit-verify-1019d: verification of result-rg-refused, result-split-s, result-xplat-linux, result-fixups2, result-tst3a, result-vit2, result-owners-fix, result-xplat-boot

Read-only auditor `aud-verify4`. No source edits, no builds, no `bun repo`, no cargo. Tests run only as explicit files, one at a time. Scratch copies and oracles are under the session scratchpad (`/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/`). Shell `find` over the whole repo was not used (it stalled); `git grep` and scoped `grep` only.

Paths: `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`, `MESH` = `🧰️framework/🔨️modules/🧊️3d/🥽️mesh`, `C` = this ticket.

## 0. Verdicts

| executor | verdict | basis |
|---|---|---|
| rg-refused | PASS WITH ISSUES | 14 crates re-measured, claims match (except server, drift). Mesh family split lossless. Facade re-exports unused, one undisclosed visibility change. Not compiled (as stated). |
| split-s | PASS for femu and fem3 solve-step; NOT VERIFIABLE for pz5e | femu and fem3 lossless by line-multiset oracle. pz5e tree moved after the split, paste/import modules absent. |
| xplat-linux | FAIL (2 defects) | directory suite 12/12 pass. But capacity accounting can be bypassed, and one tracked test still depends on the deleted loop observer and is red. |
| fixups2 | PASS | no stale callers of the old workspace command names in tracked files; declarations validated. |
| tst3a | PARTIAL, spot checks PASS | frozen census restored byte-identical; gate targets uncached as claimed. Other claims not re-run. |
| vit2 | NOT RE-RUN | out of the focus list; not verified in this pass. |
| owners-fix | PASS (partial) | `registry-consistency` test file 28/28 pass. `crate-tree` file not re-run. |
| xplat-boot | PASS (partial) | `privileged: true` removed from compose; `.devcontainer` storage and locale claims not re-verified. |

## 1. Issues (ordered by severity)

### I1 (high, xplat-linux): directory capacity accounting can be bypassed

- `NB/🧱️fixed/🟦️.ts:105` (the `measure` hook): returns before any check when `bytes < measureCommitBytes`, and `measureCommitBytes` is 16 MiB (`NB/🐧️directory/🧬️schema/🔣️.json:62`). The `bytes` compared is the reservation size (`allocation.bytes`), not the amount the producer writes.
- `NB/🟦️.ts:111`: `commit` calls `port.measure?.(backing, allocation.bytes, actual)` after the producer has written. Nothing measures at `reserve`; the journal check is reservation arithmetic only.
- Same line 105: the admission uses `requestBytes: 0`, so the incoming write is never added. It only refuses when already-measured usage exceeds `filesystemBytes`, and only after the data is on disk.
- A directory partition has no hard cap (a fixed image ended the write with ENOSPC). So: an allocation under 16 MiB is never measured; a larger write under a 16 MiB+ reservation is caught only after it landed; growth after `commit` is never measured.
- Consequence against the report claim "cannot be bypassed by a large single write": not true as written.
- Fix: measure at reserve and commit with `requestBytes` = the declared bytes (`NB/🐧️directory/🟦️.ts:84` already supports it); drop the 16 MiB gate; re-measure in `recheck`; add a test "reserve 1 MiB, write 64 MiB, commit: refused".

### I2 (high, xplat-linux): stale tracked consumer of the deleted loop observer, test is red

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:207` (`neutral compiler cache retirement parent mapping consumes actual private chain`) still asks `NB/🧱️fixed/🟦️.ts` for `withOwnedLinuxChildrenV1`. HEAD has it (2 matches); the working tree has none. Result: `bun test` of that file with `-t` fails (`Expected to contain "readNativeStorageMountedChildV1"`).
- The report says the grep for the loop identifiers is empty. True for `nativeLinuxLoopMountV1` and `readFixedKernelText`; false for `withOwnedLinuxChildrenV1` (`git grep -l` over `🧰️framework`, `.devcontainer`, `.github` returns this file).
- Fix: point the law at `observeNativeDirectoryStateV1` / `listNativeDirectoryChildrenV1`, or delete the Linux-loop clause.

### I3 (medium, rg-refused): mesh facade re-exports nobody uses

- `MESH/🦀️.rs:4-6`: `pub use semio_framework_3d_mesh_journal::{ArenaJournal, MeshJournal, MeshOverlay};`, `…_surface::MeshSurfaceJob`, `…_quality::{analyze_polygon_soup, MeshBounds, MeshMassProperties, MeshQualityReport, ScalarStats}`.
- Grep for these nine names in `🧰️framework/🔨️modules/🧊️3d`, `✏️s/🔌️plugins/🏗️fem`, and all tracked `🧰️framework`, `✏️s`, `🌎️hub` returns only the facade lines. The report's claim that "2 generation3d consumers and the mesh test crate keep one public path" is not supported by the tree.
- AGENTS: re-export explicitly only if a client needs it. Fix: delete lines 4-6 (or keep only the used names).

### I4 (medium, rg-refused): undisclosed visibility changes in the splits

- `🛎️services/👀️file-watcher/🦀️.rs:10-11`: HEAD `#[doc(hidden)] pub struct FileStamp {` becomes `struct FileStamp {` (private); `#[doc(hidden)] pub cancelled` / `pub previous` fields also lose `pub` in the moved text. No consumer of `FileStamp` found in `🧰️framework/🛍️products/💻️os`. The report does not mention this.
- `🔄️turn/🦀️.rs` (reactor-turn root): three `#[doc(hidden)]` attributes dropped (oracle: missing `#[doc(hidden)]` x3). Report says only `pub(super)` widened to `pub`.
- Fix: confirm intent; restore `pub` where the public facade needs it; list both in the report.

### I5 (medium, split-s): pz5e losslessness cannot be confirmed; tree moved after the split

- `✏️editor/` (5d, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`) now has no `🍓️paste-job` or `🥦️import-job` module (the report lists both as split children). Root `🦀️.rs` mounts (`kit`, `command_variants`, `play_app`, `retained_command_works`, `play_app_retained_commands_and_tool_proofs`, `artifact_editor`, `app_assembly`, `testkit`) differ from the report.
- `impl Puzzle5dPasteJob` / `impl Puzzle5dImportJob` text now appears in `🖌️session/📌️paste/🦀️.rs` and `🖌️session/📥️snapshot-import/🦀️.rs` (tracked, not split-s output).
- Oracle with root + the four module files that exist: 415 of 1,603 non-blank lines of `split-s-tools/orig/pz5e.rs` absent (the rest of the tree is not in the set, so this does not prove loss; it proves the split cannot be re-checked as recorded). The orig snapshot is 2,120 lines; the report says 2,116.
- Fix: split-s to re-run `modsplit.py lossless` against `orig/pz5e.rs` on the current tree, and record where the paste and import jobs now live.

### I6 (low, xplat-linux): image-era literal in a Linux witness test

- `NB/📤️publication/📚️witness/🧪️tests/🟦️.ts:109` and `:295` use `fixed.img` on Linux. The report says the publication literal was replaced in `🌐️platform` and `🎛️owner` only. Fix: `fixed.dir` (or the directory mapping).

### I7 (low, rg-refused): server crate drift and compile status

- `semio-framework-server` now measures prod 0.653 / test 0.806 (report section 6: 0.864 / 1.394 at 15:25, then a presence carve was recommended). Report section 6 is stale.
- No `cargo check` was run for any split crate (report says so, and the source-boundary refusal explains it).

### I8 (low, AGENTS): pre-existing `[DEBUG] ` console line in a test

- `NB/🧱️fixed/🧪️tests/🌱️birth/🛂️host.ts:14` (`console.log("[DEBUG] Actual host refused …")`). Present at HEAD; not introduced by this round, but AGENTS requires temporary DEBUG logs to be removed. Fix: remove or make it a permanent non-debug assertion.

### I9 (low, fixups2): route admission only

- `probe-routes.ts` checks admission (167 routes, no throw). It does not run `bun repo verify <new name>` for each renamed declaration. Open: run the new verify names once through the real route.

## 2. Verified claims (evidence)

rg-refused, canonical gate re-measured (`C/aud-census8-tools/crate-gate8-disk.ts` copied to scratchpad, output redirected there; `NAMES` filter; prod / test, ratio):

| crate | claimed | re-measured |
|---|---|---|
| repo-tickets | 0.712 / 0.740 | 0.712 / 0.740 |
| repo-mcp | 0.557 / 0.601 | 0.557 / 0.601 |
| os-vcs | 0.688 / 0.938 | 0.688 / 0.938 |
| plugin-decl-app | 0.795 / 0.875 | 0.795 / 0.875 |
| 3d-mesh-document | 0.512 / 0.875 | 0.512 / 0.875 |
| 3d-mesh-journal | 0.472 / 0.544 | 0.472 / 0.544 |
| 3d-mesh-surface | 0.194 | 0.194 |
| 3d-mesh-quality | 0.113 | 0.113 |
| os-services | 0.760 / 0.773 | 0.760 / 0.773 |
| os-services-native-io | 0.219 | 0.219 |
| reactor-turn | 0.879 | 0.879 |
| reactor-turn-limits | 0.031 | 0.031 |
| tests-01 | 0.645 | 0.645 |
| server | 0.864 / 1.394 (15:25) | 0.653 / 0.806 (drift, I7) |

No crate refused. Gate: 14 crates, 0 refused, 1 at or over 0.9 (os-vcs test).

Split losslessness (line-multiset oracle, non-blank trimmed lines):
- Mesh family (`MESH`, HEAD 62 `.rs` vs working tree 62): 20 HEAD lines missing, 16 extra; every one is a `use`, `mod`, `#[path]`, `pub use`, or path-qualifier rewrite (for example `super::super::X` becomes `semio_framework_3d_mesh_document::X`). No body line dropped.
- Reactor-turn (`🔄️turn`): missing lines are the moved items, `pub(super)` to `pub`, the `#[doc(hidden)]` drops (I4), and the `include_str!` path. No glob re-export introduced (explicit import from `semio_framework_plugin_reactor_turn_limits`).
- split-s femu (`✏️s/…/🔬️unit/🦀️.rs`, orig 1,331 non-blank lines): 6 missing, all `include_str!("../…")` with one extra `../` per fragment depth (as the report says). 0 body lines dropped; no `//#region` lost.
- split-s fem3 solve-step (orig 1,058 lines): 0 missing; extras are `use super::*;`, `impl Fem3dNumericalChild {` wrappers, region markers, `mod`/`#[path]`.
- split-s pz5e: NOT verifiable (I5).

xplat-linux:
- `bun test` of `NB/🐧️directory/🧪️tests/🟦️.ts`: 12 pass, 0 fail (explicit file, scratch artifact dir).
- `privileged: true` removed from `.devcontainer/docker-compose.yml` (only a comment mentions it).
- No importer of `nativeLinuxLoopMountV1`, `readFixedKernelText` in tracked `🧰️framework`, `.devcontainer`, `.github` (only I2's file matches the sibling name).

fixups2:
- Old single-word declarations (`verify-combination-stress`, `test-material source|native`, `window-utility-check`, `test-staged-payload-funding`, `test-retained-pack-encoding`, `test-window-utility-native`, `value-admission-check`, `test-native-receiving-original-source`, `test-retained-equality`, `viewport-presence-check`, `operation-diagnostic-host-check`) remain only as target names, `command` strings, router registrations, and package.json scripts. No CI workflow, doc, registry, or launch file references them (`git grep` over the whole tracked tree outside tickets).
- New declarations present: `"workspaceCommand": ["verify", "fem-combination-stress"]` (`✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📋️project.json:15`) and `["verify", "window-utility-check"]` (`🧰️framework/📦️packages/🟦️typescript/📋️project.json:37`); the rest are multi-line and were checked by the validator.

tst3a (spot checks):
- `🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` (239,319 bytes) sha256 prefix `c3e39ed5122709c5` equals `git show 0c3e789:` (restored byte-identical).
- `test-registry-catalog-gitlink-boundary` and `test-preflight-reference-basis` are `"cache": false` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`.

owners-fix:
- `bun test` of `LIB/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts`: 28 pass, 0 fail. The crate-tree file (claimed in the same 35-pass run) was not re-run.

## 3. AGENTS compliance of the new code (spot checks)

- Docstrings in `NB/🐧️directory/🟦️.ts` start with an emoji; no in-definition comments seen.
- No glob re-export in the new mesh crates (`pub use …::*`): none found. Pre-existing glob `pub use semio_framework_3d_mesh_document::*;` at `MESH/🦀️.rs:2` (HEAD too) violates "no glob re-exports": fix with an explicit list.
- Split children use `use super::*;` (glob import, not re-export; fem3 children, 5d modules): acceptable under the report's checks, but list it.
- `[DEBUG]`: only I8 (pre-existing). The xplat-linux DEBUG removal is confirmed (no `[DEBUG]` in `NB/🐧️directory`, preflight, readiness).
- Several new fragment files and crate roots are untracked (`??`) in git status (split-s fragments, the three mesh crates, `📦️packages` dirs). Stage them before ticket close, or the ticket's file list drops them.

## 4. Commands (exact)

- `SEMIO_TEST_ARTIFACT_DIR=<scratch>/art perl -e 'alarm 280; exec @ARGV' -- bun test ./<NB>/🐧️directory/🧪️tests/🟦️.ts` → 12 pass.
- Same with `./<F>/🧪️tests/🟦️.ts -t "parent mapping consumes actual private chain"` → 1 fail (I2).
- `./<LIB>/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests/🟦️.ts` → 28 pass.
- `NAMES=<14 crate names> bun <scratch>/gate/crate-gate8-disk.ts` (copy of the census tool; writes only to scratch) → table above.
- `oracle.ts` / `oracle2.ts` (line-multiset, scratch).
- `git grep` scoped to `🧰️framework`, `.devcontainer`, `.github`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`, `♻️mit-bestand` (excluding `🎫️tickets`).

## 5. Not verified in this pass

- vit2 (vitest configs): not re-run.
- rg-refused and split-s compile (cargo): not run by anyone (as reported).
- xplat-linux Linux runtime end to end (publication, funded namespaces): not run (as reported).
- xplat-boot: PowerShell, Windows FFI, devcontainer features: not run (as reported).
