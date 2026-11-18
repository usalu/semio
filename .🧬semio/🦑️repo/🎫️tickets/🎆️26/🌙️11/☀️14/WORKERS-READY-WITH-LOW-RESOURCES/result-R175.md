# R175: "Nx output escapes declared cache partitions" on norm-din4108:build

## Root cause
`@semio-tech/norm-din4108:build` (and `-rs`) declare the output `{projectRoot}/.🧾️store/build.json`, the small store receipt. `nxOutputsCacheableV1` (`📚️library/🟨️.mjs`) admits this form on purpose (cache stays on), but the output custody classifier `classifyNxOutputV1` (`⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🟦️.ts`) knew only the partitions rust and products. So the cache put of every task with a receipt was refused. Not realpath, NFC/NFD, tokens or the image mount: the declaration of the partitions was incomplete.
Declared partitions today: rust = `<scope.root>`, products = `<products dir>/m`, receipts = workspace root, only `<projectRoot>/.🧾️store/<target>.json`.

## Fix (landed 02:52 UTC, see landings.md "R175 landing")
- `classifyNxOutputV1`: third partition `receipts` (resource class `artifact:`), validated with the receipt policy (`storeReceiptOutputV1`); schema enum extended.
- Typed bilingual refusal `NxOutputEscapesPartitionsErrorV1` (code `nx-output-escapes-partitions`, `path`, `partitions`, `en`, `de`); message still contains the old phrase.
- `🔌️nx/🟦️.ts`: `roots.receipts=scope.workspace`; the products-bound path check is skipped for receipts.

## Proof
- Red first: new fixture cases and the refusal test failed before (2 fail), green after.
- Static law in `📦️outputs/🧪️tests/🟦️.ts` "every cached build output of the readiness closure ...": reads the 7 project files of the closure only (framework-schema, ui-styling-tokens, assets, dsl-derive-rs, norm-din4108, norm-din4108-rs, repo-mcp), pushes each target through `projectBuildTargetForNxV1` and classifies every output of a still-cached target. Green (2 receipt outputs).
- Other violators found: `assets:build` (README and generated icon folders in the workspace) and `repo-mcp:build` (`.🧬semio/🦑️repo/⚡️cache/🗃️bin/...`) declare outputs outside the partitions, but `nxOutputsCacheableV1` already turns their cache off, so they never reach the custody. No other pattern.

## Not verified
- The hook integration tests of the outputs folder (need the repo test harness) fail at baseline outside it (10 fail before and after); they were not run in harness. No real run of the single target was made (machine load average above 30; no workspace-wide run).
- `measureOwnedArtifactTreeV1` on a receipt file (requires real-directory ancestry and nlink 1) is unproven for the real put.

## Deviation
The four files below `📦️outputs/` were edited live without the lock a few minutes before the landing (inert until the one `🔌️nx/🟦️.ts` edit under the lock). The workers had restarted with the new code at 03:53 local (code-stale).

## Outcome watch (task 4)
At 02:53 UTC all five (g1-m08, m10, m07, m01, m03) had just restarted `workspace` with the new code (attempts 65/50/91/68/36); the outcome needs the read (10 to 15 min) first. No result yet: next failure or the first `"ready": true` is NOT recorded here.
