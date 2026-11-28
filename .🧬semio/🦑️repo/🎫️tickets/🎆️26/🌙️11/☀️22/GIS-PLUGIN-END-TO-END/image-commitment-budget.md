# Host-wide image commitment budget and the 32 GiB layout invariant

## Problem
1. Declared sparse capacities summed to about 330 GiB on a 228 GiB host; only the per-root commitment was budgeted, so lanes, homes and builds could together ask the host for more blocks than it had (28 -> 14 GiB free within minutes).
2. `.cache/b/r/fixed.dmg` (96 GiB) and `.cache/b/d/fixed.dmg` (36 GiB) were born before the unmap cap existed. macOS unmaps only containers up to 2^35 bytes, so their slack could never return.

## Design
### Commitment budget (schema-first, pure, language-agnostic)
`🍎️image/🧮️commitment` (schema `Protocol`: reserve floor 16 GiB, reserve 8 %, overcommit 300 %, align 1 GiB).
- `reserve = max(16 GiB, 8 % of the volume)`.
- `ceiling = align_down(max(0, min(capacity - reserve, allocated + max(0, free - reserve) * 3)))`: the declarations of all home images and all shards together never exceed what the volume could give; when the host is at or under its reserve the ceiling equals what is written, so nothing new is promised.
- `admit(ceiling, committed, request) -> {admitted, roomBytes, shortfallBytes}`.
- `measureNativeImageCommitmentV1(.cache/b)` reads the receipts (`maximumPhysicalBytes`) of every home image and the ledger of every live shard of every partition (declared and `stat` blocks held).
Wiring (lane images, rust partition):
- `LaneImagePortsV1.hostCommitment` (real ports measure `.cache/b`); `laneImageHostRoomBytesV1` = ceiling - committed.
- `nativeImageShardCapacityV1`, `nativeImageShardSelectV1`, `nativeImageShardBirthEventV1`, `ensureNativeImageShardV1` take `hostRoomBytes` (births cut to it, refused as `commitment` when it cannot hold the want, birth event validated inside the lock); `growNativeImageShardV1` bounds the growth by min(root room, host room).
- Reclaim before refusing: `admitNewLaneV1` adds the host-wide shortage (`committed + want - ceiling`) to the commitment shortage, so the least recently used idle lanes are retired whole (never claimed, live, held or product lanes) until the request fits; an uncoverable shortage retires nothing and the birth is refused (lane stays home).

### Layout invariant (every container at or below 32 GiB)
- Policy already caps every birth (`imageCapBytesV1`, elastic maxima 32 GiB, fixed `dependencies` 32 GiB, shard `needBytes` births <= 32 GiB, the rust partition `capacityBytes` is the budget of a set of images). The remaining violators were the two old containers.
- `repositoryPartitionOversizeV1(workspace, partition)` (any partition) and `repositoryPartitionRegrowDueV1` now make an oversize receipt due on its own (bloat is no condition any more); `dependencies` joins the regrow/reset profiles (`RepositoryCacheResetProfileV1`, schema `Profile`).
- `repo cache gc` (whole gc and `--idle`) retires an oversize image through the normal path: `dismantleRepositoryPartitionShardsIfQuiescentV1` (sharded partitions: exclusive nonwaiting whole-partition lease, each shard under its own exclusive nonwaiting lease, busy mounts never forced, shard directory and ledger removed once all shards are gone, see `dismantleNativeImageShardsV1`) then `regrowElasticImageOutcomeV1` (exclusive nonwaiting lease, journal admission, whole retirement). `--dry-run` reports it. The next opener births the image within the cap. Leased or in-use images answer `busy`/`held` and are left alone; the pass repeats at every gc.
- Bugs found on the way and fixed (both block the normal retirement of any detached image that an earlier reclaim cycle guarded):
  - `nativeRecomputableRetirementDetachedV1` (whole retirement) rejected the underlying mount directory after the detached guard had withdrawn its write bits (`0555` against the published `0755`): it now admits the held mode or its guarded form (`nativeImageMountModeAdmittedV1`).
  - `quarantineStrandedNativeRootEntriesV1` failed with EACCES on a guarded (read-only) unmounted root: it makes the root writable for the moves and restores its mode.

## Files
Framework (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/`):
- `🍎️image/🧮️commitment/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🐍️.py`, `🧪️tests/🔣️.json` (new)
- `🍎️image/🧩️shard/🟦️.ts` (hostRoomBytes, `commitment` refusal, `dismantleNativeImageShardsV1`), `🧩️shard/🧪️tests/🟦️.ts`
- `🧹️whole/🟦️.ts`, `🧹️whole/🧪️tests/🟦️.ts`; `🧹️cleanup/🎛️command/🐚️shell/🟦️.ts`, `🧪️tests/🟦️.ts`
- `📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧮️commitment/🟦️.ts` (new public entry), `🧩️shard/🟦️.ts` (re-export)
Repo product (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`):
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts`, `🧩️image/🟦️.ts`, `🧪️tests/🚦️admission/🟦️.ts`
- `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts`, `📥️opening/♻️reset/🟦️.ts`, `📥️opening/♻️reset/🧬️schema/🔣️.json`, `🧪️tests/🧩️shards/🟦️.ts`
- `🏗️build/🧹️cache-gc/⌨️command/🟦️.ts`

## Tests (repo test-file)
- commitment 8 pass (corpus vs Python oracle: reserve, ceiling, admission, layout fit; hand-checked rows; real receipts and ledger measurement).
- shard 27 pass (host room bounds birth and growth over all roots; dismantle with a busy shard; existing oracle tests unchanged).
- pool admission 8 pass (host-wide ceiling retires the least recently used idle lane before refusing, nothing when uncoverable); lane image 9, pool-model 6, budget 25, regrow 2, cache-gc 22, whole 8, receiver shell 5, budget shards 7 (layout invariant: every elastic maximum, every unsharded home, every shard root birth and the dependencies home <= 32 GiB; 96 GiB and 36 GiB receipts are oversize).
- `repo typecheck` (touched files): only the existing admission test line 27 error. `image-slack` suite has one failure unrelated to this change (writer registry priorities).

## Live result
- Disk free (`df -k /`, 239362496 KiB volume): before the first `repo cache gc` 41 592 216 KiB (39.7 GiB, 25 % used); after the `dependencies` retirement 63 534 592 KiB (60.6 GiB, 18 % used). The 36 GiB `.cache/b/d/fixed.dmg` (42.9 GiB allocated, `m-stranded-*` quarantine of 11 MB removed because it blocked retirement) was retired whole (idle, journal admitted); the next opener (`repo dependencies ensure`) born it at 29 176 668 160 bytes (27.2 GiB, `maximumPhysicalBytes` within the cap), 0.13 GiB allocated at that point.
- `.cache/b/r/fixed.dmg` (96 GiB declared, about 50 GiB allocated, 24.5 GiB reclaimable by the existing in-place kick only partly) answered `busy` on every gc (other lanes build in its shards and hold its lease): nothing was detached or deleted. It retires at the first gc in a quiet window (the whole-partition lease is only taken nonwaiting); not exercised live, covered by the dismantle test with a fake lifecycle and by the real `dependencies` retirement through the same `regrowElasticImageOutcomeV1`.
- Side effect to know: `node_modules` links into the dependencies image, so it is empty until `repo dependencies ensure` finishes (it was queued behind the host memory capacity gate, 16 GB host); fleet-routed `repo test-file` runs are unaffected.
- First gc runs also hit the two retirement bugs above (`Native directory inventory exceeds its owned bound` for a sharded capsule; EACCES on the guarded mount; `Actual original detached Native mount directory required`), all three are now handled (shard bookkeeping removed by the dismantle step, quarantine and detached check tolerate the guard).

## Dependency restore as a prerequisite (host-capacity gate)
- Cause of the wait: the `dependencies` operation was a `batch` fixed request, so it queued behind `graph-plan` compiles and hit the per-session share ("this agent session already holds its share") while memory above the floor was short.
- Fix at the source (schema-first): operation and request schema gained `prerequisite` (`🏗️build/🌡️host-capacity/🧬️schema/🔣️.json`); `sizeHostRequestV1` copies it onto the request; the decision treats a prerequisite like an essential worker (admitted with its minimum whatever memory, queue order and session share say; only a hard stop for critical pressure or active swap holds it) (`📏️decision/🟦️.ts`); the product policy marks `dependencies` as `prerequisite: true`, priority `interactive` (`🏗️build/🔣️policy.json`).
- Tests (`🧪️tests/🥇️essential/🟦️.ts`, 9 pass): an ordinary fixed request waits while a prerequisite is admitted with short memory, every hard-stop row still holds it, and the sizing carries the flag only for `dependencies`.
- Rerun without timeout: `[capacity] admitted dependencies after 0s`, install completed (exit 0), `.cache/b/d/fixed.dmg` is 29,176,668,160 bytes (27.2 GiB, within the cap), 20.3 GiB allocated.
- Confirmed: `ls node_modules/@semio-tech | head -3` -> `2d-compute`, `assets`, `build-parsers`; `node_modules/typescript/lib/tsc.js` exists. Disk free now 54,205,256 KiB (51.7 GiB).
- Not retired `d` again; the other tests in `🌡️host-capacity/🧪️tests/🟦️.ts` need `ajv` and were not rerun here.
