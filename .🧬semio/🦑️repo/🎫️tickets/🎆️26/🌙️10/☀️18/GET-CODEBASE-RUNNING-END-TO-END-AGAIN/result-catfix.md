# catfix: taxonomy catalog load fix and new-folder check

## Cause
`loadTaxonomy` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`, rule at line ~1332) requires every `generatorContracts[id].inputPatterns` to be unique and in default JS `sort()` order. Three contracts in `📚️library/🔣️taxonomy.json` had had 4 to 7 `📚️library/**` patterns appended after the sorted block:
`actor-typegen`, `flow-browser-package`, `repo-entity-kinds` (the appended entries were `🏗️build/📥️inputs/🧊️stable-file`, `🗂️workspaces/📁️data-directories`, `🚨️policy`, `🧪️testing`, `🏃️process`, `🎮️playground`, `🎛️commands/📝️commit`; repo-entity-kinds additionally had `🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts` misplaced). Not a duplicate problem, only ordering.

## Changes
- `📚️library/🔣️taxonomy.json`: re-sorted the `inputPatterns` array of those three contracts (same members, no pattern added or removed; validator untouched).
- `📚️library/🔣️taxonomy.json`: new `semanticDirectoryMemberKinds["members-of-bootstrap"]` (owner kind `bootstrap`, member `🕸️manifest-closure`, source registry), inserted after `members-of-teaching`. Needed by `🎓️teaching/🛂️proctor/🏗️bootstrap/🕸️manifest-closure/` (peer-created 01:50, not from a split executor).

## Verification
- `bun C/c6db1-tools/tax.ts C/c6db1-tools/newdirs.txt`: loads, `{"dirs":93,"unresolved":[]}` (the 93 c6db1 folders all resolve).
- All directories born today (macOS birthtime >= 2026-10-18, from `git ls-files --cached --others`, excluding ticket folders and node_modules; script `C/🗑️generated/catfix/todaydirs.py`): 1211 dirs (re-run after a second sweep; 1185 on the first). Result: all resolve except two:
  - `.🧬semio/🦑️repo/🎯️goals/TEST-GOAL-JSON-LIFECYCLE-TEST-1792286468902165000` and `TEST-GOAL-LIFECYCLE-TEST-1792285652461939000`: stray artifacts of a goal-lifecycle test run (03:07 and 03:21), untracked. Goal slug dirs are data, not taxonomy (existing `🎯️goals/ANIMATE` resolves to null the same way), so this is not a registration gap. They look like leaked test output; not deleted (not mine).
  - Before the fix `🛂️proctor/🏗️bootstrap/🕸️manifest-closure` was rejected; now resolves to `members-of-bootstrap`.
- Negative control: `🏗️bootstrap/🕸️not-registered-x` is reported unresolved, so the check does reject unregistered names.
- `bun repo --help` boots (prints usage, stages loading/running/complete, 1.4 s).

## Open
- Folders created after the last sweep are not covered; re-run `python3 C/🗑️generated/catfix/todaydirs.py out.txt && bun C/c6db1-tools/tax.ts out.txt` (the sweep needs `core.fsmonitor=false` since the git daemon fails).
- The leaked `TEST-GOAL-*` dirs should be removed by their owner; whichever test creates them does not clean up.
