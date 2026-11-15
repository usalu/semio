# Worker Recovery After 08:41 (g1-m01, g1-m02, g1-m03, g1-m04, g1-m06)

## What happened

- 06:15–08:25: synchronisation on g1-server was off while the workers republished their trees on old code.
- 08:41: g1-server published its newer module files. For two files (`🎣️tiers/🟦️.ts`, `🔁️replica/💾️state/🧪️tests/🟦️.ts`) a stale worker copy still won in the store at that moment.
- Every worker restarted onto that mix. The new `🔁️replica/🟦️.ts` calls `tables.dirs.adopt`, which the stale `🎣️tiers/🟦️.ts` does not have. The daemon throws while opening its state, exits, and the service manager restarts it in a loop. It never reaches the store, so it cannot fetch the corrected file by itself.
- The workers had never run the resilience code (self-check, sealed last-known-good snapshot), so they had no fallback.
- g1-server refused the same mix through its self-check and kept running. The corrected files are published (08:49).

A tested copy of the daemon's code (38 files, digest `229b2300d546`, identical to what g1-server runs) lies in the store: `<store>/v1/bootstrap/code/229b2300d546/`, laid out relative to the worktree root.

## Prompt (paste into an agent on each worker)

```
You are on a worker of the semio fleet. Its replica daemon is crash-looping since 08:41 because the module on disk is a mix of new and stale files. Repair it. Do not use any git command that changes the repository. Do not use pkill or killall. Do not edit AGENTS.md.

1. Find the worktree: /Users/Shared/semio on macOS, /srv/semio on Linux. Read <worktree>/.cache/f/machine.json and take "storeRoot" and "originId".
2. Confirm <storeRoot>/v1/bootstrap/code/229b2300d546/ exists and holds 38 files. If the store is not mounted, stop and report that.
3. Copy the code over the worktree, keeping the relative paths:
   rsync -a "<storeRoot>/v1/bootstrap/code/229b2300d546/" "<worktree>/"
4. Do not stop or start the service yourself: the service manager restarts the daemon every few seconds. Wait 90 seconds.
5. From <worktree>/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica run:
   SEMIO_WORKSPACE_ROOT=<worktree> repo ./📜️script.ts status --lang en
   It must report a running daemon and synchronisation on. If the daemon is still not running after three minutes, read the service log (macOS: `log show --last 5m --predicate 'process == "bun"'` and /var/log/semio*; Linux: `journalctl -u 'semio*' -n 80`) and report the last error verbatim. Do not try other repairs.
6. Run: SEMIO_WORKSPACE_ROOT=<worktree> repo ./📜️script.ts fleet --wait 300 --lang en
7. Write what you did, the status output and the fleet output to <worktree>/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/<originId>-recover-0841.md and report the fleet verdict.
```
