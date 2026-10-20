# result-deps5b (executor deps5b, 2026-10-20)

Re-staged and applied deps5's 15 storage edits plus the install/registration edits onto the CURRENT live files. No dependency sync or lock was started (`ps` showed none before applying).

## Cause of the apply failure
Only one anchor had moved: the test registration (`"javascript-storage":"...tests/🟦️.ts",`) left `⚡️caching/📜️script.ts` (tax-stage moved it) and now lives in `⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts` (1 occurrence). All 15 storage anchors, the `📋️project.json` anchor and the `📥️install.json` anchor still matched exactly once, so walkperf's and tsc-stage's storage changes were preserved untouched and every fence in deps5's ledger is unchanged.

## Changes (files)
- `C/deps5-tools/apply.py`: `TARGET["caching-script"]` re-pointed to the sibling module `🧪️tests/🔬️script-commands/🟦️.ts`; `bases.json` regenerated after apply.
- `C/deps5-tools/staged/fence.test.ts`: producer-law assertion relaxed. tsc-stage inserted `const finals:Awaited<...>[]=[];` between `await verifyReady();` and `for(const scope of requested)`, so the literal `await verifyReady();for(const scope of requested)` no longer matches. It now asserts `await verifyReady();` appears before `for(const scope of requested)` (same law, order preserved; the following `await fence();` assertion is unchanged).
- Applied with `apply.sh apply --rebase` (then `--rebase --overwrite-new --ignore-sync` once, for the fixed fence test; no sync was running):
  - patched `.../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (15 ops)
  - patched `.../⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts` (3 suite registrations)
  - patched `.../⚡️caching/📋️project.json` (3 targets: `test-javascript-installed-paths|endpoint-fence|provider-walk`)
  - patched `.../💾️storage/🧬️schema/📥️install.json` (`scanProgressWorkUnits: 4096`)
  - wrote `.../💾️storage/🧪️tests/{📏️paths,🚧️fence,🔭️providers}/{🟦️.ts,🔣️.json}`

## Verification
- `sh C/deps5-tools/run-tests.sh real`: paths 2 pass / 0 fail; fence 5 pass / 0 fail (first run 4/1 because of the stale literal above, fixed); providers 1 pass / 0 fail; diagnostics 1 pass / 0 fail.
- `bun test ./<storage>/🧪️tests/🟦️.ts`: 26 pass / 20 fail, identical to deps5's pre-apply baseline (counts equal). The failures inspected are stale tests, e.g. `publishJavascriptWorkspaceExportsV1` no longer exists in the file the test parses, and a lstat-count expectation of 12 versus 16 in the membership-custody test. Failure names were not diffed one by one against the old tree.
- `bun test ./<storage>/🧪️tests/👓️reading/🟦️.ts`: 42 pass / 38 fail, identical to deps5's baseline counts.
- `bun test ./<storage>/🧪️tests/📤️diagnostics/🟦️.ts`: 1 pass / 0 fail (via run-tests.sh).
- `bun build --target=bun 📜️script.ts`: OK. `bun repo --help`: prints the usage banner, no errors.
- Not run: `bun repo dependencies sync|lock` (coordinator), end-to-end sync timing, `bun repo run ...:test-javascript-*` registered targets, Windows/Linux/devcontainer.

## Open items
1. The first sync after this apply will rebuild walkperf's generation receipt once (storage module is in its closure).
2. The 20 + 38 stale existing tests remain (listed in result-deps5.md section 7).
3. The scratch logs under `C/🗑️generated/deps5b` were deleted; `/private/tmp/deps5-art` holds test artifacts only.
