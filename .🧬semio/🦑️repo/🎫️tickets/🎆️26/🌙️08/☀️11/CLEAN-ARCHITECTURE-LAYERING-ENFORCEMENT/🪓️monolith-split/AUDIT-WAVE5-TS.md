# Audit: Wave 5 TS splits (read-only)

Auditor: Haiku, foreground, no sub-agents, no modifying git command, no source edits. Date 2026-10-16.
Repo root = `/Users/ueli/Documents/semio`. Paths below are repo-relative. Raw outputs: `F/🗑️generated/audit5t-*` (`F` = this folder).

Scope: the nine RESULT files of wave 5 (ROOT-SCRIPT-SPLIT, RENDERER-TS-TESTS-SPLIT, REPO-TEST-VSCODE-SPLIT, MISPLACED-E2E-TESTS, INTERPRETER-KERNEL-TS, UI-BARREL-IMPORTS, EPHEMERAL-SINGLE-OWNER, SDK-BOUNDARY, SMALL-CROSS-EDGES).

Method and baseline: the change set is `git diff 1bae6115bc5` (working tree) plus untracked files, excluding `.🧬semio` ticket folders. That gives 19 deleted, 23 renamed, 1944 modified, 556 added entries and 19 untracked paths. Git now records one commit after the base (`36fbe9da786`), so the peers' commits are included. Earlier `git status` snapshots no longer show the split files as uncommitted, so the base diff is the reliable change set.

## Headline

The claimed entry sizes are accurate. Most delete-and-repoint work is clean. The problems are a broken launch entry, two cycle families the RESULT files did not report, a few oversized modules, and emoji-uniqueness violations. No compatibility barrel was found.

## Findings, ranked by severity

1. **Broken launch entry (Medium-High, introduced by SMALL-CROSS-EDGES P8).** `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` both contain the entry `🧪️test📜️history🔮️six-family-source`. Its `bun test` command still names `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`, which was renamed to `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/🪶️sqlite/🟦️.ts` (R081). The file is absent on disk. The rest of that command resolves. Evidence: `F/🗑️generated/audit5t-launch2.txt`, `audit5t-refs2-nontickets.txt`.

2. **New import cycle family through the worker-law suites (Medium, MISPLACED-E2E-TESTS).** `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` (line 7130, inside `if (import.meta.vitest)`) now dynamically imports `🧰️framework/🛍️products/💻️os/🧪️tests/📚️worker-law-suites/🟦️.ts`. That aggregator imports the suites, the suites import `🧰️worker-law-fakes`, and the fakes import types from the worker. The base worker had no such edge, and `worker-law-suites` is new. The cycle scan finds 44 distinct cycles through this aggregator. They are test-time cycles, not production load-order cycles. Evidence: `audit5t-cycles-interpreter.txt` (scan over the whole closure), `audit5t-cycles-summary.txt`.

3. **Ten cycles inside the kernel children (Medium, INTERPRETER-KERNEL-TS).** Every new kernel child's in-source block dynamically imports `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts`. That test imports `🚪️io-router`, `🧭️app-router`, `🗂️plugin-registry`, `🔌️plugin-source` and similar children back. Ten distinct cycles lie entirely among kernel children. The base had a single parent-test type loop (`typeof import("../../🟦️.ts")`), so the pattern existed, but it is now repeated per child. RESULT §4 claims "DAG verified: 0 cycles", which holds only for production edges. Evidence: `audit5t-cycles-intree.txt`.

4. **Dangling owner rows in library fixtures (Medium, REPO-TEST-VSCODE-SPLIT hand-off).** Two JSON rows still name the deleted harness `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json:203` and `…/🧼️remaining-package-purity-authority/🔣️.json:5553`. The library test imports and the nx inputs named in RESULT §7 were already re-pointed by a peer, so these two rows are the only remaining references.

5. **Oversized new modules versus the brief (Medium).** The brief expects about 1,500 lines. Measured with `wc -l`:
   - `…/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🟦️.ts`: 1,836
   - `…/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🟦️.ts`: 1,803
   - `…/📚️library/🎛️commands/✅️verify/🟦️.ts`: 1,539 (the class holds a 630-line `run` if-chain, per RESULT)
   - `…/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts`: 1,500
   - `…/⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🟦️.ts`: 1,490

   All are under the executor cap of about 2,500. RESULT-ROOT's "largest 1,836" is accurate. Evidence: `audit5t-added-ts-lines.txt`.

6. **Emoji not unique among siblings (Medium).** AGENTS requires "a unique and fitting emoji" for every docstring. The ROOT-SCRIPT-SPLIT result checked uniqueness only within each file. Among new siblings, the same docstring emoji repeats in five groups, which together cover about 41 modules:
   - `…/📚️library/🎛️commands` (10 children start with 🧭)
   - `…/🎛️commands/✅️verify` (5 × 🧭)
   - `…/⚖️laws/🎯️tool-job-coverage` (9 × 🧭)
   - `…/⚖️laws/🔬️interactivity-audit` (5 × 🧭 and 2 × 🎯)
   - `…/⚖️laws/🧩️plugin-consistency` (8 × 🧭 and 2 × 📐)

   Evidence: `audit5t-sibling.txt`. Separately, 33 folder-emoji sibling groups include a folder created or renamed in this wave (for example `🧪`, `🧬`, `📥`, `📦` in the kernel and SDK package). Those violate the fleet rule "never repeat an emoji among siblings", but several pre-date the wave.

7. **Policy changes admitted or not (Medium, for the parent).**
   - `ui-no-framework-packages` now reports 260 instead of 244. The 16 UI importers of the moved owner `🧰️framework/🔨️modules/🫧️transient` are the unadmitted policy line (EPHEMERAL-SINGLE-OWNER §2.3). The line was not applied.
   - SDK-BOUNDARY §3.3 added `dependencyTypesNot: ["aliased-workspace"]` to the `no-cross-technology-*` rule. That lowers the reported count from 1,807 to 470, partly by policy change. This is a relaxation of a rule, so the parent should review it deliberately.

8. **Stale comments and docs naming the deleted kernel path `🎠️kernel/🟦️.ts` (Low, comment only).** The path has no code importers. Stale references: about 12 Rust doc comments under `🧰️framework/🔨️modules/🎠️kernel/🎞️media-jobs/🦀️.rs` (lines 20, 79, 151, 170, 611), `…/🖼️ui/🦀️.rs:80`, `…/🧪️tests/⬇️media-export-encoding/🦀️.rs:2`, `…/🧪️tests/🐢️ui-dirty-scope/🦀️.rs:2`, two generation3d Rust tests, and `.cursor/plans/realtime_interaction_history_c71a5401.plan.md:31`. `🎠️kernel/🫧️transient` and its subpath have zero references. Evidence: `audit5t-refs2-nontickets.txt`.

9. **Re-export-only files (Low, classification).** The AST pass found 114 files whose statements are all `export … from`.
   - 112 are SDK facades under `🧰️framework/📦️packages/🟦️typescript/`. They are the intended public surface (`test-sdk-surface`), not compatibility shims. All 142 `exports` targets exist on disk.
   - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🟦️.ts` is new. It re-exports `📋️inventory` and is used only as the folder's owner entry by its own test. No old path ever existed there. Borderline, not a shim.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.d.ts` is a new declaration re-export.

   No file was found whose sole purpose is to keep an old path working.

10. **Docstring placement (Low).** Six kernel children start with `/// <reference types="vitest/importMeta" />` before the docstring (for example `…/🎠️kernel/🎛️activation-registry/🟦️.ts`). Twenty new TS files lack an emoji docstring. None belongs to the nine result sets; they are peer work (retention, leases, caching, `🧵️gis-inference-bridge/🟦️.ts`, `🌎️hub/🧪️tests/🎟️carried-phase/🟦️.ts`). Evidence: `audit5t-ast.tsv`, `audit5t-nodoc-list.txt`.

### Lower-severity or pre-existing items

- **Sizes verified:** root `📜️script.ts` 166 lines (claim 166), Interpreter entry `…/🗣️Interpreter/🟦️.tsx` 150 (claim 150), vscode entry `…/💻️client/🧩️vscode/🟦️.ts` 153 (claim 153), test-harness package entry 211 (claim 211). RENDERER's "largest 829" measures 828.
- **Dangling imports:** the import resolver checked 1,602 changed TS files and 9,228 relative specifiers. Nine failed. Seven are `?raw` or `?raw`-style suffixes that resolve once the query is stripped. Two are flow `✏️editor` test imports that point at absent `♻️retirement` targets, absent at base too (pre-existing): `🌊️flow/…/✏️editor/🎮️commands/♻️retirement/🧪️tests/🟦️.ts` and `…/🧪️tests/🔬️source-contract/🟦️.ts`. Evidence: `audit5t-missing-imports.tsv`.
- **Caching deletions:** `📏️inventory`, `📂️descriptors`, `💾️volume`, `📋️model/📨️receiver` (all deleted under `⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state`) have no unresolved importers.
- **Other deleted paths:** the GIS config `…/🌉️mcp/🧪️tests/🎚️config/🟦️.ts` (its importer resolves to the framework copy); `…/service-composition/🧪️tests/🟦️.ts` (only an assertion that it is absent); the Rust `🧬️context/🦀️.rs`, `👥️peer`, `🖥️host/🧪️tests/🔬️standalone` (no `#[path]` or include references). The SDK time-travel `🧭️arguments` path was added and deleted between base and HEAD, so it is in neither base diff. Zero references remain.
- **Rust include/path targets:** 1,525 references across 682 changed Rust files. `#[path]` targets all resolve. 21 `include!` targets are missing:
  - 3 in generation3d `…🔬️unit/🦀️.rs`, pointing at a fixture absent at base (pre-existing).
  - 18 in the untracked `…/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs`, which is a peer's in-progress work with its `🧫️fixtures` directory not yet present.
- **Three scripts naming the deleted sqlite test:** `…/📸️remodel/…/📦️packages/🦀️rust/📜️script.ts`, `…/🗒️note/📦️packages/🟦️typescript/📜️script.ts` and `…/🖥️host/🧪️testing/🧩️component/📦️packages/🦀️rust/📜️script.ts`. Their strings are unchanged since base and the targets are absent at base, so pre-existing.
- **`[DEBUG]` strings:** no temporary logs were found. The `[DEBUG] ` prefix is a repo convention (about 6,600 lines at base). 49 changed files gained such lines, mostly copies in the split test files, and the untracked test-host Rust file has 18 `eprintln!("[DEBUG] …")`.
- **Path length:** 11 changed paths exceed 240 UTF-8 bytes. The maximum is 258 bytes and 174 code points, under the 256-character AGENTS rule. All 11 pre-date the wave (plugin standards subset paths).
- **Launch files:** both parse as JSONC (12,815 and 9,758 configurations). 109 entries are new by name in each file. 5 of 6 path tokens resolve. The sixth points at a ticket artifact directory `…/🗑️generated/goal-framework` (`SEMIO_TEST_ARTIFACT_DIR`), an output path that is created at run time. The kernel-transient entry was renamed (documented). Duplicate names predate the wave (3 in `launch.json`, 1 in the seed).
- **Interpreter cycles touching the tree:** 60 cycles touch the Interpreter tree (none lies entirely inside it). They pass through ShellHelpers and Shell, outside the new tree. Not compared with the base.
- **Not cycles:** the test-harness, vscode, time-travel-harness and root-commands trees each have zero cycles touching their files (scans: 181, 181, 198, 184 cycles in the closure, all elsewhere).

## Per-check results

1. **Dangling importers:** see findings 1, 4, 8 and the resolver results above. The T harness and kernel paths are handled as described. The layout sqlite test has the launch-entry breakage in finding 1.
2. **Sizes:** see the lower-severity list above and finding 5.
3. **Compatibility barrels:** none found; see finding 9.
4. **Docstrings:** all 432 new TS/TSX files were checked (not a sample of 60). See findings 6 and 10.
5. **Launch files:** see the lower-severity list and finding 1.
6. **Debug and paths:** see the lower-severity list.
7. **Cycles:** the cycle scans used `dependency-cruise` with a `no-circular`-only wrapper (`F/🗑️generated/audit5t-nocycle.cjs`) over the six trees. Each scan follows the whole dependency closure, so the parser counts cycles that touch each tree. See findings 2 and 3.

## Limits

- Read-only. No `tsc`, vitest or cargo run was attempted.
- No base-commit comparison of the cycle counts was done, so "introduced" is judged from the base file contents and the structure of each cycle.
- The `git grep` checks cover tracked files plus the untracked list. Ticket folders were excluded and are not reported.

## Raw outputs (`F/🗑️generated/`)

`audit5t-status.txt`, `audit5t-diff-since-base.txt`, `audit5t-untracked.txt`, `audit5t-refs2-nontickets.txt`, `audit5t-missing-imports.tsv`, `audit5t-rust-missing.tsv`, `audit5t-ast.tsv`, `audit5t-sibling.txt`, `audit5t-launch.txt`, `audit5t-launch2.txt`, `audit5t-debug-added-by-file.txt`, `audit5t-long-paths-codepoints.txt`, `audit5t-cycles-*.txt`, `audit5t-cycles-summary.txt`, `audit5t-cycles-intree.txt`. Scripts used: `audit5t-*.cjs`, `audit5t-cycles.sh`, `audit5t-nocycle.cjs`.
