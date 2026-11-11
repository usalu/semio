# Provisioning review: server platforms, idempotent re-provisioning, reachability, limits

Scope: `*topology/` (TS module, topology JSON, schema, command, script, `*provision/` scripts, tests, Python oracle)
and the provisioning/server sections of the module README. Nothing under `*replica/` was touched.

## What changed

1. **Group servers on Linux, macOS and Windows.**
   - Schema (`*topology/*schema`): every group has `server` = `platform` (linux|macos|windows), `protocol` (nfs|smb),
     `network` (IPv4 CIDR of the workers) and, on Windows only, `directory`. The Windows worktree root is a new
     `WindowsRoot` (drive plus one directory of at most 12 characters, `C:\semio`). The committed topology got
     `linux/nfs/10.0.N.0/24` for g1..g5 (placeholders like the hosts).
   - `readTopologyV1` additionally refuses: smb off Windows; a Windows server without `directory` or with a
     multi-segment share name as `nfs.export`; `directory` off Windows; a macOS export outside `/Users` and `/Volumes`
     (the root volume is read-only); any macOS server in a fleet that has an smb group (a macOS relay cannot read SMB).
   - `serverPartsV1`/`serverPlanV1` follow `server.platform`:
     Linux (systemd, `/etc/exports.d`, fstab block with bind mount and peers by `nfs4`, `nfs` v3 or `cifs`),
     macOS (launchd cache/relay/peer-mount daemons, `/etc/exports` with a read-write network line and a `-ro` peer line,
     `/etc/nfs.conf` pinning mountd to 892, `nfsd`), Windows (`Install-WindowsFeature FS-NFS-Service` and
     `New-NfsShare` with per-peer read-only grants, or `New-SmbShare` with `semio-fleet` Change and `semio-fleet-peer`
     Read when `protocol: smb`; firewall rules; cache and relay as Scheduled Tasks; peers read by UNC path).
     New part `relay-peers` (the `--peers` value; the replica relay defaults to the Linux peer paths otherwise).
   - Client protocol per worker OS, `clientProtocolV1`: Linux server: Linux NFS 4.2, macOS NFS 4, Windows NFS 3
     (Client for NFS is NFSv3 only). macOS server and Windows Server for NFS: NFS 3 for all three client OSes
     (Linux `nolock`, macOS `nolocks`). Windows smb: only Windows workers, `net use S:` (credentials once with `cmdkey`);
     Linux and macOS workers of an smb group are refused (`facts`, `mount-plan`: "Windows workers required").
   - `workerPartsV1(..., server=true)` now works on every platform (`compact --collect-blobs`) and refuses a platform
     other than the group's server platform. `factsV1` adds `PROTOCOL`, `SERVER_*`, `SERVER_CACHE_DIRECTORY`.
   - `provision.sh --server` (Linux, macOS) and `provision.ps1 -Server` do the whole server setup themselves
     (folders, exports/share, firewall, peers, cache and relay services, compaction with `--collect-blobs`), refuse a
     platform that differs from the topology and need the tokens file (never written by the scripts). On macOS and
     Windows the server's store root is the exported folder via `--store` (recorded in `machine.json`).
   - Schema, fixture (a1 linux, b2 macos, c3 windows; 14 new invalid cases), Python oracle and tests were changed together.
2. **Idempotent re-provisioning.** Both scripts read `.cache/f/machine.json` before changing anything. A different origin
   or group stops the run with a message naming both. The same identity skips `join`/`seed` only; mounts, limits,
   environment, services and tasks are installed again (units that changed are restarted: `try-restart` on Linux,
   `bootout`+`bootstrap` on macOS). The old check was a `grep` of the origin anywhere in the file and ran after the mounts.
3. **`validate --reachable [--timeout ms]`.** Plain `validate` stays offline (the sync API throws if asked for
   `--reachable`). The async path resolves each group's store host and cache host, probes the store port (2049, 445 for
   smb) and the cache port with a timeout (default 3000 ms), all probes concurrently, one progress line per probe on
   stderr, a per-group report on stdout, exit code 1 on any failure or cancellation, cancellation through the script
   invocation's `AbortSignal`. README: the operator edits exactly one file, the topology JSON; everything else is derived.
4. **inotify.** Linux provisioning writes `/etc/sysctl.d/90-semio-fleet.conf` (`max_user_watches=1048576`,
   `max_user_instances=1024`), applies it with `sysctl -p` and verifies every key against the file. The Linux worker plan
   (`mount-plan --platform linux`) lists it as parts `sysctl-path` and `sysctl`.
5. **Windows long paths.** `provision.ps1` already set `LongPathsEnabled`; it now reads it back, refuses a worktree root
   longer than 15 characters, and the schema enforces a short root. README says so, with the reason.
6. **Static checks**, see below.

## What was executed (on this macOS host, Bun 1.4.2, python3)

- `bun test` on the topology suite: 22 tests, 22 pass, run three times in a row at the end (22/22 each).
  Covers: schema (Ajv and the repository validator agree on all 32 invalid cases), per-platform/per-protocol mount plans,
  server plans for Linux, macOS, Windows NFS and Windows SMB, inotify values, idempotence/identity messages and
  script/command consistency (every `server_part`/`Get-ServerPart` name exists), XML well-formedness of every plist and
  Task XML through Python's ElementTree, reachability with fake ports (ok, resolve failure, refused, timeout, cancellation,
  offline `validate` never touches the network) and with a real local TCP listener.
- The replica suite (not edited) still passes with the changed topology: 35 of 35.
- `bash -n`, `sh -n`, `zsh -n` on the provisioning shell script: clean.
- Python oracle (`python3 -I ... check`, `env`): clean; it validates the new `server`, CIDR, short Windows root and the
  server rules independently of the TS code.
- The sed that reads the group's `server` block from the topology JSON, run against the committed file (g3 gives
  `linux`, `nfs`); the machine.json identity check run against matching and non-matching origin and group files;
  `install_text` and `write_block` run on temporary files (changed/unchanged flags).
- `validate --reachable` against the committed placeholder hosts through the real DNS: all ten probes reported
  `getaddrinfo ENOTFOUND`, exit code 1, one second for a 1500 ms timeout; Ctrl+C-style abort reported `reachability cancelled`.
- PowerShell: no interpreter here. A script `psbalance.py` (quotes, `$()`, braces, parentheses; verified to flag broken
  input) found `provision.ps1` and the generated server snippets (`prepare`, `feature`, `share`, `firewall` for NFS and SMB)
  balanced. The whole `provision.ps1` was reviewed by hand for variable interpolation (`${GroupId}:`, `$(...)`), argument
  splitting for `icacls`/`net.exe`/`mount.exe`, `-split`/`-notmatch` regexes and statement-versus-expression use of `if`.

## What could not be executed here (no Linux, Windows, pwsh, docker, systemd, launchd root, NFS server, shellcheck)

Shell script (`provision.sh`): never run as a whole. Not executed: `sudo` re-exec, `useradd`-free path, `sysctl -p`/`sysctl -n`
on a real kernel, `exportfs -ra`, `systemctl enable --now nfs-server || nfs-kernel-server` (unit name per distribution),
package-manager installs of the NFS server/client, `mount`/`mountpoint` of bind and peer mounts, `mount -t nfs -o ro,vers=3,nolock`,
systemd units (cache, relay, replica, compact timer), `try-restart` behaviour, `launchctl bootstrap/bootout/print`,
`nfsd checkexports/enable/update/restart`, `/etc/exports` with two lines for one folder (macOS), whether `nfsd` really serves only NFSv3,
`nfs.server.mount.port` in `/etc/nfs.conf`, `mount_nfs -o vers=3,...,nolocks`, `launchctl setenv`, `dscl`/`getent`,
`chown root:wheel`, the cache wrapper `sh -c 'set -a; . /etc/semio/remote-cache.env; ...'` under launchd, the cifs mount of an SMB peer
with `credentials=`, shellcheck-level lint (not installed).

PowerShell script (`provision.ps1`) and the generated PowerShell: never parsed by PowerShell. Constructs not executed:
`#region` header and `[CmdletBinding()]` param block with the new `[switch]$Server`; `$x = if (...) {...} else {...}`;
`ConvertFrom-Json` of the topology and of `machine.json`; `Get-ItemProperty`/`New-ItemProperty` on `LongPathsEnabled`
(and that processes started afterwards honour it); `icacls` grants and `/inheritance:r /grant:r`; `net.exe use` (SMB mount,
stored `cmdkey` credentials); `mount.exe` and Client for NFS; `Enable-WindowsOptionalFeature`; `Install-WindowsFeature FS-NFS-Service`
and its absence on Windows 10/11; `New-NfsShare` (parameters `-EnableAnonymousAccess`, `-EnableUnmappedAccess`, `-AnonymousUid/-Gid`,
`-AllowRootAccess`), `Get-NfsShare`, `Grant-NfsSharePermission -ClientType host -Permission readonly` and above all whether the per-peer
read-only entry beats the all-machines read-write entry; `New-SmbShare -ChangeAccess/-ReadAccess`, `Set-SmbServerConfiguration`,
`New-LocalUser`, `Get-LocalUser`; `New-NetFirewallRule`/`Get-NetFirewallRule`/`Remove-NetFirewallRule`/`Resolve-DnsName`;
`Register-ScheduledTask -Xml` with the new Task XML (cache task `powershell.exe -Command` with the `ForEach-Object` token loader and
`&quot;` escapes, relay task with UNC `--peers`), `LogonType InteractiveToken` tasks on a server that nobody logs on to;
`Start-ScheduledTask`/`Get-ScheduledTask`; `Invoke-Expression` of the generated snippets; `(?m)` regex on the tokens file;
`tar.exe` extraction and UNC archive paths for a Windows server; `cmdkey /user:<short host>\semio-fleet` for a local account.

Behaviour never observed on any machine: the replica's `--store` pointing at a local folder on macOS/Windows servers (code path
exists in the replica command but no test of this combination), a Linux/macOS/Windows relay reading a Windows UNC peer, Windows Client for NFS
against a macOS or Linux server, NFSv3 with `nolock` for the store, the 1 MiB inotify memory estimate, `sysctl.d` ordering at boot,
and any of the firewall ports (111, 1048, 892 for NFSv3 mountd are documented, not probed by `--reachable`).

## Round 2

Scope: `topology/` and the module README only.

1. **Incomplete bootstrap.** `provision.sh` and `provision.ps1` write `.cache/f/native-bootstrap.done` (ignored local state) only after the native bootstrap succeeded and `repo` exists; a run without the marker repeats it. Missing build tools are reported before step 1 with the exact install line per package manager (apt-get, dnf, zypper, pacman), `xcode-select --install` on macOS, and winget/Build Tools on Windows. Machines provisioned earlier repeat the idempotent bootstrap once.
2. **Store client caches.** Linux unchanged (`lookupcache=none`, measured). macOS: `acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache` (reasoning, also on the macOS server's peer mounts). Windows SMB: `Set-SmbClientConfiguration` with the three lifetimes at 0 (reasoning). Windows NFS: two registry DWORDs under `ClientForNFS\CurrentVersion\Default`; **the value names are unverified** and need a check on a real Windows machine. New `cache` part of the Windows `mount-plan` and `facts` keys `SMB_CACHE`, `NFS_CACHE`, `NFS_CACHE_KEY` (the PowerShell script refuses a difference).
3. **Sizing, compaction.** README Capacity and Day 0 carry the measured and extrapolated figures and name the 2 TB machine as the group server. Compaction timers/tasks already existed on all three OSes; tests now pin them.
4. **Preflight and clocks.** `preflight` in `script.ts` (EN/DE catalogue in `texts/`, progress, cancellation, injected probes). Provisioning now checks or enables time sync (timedatectl, systemsetup, w32time).
5. **Verification.** shellcheck clean, PowerShell parser and mocked harness green (pwsh 7.6.9). Topology suite 61 tests (was 39) green three times; journal 126, boundary 38, replica 72, bootstrap 11 green. Not verified: real `nfsstat -m`, `net use`, `reg query` output formats; macOS and Windows client behaviour.
