# g1-m10: Join With An Existing Old Checkout (2026-11-14 23:05)

> **UPDATE 2026-11-14 22:31 UTC (23:31 local), package P10: step 3 below is no longer needed.** The provision script does it itself (step 2b of `🐚️.sh`, step 4b of `🔵️.ps1`): before its first `fleet`/`replica` call on an existing checkout it verifies the sealed code of the digest that g1-server attests, copies it over the checkout's fleet module (every replaced file that differs is kept under `.cache/f/lost/provision-code-<stamp>/`, nothing is deleted) and prints `fleet code <digest>: replaced R, added A, identical I`; `--plan` does the same and says so. The kit in the store now publishes itself from the same sealed files (receipt `published` is JSON now: `cat` it, `code` is the digest the server runs; first self-published at 22:30:53Z for digest `5f634aa0499e…`), so the manual republish is gone. A fleet command that still fails on an unknown option or a missing module now ends the script with exit code 5 and the message "the checkout's fleet code is older than this script and the store offers no newer sealed code: <path>". Fetch the kit again (step 2) so the script has the overlay; then you can skip step 3 and its check. Step 3 stays below, unchanged, as the manual fallback. Report the line `fleet code …` and the `lost/provision-code-*` folder in the join report.

Cause of the failed join (`Unknown option for facts: --home`): the provision script of the kit runs the fleet commands (`facts`, `join`, adopt) from the machine's OWN checkout. This Mac has an old checkout in `~/Documents/semio`, whose fleet module does not know what today's script passes. Not a setting of the machine. Until the script does it itself, step 3 below brings the checkout's fleet module to the fleet's current sealed code (55 files, every replaced file kept). Kit republished 2026-11-14 22:02 UTC; it matches sealed code `590fdef8b758…`.

```
Finish joining this Mac to the semio fleet as origin g1-m10 of group g1. End state: exactly ONE semio repository on this machine, at ~/Documents/semio, joined, synchronising, in agreement with the fleet; then follow the worker readiness protocol. The earlier attempt stopped at the adoption plan with "Unknown option for facts: --home": the provision script runs the fleet commands from this machine's own checkout, and that checkout is older than the script. Step 3 fixes that; it is an authorised step, not an improvisation. Discard any fallback you prepared earlier (stage-published-runtime.command, prepare-runtime-adapter.ts): do not run it.

Rules: no git command that changes a repository or working tree. No pkill/killall. Do not edit AGENTS.md. Change no code except what step 3 copies. Do not generate, copy or print any token (none is needed). Do not pass --server, --first-machine, --force or seed. Do not open a git GUI on the worktree. The owner types the sudo password in Terminal when asked; batch the sudo steps so that it is asked as few times as possible (run steps 2 to 4 in ONE Terminal session script and tell the owner once that a password prompt is waiting).

1. Facts first (read-only): `git --no-optional-locks -C ~/Documents/semio rev-parse HEAD`, `git --no-optional-locks -C ~/Documents/semio log -1 --format=%cd`, `git --no-optional-locks -C ~/Documents/semio status --porcelain | wc -l`, `git --no-optional-locks -C ~/Documents/semio stash list | wc -l`, `df -h ~` (at least 60 GiB free), and confirm no other semio repository exists (a folder holding .git, AGENTS.md and .🧬semio) in the home folder or /Users/Shared to depth 4. Write them into the join report.

2. Fetch the kit again (it was republished 22:02 UTC):
   sudo mkdir -p /Volumes/semio-kit && sudo mount -t nfs -o ro,resvport,nolocks 192.168.178.70:/Users/Shared/semio-store /Volumes/semio-kit
   rm -rf ~/semio-setup/topology && cp -R /Volumes/semio-kit/v1/bootstrap/kit/topology ~/semio-setup/topology && cp /Volumes/semio-kit/v1/bootstrap/kit/SHA256SUMS ~/semio-setup/
   (cd ~/semio-setup && shasum -a 256 -c SHA256SUMS)        every file OK
   cat /Volumes/semio-kit/v1/bootstrap/kit/published        JSON: "publishedAt" must be 2026-11-14T22:30:53Z or later (the kit publishes itself now)
   Keep the store mounted for step 3.

3. Bring the checkout's fleet module to the fleet's current code (the missing piece):
   CODE=$(python3 -c "import json;print(json.load(open('/Volumes/semio-kit/v1/origins/g1-server/attest.json'))['code'])")
   SRC="/Volumes/semio-kit/v1/bootstrap/code/$CODE/files"        must exist and hold about 55 files (find "$SRC" -type f | wc -l); if it does not exist, take the newest folder of /Volumes/semio-kit/v1/bootstrap/code that has a `files` subfolder and say so
   KEEP=~/Documents/semio/.cache/f/lost/provision-code-$(date +%Y%m%d-%H%M%S); mkdir -p "$KEEP"
   For every file under "$SRC" (paths are relative to the worktree root): if the checkout has a file at that path whose bytes differ, copy the checkout's version to "$KEEP/<same path>" first; then copy the file from "$SRC" over it (create missing folders). Copy as the login account, not as root. Count: replaced, added, identical. Do not delete anything.
   Check: `cd ~/Documents/semio && repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/📜️script.ts" facts --group g1 --platform macos --shell sh --home "$HOME" | grep -c '^FLEET_'` must print a number above 0. If `repo` is not on the PATH use ~/.local/bin/repo. If this check fails, stop and report its full output: do not go on.
   sudo umount /Volumes/semio-kit

4. Provision. In one shell:
   export SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
   unset SEMIO_FLEET_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_TOKEN
   bash ~/semio-setup/topology/🥾️provision/🐚️.sh g1 g1-m10 --user "$(id -un)" --plan 2>&1 | tee ~/semio-setup/plan.log
   Read the plan by its classes: `stale` (an older version of a fleet file: the fleet wins, your bytes go to .cache/f/lost) is expected in large numbers for an old checkout and needs no action. Stop and report only if the plan shows more than 50 `conflict` paths, more than 1000 paths of new local work (`publish` plus `edit`), or a git relation other than `equal`/`ancestor`. Otherwise run the same command without `--plan` (log to ~/semio-setup/provision.log).
   If the plan or the run fails with another "Unknown option" or "not found" from a fleet command, the cause is the same kind (a file outside the 55 that is too old): report the exact line and the command it came from, and stop.
   macOS consent "repo would like to access files in your Documents folder": Allow, if the owner is at the screen. If nobody answers, go on: the commands run under your own app's access (one line says so); it is not a failure.
   Exit codes: 0 done. 3 = in the fleet and synchronising, native build pending: EXPECTED, continue. 4 = neither `repo` nor your process can read the worktree: report the text and stop. 1 = error: report the failing step and the last 40 lines.

5. Checks, from ~/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica with SEMIO_WORKSPACE_ROOT=$HOME/Documents/semio:
   repo preflight --lang en
   repo ./📜️script.ts status --lang en          (daemon running, synchronisation ON)
   repo ./📜️script.ts verify --lang en
   repo ./📜️script.ts fleet --wait 600 --lang en      (the line of g1-m10 must read ok; other machines may lag)
   git --no-optional-locks -C ~/Documents/semio rev-parse HEAD
   If the replica agent is not running (`launchctl print gui/$(id -u)/tech.semio.fleet.replica` shows `state = not running`): `launchctl kickstart -k gui/$(id -u)/tech.semio.fleet.replica`, wait a minute, check again.

6. Update ~/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m10-join.md: the facts of step 1, the counts of step 3 and the folder that keeps the replaced files, the plan summary per class, provision exit code, adoption counts and the adopt report path (.cache/f/lost/adopt-*.md), status/verify/fleet output, HEAD, services (`launchctl print gui/$(id -u) | grep semio`). Remove ~/semio-setup/topology and the unused fallback files; keep the logs.

7. Then, without stopping: read ~/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/PROTOCOL.md and follow it until it tells you to stop (it exists once the replica has synchronised; if it is absent, wait two minutes and look again, at most 15 times, then report). From here on you change no code and no file outside your own workers/g1-m10/ folder.
```
