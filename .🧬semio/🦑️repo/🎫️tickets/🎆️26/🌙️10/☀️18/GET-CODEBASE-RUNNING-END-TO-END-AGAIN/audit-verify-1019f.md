# Audit verify-1019f: verification of ten executor result claims

Auditor `aud-verify6`, read-only (no source edits, no git-modifying command, no cargo, no `bun repo`). Scratch scripts in the session scratchpad (outside the repo). Freeze reference: 19:45 (brief-1019). Checked against the working tree at about 20:50.

## Verdict table

| Result | Verdict | One-line reason |
| --- | --- | --- |
| result-fem-fix.md | PASS | All 545 moved fixtures byte-identical to HEAD (sha256), 2416 include/fixture literals resolve, 0 missing. |
| result-rg-rest.md | PASS with risk | 16/16 new crates have build-owners rows; 13 `✏️s` crates are members of their artifact-level workspace; 3 split line-multisets only differ by re-pointed `#[path]`/`use` and wrapper lines. Risk: 3 `os` crates are not in `🧰️framework/Cargo.toml` (see F1). |
| result-rg-test.md | PASS | Both new crates are root members and have build-owners rows; the two visibility widenings are required by the `[[test]]` root that consumes them; declared `[[test]]` paths exist. Gate numbers not re-run. |
| result-xplat-win2.md | PASS | Seven on-disk states + ten crash points cover all four swap states (s0..s3); journal test run alone: 10 pass, 0 fail; no dangling refs to renamed Windows paths. Windows itself not exercised (as claimed). |
| result-bzl-suite.md | PASS with doc gap | All 12 contract digests recompute `ok` now; re-pins match a real schema change. Report omits a third re-pin (StorageLayout) and two `server` vector additions (F5). |
| result-bzl-breadth.md | PASS (code), test gap admitted | `.bazelignore` write, exported `bazelTaskCargoDirectoriesV1` / `issueBazelTaskEnvironmentV1` / `generateBazelWorkspaceV1` exist; no language-agnostic test for two of its four items (AGENTS violation, self-declared). Edits predate the freeze (18:58). |
| result-go-all2.md | PASS with flake | gofmt clean on all touched Go files; parity test 2/2 only with a raised timeout: under current load (load avg about 45) the default 5 s timeout fails the real-module test (F6). |
| result-tax2.md | FAIL | Renames left at least six dangling path references (F2); one post-freeze library edit (self-disclosed) (F3). |
| result-lay15.md | PARTIAL | Spot-check only: the new leaves exist and are imported; cruise and cycle counts not re-measured. |
| result-tst5.md | PARTIAL | Spot-checks pass (dead fn only in an absence assertion, `solarPvFixtureCases` exported and used, `schemaOf` unwrap present). Law needles were loosened to match widened visibility; owner sign-off still needed (F7). |
| result-lay15 / tst5 / bzl-breadth counts | not re-run | Totals (cruise counts, 876 green suites, 0 refused census) were not recomputed. |

## Findings

### F1 Workspace registration of the three `os` crates (rg-rest), risk, not verified

- The root `Cargo.toml` lists 564 members, all prefixed `🧰️framework/`. `🧰️framework/Cargo.toml` is a nested `[workspace]` with 405 relative members. The two member sets are disjoint at HEAD as well (HEAD root 553, HEAD fw 390), so this is a pre-existing dual-listing and not caused by rg-rest.
- `semio-framework-os-config-merge-policy`, `...-artifact-workflow-run-admission` and `...-plugin-instance-surface` are root members and have root `[workspace.dependencies]` rows (Cargo.toml lines 989-995), but they are not in `🧰️framework/Cargo.toml`. Their nearest workspace root is that nested file.
- The 13 `✏️s` artifact crates are members of their nearest workspace (the artifact-level `Cargo.toml`, `member=true` for all 13). Root `exclude` covers `✏️s`.
- Fix: none required for rg-rest's own claim. Ask the cargo lane to run `cargo metadata --locked --no-deps` (read-only; without `--locked` it can rewrite Cargo.lock) once the queue frees, to confirm cargo accepts the dual listing. If cargo rejects it, the three crates need `🧰️framework/Cargo.toml` rows too.
- Minor: the result header says 244 paths; `result-rg-rest-files.md` lists 247 bullet paths (it also says 244 in its header).

### F2 result-tax2: dangling importers (FAIL)

Checked by resolving quoted path literals against the removed paths (HEAD-tracked, absent now). Confirmed by `ls`:

| # | Referencing file (repo-relative) | Literal | Now exists | Frozen? |
| --- | --- | --- | --- | --- |
| 1 | `🧰️framework/📦️packages/🦀️rust/📜️script.ts:205` | `"🧪️tests/nested-io.ts"` (base `🔨️modules/🚪️io/⏱️control`) | `…/⏱️control/🧪️tests/nested-io/🟦️.ts` | no |
| 2 | `🧰️framework/🔨️modules/🏃️process/📜️script.ts:83` | `"🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts"` | `…/🎯️exact/🧪️tests/🍎️image/🟦️.ts` | no |
| 3 | `🧰️framework/🔨️modules/🌱️value/🧪️tests/🔬️script-commands/🟦️.ts:15` | `"♻️retirement/🧪️tests/🧺️heap.ts"` | `…/♻️retirement/🧪️tests/🧺️heap/🟦️.ts` | no |
| 4 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/📜️script.ts:6` | `"🧪️tests/📒️ledger.ts"` | `…/💾️storage/🧪️tests/📒️ledger/🟦️.ts` | yes (library) |
| 5 | `…/📚️library/⚡️caching/📜️script.ts:155` | `"📦️artifacts/🔒️bounded/🧪️tests/🍎️image.ts"` (and the `🍎️image` link to the same family) | `🍎️image/` directory only | yes (library) |
| 6 | `…/📚️library/⚡️caching/📜️script.ts` | `"💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🏪️scope.ts"` | `…/📥️opening/🧪️tests/🏪️scope/🟦️.ts` | yes (library) |
| 7 | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/📦️packages/🟦️typescript/📜️script.ts:12` | `"🖼️raster/🧪️tests/🟦️.test.ts"`, `"🧪️tests/🔬️unit/🟦️.test.ts"` | `🟦️.ts` sibling exists | no |
| 8 | Same package script | `"🩹️patch/🧪️tests/🟦️.test.ts"` (sibling check not run) | verify | no |

Also suspect (suffix match only, verify before fixing): the energy model package script and the stdio `🪟️bmp`/`📷️png` package scripts reference `🟦️.test.ts` files that have no `🟦️.ts` sibling either. Those may be deleted rather than renamed. Owner decision.

- Fix: update rows 1-3 and 7-8 now (not frozen). Rows 4-6 go into the post-freeze queue. Then re-run the `dangle` check (quoted literal ending with a removed path) to zero.
- The bundle check used by tax2 cannot see these: they are string paths in `resolve`/`join` calls, not imports.

### F3 result-tax2: freeze

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts` was modified at 19:47, after the 19:45 hard freeze. tax2 disclosed it as a repair of its own rename. Record it in coordination.md as a freeze exception.
- Library mtimes after 19:45: 18 files in total. Two of them (`⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` at 20:31 and `…/📥️opening/🏪️store/🧪️tests/🟦️.ts` at 20:40) are not attributable to the ten results; I did not identify the author of the other sixteen. Owner to attribute.
- The bzl-suite and bzl-breadth library edits are all before 19:45 (19:20-19:23 and 18:58). Those claims hold.

### F4 result-fem-fix: verified

- 544 HEAD-tracked FEM and engine fixtures plus the hand-moved schema-ancestor `🎟️fixed-dofs/🔣️.json`: 545 of 545 sha256 identical to `git show HEAD:<old>`. Old paths absent. No reader still uses the old `🎟️fixed-dofs/🧫️fixtures` form.
- `fem-fix-tools/verify2.py` (read-only run): checked 2416 `include_str!`/`fixture!`/`include_bytes!` literals, 0 missing; 266 `fixture!` in files not mounted by any Cargo path resolve by a heuristic (any crate whose literals all exist; all resolved).
- My own TS relative-import check: 45 fixture imports, 0 missing; 2468 `include_str!`, 0 missing.

### F5 result-bzl-suite: undisclosed pins and vectors

- `…/🔧️tools/🏗️bazel/🧪️tests/📋️contract/🔣️.json` changed in three pins, not two: `Command` (1ca805… to 2bb5b0…), `MaterializedPaths` (e22a05… to aa7543…) and `StorageLayout` (cdf17d… to e26909…). The `StorageLayout` schema (`🔧️tools/🏗️bazel/🧬️schema/🔣️.json`, +544 lines, `serverIdleSeconds` 60 to 120, new `server` owned path) belongs to another change, not xplat-win2.
- Two vectors gained `"server": "/owned/server"` (lines 218 and 234). Not in the report.
- `digest.ts` run from repo root: 12 of 12 `ok`. The re-pins match the current schemas, so they are legitimate. The gap is documentation: the report must name the StorageLayout re-pin, the owner of the schema change, and the vector additions.
- Runtime phases and canonical runner: not run (the report says so).

### F6 result-go-all2: planner parity test is timing-sensitive

- `C/gocanon/parity.test.ts` (second test runs the TS and Python planners on two real modules): under load it takes about 6.2 s and fails at bun's default 5 s timeout (`1 pass 1 fail`). With `--timeout 120000`: `2 pass 0 fail`.
- Fix: give that test an explicit timeout (for example `test(name, fn, 120_000)`) and keep the claim "2 pass" tied to the run that passed.
- gofmt -l: clean on `🔗️graphql/🧪️tests/🏹️policy/🐹️.go`, `⌨️cli/🧪️tests/🛠️tool/🐹️.go`, `⌨️cli/🧪️tests/📃️exhaustive-list/🐹️.go`.

### F7 result-tst5: loosened source laws

- Five `pub(crate)` to `pub` and about six moved-definition needles were re-derived from the present code, so the laws now accept the widened surface. `RendererAssetDecodeJob` is one example. tst5 itself asks the owners to confirm.
- Not a defect in itself, but it weakens a guard. Owners should either restore the law and move the item into the owning crate, or record the widening as an intended API in the law.

### F8 result-xplat-win2: crash recovery and renames

- Fixture `🪟️journal/🧪️tests/🧫️fixtures/🔣️.json`: states s0 (nothing renamed), s1 (first renamed away), s2 (second renamed in), s3 (all renamed, receipt left), plus torn, empty and absent receipts. Crash matrix: call 1..5 x before/after = 10 points, matching the five host calls in `hostSequence`. Every intermediate protocol state is covered.
- Refused states: 8 (torn with temporary, torn with missing volume, foreign directory at root or volume, both names holding the original, wrong temporary name, symlinked receipt, extra field). Refused renames: 4 rows plus rollback-refused test.
- Journal test run alone: `bun test` on `🪟️journal/🧪️tests/🟦️.ts`: 10 pass, 0 fail.
- Renames: `🐧️directory`, `🐧️image`, `🪟️image`, `🗂️system`, `📦️vhd`, `createNativeWindowsSystemGrantV1`, `windowsSystem`: 0 references. `fixed.vhd` remains only in the README as a deliberate "no longer selected" statement.
- `//` comments: none in the new `📁️directory` TypeScript. Docstrings start with an emoji.

### F9 result-rg-test: visibility widenings

- `HistoryFoldJob::close_step` and `terminal_is_empty` were private in HEAD (`🔁️fold/🦀️.rs`, lines 275-276 now `pub`). Both are used by `[[test]] history_transition_unit` (`🔀️transition/🧪️tests/🔬️unit/🦀️.rs`, lines 569-585), which is an external test crate. `HistoryFoldJob` is re-exported by `pub use retained_fold::*`. Justified.
- `semio-framework-plugin-window-config-replace` and `semio-framework-repo-dashboard-launcher` are root members and have build-owners rows (`@semio-tech/...-rs`, manifest `📋️project.json`).

### F10 result-bzl-breadth: test gap and deferred items

- Code checks: `.bazelignore` written by `linkBazelSources` (line 300); `bazelTaskCargoDirectoriesV1` (line 311) and `issueBazelTaskEnvironmentV1` (line 316) exported; `publishBazelImplementationV1` (line 350) and `generateBazelWorkspaceV1` (line 404) exported.
- AGENTS requires a language-agnostic test per feature. Items 2 (`.bazelignore`) and 4 (cargo directories) have none (self-declared open). Add fixture rows before the adapter is considered done.

### F11 result-lay15: partial

- Spot-checked: `🔌️retirement-backend`, `📄️retirement-file`, `🌱️birth/🧾️claim/` leaves exist; `storage/🟦️.ts`, `platform`, `claim` and `cleanup/command` import them. Cycle and rule counts (dependency-cruiser) were not re-run. Nothing here contradicts the claims.

### F12 hygiene

- An empty directory `C=.🧬semio/🦑️repo/` exists at the repo root (created 13:11, untracked, no files). It came from a shell variable assigned with the wrong quoting. Remove it when the coordinator next cleans the tree.
- `[DEBUG]` lines in `…/🏗️bazel/🧪️tests/🟦️.ts` (6 now, 5 at HEAD) are fixture markers, not temporary logs. Leave them.

## Commands run (read-only)

- `git ls-tree`, `git show HEAD:<path> | shasum -a 256`, `git cat-file -e`, `git status --porcelain`, `git diff HEAD --stat`: fem-fix identity, tax2 removed-path set, freeze mtimes via `stat`/`find -newermt`.
- `bun run` (scratchpad scripts): member/owner cross-checks (`xcheck*.ts`, `owners2.ts`), line-multiset and `#[path]` resolution (`lines.ts`, `pathcheck.ts`), dangling literal search (`dangle*.ts`, `tax2d.ts`), FEM literal resolution (`femref2.ts`, `femfixture.ts`).
- `bun test ./<file>`: journal crash-recovery test (10 pass); `gocanon/parity.test.ts` (1 pass 1 fail at default timeout; 2 pass with `--timeout 120000`).
- `gofmt -l` (go1.25 at `~/.local/go/bin`): clean.
- `python3 -I verify2.py` (read-only, fem-fix tool): 2416 checked, 0 missing.
- `bun run C/bzl-pin-tools/digest.ts` from repo root: 12/12 `ok`.
- Not run: cargo (any), `bun repo`, builds, `verify taxonomy`, dependency-cruiser, the bzl canonical runner, the 916-suite re-run, Windows.

## Concrete fixes, in order

1. tax2 (F2 rows 1-3, 7-8): edit the listed literals now; re-run the dangling check.
2. tax2 (F2 rows 4-6): queue for after the freeze lifts.
3. Coordination: record the tax2 post-freeze library edit (F3) and attribute the three later library edits.
4. go-all2 (F6): set an explicit timeout in `parity.test.ts`.
5. bzl-suite (F5): amend the report to list the StorageLayout re-pin, the `server` vectors and their owner.
6. bzl-breadth (F10): add fixture rows for `.bazelignore` and `bazelTaskCargoDirectoriesV1`.
7. rg-rest (F1): cargo metadata check when the cargo lane is free; fix the 244/247 count.
8. tst5 (F7): owners to confirm each loosened law.
9. Remove the empty `C=.🧬semio` directory (F12).
