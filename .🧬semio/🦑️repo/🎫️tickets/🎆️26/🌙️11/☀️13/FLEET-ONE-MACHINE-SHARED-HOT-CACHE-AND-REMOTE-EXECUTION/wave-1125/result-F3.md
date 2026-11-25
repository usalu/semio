# Result F3: m04 (Linux) dependencies stage, patch EACCES and disk

## Root cause (from code and the m04 evidence, not reproduced on m04: no access)
- Error: `failed applying patch file: EACCES: Permission denied (open())` for `patches/1.patch` (the nx patch), attempts 14..22 identical.
- CAS adoption `Adoption.places[bun-cache]` was mode `hardlink` with `clearWriteBits: true`: every finished file in the Bun cache over 64 KiB and older than 10 min became a hard link of a 0444 Store object. The nx patch touches `client.js` (67 KB) and `task-orchestrator.js` (77 KB), both above the threshold. Bun (backend `copyfile` on Linux) copies the cached package keeping the mode and patches the copy in place with a read-write open, which a 0444 copy refuses. The patch therefore fails deterministically on every attempt; stale `_patch_hash=` results of failed attempts add noise.
- Not the privileged unit, not store permissions, not the replicated `patches/` files (those are 0600, owner-written).

## Disk
- `disk-short`/`disk-unfit` is stale: m04 now has 129,171 of 475,758 MB free; the inode cap (25 %) and unfit/recheck logic from E56/W8 landed. The only live blocker is the EACCES.

## Fix (landed, locks `bootstrap` + `cas`, released)
1. `🏗️build/🗄️cas/🧬️schema/🔣️.json`: `bun-cache` place `hardlink` -> `reflink` (copy-on-write clones, own inode, original mode; places without reflink are skipped).
2. `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`: `unshareJavascriptDependencyPatchedCacheV1(cache, files)` runs before every install (after the scope preseed). For each `patchedDependencies` key of the staged manifests it finds `<name>@<version>@@*` cache entries (scoped aware), replaces every file with nlink>1 or no owner write bit by a private writable copy (shared object untouched, Windows readonly attribute handled), removes `_patch_hash=` leftovers. This repairs the already-adopted m04 cache without any manual step.

## Proof
- `💾️storage/🧪️tests/🔓️patched-cache/🟦️.ts`: hardlinked 0444 file in nx and scoped entries become nlink 1 and writable, object stays 0444 with 2 links, unrelated entry untouched, stale patch dir removed, idempotent, python3 `os.access` cross-check. 1 pass.
- `🗄️cas/🧪️tests/🟦️.ts`: new test (policy is reflink; adopting leaves files nlink 1 and writable). 68 pass.
- NOT verified: a real bun patch on Linux (no Linux host here); m04's next attempt.

## USER
None expected. Fallback only if the next attempt still shows EACCES (then something else owns the files): on m04 run `chmod -R u+rwX /home/deck/semio/.cache/b/d/m/bun-cache` and `ls -ld /home/deck/semio/.cache/b/d/m /home/deck/semio/.cache/b/d/m/bun-cache` (owner must be `deck`).

## Second blocker found (NOT fixed): inode gate deadlock after the failed installs
- Current `ready.json` (attempt 22): refusal `disk-short`, resource INODES: stage `dependencies` needs 300,000 free inodes in `/home/deck/semio/.cache`, 120,832 free (251,913 free at E56 time). Space is fine (129 GB). The logs `ready-log-dependencies*.txt` are from 12:53 and stale.
- Reading: attempts 14..21 each ran bun install up to the failing patch step, leaving a resumable never-ready generation (copyfile backend, 100k-600k entries each) holding the inodes. The gate then demands a FULL fresh 300,000 although the resumable generation already owns most of what the retry needs, and the 30-min inode sweep does not retire unready generations (doctor also warns "6 unready dependency generations").
- Not changed here (needs data from m04 to size correctly): make `readyDiskNeedV1` subtract the entries an unready resumable generation already holds, or let the inode sweep retire never-ready generations of a stage that is not running. Lane owner for `🚦️ready` should take it; evidence needed from m04 is only `df -i /home/deck/semio/.cache` and `ls /home/deck/semio/.cache/b/d/m/j`.
- Consequence: with the EACCES fixed the stage may still wait on inodes until that is done or the sweep frees a generation.
