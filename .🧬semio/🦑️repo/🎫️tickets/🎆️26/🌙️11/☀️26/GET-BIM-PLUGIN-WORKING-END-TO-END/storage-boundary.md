# Storage boundary: `Native storage path crosses filesystem boundary`

## Symptom
Every `repo cargo ...` failed before compiling with `Internal error: Native storage path crosses filesystem boundary (reference 1faf526b)`, preceded by notices "Unrelated attached image ignored, it is no funded child" for `.cache/b/r/m/cargo`, `.cache/b/r/m/u/sealed`, `.cache/b/r/m/u/w`.

## Root cause
Not a stale mount. The rust partition `.cache/b/r` is now sharded (`shards.jsonl`, `s/<id>/fixed.dmg`): each shard image is mounted at its logical path inside the home mount `.cache/b/r/m` (`cargo`, `u/sealed`, `u/w`, ...). That is the designed layout (`nativeImageShardLayoutV1`: "every path below it keeps its length and meaning"), but two consumers still assumed one image per partition:

1. `requireNativeStoragePathV1` (`💾️storage/🌐️platform/📋️contract/🟦️.ts`) compared every path component's device with the home root device and refused any change. The compiler layout lives in `m/cargo/prototype/...`, and `m/cargo` is the `cargo` shard, so `m/cargo/prototype/captures` was refused (confirmed with a `[DEBUG]` log: `root=.cache/b/r/m`, `current=.cache/b/r/m/cargo`; caller `withRepositoryCompilerStore*`).
2. `unfundedMountV1` plus the persistent child verification (`💾️storage/🧹️cleanup/🎛️command/🟦️.ts`) classified a funded child only as `<dir>/m` + `<dir>/fixed.dmg`, so the shard mounts were reported as "unrelated attached image" (noise, plus a sandbox-reap sweep per run).

## Fix (framework, no local state change needed)
- `🍎️image/🧩️shard/🟦️.ts`: new pure readers over the ledger: `nativeImageShardLiveLayoutsV1`, `nativeImageShardMountPointV1(directory, path)` and `nativeImageShardRowV1(directory, {root,image})`.
- `🌐️platform/📋️contract/🟦️.ts`: `requireNativeStoragePathV1` now lets the device change exactly once per component only when the component is a directory that is the mount point of a live shard in the ledger of the root's partition (root must be `<partition>/m`); the device is then rebound to the shard's, so a further unregistered crossing is still refused. The ledger is read lazily, only on a device mismatch (no cost on the hot path). Symlinks, escapes and unregistered mounts are refused as before; a retired shard's mount is refused again.
- `🧹️cleanup/🎛️command/🟦️.ts`: an attached image that is exactly a live shard of the partition (mount point and backing from the ledger) is tolerated silently, before the "unfunded" classification, so no false notice and no sandbox sweep.

Local state: nothing was unmounted or modified; all mounts (`cargo`, `u/sealed`, `u/w`, and the other shards) are legitimate shards and were left attached.

## Verification
- `cd ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model && repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --no-deps` now exits 0 (was 1 on every run).
- Tests (via `repo test-file`):
  - `💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts`: new macOS test mounts a real home image and a real shard image at `m/u/w`; asserts the crossing is refused without a ledger entry, accepted with a live `born` entry (below the mount and at it), unregistered siblings stay in the home device, and refused again after `retired`; device ids are cross-checked against `/usr/bin/stat -f %d` (independent oracle).
  - `💾️storage/🍎️image/🧩️shard/🧪️tests/🟦️.ts`: new ledger-only test for `nativeImageShardRowV1` / `nativeImageShardMountPointV1` (live, retired, wrong backing, wrong parent, no ledger).
  - cleanup command tests re-run: 8 pass.

## Files
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`
- tests: `…/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts`, `…/💾️storage/🍎️image/🧩️shard/🧪️tests/🟦️.ts`

## Follow-up: whole-partition retirement (`🧹️whole/🟦️.ts`)
A terminal retirement removes the capsule tree, so crossing into a mounted shard must never be allowed (it would delete shard contents, and a mount point cannot be removed anyway). Teaching it "the same way" therefore means recognising live shard mount points to give an explicit reason instead of the generic one:
- New `nativeRecomputableRetirementCrossingV1(directory, path)` (reuses `nativeImageShardMountPointV1`) answers "Live image shard must be retired before its partition: <path>" for a live shard mount point, else the unchanged "Terminal capsule crosses filesystem boundary". `inventory` uses it. Detached shards (backing under `s/<id>`) are ordinary host-device files and are inventoried as before.
- Test: `🧹️whole/🧪️tests/🟦️.ts` (born, retired, sibling path, no ledger); whole tests 7 pass, 0 fail.

## Corrupt ledger
The shard schema (`🍎️image/🧩️shard/🧬️schema/🔣️.json`) defines only the `refused` (no capacity) and `partial` messages and has no refusal shape for a corrupt ledger, so none was invented. A corrupt ledger still fails closed with `Closed image shard event required`. A typed refusal needs a schema addition (decision for the coordinator).

## Open
Nothing else. No cargo builds were started for the follow-up.
