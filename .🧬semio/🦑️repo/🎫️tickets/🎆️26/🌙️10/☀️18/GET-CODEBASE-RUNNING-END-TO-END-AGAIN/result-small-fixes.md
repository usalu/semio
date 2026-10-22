# result-small-fixes (wave 1022a, executor small-fixes)

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `R` = `🧰️framework`, `I` = `L/🏗️build/🕸️graph/📥️inference`, `C` = this ticket folder. Backups of every edited file: `C/🗑️generated/small-fixes/backup/`; patch scripts `C/small-fixes-tools/{patch.py,tsc.sh,runtests.sh}`; logs `C/🗑️generated/small-fixes/`.

## Status
DONE, all live. No git writes, no sub-agents, no process left running (runner `runtests.sh` detached at 06:57 finished, `DONE` in its summary).

## 1. Declaration drift (tsc)
- Baseline `tsc -p L/📦️packages/🟦️typescript/tsconfig.json --noEmit --incremental` (same config as ts-health, `NODE_OPTIONS=--max-old-space-size=8192`, log `C/🗑️generated/small-fixes/base1.txt`): 2 errors. (a) graph host `L/🏗️build/🕸️graph/🟦️.ts:328` `classifyImplementationRowsV1` missing from `I/🟨️.d.mts`. (b) graph-receipt test `I/🧪️tests/🔖️output-stamps/🟦️.ts:99` TS7053 (`moved` typed as a literal object, indexed by string).
- Fixed:
  - `I/🟨️.d.mts`: `import type` of `ImplementationRowV1, InferenceProtocolV1` from `./🔖️stamps/🟨️.mjs`; declared `classifyImplementationRowsV1`, `unclassifiedImplementationCodeV1`, `inferenceProtocolV1` (types from the stamps declaration, no `any`).
  - `I/🧪️tests/🔖️output-stamps/🟦️.ts:95`: `const moved:Record<string,string>=...`.
- Other new modules checked for .mjs/.d.mts drift: `I/🔖️stamps/🟨️.d.mts` `wire(registry?)` -> `wire()` (the .mjs takes no argument); `L/🏗️build/🧷️format-stamps/🟨️.d.mts` `createFormatStampsV1(options)` -> `options?` (.mjs defaults it to `{}`); `🔄️reexec/🟨️.d.mts`, `L/🏗️build/👁️observation/🟨️.d.mts` (`registerInferenceStampsV1`, `observedInferenceStampsV1`) match. `requireIssuedResourceControlDescendantV1` is TypeScript (no d.mts). 
- Result: `tsc` after the edits: rc=0, 0 errors (`C/🗑️generated/small-fixes/after1.txt`). Note: the last edit (section 2, removing three import lines) happened after this run; it only removes imports.

## 2. Test registration
Sibling pattern found: build tests are imported by the aggregator `L/🏗️build/🧪️tests/🟦️.ts` (lane `workspace-build:contract`); cache/dependency lanes are named entries in `L/⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts` (`inferenceTests`) plus `test-<lane>` targets in `L/⚡️caching/📋️project.json`; process suites are entries of `R/🔨️modules/🏃️process/📜️script.ts` (`suites`) plus targets in `R/🔨️modules/🏃️process/📋️project.json`.

| test file | registration (new) | already registered before |
|---|---|---|
| `L/🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts` (stale-impl-guard) | import in `L/🏗️build/🧪️tests/🟦️.ts` | no |
| `I/🔖️stamps/🧪️tests/🟦️.ts` (graph-receipt stamps) | import in build aggregator | no |
| `I/🧪️tests/🔖️output-stamps/🟦️.ts` (graph-receipt output-stamps) | import in build aggregator | no |
| `L/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts` (loader-own) | import in build aggregator | no |
| `L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts` (loader-own) | import in `…/🔏️authorizers/🧪️tests/🟦️.ts` (the `authorizers` lane aggregator) | no |
| `…/📦️dependencies/🧾️generation/🧪️tests/🟦️.ts`, `…/🚪️ingress/🟦️.ts`, `…/🛡️followup/🟦️.ts` (dep-receipt) | lanes `javascript-generation`, `javascript-generation-ingress`, `javascript-generation-followup` in script-commands map + `test-javascript-generation*` targets in `⚡️caching/📋️project.json` (siblings are the `javascript-*` lanes, not the aggregator) | no |
| `R/🔒️leases/🧪️tests/🎟️writer/🟦️.ts` (lease-proof interleaving + writer tests) | suite `resource-lease-writer` in process `📜️script.ts` + `test-resource-lease-writer` in process `📋️project.json` | no |
| dep-receipt `…/🧾️incremental/🧪️tests/🟦️.ts`, `…/🚦️concurrent/🧪️tests/🟦️.ts` | - | yes (`javascript-incremental-sync`, `javascript-concurrent-generations`) |
| loader-own `…/🔏️authorizers/🧪️tests/🧭️closure-law/🟦️.ts`, `…/🧾️diagnostics/🧪️tests/🟦️.ts`, `…/📥️data/🟦️.ts` | - | yes |
| lawcov-tests' 13 files | not touched (integrate-r7 owns them) | - |

Both JSON files still parse (checked with `json.load`); 5 new `test-*` targets (3 caching, 1 process) exist.

## 3. Runs (each file once)
| file | command | result |
|---|---|---|
| format-stamps | `bun test ./…/🧷️format-stamps/🧪️tests/🟦️.ts` | 13 pass |
| graph stamps | `bun test ./…/🔖️stamps/🧪️tests/🟦️.ts` | 17 pass |
| output-stamps | `bun test ./…/🧪️tests/🔖️output-stamps/🟦️.ts` | 4 pass |
| closure sites | `bun test ./…/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts` | 2 pass |
| authorizer sites | `bun test ./…/🔏️authorizers/🧭️sites/🧪️tests/🟦️.ts` | 3 pass |
| dep generation (main) | `bun test ./…/🧾️generation/🧪️tests/🟦️.ts` | 11 pass, 1 FAIL (pre-existing, not mine: `stamp and anchor … storage laws stay pinned` asserts `projectListing(` occurs 2x in `🗂️workspaces/🟦️bun/🟨️.cjs`, now 6x; same failure dep-receipt reported) |
| dep ingress | `bun test ./…/🚪️ingress/🟦️.ts` | 10 pass |
| dep followup | `bun test ./…/🛡️followup/🟦️.ts` | 7 pass |
| lease writer | `bun C/lease-proof-tools/run-test.ts ./R/…/🎟️writer/🟦️.ts "" <scratchpad dir>` (plain `bun test` refuses: "Original parent Script process invocation required") | 5 pass, 0 fail |

The lease writer run uses the envelope helper and a scratchpad artifact dir (ticket path exceeds the 256-char lease path limit).

## Not verified
- The lanes themselves (`bun ./📜️script.ts test <lane>`) were not executed: they require a Script process envelope and the workspace-build aggregator was not run as a whole.
- Not checked: lease-proof's remark that `requireIssuedResourceControlDescendantV1` is "not re-exported from the package". `R/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` (line 5) and `…/📥️invocation/🏃️process/🟦️.ts` (line 1) do re-export it. I did not change them; the owner should confirm.
