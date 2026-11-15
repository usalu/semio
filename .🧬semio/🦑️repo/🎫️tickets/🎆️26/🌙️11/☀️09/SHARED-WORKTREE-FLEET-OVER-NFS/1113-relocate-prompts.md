# Paste-ready Prompts: Relocate Or Adopt Into The Existing Repository (2026-11-13)

Facts: `1113-fleet-setup-existing-repo.md`. Canonical worktrees: macOS `~/Documents/semio`, Linux `~/semio`, Windows `C:\semio`. Addresses: `SEMIO_FLEET_GROUP=g1`, `SEMIO_FLEET_NFS_HOST=<g1 host>` and `SEMIO_FLEET_NETWORK=<g1 worker CIDR>` as in `address-environment.md` and `1113-worker-prompts.md`; `SEMIO_FLEET_CACHE_ENDPOINT` unset; `SEMIO_LANGUAGE=en`. The landing of the new module must be in `1113-landings.md` and the machine must have received it (module files present, sources settled) before any of these.

Common rules for every prompt: no git command that changes a repository (no checkout, reset, clean, stash, merge, rebase, pull; the existing repository at the canonical path is read, adopted and left as it is), no `pkill`/`killall`, never delete the old worktree, never edit `fleet-setup.md`, no secrets in records, results go to the ticket folder.

## (a) Relocate an already joined Mac worker from /Users/Shared/semio to ~/Documents/semio

```
Move this Mac, an already joined worker of the semio fleet, from /Users/Shared/semio to its existing repository at ~/Documents/semio. Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-fleet-setup-existing-repo.md` and the module README sections "Existing repository, adopt and relocate" and "macOS privacy". Fill in: origin <e.g. g1-m02>, account <login account>. The origin id stays the same.

1. Preconditions: the account is logged in at the screen (the agents need the session). Both trees contain the new module (the directory 🔐️access exists beside 🔁️replica). Copy the topology directory of ~/Documents/semio to a temporary folder outside both trees.
2. Dry run, record the output: `bash <copy>/provision.sh g1 <origin> --user <account> --plan`. It prints `relocate --dry-run`: counts, decisions, git relation. Adopt decides by content (`stale` = fleet wins, expected; `edit`/`publish` = local work, published; `conflict` = both sides changed, fleet's version stays and the local one is kept in .cache/f/lost). Stop and report, changing nothing, if the plan lists more than 50 `conflict` paths or more than 1000 paths of new local work (`publish` plus `edit`), if the git relation is `unrelated`, or if the plan fails; paste the conflict list into the record.
3. Real run: `bash <copy>/provision.sh g1 <origin> --user <account>` (it stops the old services, runs relocate prepare and move, installs the LaunchAgents at the new path, and runs `relocate --phase finish --wait 120`). When macOS asks "repo would like to access files in your Documents folder", answer Allow; exit code 4 means the consent is still pending: accept it, then `preflight --lang en` and rerun the same command.
4. Verify with the same SEMIO_* exports, from ~/Documents/semio/<replica directory>: `repo ./📜️script.ts status --lang en`, `verify --lang en` (exit 0) and `fleet --wait 120 --lang en`; `preflight --lang en` from the topology directory.
5. Never delete /Users/Shared/semio: it is retired (marker `.cache/f/relocated.json`, sync off) and stays until the owner deletes it.
6. Record in the ticket folder as `<origin>-relocate.md`: the plan, the lost report path (`.cache/f/lost/adopt-*.md`), exit codes, status, verify and fleet output.
```

## (b) Relocate the Linux worker from /srv/semio to ~/semio

```
Move this Linux machine, the joined worker g1-m04 of the semio fleet, from /srv/semio to ~/semio (the home of the login account). Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-fleet-setup-existing-repo.md`. Origin g1-m04 stays, account <login account>.

1. If ~/semio does not exist yet, it is created from the bootstrap archive; if it holds the account's own repository it is adopted. Copy the topology directory (from /srv/semio now, or from ~/semio if it exists) to a temporary folder outside both trees.
2. Dry run, record it: `bash <copy>/provision.sh g1 g1-m04 --user <account> --plan`. Stop and report if the plan lists more than 50 `conflict` paths or more than 1000 paths of new local work, or the git relation is `unrelated` (`stale` files are expected).
3. Real run: `bash <copy>/provision.sh g1 g1-m04 --user <account>` (it stops the systemd units, relocates, rewrites the unit working directory to ~/semio, restarts, and finishes with verify and fleet).
4. Verify: `status`, `verify` (exit 0), `fleet --wait 120`, `preflight --lang en` with the same exports.
5. Never delete /srv/semio. Record `g1-m04-relocate.md` in the ticket folder.
```

## (c) Join a new machine whose existing repository is at the canonical path

```
Join this machine to the semio fleet as origin <next unused origin: list v1/origins in the store first, never reuse an id>, account <login account>, OS <macOS | Linux | Windows>. The existing repository of the account is at the canonical path (macOS ~/Documents/semio, Linux ~/semio, Windows C:\semio) and becomes the worktree: it is adopted, nothing is refused, decided by content (see the rules above); losing versions are kept in .cache/f/lost. Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-fleet-setup-existing-repo.md`; the join steps of `1113-worker-prompts.md` (b)/(c) apply for tokens, native build and exit codes.

1. The repository must contain the fleet module (update it first if it does not: the script says so). Copy the topology directory to a temporary folder outside it. Export the SEMIO_* variables together.
2. Dry run, record it (macOS/Linux `bash <copy>/provision.sh g1 <origin> --user <account> --plan`; Windows elevated `provision.ps1 g1 <origin> -User "$env:USERDOMAIN\$env:USERNAME" -Plan`). Stop and report if the plan lists more than 50 `conflict` paths or more than 1000 paths of new local work, or the git relation is `unrelated` (use --force/-Force only after the owner agrees); `stale` files are expected.
3. Real run without --plan. macOS: be logged in at the screen and Allow the one prompt "repo would like to access files in your Documents folder"; exit 4 = consent pending. Windows: keep the account logged on.
4. Then `preflight --lang en`, `status`, `verify` (exit 0), `fleet --wait 120`. Exit 3 (native build pending): fix the cause, rerun the same command.
5. Do not run any history-changing git command on the repository. Record `<origin>-join.md` with the plan, the adopt report path, exit codes and outputs.
```

## (d) The group server (this Mac: store, cache, relay) relocates to ~/Documents/semio

```
Move the group server g1-server (this Mac; it also hosts the NFS store /Users/Shared/semio-store, the cache and the relay) from /Users/Shared/semio to its existing repository ~/Documents/semio. The store stays; origin g1-server stays. Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-fleet-setup-existing-repo.md` and the module README "macOS privacy".

Facts you must respect: on macOS the replica, compact, relay and cache run `repo` and become per-user LaunchAgents of the account (the store mounts, NFS export and peer mounts stay LaunchDaemons). That needs (1) a logged-in session at all times: turn on automatic login for the account (System Settings > Users & Groups > Automatically log in) and do not log out; (2) one consent: the first agent start asks "repo would like to access files in your Documents folder": Allow in the session (the consent belongs to the launcher and survives runtime updates). Other machines are served by the cache and relay only while the agents run: do the move at a quiet time and announce it. The tokens file /etc/semio/remote-cache.env stays.

1. Dry run, record it, change nothing: `bash <copy>/provision.sh g1 g1-server --user <account> --server --plan` (copy of the topology directory outside both trees; --archive is not needed because the new tree already holds a checkout).
2. Stop and report if the plan lists more than 50 `conflict` paths or more than 1000 paths of new local work, the git relation is `unrelated`, or `fleet` is not ok before you start. This Mac's plan is in `1113-adopt-plan-g1-server.md`.
3. Real run: the same command without --plan. It removes the system daemons tech.semio.fleet.{replica,compact,relay,cache}, installs the four LaunchAgents at ~/Library/LaunchAgents, relocates, restarts and finishes. Exit 4 means the consent is pending: Allow it, then `preflight --lang en` and rerun the command.
4. Verify: `status`, `verify` (exit 0), `fleet --wait 180` (every other machine must stay ok), `repo remote-cache status`, `showmount -e localhost`; every worker's `fleet` row stays fresh.
5. Never delete /Users/Shared/semio; keep it until the owner says so. Record `g1-server-relocate.md`.
```
