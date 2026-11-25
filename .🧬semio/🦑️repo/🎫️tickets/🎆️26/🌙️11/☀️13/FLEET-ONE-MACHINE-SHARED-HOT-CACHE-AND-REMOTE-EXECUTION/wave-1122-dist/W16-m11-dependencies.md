# W16: g1-m11 Ready stage "dependencies" failing every ~92 s (2026-11-25 08:54 to 09:11)

NB = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/`. Evidence read-only: `/Users/Shared/semio-store/v1/origins/g1-m11/{ready.json,ready-timings.json,ready-log-dependencies*.txt,service-failure.txt,ready-log-workspace-containment.txt}`, `repo fleet status`, the coordinator's own receipts `.cache/b/*/image.json`. No sub-agent, nothing restarted, nothing run on a worker, no scratch build. Lock `lock.native-image-mount` 09:03 to 09:07 (released). Landing record: `T/landings.md` "W16 m11 dependencies".

## Verdict
- **Error** (every attempt since 01:52, 3 s runs, `ready-log-dependencies-failed.txt`): `Internal error: Original underlying mount directory changed (reference 976fcdee)`, right after `boot-recovery:observe:0/0`, while opening the dependencies image `.cache/b/d`. The disk facts of the same log list only `l/m` and `o/m` as attached (18:08 on 11-24 it also had `d`, `g`, `r`, `t`); `fleet status` therefore says "partition dependencies is not attached, so the area remote-execution-worker has no home yet".
- **Source**: `NB/🌐️platform/📋️contract/📂️mount/🟦️.ts` `requireNativeImageMountDirectoryV1` compared the underlying mount directory's `dev`/`ino`/`mode` with the receipt's `mountDirectory` exactly (`mode` 16877 = 0o40755 in every receipt, e.g. coordinator `.cache/b/d/image.json`).
- **Root cause**: `NB/🍎️image/♻️reclaim/🟦️.ts` `guardDetachedNativeImageMountV1` (landed with the 11-24 auto-commit 755) runs after every canonical detach of the reclaim/refresh/grow port (callers: `📥️opening/♻️reset`, `📥️opening/🔄️refresh`, doctor `storage-grow`) and `chmod`s the plain mount point to 0555. It is the same directory (same dev/ino), but its mode is now 0o40555 = 16749, so every later opener (persistent open, inspection, retirement, attach preflight `requireEmptyNativeImageMountDirectoryV1`) refuses it forever. While the image stays attached nobody looks at the underlying directory, so it hides until the image is detached at the next open: on m11 the 01:52 attempt (24 s, 615 MB, the only longer one) is the one where `d` was detached by such a cycle; after that every run failed in 1 to 5 s (25 failures in 12 h, `ready-timings.json`). Not m11-specific: any macOS machine whose image went through a canonical detach (refresh, reset, grow) and is then detached (reboot, retirement) hits it; at 09:00 only m11 showed it in `origins/*/ready*.json`.

## Fix (landed 09:06, compile-atomic)
- Schema first: `NB/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json` new `DetachedGuardProtocol` (`withdrawnModeBits` 146 = 0o222, identity dev, ino, type, permissions-or-guarded) and `ModeCase`.
- `mount/🟦️.ts`: `nativeImageGuardedMountModeV1(mode)` (held mode without write bits) and pure `nativeImageMountModeAdmittedV1(held, actual)` = actual equals held or its guarded form; `requireNativeImageMountDirectoryV1` uses it. A wider mode (0777), other narrowing (0700), other type or a replaced inode still refuse. No receipt is rewritten, no migration: existing 16877 receipts with 0555 directories reopen as they are.
- `♻️reclaim/🟦️.ts`: the guard derives its mode from the contract (`nativeImageGuardedMountModeV1(row.mode)&0o7777`) instead of a fixed 0o555, so guard and identity law have one source (0555 for the usual 0755, 0500 for a private 0700 child instead of widening it).
- Tests: `mount/🧪️tests/🔣️.json` 9 mode cases (Ajv-validated, SQLite oracle `actual=held OR actual=(held&~146)`), a real-filesystem test (guard after mkdir reopens incl. empty preflight; 0777 refuses; rename + new dir + guard refuses), feature scenario in `🥒️.feature`.
- Side fix (stale test debt in the same member, not caused here): `mount/🧪️tests/📥️inspection/🟦️.ts` extracts `inspectPersistentReceiver` from `🧹️cleanup/🎛️command` source and lacked the helpers that function now uses (`admitNativeImageRelocationV1`, `firstEntry`, `recoverNativeImageStrandedV1`): 0/2 before, 1/0 after.

## Tests (`repo test-file`, one at a time, load 14 to 22)
| file | result |
|---|---|
| `NB/🌐️platform/📋️contract/📂️mount/🧪️tests/🟦️.ts` | 4 pass / 0 fail (2 new) |
| `.../📂️mount/🧪️tests/♻️retirement/🟦️.ts` | 2 / 0 |
| `.../📂️mount/🧪️tests/📥️inspection/🟦️.ts` | 1 / 0 (was 0 / 2) |
| `NB/🍎️image/♻️reclaim/🧪️tests/🟦️.ts` | 16 / 0 (native-image-availability) + 42 / 0 (native-storage) |
| `.../♻️reclaim/🧪️tests/⏱️bounded/🟦️.ts`, `🛑️sigterm/🟦️.ts` | pass, pass |

## Fleet confirmation (nothing restarted; the supervisor's own back-off retry)
- 09:08:02 attempt 26 (first with the replicated code): past the mount check into `bun:dependency-storage` (`dependencies:generation-verify`), **dependencies ok in 111 s, exit 0, peak 2,381 MB**; disk facts now list `d/m` (16,667 of 36,863 MB free), `r/m`, `t/m` attached again.
- 09:10 `ready.json`: `state ready, ready true`; `repo fleet status`: "g1-m11 macos: ready", "worker idle" (was "worker absent: ... partition dependencies is not attached"), in 2 / refused 1.

## Open items
- `fleet status` shows m11 "CODE 36d62b28 differs from the tree's f344e9eb" (the tree moved on with this and other landings; its worker picks it up on its own) and swap 6.9 GB (42 %).
- Files: `W16-sha-before.txt`, `W16-sha-after.txt` (this folder).
