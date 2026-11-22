# Closure inventory leaves: torn writes and recovery (2026-11-22)

## Root cause
Content-addressed leaves in `.cache/b/g/m/build-input-closures/<sha256>.json` were published by `publishInputClosureV1` (`build/📥️inputs/🟨️.mjs`) with `writeFileSync` + `renameOverBusyFileSyncV1`, with no fsync. After a crash or forced restart the rename can reach disk before the data (zero-filled leaves, 4661-5151 bytes, same millisecond). Afterwards:
- the lean reader threw `Inventory record content differs from its name`, and the reuse path (`👁️observation/📥️reuse`) propagated it, failing the graph build on every run;
- the writer could not heal the leaf either: `persistedExists(path)` then `verify()` threw `Original immutable input closure bytes required` for a stable leaf with other bytes.

All closure leaf writers (inference adoption, farm staging adoption, closure repair `closureLeafPutterV1`) go through `adoptStagedGraphLeavesV1` -> `publishInputClosureV1`, so that is the single writer.

## Fix
1. Durability: new shared helpers in `filesystem/🧾️observation/🔁️churn/🟦️.ts`: `publishFileDurablySyncV1` (exclusive temp in the target directory, full write, fsync file, close, rename over busy target with retry, fsync directory, temp removed on failure before rename) and `syncDirectorySyncV1` (no-op on win32, tolerates filesystems refusing directory sync with EINVAL/ENOTSUP/EPERM/EISDIR/EBADF). `publishInputClosureV1` uses it.
2. Recovery, writer side: `publishInputClosureV1` verify now returns false for a stable leaf whose size or bytes differ from the content-addressed bytes (still throws for non-regular/symlink/multi-link, and throws a typed concurrent edit when identity changes while reading). A false result retires the corrupt leaf by atomically replacing it with the correct bytes under the writer's custody.
3. Recovery, reader side: `readInventoryRecordLeanV1` throws an error with code `BUILD_INVENTORY_CORRUPT` (`inventoryRecordCorruptV1`) for sha256 mismatch or invalid JSON/UTF-8 (all other refusals unchanged). `missingInventoryNamesV1` treats it as a cache miss: the name goes into the missing set with its identity stamp; the final recheck requires that stamp to be unchanged (a leaf that changed meanwhile still raises `BUILD_STORE_CHANGED`, as today), so concurrent changes are still refused.

## Tests
- `lean-inventory/🧪️tests` (fixture + schema extended with `zeroBytes`, `corrupt`): zero-filled leaves (4661 and 1 byte), name/content mismatch and non-JSON are corrupt; other refusals are not. Python hashlib oracle kept. 16 pass (bun test, direct).
- `churn/🧪️tests/💾️durable` (imported by the churn suite): operation order open(wx), write, fsync, close, rename, open dir, fsync dir; win32 skips dir sync; refused dir sync tolerated, EIO propagates; failed fsync leaves nothing; real filesystem bytes. 5 pass; `@semio-tech/filesystem-observation:test` via native-exec: 61 pass, 0 fail.
- `storage/🧪️tests/🩹️torn-closure`: zero-fill a published leaf, detect corrupt, republish via `adoptStagedGraphLeavesV1` inside `withWriter`, assert bytes and Python sha256. NOT verified here: the whole storage project fails in this environment before my test (pre-existing `SQLITE_MISUSE: flags must include SQLITE_OPEN_READONLY or SQLITE_OPEN_READWRITE` at `new Database(..., {create:false})` in `💾️storage/🟨️.mjs:187`, bun 1.4.2; the pre-existing refusal test fails identically). The replace logic was verified instead with `$T/verify-closure-replace.ts` (extracts the real `publishInputClosureV1` text; zero-filled and short leaf are healed, no temp left, nlink 1): prints OK.
- `observation/🧪️tests/📥️reuse` (existing, covers the reuse reader incl. `{}` mismatch and malformed leaves): 12 pass.

## Follow-ups (not done)
- `closureLeafPresentV1` (closure-repair) is existence-only, so a corrupt local leaf is reported `present` on the fleet-refetch path and not refetched; `readExactSourceInventoryV1` still throws on corruption there.
- Inference optional leaves (`adoptStagedGraphLeavesV1` optional branch) still use unsynced writes; left unchanged for performance and because they are sealed/verified on use.

## Changed files
- filesystem/🧾️observation/🔁️churn/🟦️.ts (helpers)
- filesystem/🧾️observation/🔁️churn/🧪️tests/🟦️.ts (import), 🧪️tests/💾️durable/🟦️.ts (new)
- build/📥️inputs/🟨️.mjs (publishInputClosureV1)
- build/📥️inputs/🪶️lean-inventory/🟨️.mjs, 🟨️.d.mts, 🧪️tests/🟦️.ts, 🧪️tests/🔣️.json, 🧪️tests/🧬️schema/🔣️.json
- build/👁️observation/📥️reuse/🟨️.mjs
- build/📥️inputs/💾️storage/🧪️tests/🟦️.ts (import), 🧪️tests/🩹️torn-closure/🟦️.ts (new)
- ticket: verify-closure-replace.ts, this report
