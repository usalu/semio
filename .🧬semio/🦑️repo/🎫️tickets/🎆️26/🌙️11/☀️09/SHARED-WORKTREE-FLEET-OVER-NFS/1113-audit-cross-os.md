# 1113 cross-OS audit of the shared-worktree module

- Date: 2026-11-13
- Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree` as replicated at `/Users/Shared/semio` (read live; other agents edit it concurrently, so line numbers may drift a few lines).
- Mode: read-only. No source file edited, no git command that changes anything, no process stopped, no sub-agents. Scratch experiments ran in temporary repositories under the session scratchpad and were deleted.
- Severity legend: **P0** blocks a whole OS or silently corrupts shared state; **P1** divergence or outage on one OS or class, or a wrong state reachable from normal input; **P2** hardening, diagnostics, latent or documentation drift.
- Evidence legend: **verified** (run here, macOS APFS, or measured on the repo), **traced** (read in code, path followed step by step, not run), **unconfirmed** (depends on Windows, Linux or NFS behaviour that was not run here).

## 1. What was verified and how

Verified on this machine (macOS, Darwin 27, APFS case-insensitive):

- E1 `.gitignore` `Backup/` with default `core.ignorecase=true` hides `backup/x.txt`; with `-c core.ignorecase=false` it shows as `??`.
- E2 case-only rename `Foo.txt` to `foo.txt` on disk: default git shows nothing; `-c core.ignorecase=false` shows `?? foo.txt` only. On Linux the same state shows ` D Foo.txt` plus `?? foo.txt` (traced, the index still holds `Foo.txt`).
- E3 `chmod +x` on a tracked file: `core.filemode=true` shows ` M`; `core.filemode=false` shows nothing.
- E4 `$XDG_CONFIG_HOME/git/ignore` hides an untracked file by default; `-c core.excludesFile=` and `-c core.excludesFile=/dev/null` disable it.
- E5 a symlink shows as `??` on macOS.
- APFS names: an NFD name is stored and `readdir` returns it as NFD; `lstat` of the NFC spelling succeeds and finds the NFD file. Writing `ss.txt` after `ß.txt` leaves one file; writing `οδοσ.txt` after `ΟΔΟΣ.txt` leaves one file (APFS folds `ß` to `ss` and final sigma to sigma).
- Repo measured with read-only `git ls-files` (272,323 index entries): 0 symlinks (mode 120000), 0 gitlinks, 2,701 executables (100755). Longest path 237 code points, 254 UTF-16 units, 328 UTF-8 bytes. 0 non-NFC names, 0 Windows-illegal characters, 0 reserved device names, 0 trailing dot or space, 0 case-collision groups. **2 tracked paths exceed 260 UTF-16 units once the worktree prefix `C:\semio\` (9 units) is added.**
- Module files: no CR bytes anywhere in the module (LF only).

Read but not executed: every module source file listed in the brief plus `repo` launcher install (for the Windows executable name), the test files (they mock platforms and check substrings), and the ticket context (`1113-git-state-contract.md`, `1113-audit-os-readiness.md`, `windows-ci-report.md`, `provisioning-review.md`, `1113-worker-prompts.md`).

Not verified (marked **unconfirmed** below): Windows of any kind (NTFS, Task Scheduler, Client for NFS, `net use`, `schtasks`, `w32tm` output, git for Windows), Linux runtime, NFS server behaviour, launchd and systemd runtime, Bun or Node behaviour on the three OSes.

## 2. Direct answers to the brief's questions

- **Which unit does `refusePathV1` use?** Code points for the 256 limit (`Array.from(path).length`, boundary:142, :153). The `path.length>256 &&` conjunct is redundant (UTF-16 length is never below the code-point count), so the effective limit is 256 code points. Component limit is UTF-8 bytes of 255 (boundary:144, :154), which is stricter than NTFS (255 UTF-16 units) and APFS or ext4 (255 bytes). Windows MAX_PATH is 260 UTF-16 units including the drive and the NUL, so the policy does not match the Windows limit. No repo path is refused today, but two paths pass the policy and exceed MAX_PATH with the worktree prefix (X02).
- **Does `gitViewV1` pin its inputs?** No. It passes only `-c gc.auto=0 -c core.fsmonitor=false` (git:104) and `--no-optional-locks` with `GIT_OPTIONAL_LOCKS=0` (git:223). Everything else is inherited: `.git/config` (eight keys normalised, git:430), global and system config, `$XDG_CONFIG_HOME/git/ignore` (E4), `.git/info/exclude` (private, never replicated), `core.excludesFile` and attributes files, the git binary and its version. See X15.
- **Can a Windows worker publish mode 100644 for a file that is 100755 elsewhere?** Yes, two ways. A new Windows file is published with `executable=false` (replica:944 gives a new file `false` on Windows), and Linux and macOS workers then chmod it to 0644 (replica:1277). Separately, the index mode never follows the file flag at all (X05). A Windows `git add` of a new file gives 100644 because of `core.filemode=false`.
- **Symlinks on Windows without Developer Mode.** The file realm drops the link (replica:1255, `symlink-unsupported`, then dequeued), but the index realm still applies the link entry (git:332-340, no capability check). Windows `git status` then shows ` D <link>` forever and the view digest differs from Linux and macOS (X16). The repo tracks zero symlinks today, so this is latent.
- **Line endings.** `.gitattributes` sets `* text=auto eol=lf` and `*.sh text eol=lf`. The replica copies raw bytes and never applies those rules, so `eol` only takes effect in git checkouts. Git's `text=auto` hides CRLF in `git status`, so the view is unaffected today, but a CRLF `.sh` breaks bash on Linux and macOS (X27).
- **Timestamp granularity and the racy window.** The replica's racy window is 2000 ms (replica:92) and git's index signature uses inode, size and `mtimeNs` (git:181-184, new inode on every `rename`). Both are sound for NTFS, APFS, ext4 and NFS with nanosecond attributes. HFS+ (1 s) and FAT (2 s) are outside the supported set. Low risk; no finding.
- **Sharing violations.** A held file (Windows editor without share-delete) turns a rename or unlink into `EPERM`, which is retried for a few hundred milliseconds and then reported as `blocked` and queued with backoff up to 30 s (replica:1288, :1226, :1319-1320). Good behaviour, but untested on Windows (windows-ci-report:57).

## 3. Ranked findings

| ID | Sev | OS pair | Title | Location | Scenario (short) | Consequence (short) | Guard | Fix direction |
|---|---|---|---|---|---|---|---|---|
| X01 | P0 | Windows | Task XML child order may not match the Task Scheduler schema (unconfirmed) | topology/🟦️.ts:238, :244, :266 | `Settings` puts `RestartOnFailure` before `ExecutionTimeLimit`; compact `CalendarTrigger` puts `Enabled` before `StartBoundary` | `Register-ScheduledTask -Xml` may reject the replica, compact, cache and relay tasks | no (mock test only checks substrings, topology test:223-239) | Validate rendered XML against the Task XSD; emit children in exported order |
| X02 | P0 | Windows vs Linux, macOS | Worktree prefix pushes two tracked paths past MAX_PATH; `core.longpaths` never pinned | boundary:142, :153; topology `worktree.windows`; ps1:146-152; git:430 | 2 tracked paths are 263 UTF-16 units under `C:\semio\`; git for Windows defaults to `core.longpaths=false` | `git status` on Windows may skip or error those lines, so `view.digest` diverges (unconfirmed) | no (preflight checks only the registry value, command:395-398) | Pin `-c core.longpaths=true` on win32 in `git()` and in normalisation; budget paths in UTF-16 with the real prefix |
| X03 | P1 | macOS vs Linux, Windows | `core.ignorecase=true` applies `.gitignore` case-insensitively; replica ignore is case-sensitive | git:430; boundary:95 (regex flags `su`), :195 | `Backup/` hides `backup/x` in the Mac view (E1); replica shares it anyway | Mac view omits `?? backup/...`; other OS show it; `view.digest` diverges | no | Pin `core.ignorecase=false` in every git call; make replica ignore case-sensitive (already); lint case-variant patterns |
| X04 | P1 | macOS, Windows vs Linux | Case-only rename: case-insensitive git shows clean, Linux shows deletion plus untracked | replica:1041-1052, git:258-264 (index never follows disk) | E2: Mac clean; Linux ` D Foo.txt` and `?? foo.txt` (traced) | Views differ until someone runs `git add` on the Mac | no | Detect index-versus-disk case mismatch in publish and preflight; report it as a staging action |
| X05 | P1 | Windows vs Linux, macOS | Executable bit has two channels that never reconcile | replica:944, :1016, :1243, :1277; git:156-162, :280-318 | chmod on Linux replicates and flips the file; index keeps 100644: Linux and Mac show ` M`, Windows shows nothing (E3); new Windows file gets `executable=false` and Linux chmods 0644 while the index says 100755 | View digests diverge by OS; a script loses `+x` | no | Pin `core.filemode=false` in the attested view on all OS; add a mode check (file flag versus index mode) to verify and preflight |
| X06 | P1 | Windows, macOS (latent) | Index registers skip the path policy; refused names reach every stage | git:255 (publish side only), :258-264, :332-340, :156-162; replica refusal only in file realm (replica:1178-1187) | Linux commits `a:b`, `CON.txt`, `x.` or a case-variant pair; the index realm applies it everywhere; the file is refused on Windows | Windows `git status` shows `D` for entries that cannot exist there; APFS collapses case variants; one bad entry fails the whole `update-index` batch | no | Apply `refusePathV1` plus the case rule to index registers on publish and apply, with the strictest rule of all OS |
| X07 | P1 | macOS vs Linux | NFD names published under the NFC name by the watch path | replica:972, :1442, :956-991 | APFS finds the NFD file by NFC `lstat` (verified); observe publishes it; full scan refuses it `not-nfc` (boundary:152) | Event path and safety-net scan disagree; Linux receives a file the scanner rejects; verify drift | partial (`listed()` exact compare exists at replica:101, not in observe's file path) | Require the exact on-disk name (readdir compare as at replica:1052) before publishing; refuse non-NFC on-disk names before `lstat` |
| X08 | P1 | macOS (and Windows) vs Linux | Case-fold mismatch: replica folds with `toLowerCase`, scanner with upper-then-lower; APFS folds `ß`/`ss` and final sigma | replica:183, :401, :405, :417, :1184; boundary:28, :230 | Verified on APFS: `ß.txt` plus `ss.txt` is one file; replica folds do not see the pair | Mac materialise overwrites the other file; the lost copy is kept only when the local mtime loses | no | One fold function for scanner, replica and index; refuse collisions under the union of folds on every OS |
| X09 | P1 | all (order-dependent) | Case-variant pair handled differently by watch path and full scan | replica:982-986, :1213; boundary:225-231 (sorted first wins); replica:1041-1052 | `foo` published first; `Foo` created: observe skips `Foo`; the next full scan shares `Foo`, refuses `foo`, and `forget`s it without a delete | Manifest keeps both names; Linux verify reports missing-local; Mac queue keeps a case-collision item, verdict `lagging` (traced, unconfirmed run) | no | One winner rule in observe and scan; publish a delete for a previously shared name that becomes refused |
| X10 | P1 | Linux, macOS (store) | No timeouts on store I/O; a hung NFS hard mount blocks the daemon and its lock | replica:519-521, :553-566, :1425-1497; journal:235-251 | Server down under a `hard` mount: `stat` and `readFile` do not return; SIGTERM is not served; a restarted daemon sees a live pid and gets `ReplicaBusyV1` (replica:584) | Replica stops; Linux restart loops; processes in D state survive a stop | partial (preflight probes time out; the daemon does not) | Run store I/O under a watchdog that reports store-unreachable and keeps local work; lock by pid plus start time |
| X11 | P1 | Windows | Drive mapped in the elevated provisioning session is invisible to the non-elevated logon task (unconfirmed) | ps1:168, :180; topology `mountParts` task :205, :211 | Provisioning maps `S:` in the elevated token; the task runs least-privilege and sees nothing until the logon mount task runs | Store unreachable at first start; race between mount task and replica task | no | Map the drive from the logon session before the task starts, or set `EnableLinkedConnections`; check with `whoami /groups` and both token views |
| X12 | P1 | Windows (German) | Preflight and status parse localised command output | command:329-330 (`Status: Running`), :382 (`Network Time: On`), :384 (w32tm labels), :272-278 and :315 (`net use` columns); bash:338-340 | German Windows writes localised state words; regexes miss | False FAIL for `service.replica` and WARN for time sync on German workers (the project requires EN and DE; unconfirmed on a German host) | no | Locale-invariant sources: `Get-ScheduledTask` State, `Get-SmbMapping`/CIM for drives, `Get-Service w32time`; keep localised text for humans only |
| X13 | P1 | Windows NFS | Preflight mount check uses `net use` for an NFS mount | command:315, :272-278 | Client for NFS mounts are listed by `mount.exe` with no arguments, not by `net use` (unconfirmed) | `mount.missing` for every Windows NFS worker | no | Parse the `mount.exe` list (the tool that mounts) and cross-check with `Test-Path` |
| X14 | P1 | Windows NFS | Cache registry key differs between code and ticket review; value names unverified | topology:115-116; provisioning-review.md:107; windows-ci-report.md:77 | Code writes `...\ClientForNFS\CurrentVersion\Users\Default\Cache`; the review says `...\CurrentVersion\Default` | Settings silently not applied; preflight `clientcache` reads the same wrong key and cannot detect it (command:400-413) | partial | Confirm the key on a host (`reg query`); fix one path; have preflight read the key it writes |
| X15 | P1 | all | Git view depends on machine-local inputs that `gitViewV1` does not pin | git:104, :220-227; boundary:177-186 (`.git/info` private); git:430 | E4: XDG ignore file hides untracked files; `-c core.excludesFile=` disables it; system and global config and git version differ per machine | Attested `view.digest` differs by machine config, not by repo state | no | Pin for `status`: `-c core.excludesFile=`, `-c core.ignorecase=false`, `-c core.filemode=false`, `-c core.longpaths=true` (win32), `GIT_CONFIG_GLOBAL` to a null file, `GIT_CONFIG_NOSYSTEM=1`; normalise `.git/info/exclude`; attest `git --version` |
| X16 | P1 | Windows vs others (latent) | Symlinks: file realm drops them on Windows, index realm still applies them; absolute-target links are refused in the file realm but reach the stage | replica:1255-1258; boundary:162-170 (`linkRefusalV1`); git:258-264, :332-340 | Windows without Developer Mode: link in stage, no file, ` D` forever. Absolute target: refused, still in stage (brief item c) | Windows view and verify drift; the same `D` on every worker for absolute links | partial (scan refusal; no capability probe) | Apply the link policy to index registers; add a symlink capability probe to preflight that fails when the repo tracks links; never ship absolute links |
| X17 | P2 | macOS vs Linux | `repairV1` uses `lstat`, which cannot see a case-only mismatch on APFS | replica:715-720 | Repair on Mac keeps the wrong-case name; Linux re-queues it | Repair result differs by OS | no | Use the exact readdir compare as at replica:1052 |
| X18 | P2 | all | Attribute and name cache differs per OS: Linux `lookupcache=none` plus `actimeo=1`; macOS has no positive name-cache control; Windows key unverified | topology:29, :31, :33, :113, comment at :215 | `head.json` visibility latency differs per OS; measured on Linux only | Follower lag differs by OS (unconfirmed) | no | Measure head visibility per OS with a probe and record it in preflight |
| X19 | P2 | macOS | AppleDouble `._*` and `.DS_Store` written to the NFS store by macOS clients (unconfirmed for this build) | replica:1675, :1691 (hex names only); journal:43-44 | `._<sha>` appears in `blobs/`, never collected | Unbounded garbage; readers ignore it | no | Compact removes `._*` and `.DS_Store` under `v1/`; preflight warns on them |
| X20 | P2 | macOS, Windows | Boot order: macOS replica LaunchDaemon has no mount dependency; Windows task starts only at logon | topology:263 (macOS), :251 (Linux `RequiresMountsFor`), :237, :242 | Daemon starts before the NFS mount | Store-unreachable noise until mounted; feeds X10 | no | Wait for the mount in the daemon start (or a wrapper), report it |
| X21 | P2 | Windows, macOS | No log for the Windows daemon; macOS log unbounded | topology:228, :232-241, :263-266 | Windows task output is not captured; `/var/log/semio-fleet-*.log` never rotated | Failures invisible on Windows; macOS log grows | no | Wrapper writes a size-limited log under `.cache/f`; `newsyslog` for macOS |
| X22 | P2 | Windows | Task runs only with an interactive logon | topology:237, :242 | Reboot without logon leaves the fleet down | Outage until someone logs on (sleep behaviour unconfirmed) | no | Document autologon or a service account; preflight checks the logon session |
| X23 | P2 | Linux vs others | Watch failure is swallowed; Linux without inotify limits silently falls back to the 15-minute scan | replica:1440-1449 | `watch` throws and is caught without an event | Per-OS latency differs | no | Emit `watch-unavailable`; preflight reports the watcher mode |
| X24 | P2 | all | Git resolution: daemon PATH lacks Homebrew; git version not attested; no timeout on git spawn or `works()` | git:56-66, :95-114; topology:220 | Different git per OS or per context | View computed by different git versions; a hung git stalls attestation | no | Attest git version; time out git calls and mark the view unavailable |
| X25 | P2 | macOS | `protectHFS` defaults on for macOS only; an HFS-ignorable `.git` spelling is accepted on Linux and refused by macOS git (unconfirmed by version) | git:255 (exact `.git` only), :156-162 | Mac `update-index` batch fails whole | Mac stage blocked (`index-busy`) | no | Refuse HFS-ignorable and NTFS-special names in `representable`; pin `core.protectHFS` and `core.protectNTFS` |
| X26 | P2 | Windows | Reserved-name list incomplete (`CONIN$`, `CONOUT$`, `CLOCK$`; unconfirmed per Windows version) | boundary:22 | A name the scanner accepts cannot be created on Windows | Materialise blocked forever | partial | Extend the list, add to fixtures |
| X27 | P2 | Windows vs Linux, macOS (latent) | CRLF bytes replicate raw; `eol=lf` only applies to checkouts | `.gitattributes`:1-5; replica writes bytes (replica:1263-1277) | A Windows editor saves a `.sh` as CRLF | Bash breaks on Linux and macOS | no | Refuse CR in `*.sh` at publish or check in preflight |
| X28 | P2 | Windows | User PATH rewritten as REG_SZ | ps1:236-237 | REG_EXPAND_SZ entries lose expansion (unconfirmed) | Tool lookups break for the account | no | Append without rewriting or use expand semantics |
| X29 | P2 | Windows | `$User` default is `USERDOMAIN\USERNAME` | ps1:29 | Microsoft or Entra accounts may not resolve to the task principal (unconfirmed) | Task registration fails | no | Resolve the principal explicitly |
| X30 | P2 | Windows | 32-bit PowerShell resolves `System32` to SysWOW64 (no `tar.exe`) | ps1:205 | Run from 32-bit host | Bootstrap fails | no | Require 64-bit host or use `Sysnative` |
| X31 | P2 | all (design) | Hint channel and `touch --hook` are documented but not implemented; design risks: mode 1777 has no meaning on NTFS or APFS, payload separators, shell quoting, `$CLAUDE_PROJECT_DIR` expansion per shell | contract:55 (grep finds no `hints` or `touch` in the module) | Future implementation | Quoting or separator bugs on Windows (unconfirmed for Claude Code hook environment) | n/a | Implement as a `repo` subcommand reading JSON from stdin; normalise separators with the scanner's function; no shell |
| X32 | P2 | docs | Contract and readiness audit out of date | contract:49 (lists `core.untrackedCache`, `index.version`, not set by code at git:430); contract:9 (`.git/config` shared) versus :22 (private); contract:53 (`run` accepts `watch:false`; no such option, replica:14); readiness audit says nothing normalises config (now git:418-444) | Misleading to the next reader | Documentation | n/a | Update the contract to match code, or implement the two keys |
| X33 | P2 | all | Attestation `platform` is `process.platform` (`win32`, `darwin`), fleet tables use `windows`, `macos` | attest:278 | Cosmetic mismatch in `fleet` output | Confusion only | n/a | Map through `platformV1` |

## 4. Detail for P0 and P1

### X01 (P0, unconfirmed): Windows task XML child order

- Location: `topology/🟦️.ts:238` (`Settings` assembled as `MultipleInstancesPolicy`, `DisallowStartIfOnBatteries`, `StopIfGoingOnBatteries`, `StartWhenAvailable`, then the caller's `settings`), `:244` (`FOREVER` = `RestartOnFailure` then `ExecutionTimeLimit`), `:266` (compact `CalendarTrigger`: `Enabled` before `StartBoundary`).
- Scenario: `ps1:277`, `:282` pass this XML to `Register-ScheduledTask -Xml`. If Task Scheduler validates child order (I believe the schema is a sequence and that exported tasks list `RestartOnFailure` after `Priority`), registration fails.
- Consequence: no Windows worker can register its replica task, so the machine never joins; `ps1` stops with an exception at step 8 (so it fails loudly, not silently).
- Evidence: the test at topology test:223-239 checks substrings (`toContain`) only, so order is untested. The XML was never validated on a host (windows-ci-report:79 asks for it).
- Guard: no. Fix: validate the rendered XML against the Task schema in a test and in preflight; emit children in exported order.

### X02 (P0 candidate, unconfirmed behaviour): path budget and `core.longpaths`

- Location: boundary:142 and :153 (code-point check), boundary:144 (component bytes); topology `worktree.windows = C:\semio` (topology `🔣️.json`); `ps1:62` (only the root length is checked); `ps1:146-152` (`LongPathsEnabled` registry); git:430 (no `core.longpaths` key); command:395-398 (preflight reads the registry only).
- Measured: the two longest tracked paths are 254 UTF-16 units (0 paths above 260 without the prefix). With `C:\semio\` (9 units) they are 263 units, above MAX_PATH (260). Both sit under `.🧬semio/.../GET-CODEBASE-RUNNING-END-TO-END-AGAIN/.../backup/`.
- Scenario: a Windows worker with `LongPathsEnabled=1` (provisioning sets it) still runs git with `core.longpaths=false` (git default on Windows). Git's `lstat` on those paths may fail or skip them.
- Consequence: the Windows `git status` lines differ from Linux and macOS, so the attested `view.digest` never matches (unconfirmed: exact git behaviour on the installed version).
- Also: the replica's policy measures code points, so a 300-unit path of emoji can pass the policy and still exceed MAX_PATH.
- Guard: no (partial: registry read only). Fix: `-c core.longpaths=true` on win32 in `git()` (git:104) and in normalisation; check UTF-16 length of `worktree + '\' + path` against 259 in preflight and in the policy for win32.

### X03 (P1): case-insensitive ignore on macOS

- Location: git:430 (`core.ignorecase` probed and written locally), boundary:95 (`new RegExp(..., "su")`: no `i` flag), boundary:195 (`sharedV1` case-sensitive).
- Verified: E1 (macOS git applies the `Backup/` pattern to `backup/`).
- Scenario: a Mac user keeps `build/` output in `Build/`; git on the Mac (ignorecase) hides it, the replica (case-sensitive) shares and publishes it, Linux and Windows show `??` for it.
- Consequence: Mac view omits lines; `view.digest` diverges; tracked-but-gitignored files behave differently per OS (brief item a and c).
- Guard: no. Fix: pin `core.ignorecase=false` in every git invocation. Trade-off: on macOS, index names that differ only in case then show as `D` and `??` (E2 shows the Linux-equivalent result); this is the consistent answer. Also lint `.gitignore` for case-variant duplicates.

### X04 (P1): case-only rename

- Location: replica:1041-1052 (deletions use `listed()`, exact compare, so the Mac publishes delete plus put); git:258-264 and :280-318 (the index is never updated from a disk rename).
- Verified on macOS: E2 (clean view with ignorecase; with `ignorecase=false` only `?? foo.txt`, because `lstat` finds `Foo.txt` on APFS).
- Scenario: rename `Foo.txt` to `foo.txt` on the Mac. The file realm replicates it correctly, but the index still holds `Foo.txt`.
- Consequence: Linux shows ` D Foo.txt` plus `?? foo.txt`; Mac shows nothing. Only a `git add` on the Mac fixes both (the index register then replicates).
- Guard: no. Fix: in publish, compare the index names with disk names (`git ls-files` against readdir with exact compare) and report the case mismatch as a staging action; `verify` should fail on it.

### X05 (P1): executable bit has two channels

- Location: replica:944 (`prepare`: Windows keeps the store flag, a new Windows file gets `false`); replica:1016 (`unchangedSince` ignores exec on Windows); replica:1243 and :1277 (materialise chmods on Linux and macOS only); git:156-162 and :280-318 (index mode changes only through registers or `git add`); git:430 (`core.filemode=true` on Linux and macOS, `false` on Windows).
- Verified: E3 (`core.filemode=true` shows ` M` for a chmod-only change; `false` shows nothing).
- Scenario A: chmod +x on Linux of a tracked file. The file flag replicates; Linux and Mac workers chmod; their index keeps 100644; their view shows ` M`; Windows shows nothing.
- Scenario B: a Windows user recreates `tool.sh` (new file). It is published `executable=false` (no prior store entry, or a delete was published first). Linux chmods 0644 while its index says 100755: ` M`, and the script loses `+x`.
- Consequence: attested views diverge by OS; scripts lose their executable bit on Linux and macOS.
- Guard: no. Fix: pin `core.filemode=false` in the attested view (so view digests ignore mode on every OS) and add a separate check that compares the file flag to the index mode (fails in verify and preflight). Alternatively derive the file flag from the index mode on publish.

### X06 (P1, latent): index registers skip the path policy

- Location: git:255 (`representable` runs on the publish side only and checks backslash, NUL, empty or dot parts, `.git`, and length; not Windows-illegal characters, reserved names, trailing dot or space, or case variants); git:258-264 (`changeOf`) and git:332-340 (`consider`) apply no policy; git:156-162 (`updateIndexV1`) is one batch, so one refused entry fails the batch (`index-busy` at git:360 or replica:1288).
- Scenario: a Linux user commits `a:b.txt`, `CON.txt`, `x.` or a case-variant pair. The index register reaches every worker; the file is refused on Windows (boundary:21-22, :155, :157) and never materialises.
- Consequence: Windows `git status` shows ` D a:b.txt` forever; on APFS a case-variant pair collapses in the index and the fold check (verify) then disagrees with the scanner. Whether git itself refuses `a:b.txt` on Windows is unconfirmed; if it accepts it, the entry stays and the consequence above holds.
- Repo today: 0 such names (measured), so latent.
- Guard: no. Fix: apply `refusePathV1` and the case rule to index registers on publish and apply, using the strictest rule of every OS; `verify` should list refused index entries.

### X07 (P1): NFD names and the watch path

- Location: replica:972 (`child.normalize("NFC")` for directory children), replica:1442 (watch names normalised to NFC), replica:956-991 (`observe` `lstat`s the NFC spelling and publishes the entry); boundary:152 (scanner refuses `not-nfc`).
- Verified on APFS: an NFD file is stored as NFD; `lstat` of the NFC spelling succeeds and finds it.
- Scenario: a tool on the Mac creates `café.txt` in NFD. The watch event normalises to NFC, `observe` stats and publishes it under the NFC name. The full scan lists the NFD name and refuses it.
- Consequence: the event path and the safety-net scan disagree; Linux materialises a name the scanner would refuse; `verify` and `status` can flip-flop.
- Guard: partial (`listed()` at replica:101 is exact but used only at replica:984 and :1052, not on observe's file path). Fix: require the exact on-disk name before publishing; refuse non-NFC on-disk names before `lstat`.

### X08 (P1): case-fold mismatch, APFS `ß`/`ss` and final sigma

- Location: replica:183 (`hashText(path.toLowerCase())`), :401 and :405 (`ReplicaFoldsV1.find`), :417 (`add`), :1184 (`refusalOf` uses `folds.find`); boundary:28 (`foldNameV1` = upper then lower), :230 (scanner fold).
- Verified on APFS: `ß.txt` then `ss.txt` gives one file; `ΟΔΟΣ.txt` then `οδοσ.txt` gives one file. `"ΟΔΟΣ".lower()` is `οδος` but `"οδοσ".lower()` is `οδοσ`, so the replica fold does not see the pair.
- Scenario: Linux holds `ß.txt` and `ss.txt` (both legal). The scanner refuses one on Linux (the same fold), but if both names reach a Mac worker (via the index, or an origin whose scanner did not see the pair), `refusalOf` accepts the second and `materialise` (replica:1236-1295) writes it over the first.
- Consequence: silent overwrite on the Mac; the first file's bytes survive in `.cache/f/lost` only if `conflictWinnerV1` (replica:482) judges the local copy as losing (otherwise `keepNewer` skips the write and state disagrees with disk).
- Guard: no. Fix: one fold function for scanner, replica and index. Use the union of APFS, NTFS and Linux case-insensitive folding and refuse both names on every OS.

### X09 (P1): order-dependent case-variant handling

- Location: replica:982-986 (`observe`: `folds.find` then `listed` exact), boundary:225-231 (scanner: sorted first wins), replica:1213 (`case-collision` returns `queued`), replica:1041-1052 (scanner forgets refused names without a delete).
- Traced scenario: `foo` is published first; then `Foo` is created on Linux. `observe` sees `foo` in folds and `listed(root,'foo')` is true, so it skips `Foo`. The next full scan sorts `Foo` before `foo`, shares `Foo` and refuses `foo`; `foo` is then `forget`-ed (replica:1049-1052) without a delete event, so the manifest still holds `foo`.
- Consequence: the manifest keeps both names; Linux `verify` reports `missing-local` for `foo` (replica:1737-1742); Mac receives `foo`, refuses it as `case-collision` and keeps it queued, so `incoming` never reaches 0 and the verdict is `lagging` (judgeFleetV1 `settled`).
- Evidence: traced, not run. Latent (repo has no case-variant pairs; measured).
- Guard: no. Fix: one winner rule in observe and scan; when a name becomes refused, publish a delete for it.

### X10 (P1): store I/O without timeouts, hung NFS

- Location: replica:519-521 (`storeReachableV1` = `stat`), journal:235-251 (`readHeadV1` = `stat` and `readFile`), replica:553-566 (`readLock`), replica:566-588 (`acquireLock`, pid-alive takeover, `ReplicaBusyV1` at :584), replica:1425-1497 (`runV1` loop: each round calls these without a timeout).
- Scenario: the NFS server goes away under a `hard` mount (topology :29-33). `stat` and `read` block in the kernel; the loop never reaches `pause()`, so the abort signal is not served. `launchctl bootout`, `systemctl stop` or SIGTERM do not free the lock; a restarted daemon sees a live pid (`alive()` at :553) and throws `ReplicaBusyV1`.
- Consequence: the replica stops working until the mount is forced (`umount -f`) and the process is killed; processes in D state survive kill. On Linux `Restart=always` loops with `RestartSec=2`.
- Evidence: traced from code and NFS `hard` semantics; runtime unconfirmed.
- Guard: partial (preflight probes use `within()` timeouts; the daemon does not). Fix: run store I/O under a watchdog that emits `store-unreachable` and keeps local work going; lock by pid plus process start time; document the forced unmount runbook.

### X11 (P1, unconfirmed): elevated mount invisible to the logon task

- Location: `ps1:168` (`net.exe use /persistent:yes`) and `ps1:180` (`mount.exe`) run in the elevated shell; `topology:205` and `:211` (the mount task and its arguments; the task has no run level, so it runs least-privilege); the replica task `topology:265` runs `InteractiveToken`, `LeastPrivilege`.
- Scenario: Windows keeps separate drive mappings per token when linked connections are off. Provisioning maps `S:` in the elevated token; the non-elevated replica does not see it until the logon mount task maps it in the logon token. The ps1 `Test-Path "$Drive\"` check (ps1:167, :179) runs in the elevated token and passes.
- Consequence: first start can report `store-unreachable` (the loop recovers when the mount appears; no permanent failure). Race between the mount task and the replica task at logon.
- Guard: no. Fix: map the drive from the logon session (or start the mount task before the replica task); set `EnableLinkedConnections` if the design needs elevated mappings; verify with `whoami /groups` and `net use` from both tokens.

### X12 (P1, unconfirmed on a German host): localised output parsed

- Location: command:329-330 (`schtasks /Query /FO LIST` value `Running`), command:382 and bash:338-340 (`Network Time:` / `: On`), command:384 (`Local CMOS Clock`, `Free-running` from `w32tm /query /status`), command:272-278 (`net use` columns), command:315 (`net.exe use` as the mount source).
- Scenario: German Windows prints the state of the task and of the clock in German, and the value word `Running` is localised.
- Consequence: `service.replica` fails and reports `service.down` (preflight FAIL); time sync warns for working workers. The project requires EN and DE output, so both languages must be correct.
- Guard: no. Fix: locale-invariant sources: `Get-ScheduledTask` State enum, `Get-SmbMapping` or CIM drive objects, `Get-Service w32time` status; keep localised text for humans only.

### X13 (P1, unconfirmed): NFS mount listed by `net use`

- Location: command:315 (`probes.run("net.exe",["use"])`), command:272-278 (`mountedWindows`).
- Scenario: Client for NFS mounts are listed by `mount.exe` with no arguments, and I believe not by `net use` (unconfirmed).
- Consequence: `mount.missing` (preflight FAIL) on every Windows NFS worker even when the store works.
- Guard: no. Fix: parse `mount.exe` with no arguments (the tool that mounts) and cross-check with a `Test-Path` probe of a store file.

### X14 (P1, unconfirmed): NFS cache key path disagreement

- Location: topology:115-116 (`HKLM:\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Users\Default\Cache`, values `RemoteWriteCache`, `FileAttributeCache`); `ticket provisioning-review.md:107` says the values sit under `ClientForNFS\CurrentVersion\Default` and that the names are unverified; `windows-ci-report.md:77` uses the `Users\Default\Cache` path.
- Scenario: the key path is wrong for the installed Client for NFS version; `ps1:100-106` creates an empty key and writes the DWORDs there.
- Consequence: attribute caching stays on for the Windows worker; preflight `clientcache` (command:400-413) reads the same wrong key, so it passes or fails for the wrong reason.
- Guard: partial. Fix: confirm the key on a host (`reg query "HKLM\SOFTWARE\Microsoft\ClientForNFS" /s`), keep one path, and make preflight also check the effective mount behaviour.

### X15 (P1): git view inputs not pinned

- Location: git:104 (`-c gc.auto=0 -c core.fsmonitor=false` only), git:220-227 (`gitViewV1`), git:430 (eight normalised keys, none of the view-relevant `ignorecase`-for-view or `excludesFile` pins), boundary:177-186 (`.git/info/` is private, so each machine has its own `info/exclude`).
- Verified: E4 (XDG ignore changes the untracked lines; `-c core.excludesFile=` and `/dev/null` remove the effect).
- Scenario: a Windows worker has `%USERPROFILE%\.config\git\ignore` (git's default excludes location) from a tool; the Linux worker does not. Their views differ with identical repo state.
- Consequence: `view.digest` diverges with no repo difference; fleet `fleet --diff` shows lines that only differ by machine config.
- Guard: no. Fix: pin in `git()` for `status`: `-c core.excludesFile=`, `-c core.ignorecase=false`, `-c core.filemode=false`, `-c core.longpaths=true` (win32), `GIT_CONFIG_NOSYSTEM=1`, `GIT_CONFIG_GLOBAL` to a null file (git 2.32 or later; record the git version and fail below it); normalise `.git/info/exclude` to empty; attest `git --version`.

### X16 (P1, latent): symlinks on Windows and absolute targets

- Location: replica:1248-1260 (`materialise` link branch: `symlink` fails with EPERM on Windows without Developer Mode, returns `dropped`, dequeues), replica:1258 (no file state set), boundary:162-170 (`linkRefusalV1`: absolute and drive-qualified targets refused), git:258-264 and :332-340 (index entries for links applied with no capability check).
- Verified: windows-ci-report:58 says no test covers the Windows EPERM path.
- Scenario A (Windows without Developer Mode): a tracked symlink gives an index entry on every worker; the Windows file is dropped; `git status` shows ` D link` forever.
- Scenario B (brief item c, any OS): an absolute-target link is refused by `linkRefusalV1` (never shared), but its index entry still replicates; every worker shows the link as deleted, the origin does not.
- Consequence: Windows view and verify drift; the same deletion appears on all workers for absolute links.
- Repo today: 0 symlinks (measured), so latent.
- Guard: partial (scan refusal). Fix: apply the link policy to index registers; a preflight symlink capability probe that fails when the repo tracks links; refuse absolute links at commit time (fixture and hook).

## 5. Checked, no finding

- Bash 3.2 versus 5.x: `shopt -u patsub_replacement` guards the only `${var/pat/rep}` with `&`-sensitive content (bash:516, :525, :536); no `mapfile`, `declare -A` or `printf -v`; `[[ =~ ]]` is supported on 3.2; `PIPESTATUS` is read correctly (bash:329). No difference found that breaks macOS.
- GNU-only commands are reached only on Linux branches: `fallocate` and `dd status=none` (bash:301), `stat -c` (bash:306), `mountpoint` and `findmnt` (bash:462, :466), `timedatectl` (bash:344-347), `mount -t nfs4` (bash:371). `dscacheutil` (bash:101) runs only inside the macOS server branch (bash:490). `chown root:wheel` (bash:203) is macOS-only. `set -u` is not used, so empty-array expansion on 3.2 cannot abort.
- BSD `sed` and `tar`: `sed` expressions use POSIX classes; `tar -tf | wc -l` counts pax entries the same way on bsdtar and GNU tar (the pax header is not listed). Verified by reading only.
- Repo path policy: 0 non-NFC, 0 illegal characters, 0 reserved names, 0 trailing dot or space, 0 case-collision groups, 0 over 256 code points, 0 names over 255 bytes per component (measured).
- Store locking: the store is single-writer per origin with rename-based publication; `nolock` or `nolocks` do not weaken it. The only lock file (`.cache/f/daemon.lock`) is local.
- `.gitattributes` binary and `-text` rules for fixtures and PDFs are consistent with the byte-exact fixtures.
- Launcher: the Windows executable is `repo.exe` (launcher `📥️install/🔵️.ps1:189-191`), so the scheduled tasks' bare `repo` command resolves.
- Submodules: 0 gitlinks, so `diff.ignoreSubmodules` is not a view input today.
- Timestamps and racy window: sound for NTFS, APFS, ext4, NFS with ns attributes (see section 2).

## 6. Proposed automated cross-OS guards

Each guard runs in one of three places: `preflight` (per machine, in the topology command, read-only), `verify` (per replica, on demand or in the fleet), or the cross-OS fixture matrix (CI on Linux, macOS and Windows with real file systems).

- **G1 view parity (preflight and fleet).** Compute the pinned `git status` lines (see X15 pins), the exec-mode mismatches (X05), the case-variant index entries and case-only index mismatches (X04, X08), and the index entries that the path policy refuses (X06). Record the git version. Fleet already compares `view.digest`; add these counts so the cause is visible.
- **G2 path budget (preflight, fixture matrix).** For each tracked path: UTF-16 length of `worktree + separator + path` must be at most 259 on win32 (X02); UTF-8 bytes of the absolute path at most 1024 on macOS; 4096 on Linux. Fail preflight when any repo path exceeds the budget for its own OS.
- **G3 git config pins (preflight).** `git config --show-origin --get-all` for every pinned key, the XDG and `~/.config/git/ignore` files, `core.excludesFile`, `core.attributesFile`, `.git/info/exclude`, and system and global config. Warn on any value that the pins override (X15).
- **G4 store probe (preflight, once per mount).** Write a file, rename it over an existing file, `lstat` a case variant, read `mtime` at 1 ms spacing (granularity), count `._*` and `.DS_Store` under `v1/` (X19), and report the mount options the OS actually applied. Compare the results in attest (X18).
- **G5 cross-OS name fixture matrix (CI, real file systems).** One table of names: NFD, `ß`/`ss`, `ΟΔΟΣ`/`οδοσ`, VS16, device names with and without extensions, `CONIN$`, trailing dot and space, `:`, backslash, 255 and 256 code points, 259 and 260 UTF-16 units. Each OS must produce the same refusal set from scanner, replica folds, and `git update-index` (X06, X08, X26). Use the boundary fixture file for the expected table.
- **G6 event-versus-scan property test (macOS and Linux).** Every path `observe` publishes must be accepted by `scanSharedV1` over the same tree, and every path the scan shares must be publishable by `observe` (X07, X09). Include NFD names and case-variant pairs created in both orders.
- **G7 service smoke test (per OS).** Kill -9 restart, `umount` or `unmount` leading to `store-unreachable` within 10 s, reboot start (launchd, systemd, scheduled task after logon), and a log-size bound after a simulated day (X10, X20-X22).
- **G8 script and task lint (CI).** `shellcheck --shell=bash` plus `/bin/bash -n` on the macOS 3.2 runner; PSScriptAnalyzer and XSD validation of every rendered task XML (X01); `schtasks /Create /XML` on a Windows runner; `schtasks` and `w32tm` output parsed under a German UI culture on a German runner (X12).
- **G9 link capability (preflight and verify).** Create a symlink in the store scratch directory; fail when the repo tracks links and the probe fails; in verify, fail on any index link entry whose file is absent on this OS (X16).

## 7. Confirm on real hardware (unconfirmed items)

- X01: render the replica task XML with `repo ... worker-service --platform windows` and run `schtasks /Create /TN semio-test /XML <file>` on a Windows 10 or 11 host; then `schtasks /Query /TN semio-test /XML`.
- X02: on a Windows host with `C:\semio` and both long paths present, run `git -c core.longpaths=false -C C:\semio status --porcelain=v1 -z` and `git -c core.longpaths=true ...`; diff the two outputs and count the lines that differ.
- X05: on Linux, `chmod +x` a tracked file; compare `git status` on Linux, macOS and Windows after the replica settles.
- X11: `whoami /groups` in the elevated and non-elevated tokens; `net use` and `mount.exe` in each; reboot test with the mount task disabled.
- X12: on a German Windows host, `schtasks /Query /TN semio-fleet-replica /FO LIST` and `w32tm /query /status`; on a German macOS host, `systemsetup -getusingnetworktime`.
- X13: `mount.exe` with no arguments and `net use` on a host with an NFS mount at `S:`.
- X14: `reg query "HKLM\SOFTWARE\Microsoft\ClientForNFS" /s` on a host where the Client for NFS feature is enabled.
- X10: in a lab, put a Linux NFS server behind a firewall while the replica runs; observe whether SIGTERM ends the process and whether a restarted daemon reports `ReplicaBusyV1`.
- X16: on Windows without Developer Mode, check `symlink` creation and the `skipped` event; on macOS and Linux, track a symlink in a scratch repo and compare `git status` with a Windows worker.

## 8. Documentation drift (for the next owner)

- `1113-git-state-contract.md:49` lists `core.untrackedCache` and `index.version`; code (git:430) sets neither.
- `1113-git-state-contract.md:9` says `.git/config` is shared; `:22` says it is private; code (boundary:177-186) keeps it private.
- `1113-git-state-contract.md:53` says `run` accepts `watch: false`; `ReplicaOptionsV1` (replica:14) has no such field and `runV1` always watches.
- `1113-audit-os-readiness.md:93` says nothing normalises `.git/config`; git:418-444 does now.
- `provisioning-review.md:107` and `🔵️.ps1:65` disagree on the NFS cache key (X14).
- `windows-ci-report.md:57-58` lists rename-over-existing and symlink EPERM as untested; still untested (X16).
