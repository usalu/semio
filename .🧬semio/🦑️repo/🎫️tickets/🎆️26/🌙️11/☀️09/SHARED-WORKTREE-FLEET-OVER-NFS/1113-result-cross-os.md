# Result: cross-OS defects of the replica (2026-11-13, agent claude-cross-os)

Module `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Landed under the landing protocol (anchored edits `🧰️scripts/1113-cross-os-land/*.py`, entry in `1113-landings.md`). README (new section "The same view on every operating system" and the touched tables) and `1113-git-state-contract.md` (configuration, attestation, new section on names, bits and guards) are rewritten. Not done: the repo MCP was not used (it did not connect), so the ticket is not closed.

## Test counts (every test file of the module, against the live tree)

macOS 27 (APFS, bun 1.4.2): git 41, tiers 57, journal store 18, journal 127, state 8, relocate 11, bootstrap 12, chaos 9 (+1 skip), replica git 20, race 14, replica store 8, replica 85, adopt command 4, adopt 11, access 5, boundary 69, attest 73, topology 63, topology PowerShell suite 0 (20 skipped, no `pwsh` on this Mac). Linux arm64 container (ext4/overlay, bun 1.3.14, git 2.47.3, bash 5.2.37) on an identical copy: journal 127, boundary 67 (+2 root-only skips), git 41, attest 73, replica 85, race 14, replica git 20, chaos 9 (+1), bootstrap 12, topology 63, **PowerShell 7.7.0 suite 20 of 20**. Python oracles: boundary 281,538 vectors (every assigned code point against `str.casefold`), attest 48 verdict vectors incl. 10 guard cases. Bash 3.2.57 and 5.2 `bash -n` ok, shellcheck 0.10 at warning level: two benign notes (SC2120, SC2010).

## Per audit finding

| ID | Verdict | How / rule implemented / test |
|---|---|---|
| X01 | **partly confirmed** | Validated with libxml2 against Microsoft's published Task Scheduler XSD (`🧪️tests/🧫️fixtures/🪟️task-schema.xsd`, taken from learn.microsoft.com). Refuted: `Settings`, `Task`, `Exec`, `Repetition` are `xs:all` (any order) and the compact `CalendarTrigger` is in order. **Confirmed (new location):** the replica task's `TimeTrigger` had `Repetition, StartBoundary, Enabled`, the schema sequence is `Enabled, StartBoundary, EndBoundary, Repetition, ExecutionTimeLimit` then the trigger's own: that task would have been rejected at registration. Fixed; test `every scheduled task ... follows the element order` (own rule copy plus xmllint when present). |
| X02 | confirmed (behaviour of git for Windows unconfirmed) | `core.longpaths=true` is in the pins of every spawn and written to `.git/config` on Windows; `pathBudgetV1`/`absoluteUnitsV1` measure `worktree + \ + path` in UTF-16 (259 without, 32,766 with long paths; macOS 1,023 bytes, Linux 4,095); policy counts UTF-16 units (256), no longer code points; guard `overBudget` + preflight `path length` (fail on Windows without long paths); provisioning sets and reads back `LongPathsEnabled` (already did). Tests: boundary fixture vectors (astral paths), `the path budget ...`, guard tests. |
| X03 | confirmed (E1) | The attested view runs with `core.ignorecase=false`; test `the view ignores every machine-local input ... local ignorecase`. Guard `caseIgnore` + preflight (`caseDependentIgnoresV1`). |
| X04 | confirmed (E2) | No flag can make macOS print Linux's ` D Foo.txt`/`?? foo.txt` (lstat finds the file). The view adds ` D <index path>` for a tracked path whose exact spelling is absent on an insensitive volume, and `?? <path>` for NFD-only spellings; test `a case-only rename, a renamed directory and an NFD spelling give the lines that Linux git prints` has one expectation and passes on APFS and ext4. `git status -c core.ignorecase=false` with index and disk agreeing is clean (first assertion of that test). Guard `caseMismatch`. |
| X05 | confirmed | Rule in the README: POSIX disk bit is truth; `modeless` replicas (Windows, seam `ReplicaOptionsV1.modeless`) keep the manifest bit on change, take the staged `100755` for a new file, default false, never write; they publish their stage before their files; the view pins `core.filemode=false`, guard `modeMismatch`. Test (three real worktrees, one modeless): Windows-made file, `chmod +x` on POSIX, edited on Windows stays executable everywhere, a file staged with `--chmod=+x` is created executable, all `git diff`/view clean. |
| X06 | confirmed, **not fixed** | Index registers still skip the path policy (publish/apply of stage registers belongs to "git robustness"). Detected: guard `trackedRefused` (fleet verdict `diverged` on a difference, preflight fail). |
| X07 | confirmed, **not fixed** | `observe` territory. The effect is visible through `caseMismatch` (composed variants) and the view supplement. |
| X08 | confirmed | One fold (`foldNameV1`, generated table, union of APFS, NTFS, Linux) in scanner, `ReplicaFoldsV1` and the path book. Verified on this Mac's APFS: 26 pairs (incl. ß/ss, ẞ, final sigma, İ, ſ, K, ligatures, Cherokee, Georgian, µ, ŉ, NFC/NFD) are stored as one file exactly where the fold says one name, ext4 stores none as one; `ı`/`i` is merged though APFS keeps them apart (NTFS table, unconfirmed). **VS16 `U+FE0F` is significant** (two files on APFS; NTFS keeps code points). Generator `🚧️boundary/🔤️fold/📜️script.ts [--check]`; Python `casefold` agrees over all assigned code points. |
| X09 | confirmed (trace), **not fixed** | observe/scan winner rule is replica territory outside fold/bit. |
| X10 | not addressed | "resilience". |
| X11 | design fixed, unconfirmed | Mount task `semio-fleet-store-mount` generated by `worker-service --part mount-definition` (XML, validated), at logon and every minute while `S:` is missing, in the logon session of the replica; provisioning registers it from that XML. |
| X12 | fixed (unconfirmed on a German host) | Windows task state via `Get-ScheduledTask` enum; time sync via `sc.exe query` state number plus the `W32Time` registry type; macOS falls back to `defaults read ... TMAutoNetworkTimeEnabled` when `systemsetup` text is unrecognised. |
| X13 | fixed defensively | Windows mount check reads `mount.exe` (Client for NFS) first, then `net use`; the row parser no longer needs a status token. |
| X14 | **refuted** | The code's key `HKLM\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Users\Default\Cache` is the key in Microsoft's own article (fetched: `.../Users/Default/Cache`, values `RemoteWriteCache`, `FileAttributeCache`); the ticket review note was the wrong one. (The article also names `AttributeTimeDelta=10`; not set.) |
| X15 | confirmed | `GIT_PINS_V1`, environment pins, `gitPinsHashV1` attested; `core.excludesFile=` disables the XDG ignore (E4). `.git/info/exclude` cannot be pinned: guard `infoExclude` (preflight fail, verdict diverged), documented. git below 2.32 fails preflight. |
| X16 | confirmed, **partly** | Guards `links`, `trackedRefused`, preflight `symbolic links` (fail when the repo tracks links and the volume cannot create them); applying the link policy to index registers not done (see X06). |
| X17, X18, X19, X23 | not addressed | outside my territory / needs infrastructure. |
| X20 | not addressed | boot order noise; the replica retries. |
| X21, X22 | not addressed | Windows log capture and logon-only service remain documented limits. |
| X24 | fixed | git spawns have timeouts (10 s probe, 10 min run); git version attested (`guards.version`). |
| X25 | fixed | `core.protectNTFS=true`, `core.protectHFS=true` pinned and written. |
| X26 | fixed | `CONIN$`, `CONOUT$`, `CLOCK$` added to the reserved names (unconfirmed per Windows version). |
| X27 | open | see `.gitattributes` below. |
| X28, X29, X30 | fixed | user PATH written as `REG_EXPAND_SZ` via `reg.exe`; principal = `WindowsIdentity.Name`; 32-bit PowerShell refused. |
| X31 | n/a | |
| X32 | fixed in the touched passages | contract and README. |
| X33 | open | cosmetic. |

Bash 3.2/5.x and GNU/BSD: the audit's "no `set -u`" was wrong (`set -euo pipefail`); every array and variable use was re-read: no confirmed defect beyond the already fixed `patsub_replacement` one. Not run: real BSD `install -o` as root.

## Lists for the coordinator

1. **Tracked paths too long for Windows** (UTF-16 units with `C:\semio\`, limit 259; without long paths): 2, both below ticket `GET-CODEBASE-RUNNING-END-TO-END-AGAIN` (263 and 262 units); list in `🧰️scripts/1113-cross-os-over-budget-paths.txt`. Longest relative path 254 units (policy 256: none refused). Not renamed. `LongPathsEnabled=1` and `core.longpaths=true` make them work on Windows; on macOS/Linux there is no issue.
2. Case-collision groups under the new fold over the 272,351 tracked paths: 0. Paths refused by the policy (illegal, reserved, trailing dot or space, non-NFC): 0. Tracked links: 0. `.git/info/exclude`: absent.
3. `.gitattributes` (`* text=auto eol=lf`, `*.sh text eol=lf`): no rule is provably wrong for a no-checkout worktree. Proposal only: the replica copies bytes, so `eol=lf` never converts anything; a CRLF `.sh` saved on Windows breaks bash on Linux and macOS (X27). Suggested guard (not implemented): refuse CR bytes in `*.sh` at publish or report them in preflight.
4. Open items for their owners: X06/X16 (apply path and link policy to index registers), X07/X09/X17 (observe/scan/repair spelling), X10, X18-X23.

## Checked only against documentation (no Windows machine exists)

Task Scheduler XSD (libxml2 against the schema, not `Register-ScheduledTask`), the Client for NFS registry key (Microsoft article), PowerShell 5.1 versus 7 semantics of every cmdlet and parameter in `🔵️.ps1` (`Register-ScheduledTask -Xml -User -Force`, `Set-SmbClientConfiguration`, `Enable-WindowsOptionalFeature`, `Get-FileHash`, `[type]::new`: all present in 5.1; the script keeps its BOM), `mount.exe`/`net use`/`reg.exe`/`sc.exe` output formats, git for Windows behaviour with `core.longpaths`, NTFS `$UpCase` for `ı` and `ſ`, reserved device names on each Windows version, `LongPathsEnabled` reach for each tool, sleep/wake behaviour (`StartWhenAvailable` plus the repeating trigger). The PowerShell suite (20 tests, PowerShell 7.7.0 on Linux) runs the whole script against mocks.

## Notes

- A test of mine, run before the preflight probes were stubbed, created `/Users/Shared/semio/.git/semio-pin/` and touched `.cache/f/tmp`; I removed the directory. The pin directory is now created by every git spawn of the replica (private, under `.git`, not shared); the running daemon of this Mac recreated it after the landing, which is expected.
- `caseDependentIgnoresV1` costs about 10 s CPU on 272k paths; it yields every 2,000 judgements and its answer is kept for an hour while the root `.gitignore` is the same.
- The guards are measured in the background; until the first measurement the attestation says `guards: null` (exempt). The attestation schema gains required `guards` and `warnings` in the report, so daemons on the old code are read as invalid by new readers until they restart.
- State persistence: `ReplicaFoldsV1` hashes now come from the new fold; a checkpoint written by older code holds the old fold hashes (restored folds would not find case variants until rebuilt). If the warm-restart checkpoint stores `folded`, bump its code digest check accordingly.
