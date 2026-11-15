# Server Relocation: g1-server moves to `~/Documents/semio` (request for a go, 2026-11-14)

From the session "g1-server canonical" (report: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-server-canonical.md`).

## Status

**Not switched yet (state 2026-11-15 00:40).** Ready for the go. Open: the coordinator's `go`, then the owner at the keyboard for one script with sudo and the consent clicks. The owner's two decisions are answered: the three linked worktrees are deleted (60.3 GiB freed), automatic login is the owner's own step after the switch.

## Path Mapping (valid from the switch on)

| before | after |
|---|---|
| `/Users/Shared/semio` (live fleet tree, origin `g1-server`) | `/Users/ueli/Documents/semio` (same tree, same origin, renamed) |
| `/Users/ueli/Documents/semio` (old pre-fleet checkout) | `/Users/ueli/Documents/semio.old-20261114` (renamed aside, deleted after verification) |
| `/Users/Shared/semio-store` (group store) | unchanged |

After the switch nothing exists at `/Users/Shared/semio`: no symlink, no copy. Every brief, script, lock path and running command that names `/Users/Shared/semio` must use `/Users/ueli/Documents/semio`. Ticket folders move with the tree, so the two landing locks are then at `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-landing.lock` and `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landing.lock`.

## Planned Switch Time

As soon as the go is given and the owner is at the keyboard. Expected outage of the server (replica, relay, build cache 6094, scheduler 8980/8981, graph server): 4 to 8 minutes for the script (stop, detach ten images, two renames, re-bind check, provision), plus the time the owner needs for the clicks "Allow" (Terminal, then `repo`); the replica daemon and with it the sidecars start only after the click for `repo`.

## What The Coordinator Must Ensure Before Saying Go

1. No landing lock is held (the script refuses otherwise): at 11:52 both were held (`claude-package-AD` since 11:43, `N4` since 11:50).
2. No agent runs a command, test, build or daemon in `/Users/Shared/semio` on this Mac at the switch. The script stops every process that still holds a file in the tree (plain `kill` of each pid, `kill -KILL` after 12 s), including test daemons, `semio-repo-watch` and shells whose current directory is inside the tree. Work on the workers is not touched.
3. Agents on this Mac resume with the new path only after the note "switched" is appended here.

## What Changes Besides The Path

- **Services:** the four system LaunchDaemons are retired; replica, relay and compact become LaunchAgents of `ueli`, rendered by the current provision script (7eb0cc06e2fb at 00:30; the script refuses a stale copy of the topology directory). They start at login, not at boot. Logs move from `/var/log/semio-fleet-*.log` to `~/Library/Logs/semio-fleet-*.log`.
- **Launcher and consent:** the script first runs the current launcher installer (this Mac still has the old single file `launcher/bin/repo`, identifier `a.out`, from 12:38; the new one is `launcher/bin/versions/<hash>/repo` behind the link, signed `tech.semio.repo`). A new file loses the old consent, so the owner clicks "Allow" once for `repo` during the script; the script waits up to 10 minutes for the replica daemon and for the marker `~/Library/Application Support/semio/access/<hash>/launcher.json` and prints the checks of `ROUND-NOTE-P6.md` (launcher link, identifier, `bootVersion`, loaded jobs, listeners, graph-server lines).
- **Old cache settings removed:** `/etc/zshenv` and `/etc/profile` lose `SEMIO_REMOTE_CACHE_ENDPOINT` (port 6090), `SEMIO_REMOTE_CACHE_ACCESS` and `SEMIO_REMOTE_CACHE_INSTANCE` (`semio`): the current provision script writes only the three `SEMIO_FLEET_*` address lines into the block. The rendered agents carry no `SEMIO_REMOTE_CACHE_*` either. Nothing is set through `launchctl setenv` today (checked). Shells that are already open keep the old values until they are restarted.
- **Old cache server on 6090 removed from this Mac:** `tech.semio.fleet.cache` is the pre-fleet-facts server (its definition: `repo remote-cache serve --port 6090`, instance `semio`, no client connected at 00:20); the fleet facts `v1/fleet/cache.json` name the sidecar `http://192.168.178.70:6094`, instance `main`. The script unloads the job and removes its agent file after the provision step, only when the facts name 6094. **Open point for the coordinator:** the topology still renders this job for every group server (`server-plan --part cache-unit`, step 6b of both provision scripts, `FLEET_CACHE_ENDPOINT`/`FLEET_CACHE_INSTANCE` in the facts, README), so every later provision run of a server installs it again. Removing it from the topology is a change across all groups and both provision scripts; I did not make it.
- **Build caches: kept through a re-bind, cold only as the fallback.** As before: hint `.cache/b/image-relocation.json`, per partition `bound`/`unbound`, refused partitions set aside, whole folder set aside only when `repo cache status` is refused by the image layer. Not re-bound: state inside the images that names the old absolute path (dependency generations, cargo fingerprints).
- **User links:** `~/.local/bin/go` and `gofmt` point into `/Users/Shared/semio/.cache/b/t/m`; the script points them below the new path. `/usr/local/bin/repo` is linked to the launcher by the provision script.
- **Graph server:** it runs in the build-agent sidecar of the replica daemon, so it is down from the stop until the daemon runs again at the new path. Verification after the switch: `/Users/ueli/Documents/semio/.cache/f/sidecars/build-agent.log` shows `graph-server: alive … machines seen` and a `serving … for g1-m0x` line within minutes; the published facts carry `publisher.directory` below the new path.
- **Graph service jobs of `ueli`** (`tech.semio.repo.service.*`, `tech.semio.repo.probe.*`) are unloaded by the script; the next `repo` command starts them again for the new path.

## Answer Expected

One line: `go` (with the earliest time), or what has to finish first.
