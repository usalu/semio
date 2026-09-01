# Report — Nx `{workspaceRoot}` file-reference gap, and the next blocker it exposed

## Diagnosis confirmed

`jsonTokens` in `🧹️normalization/🟦️.ts` only ever matched `{workspaceRoot}/<path>` when the value
ended in a `/**/*` glob (`workspaceGlob` regex at the old line 3737). A literal single-file value —
`{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🟦️component.ts` in
`🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📋️project.json` line 5 — fell through
every branch (not a glob, not escaped, not a mutation-structural test path) and produced **zero**
reference token. This was a DETECTION gap, not a rewritability gap: before the fix the plan generated
no `edits` row for that value at all (proven by the fixed plan producing exactly one new
`oldValue`→`newValue` edit at that JSON offset, below). `{projectRoot}/…` was checked repo-wide (99
`📋️project.json` files) and never names a single file — only `**/*` globs and two unrelated build
`outputs` directories (`../../🤖️generated`, `pkg`) — so it is correctly left untouched.

## Fix

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`, `jsonTokens`: added a
`workspaceFile` branch — `/^\{workspaceRoot\}\/([^*]+)$/u` — parallel to the existing `workspaceGlob`
branch, plus its escaped-value unsupported-reason counterpart. `[^*]+` naturally excludes every glob
(they all contain `*`), so no double-counting with `workspaceGlob`; verified by test.

## Test (language-agnostic, both ways)

New: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️nx-workspace-root-file-reference/🟦️.test.ts`
— extracts the actual `jsonTokens` implementation via the TypeScript AST (same pattern as the
neighbouring `🧪️json-reference-owner-lookup` suite) and runs it through two independent compilers
(Bun's transpiler, `typescript`'s). 7 cases: non-glob file value detected + exact token shape; glob
still detected and NOT double-counted as a file; `{projectRoot}/**/*` never rewritten; both compilers
agree byte-for-byte. **Before the fix: 2/7 failed** (`Expected length: 1, Received length: 0` — the
exact detection gap). **After: 7/7 pass.** Registered end-to-end: `project.json` target
`test-nx-workspace-root-file-reference`, `📜️script.ts` route, both launch catalogs (order 410.2135);
verified via `bun x nx run @semio-tech/repo-lib:test-nx-workspace-root-file-reference --skip-nx-cache`.

Fixture paths in the test are synthetic (`🧪️nx-workspace-root-fixture/…`), not real repo paths —
using the real replication path made the test file itself a live physical reference the plan then
had to rewrite, which produced a false `unresolved=1` on the very first plan re-run (caught and
fixed before proceeding).

## Plan re-run (real command)

```
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules/📡️replication" --baseline bb06c41f73f0122fbed315b7487428b976f99921 \
  --plan "$T/🗑️temp/🔣️nxref.json" --workers 4
[clean taxonomy plan] moves=64 roots=0 relocations=0 symlinks=0 removals=0 edits=90 regenerations=2
  unresolved=0 digest=c521cd8a1f6b4571421a7c5ef560087909ddece5ffa1236f52d7d9b78fd7c793
```

`edits` grew from 89 → 90. The new row, confirmed present with a real rewrite pair (not merely absent
from `unresolved`):

```json
{
  "path": "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📋️project.json",
  "structuredLocation": "/@value[6]/workspace-file@160",
  "oldValue": "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🟦️component.ts",
  "newValue": "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🟦️.ts"
}
```

## Apply (real command) — rolled back again, with a DIFFERENT error

```
bun ./📜️script.ts clean taxonomy apply --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --baseline bb06c41f73f0122fbed315b7487428b976f99921 --plan "$T/🗑️temp/🔣️nxref.json"
[clean taxonomy apply] state=rolled-back moves=0 edits=0
journal: .🧬semio/…/🧾️taxonomy-transaction/🔖️c521cd8a1…/🔂️attempts/🔢️000001/🔣️.json
error: "Post-state contains 1 structured reference(s) to disposed source paths"
```

Same top-level message text as the original defect, but a **different culprit** — proof this is
progress, not a recurrence. Root-caused the same way (grepped all 64 `move.sourcePath` values across
tracked files with `rg -F -f`, cross-checked against `plan.edits` per file):

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json`
— a **0-edit** file — contains, as plain JSON string values inside a large census array:
`🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📦️glue.rs` and
`🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🧪️vitest.config.ts` (both real
`move.sourcePath` values in this plan). This file self-declares
`"decisionState": "non-authoritative-concurrent-source-byte-drift"` and
`"observedAt": "2026-08-27"` — it is a periodically-regenerated repo-wide inventory snapshot (1050
admitted package files, 229 candidates), not a hand-authored reference. The post-apply
`lexicalTargetIncomingReferences` check (`🟦️.ts:11446` area) treats its literal path strings as live
structured references anyway, but the reference planner emits **zero** edits for this file, so any
apply that moves a file appearing in this census will roll back. This is a separate, unscoped guard
(possibly guard-chain item #11: checks "path string matches a real repo path" as a proxy for "this is
a reference the plan must keep live", when the file itself declares it is a non-authoritative,
externally-regenerated snapshot) — not fixed here per the ticket's explicit scope; needs its own
diagnosis session on whether the fix is (a) exempting self-declared non-authoritative census fixtures
from the disposed-reference check, or (b) teaching the reference planner to rewrite census entries too.

## Disk state after rollback

`🧰️framework/🔨️modules/📡️replication/🟦️component.ts` and `…/🦀️component.rs` both still present,
unmoved — rollback restored the pre-apply tree as designed.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` (fix)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️nx-workspace-root-file-reference/🟦️.test.ts` (new test)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` (new target)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` (new route)
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (new launch entries)
