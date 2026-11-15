# New Mac Worker: Clean Join With One Canonical Repository (2026-11-14, revised 23:31)

Facts: `1113-fleet-setup-existing-repo.md`, `g1-m07-join.md`. The provisioning kit (the topology directory with the provision script) lies in the store: `v1/bootstrap/kit/` (`topology/`, `SHA256SUMS`, `published`), published by the group server itself from its sealed code (since 2026-11-14 22:30 UTC no hand publication: `published` is a JSON receipt whose `code` is the digest the server runs, `SHA256SUMS` and the files equal the sealed files of that digest). Origins taken on 2026-11-14 16:50: `g1-m01` … `g1-m09`, `g1-server`. Not yet run in this form.

Use `g1-m10` for the next new Mac, `g1-m11` for the one after. Replace `<origin>` below. (`g1-m08` registered at 04:58 and has been silent since about noon: if that is the machine you are setting up again, keep `g1-m08` and say so in the report.)

```
Join this Mac to the semio fleet as origin <origin> of group g1. End state: exactly ONE semio repository on this machine, at ~/Documents/semio, joined, synchronising, in agreement with the fleet. Work as the login account at the screen (a consent dialog will appear) and keep that account logged in.

Rules: no git command that changes a repository or working tree. No pkill/killall. Do not edit AGENTS.md or fleet-setup.md. Change no code in the repository: you set the machine up and report; fixes are made in one place by the coordinator. Do not generate, copy or print any token (the build cache needs none: machines are authorised by the fleet itself). Do not pass --server, --first-machine, --force or seed. Do not edit the topology json. Do not open a git GUI (GitKraken and the like) on the worktree. Delete nothing except what step 2 names.

1. Preconditions, report and stop if one fails:
   - `sw_vers`, `sysctl -n hw.memsize`, `df -h ~` : at least 90 GiB free on the home volume.
   - `showmount -e 192.168.178.70` lists `/Users/Shared/semio-store` for this network.
   - The account can use sudo (the script calls `sudo -E` itself; the owner types the password when asked).

2. One repository only. A semio repository is a folder holding `.git`, `AGENTS.md` and `.🧬semio`. Look at `~/Documents/semio`, `/Users/Shared/semio`, and search the home folder and `/Users/Shared` to depth 4 for others.
   - None anywhere (expected on a new machine): continue with step 3; `~/Documents/semio` must not exist or be empty.
   - `~/Documents/semio` is the only one: it will be adopted in step 4 (nothing is refused; differences are decided by content and every replaced file is kept under `.cache/f/lost/`).
   - A semio repository exists anywhere else, or two exist: stop and report the paths, their `git rev-parse HEAD`, `git status --porcelain | wc -l` and `git stash list | wc -l`. Do not move, merge or delete anything.

3. Fetch the provisioning kit (the machine has no checkout yet):
   mkdir -p ~/semio-setup && sudo mkdir -p /Volumes/semio-kit
   sudo mount -t nfs -o ro,resvport,nolocks 192.168.178.70:/Users/Shared/semio-store /Volumes/semio-kit
   cp -R /Volumes/semio-kit/v1/bootstrap/kit/topology ~/semio-setup/topology
   cp /Volumes/semio-kit/v1/bootstrap/kit/SHA256SUMS ~/semio-setup/
   ls /Volumes/semio-kit/v1/origins          (confirm <origin> is NOT listed, unless you are re-running g1-m08; if it is, stop and report)
   sudo umount /Volumes/semio-kit
   (cd ~/semio-setup && shasum -a 256 -c SHA256SUMS)    must report OK for every file
   Always run the kit's script, not the one in the checkout: if `~/Documents/semio` already holds the repository (second case of step 2) its fleet code may be older than the script. The script brings the checkout's fleet code to the sealed code of the fleet by itself (next step), whatever age the checkout has.

4. Provision. In one shell:
   export SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
   unset SEMIO_FLEET_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_TOKEN
   - Empty machine (first case of step 2): run the real command directly. Do NOT run `--plan` first: on an empty worktree it extracts the archive and the next run would then adopt it instead of settling it.
       bash ~/semio-setup/topology/🥾️provision/🐚️.sh g1 <origin> --user "$(id -un)" 2>&1 | tee ~/semio-setup/provision.log
     The bootstrap archive is about 9.5 GB; the copy over the network takes some minutes.
   - `~/Documents/semio` already holds the repository: dry run first, then the real run.
       bash ~/semio-setup/topology/🥾️provision/🐚️.sh g1 <origin> --user "$(id -un)" --plan 2>&1 | tee ~/semio-setup/plan.log
     Stop and report if the plan shows more than 50 conflicts, more than 1000 paths of new local work, or a git relation other than `equal`/`ancestor`. Otherwise run the same command without `--plan`.
   Both runs (also `--plan`, and the extraction of the archive, which is older than the sealed code) print `step 2b` and then `fleet code <digest>: replaced R, added A, identical I`: the sealed fleet code of the store was copied over the checkout, every replaced file that differed is kept under `.cache/f/lost/provision-code-<stamp>/`. Put that line and the folder into the join report. If the store has no sealed code or it is not intact the script warns and goes on with the checkout's own code.
   When macOS asks "repo would like to access files in your Documents folder": Allow.
   Exit codes: 5 = the checkout's fleet code is older than the script and the store offers no newer sealed code (message names the checkout): report the message and the `fleet code` line, change nothing, the coordinator decides. 0 done. 3 = the machine is in the fleet and synchronises, the native build is pending: this is EXPECTED today (the native build is being fixed and is driven separately, see step 7). Do not retry the native build in a loop; note it and continue. 4 = the consent is pending: Allow it, run `repo preflight --lang en`, run the command again. 1 = error: report the failing step and the last 40 lines of the log; do not improvise repairs.

5. Checks, from `~/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica` with `SEMIO_WORKSPACE_ROOT=$HOME/Documents/semio`:
   repo preflight --lang en
   repo ./📜️script.ts status --lang en          (daemon running, synchronisation ON; if it says off, report it and do not switch it yourself)
   repo ./📜️script.ts verify --lang en
   repo ./📜️script.ts fleet --wait 600 --lang en      (the line of <origin> must read ok; other machines may be stale or lagging)
   git --no-optional-locks -C ~/Documents/semio rev-parse HEAD        (must print a commit)
   git --no-optional-locks -C ~/Documents/semio status --porcelain | wc -l
   If `rev-parse HEAD` fails (objects missing): report it with the output of `status`; do not repair by hand, the fleet repairs it.
   Confirm again that `~/Documents/semio` is the only semio repository on the machine (step 2 search) and that `/Users/Shared/semio` does not exist.

6. Record everything in `~/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/<origin>-join.md`: machine name, addresses, macOS version, memory, free disk, which case of step 2 applied, provision exit code, the adoption counts if any, status/verify/fleet output, the HEAD commit, the services installed (`launchctl print gui/$(id -u) | grep semio`), anything that stopped you. Remove `~/semio-setup/topology` when done; keep the logs. Report the fleet line of <origin>.

7. Then become a worker of the readiness rounds: read `~/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/PROTOCOL.md` and follow it exactly until it tells you to stop. You run the native bootstrap when a round says so and report into that folder; you change no code and no file outside your own `workers/<origin>/` folder.
```
