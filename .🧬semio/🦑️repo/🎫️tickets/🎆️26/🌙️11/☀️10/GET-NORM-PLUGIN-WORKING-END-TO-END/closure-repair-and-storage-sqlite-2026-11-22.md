# Closure repair presence, durable optional leaves, storage SQLITE_MISUSE (2026-11-22)

Follows `closure-inventory-torn-write-2026-11-22.md`.

## 1. Presence means "content matches its name"
`closureLeafPresentV1` (closure-repair) was `existsSync`; a corrupt local leaf was `present` on the fleet-refetch path and never refetched. It now calls `readInventoryRecordLeanV1` (the lean sha256 verification, bounded by `maximumBytes`, no duplicated hashing); any refusal (absent, torn, corrupt, multi-link) is not present. `obtainClosureLeafV1` passes its `maximumBytes` through. The refetch writes through `closureLeafPutterV1` -> `adoptStagedGraphLeavesV1` -> `publishInputClosureV1`, which already replaces a corrupt stable leaf.
Test (`closure-repair/🧪️tests`): zero-filled, `{}` and truncated leaves are not present, are refetched, bytes equal the content addressed bytes, python3 hashlib agrees with the file name. 7 pass (bun test, direct).

## 2. Optional leaves are durable
All unsynced write+rename pairs of optional graph leaves now use `publishFileDurablySyncV1`:
- `build/📥️inputs/🟨️.mjs` `adoptStagedGraphLeavesV1` optional batch (unused `writeFileSync`/`renameOverBusyFileSyncV1` imports removed)
- `build/👁️observation/🟨️.mjs`: inference record cache (two sites), producer/publication image, receipt, observation block (the fence calls stay before and after the publish)
- `build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`: precision receipts (`ip/`)
Owner checkpoint packs (`📥️load/💾️owner-checkpoint`) already fsync the file before rename and were left unchanged.
Tests updated to the new seam: `inputs/🧪️tests/📤️publication/📏️boundary` (fault injection now through the helper's `operations`), `inputs/🧪️tests/📤️publication` (the "Corrupt Closure" and "Corrupt Replacement" fixture cases now expect `accepted`, i.e. healed, and assert the bytes; this is the behaviour the previous fix introduced), `resident/🧪️tests/✂️cut-order` (source contract), `closures/🧪️tests/📇️formats` (port). Results: boundary + publication 3 pass; cut-order 1 pass; observation reuse 12 pass. `formats` has 3 failures unrelated to this change (`requireClosureReadDescriptorV1 is not defined` in the closure read worker, "Bounded renamed graph retirement path required"): other agents' work in progress.

## 3. Storage project SQLITE_MISUSE
Root cause: not a connection lifetime problem. `refusal/🟦️.ts:33` opened the ledger gate with `new Database(path,{create:false})`; bun 1.4.2 refuses an options object with neither `readonly` nor `readwrite` (flags 0, `SQLITE_MISUSE`). The test then runs `begin exclusive`, so it needs `{readwrite:true,create:false}`. Fixed. The error surfaced at `storage/🟨️.mjs:187` only because that frame is the caller (the test body runs inside `withGraphFactWriterV1`), and the first failure closed process admission for the rest of the file.
Run: `repo run repo-graph-maintenance:test` and native-exec. Torn-closure and refusal tests now run and are not among the failures. Totals: 21 pass, 32 fail.

### Remaining storage failures (separate cause, not fixed here)
All remaining failures start with `Exact Original Graph format owner required` (then "Build test receiver admission closed"). With a temporary `[DEBUG]` probe (reverted) the check showed `callerRepositoryRootV1(control)` = the checkout (`/Users/ueli/Documents/semio`) while the workspace is the private repository (`.cache/test-sandbox/<id>`): the tests use `withBuildTestInvocationV1`, whose birth is issued for the checkout, against `repository.root`. The check in `📇️format/🟦️.ts` is unchanged from HEAD, so this predates this work and needs the tests (or the test birth grant `native-build-caller` with `scratch:"sandbox"`) to issue the invocation for the private repository (`repository.invoke`). Also failing: three source-extraction tests (`warm-lease-infrastructure`, `no-pressure`, `deferred-optional-funds-original-writer`) and one import-list expectation, all after the admission closed or from concurrent edits of the extracted sources.

## Changed files (relative to build = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`)
- build/🕸️graph/🩹️closure-repair/🟨️.mjs, 🟨️.d.mts, 🧪️tests/🟦️.ts
- build/📥️inputs/🟨️.mjs
- build/👁️observation/🟨️.mjs
- build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs
- build/📥️inputs/🧪️tests/📤️publication/🟦️.ts, 🔣️.json; 📏️boundary/🟦️.ts
- build/📥️inputs/💾️storage/🧪️tests/🪣️refusal/🟦️.ts
- build/🏡️resident/🧪️tests/✂️cut-order/🟦️.ts
- ⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📇️formats/🟦️.ts
