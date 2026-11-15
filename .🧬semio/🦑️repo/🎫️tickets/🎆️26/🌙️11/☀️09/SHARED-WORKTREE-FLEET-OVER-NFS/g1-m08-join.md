# Join g1-m08 To The Fleet

Status: joining and synchronizing; final verification and native bootstrap still in progress. Updated 2026-11-14, Europe/Berlin.

## Machine And Preconditions

| Fact | Value |
| --- | --- |
| Origin / group | g1-m08 / g1 |
| Machine | Ueli’s Mac mini |
| Hostname | ueli-workstation-mac-mini-m4-16gb-256gb-05-wlan.fritz.box |
| Login and console account | ueli, uid 501; kept logged in |
| IPv4 | en1: 192.168.178.88/24; en0 inactive; loopback 127.0.0.1 |
| macOS | 27.2, build 26B123 |
| Physical memory | 17179869184 bytes (16 GiB) |
| Home volume free at precondition check | 171 GiB; required minimum 90 GiB |
| NFS | 192.168.178.70:/Users/Shared/semio-store, export visible for 192.168.178.0 |
| Privileges | ueli is in admin; sudo privilege confirmed and authenticated locally |
| Canonical worktree | /Users/ueli/Documents/semio |

The origin listing held g1-m01 through g1-m07, g1-m09, and g1-server. The user requested the next free number; g1-m08 was absent and was checked again before transfer.

## Repository Case

At first the destination held an incomplete 1.1 GiB `.git`, with no refs and HEAD pointing to `refs/heads/.invalid`, but no `AGENTS.md` or `.🧬semio`. It disappeared outside this agent’s actions. The user authorized preserving it at `~/semio-setup/incomplete-git-backup`, but no move or deletion was necessary or performed.

At provisioning the destination did not exist: the empty-machine case applied. No empty-machine plan was run. `/Users/Shared/semio` was absent. Home and `/Users/Shared` were searched to directory depth 4 for folders holding all three repository markers. No other repository was found in accessible folders. macOS privacy denied several Library folders, Photos Library and Trash; those exclusions prevent an unconditional claim about their contents.

## Provisioning And Recovery

All eight provisioning-kit SHA256SUMS entries passed. Required language, group, NFS host and network variables were set together. Cache endpoint and cache token variables were unset.

1. The first provisioning command exited 1 at step 2: the server manifest omitted the `archive` field, while its `worktree.tar` existed.
2. The user explicitly authorized a local manifest workaround and provisioning retry, overriding the original stop-on-error/no-repairs rule. The server archive, topology and server manifest were left unchanged.
3. A temporary mount-point recreation was necessary because macOS removed `/Volumes/semio-kit` after unmounting (one wrapper attempt exited 66 before provisioning).
4. The first NFS metadata copy failed with ESTALE as the server refreshed its archive. No extraction occurred from that transfer. SSH was checked with existing-trust-only options; port 22 refused connections. No keys or host trust were created.
5. A plain read-only byte transfer produced a local archive snapshot at `~/semio-setup/archive-snapshot-20261114-042419/`. It transferred 17843653632 bytes. The refreshed server manifest described 274821 entries. Its SHA-256, verified locally and again by the provisioner, was `939a46d072efabad57f602b7025e96ad6d46301cfabbdd570553b8ddb83ebedf`. The local manifest added only the required archive filename. Extraction then completed.
6. Launcher, store mount, environment and service definitions were installed. SIP refused session-wide `launchctl setenv`; the script retained SIP and used the profile and service environments.
7. Bootstrap settlement matched 274800 paths and preserved 11 differing files in `.cache/f/lost/bootstrap-1794627675569/`. The provisioning run then exited 1 because three files changed in the fleet between settlement and strict join. The local identity had already been initialized, but the origin was not yet registered.
8. A supported adoption dry run reported Git relation `equal`, zero conflicts, zero new local-work paths, 273127 identical, 11 missing and 10 stale paths. The local and fleet tips were both `5ececaa78b92…`. The plan stayed within the user’s limits. The previous index (271500 staged paths) was preserved under `lost/` by adoption.
9. Actual adoption reported 273126 identical, 11 taken from the fleet, 11 stale, zero conflicts, zero new work and zero edits published; one item remained queued as `pack-incomplete`. Its full report is `.cache/f/lost/adopt-2026-11-14T03-51-10-834Z.md`. No object repair was attempted by hand.
10. Both installed user LaunchAgents were started with `launchctl bootstrap gui/501`. A separate administrator-helper provisioning pass exited 1 because its responsible process could not read Documents; its log is retained. No privacy bypass was applied. System Settings computer control was not approved.
11. The user-session replica subsequently reported `Synchronization: on` and `Daemon: running (pid 19601)`, with g1-m08 registered in the store. It is automatically fetching the queued Git packs. The native bootstrap was started once directly as ueli with cache-token variables unset, performing the final workflow step without the administrator helper’s Documents restriction.

## Checks So Far

`repo preflight --lang en` exited 0 and provisioned the required pinned Rust tooling. The initial verification exited 1 while the daemon was not running and fleet changes were pending. The initial `fleet --wait 600` ended with ESTALE before its time limit. These are not acceptance passes; final results will follow below.

The required read-only HEAD check succeeded:

```
5ececaa78b9265fd72b4f7e8f411afac3c9e1df1
```

An earlier read-only porcelain status count was 4275 while Git-pack repair and synchronization were still pending; it is not a final convergence result.

Installed user services observed through `launchctl print gui/501 | grep semio`:

```
tech.semio.fleet.compact
tech.semio.fleet.replica
```

System store mount service installed: `tech.semio.fleet.store-mount`.

## Logs And Constraints

Machine-local evidence is retained under `~/semio-setup/`: origins and checksum logs, bootstrap diagnosis, provision logs and exit files, adoption plan and application logs, status/verify/fleet logs, read-only Git results, service inventory, native bootstrap log, and a launcher stack sample. The user explicitly requested that setup logs stay there; they are not copied into this shared ticket.

No mutating Git commands, pkill/killall, cache-token generation/copy/output, manual topology edits, AGENTS.md edits, fleet-setup.md edits, seed, --server, --first-machine, or --force were used. The provisioning kit topology will be removed after the final checks; local archives and logs are retained.

The repository MCP goal resource and ticket lifecycle tools are not available in this Codex session (tool and resource discovery found no repo server). This user-requested join record is therefore written directly inside the explicitly named existing ticket.
