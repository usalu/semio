# R100: an owner's stored result is no longer keyed by the build library

Status 23:04 UTC: LANDED 23:00:49 to 23:02:13 UTC (window 23:00 to 23:05, lock `landing.lock` waited for R99's hold until 23:00:46, held 1 min 24 s, released). Live `script-inputs/🟨️.mjs` a5efb64a6bde (before 7ce68847148c).
Scripts: `T/🧰️scripts/R100/` (`mk.py` the anchored edits, `land.py`/`restore.py`/`window.sh`/`release.sh`/`restore.sh`, `batch.sh` the parallel test battery, `run.py` runs a staged test or the live tests against a staged module, `measure.ts`).

## 1. Which key carried the library bytes (measured)

Not a record key: `projectRecordKeyV1` = `[projectAuthority, registryDigest, commandRevision, assemblyAuthority, "closure-references/4", root, json, targets, inputs]` holds the digest of the generator contracts, the catalog providers and the recipe key of the native command, and the declared inference stamps keep the bytes of the inference code out of every record (`🔖️stamps`, test "no record, parser identity or generation fact is keyed by the bytes of the implementation"). `implementationRevision()` (bytes of the inference component files) only names the reload of the in-process graph and the `emoji-project-json` nodes result, not an owner.

The library entered through the freshness rows, not through a key: the late closure of an owner (`target-sources`, `native-target`, `generic-target`) is a walk over the modules of its command script. A script that imports a declared tool (`process-router`, `cargo-command`, `owner-command` ... 29 tools in `🔌️provider-closures/🔣️.json`) was walked through the tool's modules, the file list was cut afterwards (`toolFinishV1` removed the covered files and named the tool by reference), but every node of the walk stayed in the entry as rows: analysis rows of each module of the build library, a `resolve`, `lstat` and `exists` row per import. `toolFinishV1` also derived the tool group inside the owner's scope, so the first owner of a process carried the walk of the tool group as well. Measured on the live tree before the change (`measure.ts`, late closure of three real targets, resolved like a project record holds it):

| target | rows | derived walk facts | support rows | library content rows (analysis, file, field) |
|---|---|---|---|---|
| dev `prepare stdio-deflate wgpu prototype` | 1 872 | 25 | 233 435 | 28 917 |
| schema registry rust `test` | 1 813 | 18 | 162 344 | 19 899 |
| job rust `test-session-return source` | 1 813 | 20 | 165 486 | 19 903 |

A store sample (194 `target-sources` entries of this machine) had 114 entries with 1.7 thousand build rows each. A change of the imports of any of those modules (a landing adds a file, an import or a directory) failed the analysis of that module (an analysis row survives a body edit: replay of the parser compares the result image; checked: comment, inline and new statement hold, a new import fails), so the walk answer, the entry and every record holding it were stale; in a take pass a stale entry cannot be derived (`BUILD_TAKE_MISS`), so a worker infers every owner again.

## 2. The change (schema-first: the declaration already exists, the walk now obeys it)
The declared tools of `🔌️provider-closures/🔣️.json` are the schema; no new file format. In `🧭️script-inputs/🟨️.mjs`:
- `reachWalkV1` gets `stops` (tool entry path to tool name) and returns `tools`: a module that is the entry of a declared tool, is no root of the walk and no router selected by command words, is not visited; its rows are no rows of the walk. `boundedScriptInputsV1` takes `stops` (part of the memo key, so a walk without stops is untouched), keeps the crossed tools in `cache.walkTools`.
- `targetScriptSources` passes `toolStopsV1(scripts, cover === "generic")` (a generic command stops only at tools whose view carries the external packages, like the cover of an entry script) unless the target keeps its whole walk (`keepsWholeWalkV1`: module globs, TypeScript project, runtime entries, module literal; same predicate as `toolCoverV1`, now shared).
- `toolFinishV1` names a crossed tool by reference and no longer derives the tool group inside the owner's scope (that derivation put the walk of the tool group into the first owner of a process). Files an owner reaches directly stay in its list also when a tool group holds them (sound without tracking the group; `files` of a real owner grew, dev 248 to 1 856, job 10 to 102, none of them library content; the keyed set is a superset of before).
- The walk question names the stops (`walkQuestionV1`), `deriveClosureFactV1` admits and applies them (a question with stops is never answered by the reach-set walk), so the derived fact of an owner is asked again exactly as it was recorded.
No migration: an entry or record written before keeps its own question and holds while the library does not change; the next change re-derives it under the new rules.
Not done by choice: the freshness of an owner still carries what its script imports outside the 29 declared tools (a TypeScript script that imports library modules directly, 2 725 rows for the dev owner; the repo mcp crate). The next step is one declared tool for the library tree (a tree tool whose group is a glob named input), then every `repo ...` script stops at the library root.

## 3. How each behaviour is proven
`🧭️script-inputs/🔌️provider-closures/🧪️tests/🛑️tool-boundary/🟦️.ts` (fixture workspace of the existing tool corpus, two owners that import the cargo driver tool):
1. "no content row of a declared tool is a row of an owner that imports the tool": RED before (rows of `lib/` analysis), green after.
2. "an edit inside a declared tool, one that adds a file and an import, leaves every owner that imports the tool fresh" (rows hold, supporting rows hold, derived answer equal): RED before, green after. This is the progress-reporting edit of the task.
3. "an edit of the own script of one owner stales that owner alone": green before and after (guards that the cut does not hide an owner's own input).
4. "an owner closure still names the tool and the tool group still holds the files the owner no longer walks": green both (guards soundness).
5. "a row that names the stamp of an inferrer holds while the stamp is declared and stops holding when the declaration is bumped, whatever the bytes of its sources": the contract of an inferrer is its declared stamp (`🔖️stamps`, bumped deliberately); green both (existing mechanism, pinned here).
6. Independent oracle: `🐍️oracle/🐍️.py` walks the relative imports with a regular expression and stops at the tool entry, the kept list equals it; esbuild 's metafile of the owner script is covered by the expanded list (third-party cross check, as `tool-cover`).
Before: 4 pass 2 fail on the old code (run through `🧰️scripts/R100/run.py`, staged module swapped in by a bun resolve plugin). After: 6 pass, in the live tree under the lock, plus the 33 test files listed in `T/landings.md`: 0 red (take-release has 2 reds that it had before).
Measured on the real tree (`measure.ts`, `m3.ts`): see section 1 for before; after: dev owner 28 917 to 2 725 library content rows (support 233 435 to 24 453), schema registry owner 19 899 to 4 (support 162 344 to 66), job owner 19 903 to 4 (support 165 486 to 2 899); 13 of 14 sampled Rust package owners carry 4 library rows, one 2 668 (the repo mcp crate, part of the tool).

## 4. NOT verified
- No worker was observed after the landing: whether `present` rises from 0 on g1-m07 and g1-m08 is open (the stores were reset by a peer at 22:28 UTC, so there is no before/after of one store). Expected: the next landing that touches library files outside a tool-reachable import leaves the closure rows of Rust owners as they are.
- The closure store of this machine was wiped at 22:28 UTC; the old-code staleness table above rests on the 22:20 sample.
- The 4774 owners were not all measured: 3 named targets and 14 sampled Rust package owners.
- The remaining direct library imports of TypeScript script owners (dev, repo mcp) still go stale with the library.
- Windows and Linux were not run; nothing in the change is platform specific (paths come from `resolve` like the existing tool table).

## 5. What other agents must know
- R97 (owner worker memory): nothing in `project-nodes/` was touched; the take pass is unchanged. R99: nothing under `🕸️graph/`.
- A real owner value (`closure` list) can be longer than before (see section 2); consumers that expect the pruned list should read `expandedCommandInputsV1` as before.
