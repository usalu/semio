# Windows provisioning and CI: what ran for real, what was fixed, what only a Windows machine can confirm

Host: macOS arm64, no Windows. Tools downloaded into the session scratch directory only, each checked against the published checksum:
PowerShell 7.6.9 (tar.gz, sha256 from `hashes.sha256`), actionlint 1.7.12 (sha256 from the release checksums), ShellCheck 0.11.0 (release digest),
PSScriptAnalyzer 1.25.0 (nupkg, SHA512 from the PowerShell Gallery feed), `xmllint` (macOS system) with the Task Scheduler schema taken from the
Microsoft documentation page (not committed; set `SEMIO_TASK_SCHEMA` to a saved copy to make the test use it). Two npm test libraries (`yaml`, `yargs-parser`) went into the scratch `node_modules` only.

## Executed with real tools

| What | Tool | Result |
| --- | --- | --- |
| `provision.ps1` and 31 generated snippets (mount, credential, task line, `prepare`, `provision`, `start`, `feature`, `share`, `firewall`; NFS and SMB; groups a1, b2, c3) | `[Parser]::ParseFile/ParseInput`, PowerShell 7.6.9 | 32 inputs, 0 errors (also a test) |
| 20 Task Scheduler XML documents (replica, compact worker and server, cache, relay; NFS and SMB) | well-formedness + published XSD with `xmllint` | 3 classes of violation found, fixed, 20 of 20 validate |
| `provision.ps1` | PSScriptAnalyzer 1.25.0 | 4 warnings: 1 fixed (`Update-ProcessPath` renamed, ShouldProcess), 1 removed (`Invoke-Expression` of the facts replaced by a strict regex parse); left on purpose: `Invoke-Expression` of the generated server parts (the plan text is the only source), `Write-Host` (operator log) |
| `provision.ps1` end to end under pwsh with a mock layer | 13 new tests (`🗺️topology/🧪️tests/🔵️.ts` + `🔵️.ps1`, skipped when `pwsh` is missing) | all pass, about 17 s |
| Boundary rules on a simulated `win32` | 6 new tests | all pass |
| 4 workflows | actionlint with ShellCheck and the PowerShell parser on the 7 `pwsh` blocks and 91 bash blocks | 0 findings after the fixes (5 ShellCheck infos before) |
| range logic of `build-system.yml` | extracted `plan` and `Use the whole workspace` steps run under `bash -eo pipefail` | 10 plan cases, 8 clone cases, see below |

The mock layer defines every Windows-only cmdlet and executable as a function with the real parameter names and `ValidateSet` values
(`New-NfsShare`, `Grant-NfsSharePermission`, `New-SmbShare`, `New-NetFirewallRule`, `Register-ScheduledTask`, `Install-WindowsFeature`,
`Enable-WindowsOptionalFeature`, `Get-LocalUser`, `Resolve-DnsName`, registry, `icacls`, `net.exe`, `mount.exe`, `powershell.exe`, `repo`, ...), so a
misspelled parameter fails the run. `C:` is a PowerShell drive onto a temp directory, `S:` appears when `mount.exe` or `net.exe use` succeeds, `tar.exe` is the host bsdtar
(real extraction of a real pax tar with the emoji paths, real sha256 and entry-count check against `manifest.json`), the machine environment is a C# stand-in, the elevation check is a stand-in; the `repo`
facts, env, worker-service and server-plan output is the real topology command output. Driven branches: worker first machine, joining worker (archive extract,
bootstrap, settle, join), repeated run of the same identity (also with `-FirstMachine`), different origin, different group, empty identity, SMB worker (and `net use` failure text), Windows NFS
server, Windows SMB server (and Windows 10/11 client OS, and a missing local account), `-Server` on a Linux or macOS group, missing archive path for a server, missing and placeholder tokens file, Windows 10/11 without `Install-WindowsFeature`,
not elevated, wrong `-User`, unknown group, stray files in the worktree, `-FirstMachine` without checkout or with origins in the store, archive hash mismatch (5 attempts), entry-count mismatch,
`icacls`, `mount.exe` and `seed` failures. Asserted: the exact call sequence of the first-machine run, the order and arguments of the NFS and SMB server runs, the stored task XML equals the plan text, and that nothing changed before a refusal.

## Findings fixed

Windows and PowerShell
1. Task XML violated the published schema in three ways (`*topology/🟦️.ts`, `taskOf` and the compact trigger): `version="1.4"` (the schema fixes 1.3), `RestartOnFailure/Count` 999 (the type is `unsignedByte`, maximum 255; now 255), and `Enabled` after `StartBoundary` in `CalendarTrigger` (the base type puts `Enabled` first).
2. The Windows mount plan printed `mount -o ...`; in PowerShell `mount` is an alias of `New-PSDrive`. Now `mount.exe -o ...` (code and test).
3. A Windows group server could not read NFS peers: its `feature` part installed Server for NFS only, but `relay`, the peer archive and `-ArchivePath` read `\\host\export` by UNC, which needs Client for NFS. The `feature` part now adds `NFS-Client` (Windows Server) or the two optional features (Windows 10/11) when any peer is served over NFS; README line updated.
4. Start order: the replica task was started before anything mounted `S:` in the user's own (non-elevated) session; a mapped drive of the elevated provisioning shell is invisible there. The script now starts `semio-fleet-store-mount` first on workers.
5. `Invoke-Expression` on the topology facts replaced by a strict per-line regex (`$FLEET_NAME='value'`) with `Set-Variable`; any other line is refused.
6. Boundary (`*boundary/🟦️.ts`): a Windows `readlink` target with backslashes was stored verbatim, so a link written as `a/b` was rescanned as `a\b` (change loop, wrong link on Linux): targets are now reported with `/` on win32. Device names `COM¹ COM² COM³ LPT¹ LPT² LPT³` are now reserved (fixture rows added, Python oracle changed, 71 vectors agree). The case-collision fold was `toLowerCase`; it is now `foldNameV1` (`toUpperCase().toLowerCase()`), so `ας`/`ασ` and `ß`/`SS` collide as they do on NTFS and APFS.

CI
7. `ci-report` fixture and test, and the dashboard CI test, still described the old `pull_request` design (range `--head=HEAD`, group `build-system-push-...`, boolean `cancel-in-progress`): scenarios replaced by dispatch scenarios, push range now `--base=<before> --head=<sha>`, 18 of 18 and 6 of 6 green.
8. `git diff --name-only` in `affected` used rename detection: it lists only the new name (the project that lost the file is not selected) and, in the `blob:none` clone with `persist-credentials: false`, it tries to fetch blobs from the promisor remote and fails. Reproduced with a local blobless clone whose remote was removed; `--no-renames` fixes both (`🧭️selection/🟦️.ts`, 5 fixture command lists, 6 of 6 green).
9. `workflow_dispatch` input `base` was written unquoted into `$GITHUB_OUTPUT` (a line break injected a second output, a space an extra flag). Now refused unless it is a plain revision name.
10. ShellCheck: unquoted `$AFFECTED_RANGE` in 4 commands (arrays now) and one SC2129 grouping.
Range cases run: normal push, first push (all-zero `before`), empty `before`, dispatch with no base (`origin/main`), scope all, tag, sha, plus the two injections (refused). Clone cases: valid sha, missing sha (warning, `--all`), `origin/main`, tag, `--all`, missing ref, `HEAD~1`, clone without `origin/main` (warning, `--all`).

## Reviewed, not changed (other owners or no gap found)

| Windows branch | Handled | Tested |
| --- | --- | --- |
| path separators, `\` and `:` in names, trailing dot or space, `CON`/`aux.txt`, superscript device names | yes (`refusePathV1`) | yes, fixture + oracle |
| drive-letter and UNC links | refused (`linkRefusalV1`) | yes, also scanned on simulated win32 |
| executable bit absent | `executable:false` on win32 | yes (simulated) |
| two paths differing only in case | refused, first in sorted order wins, Unicode fold fixed | yes (fold unit test; scan test only where the file system is case sensitive) |
| `S:\`, `C:\store`, `\\host\share` store roots | all paths built with `join`/`resolve`, `--store S:\` and `--peers a=\\h\s` reach the replica unchanged in the mock run | the host cannot simulate `path.win32` inside the modules: not unit-tested |
| rename over existing, EBUSY/EPERM/EACCES | journal `renameWithRetry` (5 attempts, about 0.8 s, then throws); replica `sharing()`/`retrying` (6 attempts, then a `blocked` event and later retry) | **no test** in `*replica` or `*journal` (the replica reads `process.platform` at load and imports `node:fs/promises` directly); owner of `*replica` should add an injected-error test |
| symlink creation without privilege | replica drops the entry with a `skipped` event on EPERM/EACCES/ENOSYS/UNKNOWN | **no test** (`*replica`) |
| a journal rename that stays blocked longer than the retries | throws to the caller | not examined further; the daemon's reaction is unverified |

## Edits outside the files I own

`*topology/🥾️provision/🔵️.ps1`, `*topology/🟦️.ts` (Windows parts), `*boundary/**` and the new topology tests are mine. Re-read before editing and minimal: module `README.md` (two sentences), `🏗️build/🧭️selection/🟦️.ts` (one token) and its fixture, `🏗️build/🧾️ci-report/🧪️tests/🟦️.ts` and fixture, `🎛️dashboard/🧪️tests/🏗️ci/🟦️.ts`, `.github/workflows/build-system.yml`, `*topology/🧪️tests/🟦️.ts` (3 assertions). Nothing under `*replica/**` and no shell script was touched. I ran two broad `pkill` commands (`bun test`, `pwsh`) while a runaway test run of my own hung; they may have ended a test run of another agent.

## Not fixed, observation only

- `acceptance.yml` is schedule plus dispatch, `dashboard.yml` is push plus dispatch without any range; only `build-system.yml` has the `before..sha` logic.
- `concurrency` with `cancel-in-progress: false` still drops *pending* runs when newer pushes arrive; the range of a dropped push is never selected by `affected` (the next push compares only its own `before..sha`). Needs a "last tested sha" if every push must be covered.
- The bench acceptance test "child sees the peer edit while it runs" failed once under heavy machine load (other agents' daemons) and passed twice afterwards: timing sensitive.
- Windows server tasks use `InteractiveToken`: after a reboot nothing runs until the account logs on (README says: automatic logon).

## Checklist for the first Windows machine (elevated PowerShell unless noted)

1. Windows PowerShell 5.1 parse: `$e=$null;$t=$null;[void][Management.Automation.Language.Parser]::ParseFile('<provision ps1>',[ref]$t,[ref]$e);$e.Count` -> `0` (BOM decoding of the emoji literals).
2. Run `provision.ps1 <group> <origin> -FirstMachine` on a Windows 10/11 worker with a checkout: `(Get-ItemProperty HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem).LongPathsEnabled` -> `1`; in a new shell `Test-Path` of a 300-character path under `C:\semio` -> `True`.
3. `Get-WindowsOptionalFeature -Online -FeatureName ServicesForNFS-ClientOnly,ClientForNFS-Infrastructure | Select FeatureName,State` -> both `Enabled` (names exist, no reboot needed).
4. `mount.exe -o anon,nolock,mtype=hard \\<nfs host>\<export> S:` against a Linux server and against a macOS server -> `S:\v1` lists, `mount.exe` with no arguments shows `mtype=hard` and NFS v3. Client cache values present after provisioning: `Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Users\Default\Cache' | Select RemoteWriteCache,FileAttributeCache` -> both `0` (DWORD, documented by Microsoft for older Windows; the script restarted the Client for NFS service before mounting).
5. Mapped drive visibility: in a normal (non-elevated) shell right after provisioning `Test-Path S:\` -> `True` after the script started `semio-fleet-store-mount`; after log off and on -> `True` within 1 minute.
6. `Get-ScheduledTask semio-fleet-* | Select TaskName,State` -> replica `Running`; `schtasks /Query /TN semio-fleet-replica /XML` shows version 1.3, `Count` 255, `InteractiveToken` (no password prompt for `-User`). Follower freshness: with the group's store mounted, make a change on another machine (Linux or macOS origin) and time on this Windows follower how long until its replica reports the new head: it must be within 1 s; if it is slower, the two cache values did not take effect on this Windows version and the finding goes into the README table.
7. Re-run the same command -> `already joined`, no `join`, tasks registered again without error; run with another origin -> refusal naming both origins, nothing changed.
8. Bootstrap path (joining worker, empty `C:\semio`): `tar.exe` entries equal `manifest.json` `files`, the checkout appears, a path above 260 characters is readable, `repo --version` works in a new shell.
9. Environment: `[Environment]::GetEnvironmentVariable('SEMIO_REMOTE_CACHE_ENDPOINT','Machine')` equals the topology value, and the replica task sees it (`Get-Content` of its log or `repo` in the task).
10. Server NFS (Windows Server): `Get-NfsShare store-c`, `Get-NfsSharePermission -Name store-c` lists the all-machines read-write entry plus one read-only host entry per peer; a write from a peer host must fail, a write from a worker must succeed (precedence of the host entry), anonymous uid/gid 1500 owns new files.
11. Same server: `Get-NetFirewallRule -DisplayName semio-fleet-* | Get-NetFirewallPortFilter` shows 2049,111,1048 (445 for SMB) and `rpcinfo -p <server>` from Linux shows the mountd and nfs ports that are really in use (1048 is documented, not probed).
12. Server reads NFS peers: `Get-ChildItem \\<peer nfs host>\<export>\v1\bootstrap` as the task account -> lists (needs the `NFS-Client` feature the `feature` part now installs; check the anonymous identity is allowed by the peer); `repo ... relay --once --peers a=\\...` -> exit 0.
13. Server SMB (Windows 10/11 or Server): `Get-SmbShareAccess store-c` shows `semio-fleet` Change and `semio-fleet-peer` Read; from a worker `cmdkey /add:<host> /user:<host short>\semio-fleet /pass:...` then `net use S: \\<host>\store-c /persistent:yes` -> `S:` lists; `Set-SmbServerConfiguration -EnableSMB2Protocol $true -Force` raises no error.
14. Server after reboot with automatic logon: cache, relay and replica tasks `Running` within 2 minutes; without logon they stay `Ready` (documented limit).
15. Real NTFS laws of the replica: hold a shared file open with `[IO.File]::Open($p,'Open','Read','None')` while a peer edits it -> `blocked` event, applied after release, no crash; create a symlink as a normal user -> `skipped ... symlink-unsupported`; with Developer Mode on -> applied; two names differing only in case -> second is `case-collision`.
16. GitHub only: first push to `main` after the change shows `affected range: --base=<before> --head=<sha>` in the `plan` summary; a force-push shows the `::warning::base revision ... is not in the clone` and `--all`; `windows-latest` accepts `read -ra range <<< ...` in the default bash; a dispatch with `scope: affected` and the default base resolves `origin/main` in the full clone.
