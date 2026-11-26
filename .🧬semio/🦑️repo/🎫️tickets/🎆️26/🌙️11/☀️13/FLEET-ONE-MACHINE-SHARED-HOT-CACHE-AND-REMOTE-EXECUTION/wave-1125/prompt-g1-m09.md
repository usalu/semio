# Paste-ready prompt: bring g1-m09 into the fleet

Situation: `S/origins/g1-m09/` holds only `fleet-auth.json` (registered 2026-11-14 02:13). The machine never ran provisioning or a replica daemon, so there is no `ready.json`, `attest.json` or `head.json`. Nothing on the coordinator side can start it. Someone must run this on that Mac, logged in at the screen (a consent dialog appears).

Decide first which Mac this is. If it is the Mac that was enrolled as g1-m09 on 2026-11-14 (it may already hold `~/Documents/semio` and an identity under `~/Library`), use origin `g1-m09`. If that Mac is gone or unknown, use `g1-m15` instead (the next free number; `g1-m01` to `g1-m14` and `g1-server` are taken) and tell the coordinator that `g1-m09` should be retired.

```
Join this Mac to the semio fleet as origin <ORIGIN> of group g1 (<ORIGIN> = g1-m09, or g1-m15 for a new Mac). End state: exactly ONE semio repository at ~/Documents/semio, joined, synchronising, in agreement with the fleet. Work as the login account at the screen and keep it logged in.

Rules: no git command that changes a repository or working tree. No pkill/killall. Do not edit AGENTS.md. Change no code in the repository. Do not generate, copy or print any token. Do not pass --server, --first-machine, --force or seed. Do not open a git GUI on the worktree. Delete nothing except what step 2 names.

1. Preconditions (report and stop if one fails): `sw_vers`, `sysctl -n hw.memsize`, `df -h ~` (at least 90 GiB free); `showmount -e <STORE_ADDRESS>` lists /Users/Shared/semio-store (<STORE_ADDRESS> is the current address of origin g1-server, given to you by the coordinator from `S/origins/g1-server/addresses.json`; it is a runtime value, never written into any file; if it does not answer, stop and report); the account can use sudo.
2. One repository only: look at ~/Documents/semio and /Users/Shared/semio and search the home folder and /Users/Shared to depth 4. None: continue. Only ~/Documents/semio: it is adopted in step 4. Any other or two: stop and report the paths, `git rev-parse HEAD`, `git status --porcelain | wc -l`.
3. Fetch the kit:
   mkdir -p ~/semio-setup && sudo mkdir -p /Volumes/semio-kit
   sudo mount -t nfs -o ro,resvport,nolocks <STORE_ADDRESS>:/Users/Shared/semio-store /Volumes/semio-kit
   cp -R /Volumes/semio-kit/v1/bootstrap/kit/topology ~/semio-setup/topology
   cp /Volumes/semio-kit/v1/bootstrap/kit/SHA256SUMS ~/semio-setup/
   sudo umount /Volumes/semio-kit
   (cd ~/semio-setup && shasum -a 256 -c SHA256SUMS)    every file must be OK
   Note: g1-m09 is already listed under v1/origins (registered 2026-11-14). That is expected for g1-m09 (a re-run of the same origin: keep it and say so in the report). For g1-m15 it must NOT be listed.
4. Provision:
   export SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_STORE_ADDRESS=<STORE_ADDRESS>   (read once by the script, never stored)
   Do NOT type any host name for this Mac.
   unset SEMIO_FLEET_NFS_HOST SEMIO_FLEET_NETWORK SEMIO_FLEET_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_TOKEN
   Empty machine: bash ~/semio-setup/topology/🥾️provision/🐚️.sh g1 <ORIGIN> --user "$(id -un)" 2>&1 | tee ~/semio-setup/provision.log   (no --plan first; the bootstrap archive is about 9.5 GB)
   Existing ~/Documents/semio: run the same with --plan first (tee ~/semio-setup/plan.log); stop and report if it shows more than 50 conflicts, more than 1000 paths of new local work or a git relation other than equal/ancestor; otherwise run it without --plan.
   Allow the macOS "access files in your Documents folder" dialog. Exit codes: 0 done; 3 in the fleet, native build pending (expected, do not loop); 4 consent pending (Allow, `repo preflight --lang en`, run again); 5 checkout code older than the script and no newer sealed code (report, change nothing); 1 error (report the failing step and the last 40 log lines, do not improvise).
   If the script refuses because an identity for <ORIGIN> already exists elsewhere or its key does not match the store, stop and report; the coordinator then retires g1-m09 and you re-run as g1-m15.
5. Checks from ~/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica with SEMIO_WORKSPACE_ROOT=$HOME/Documents/semio:
   repo preflight --lang en ; repo ./📜️script.ts status --lang en ; repo ./📜️script.ts verify --lang en ; repo ./📜️script.ts fleet --wait 600 --lang en (the line of <ORIGIN> must read ok)
   git --no-optional-locks -C ~/Documents/semio rev-parse HEAD
5b. Name this Mac after its origin, with the one command that derives the name from `.cache/f/machine.json` (never type a name): `cd ~/Documents/semio && repo fleet hostname apply` (it prints the planned name and asks before it runs `sudo scutil`; answer y; it does nothing when the name is already right).
6. Record machine name, addresses, macOS version, memory, free disk, case of step 2, provision exit code, status/verify/fleet output and HEAD in ~/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1125/join-<ORIGIN>.md. Remove ~/semio-setup/topology when done. Report the fleet line of <ORIGIN>.
```

Success check on the coordinator side: `S/origins/<ORIGIN>/` gains `attest.json`, `head.json`, `pool.json` and `ready.json` within a few minutes after step 4.
