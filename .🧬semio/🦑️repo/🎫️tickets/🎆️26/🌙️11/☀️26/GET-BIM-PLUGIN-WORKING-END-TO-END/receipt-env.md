# Receipt Env Fragility (BUN_ENABLE_CRASH_REPORTING)

## Root cause
- The dependency generation receipt seals every ambient variable matching the manager's `producerInputs.environmentPrefixes` (bun: `BUN_`, `NPM_CONFIG_`) unless listed in `producerInputs.environmentIgnored` (`observeJavascriptDependencyGenerationProducerV1`, `🧾️generation/💾️storage/🟦️.ts`).
- `BUN_ENABLE_CRASH_REPORTING` (set to `0` by the Claude Code harness, absent for peers) matched `BUN_` and was not ignored, so each env flip made the receipt stale with `class=configuration-changed path=env:BUN_ENABLE_CRASH_REPORTING`.
- Reproduced before the fix: `repo dependencies ensure` printed "receipt of scope . is stale ... env:BUN_ENABLE_CRASH_REPORTING" for both `=0` and unset, alternating (log evidence in earlier run, deleted after).
- The literal message "Actual declared requested Source scope required" is raised in `javascriptDependencySourceOwnerRequirementsV1` (`💾️storage/🟦️.ts` line ~432) when a requested scope is not in the declared owner model. I could not reproduce that crash cheaply: `ensure` handles the stale receipt typed and exits 0 (stale DATA line, then re-sync). The untyped crash was only seen in `repo run` under concurrent reseal and is a secondary symptom of the stale receipt; not changed (open).

## Decision (schema-first)
Exclude, do not force from the launcher: the variables are tool telemetry/crash reporting and do not influence installed bytes. An existing schema mechanism already models this (`environmentIgnored`, same pattern as the heap-sizing and `BUN_INSPECT*` entries; the process tool schema already denies `BUN_INSPECT*`). Reused it.

## Fix
Added to bun `producerInputs.environmentIgnored`: `BUN_ENABLE_CRASH_REPORTING`, `BUN_CRASH_REPORT_URL`, `BUN_FEATURE_FLAG_DISABLE_TELEMETRY`.
Updated the sealed ledger fixture and re-minted the contract vectors with the independent Python oracle (`python3 -I 🐍️.py mint`; two digest lines changed: bun `descriptorSha256`, `materialized.generation`).

## Files (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/`)
- `🔌️managers/🍞️bun/🔣️.json`
- `🔌️managers/🧪️tests/🧫️fixtures/🧾️generation.original.json`
- `🔌️managers/🧪️tests/🔣️.json` (minted vectors)
- `🧾️generation/🧪️tests/🌡️heap-environment/🟦️.ts` (new test: telemetry vars never change the fingerprint; registry var still does)

## Verification
- `repo test-file` heap-environment (3 pass), generation tests (17 pass), managers seam (93 pass, 1 fail).
- The 1 failure is the pre-existing, unrelated ratchet "production code outside the adapters names no package manager ..." hitting others' files (`remote-execution/sidecar/ready`, `host-capacity/disk-recovery`, `cache-gc/command`); not touched.
- After the fix `repo dependencies ensure` with `BUN_ENABLE_CRASH_REPORTING=0`, then unset, then `=0`: 0 stale receipts, all exit 0. No cargo builds run.

## Open
- Untyped crash text in `repo run` on a stale receipt: the `run` path should call the same typed refresh as `ensure`; not reproduced, so not changed.
