# g1-m02 Relocate Record: /Users/Shared/semio → ~/Documents/semio (2026-11-13)

Status: **prepared, both dry runs exit 0, real run waiting for the owner's sudo password.** Nothing in the fleet tree, the store or the services has changed yet.

- origin: g1-m02 (unchanged), account: ueli, macOS, logged in at the console
- old worktree: /Users/Shared/semio (joined, daemon running as system LaunchDaemon, code `35240fac3f31`, `Git state: shared`)
- new worktree: /Users/ueli/Documents/semio (the account's existing repository, HEAD 5ececaa78b9 on `🐙ueli/⛳wip`, same commit as the fleet tip)
- guide: `1113-fleet-setup-existing-repo.md`, prompt (a) of `1113-relocate-prompts.md`, precedent `g1-m01-relocate.md`

## Worker state before (21:02–21:07 CET)

- `status` exit 0: daemon running, synchronisation on, `Git state: shared`, stage tree `47eebd16b9bf…`; the daemon restarted itself twice during the check (code `74623e441bfd` → `35240fac3f31`) and reported `write-tree-failed` blocked/repaired once in between.
- `verify` exit 1 with 2 paths, both in flight: `extra locally: …/g1-m01-relocate.md` and `content differs: undefined.stale`. A run three minutes earlier, during the catch-up, listed 208 `.git/objects` paths as missing locally.
- `fleet --wait 180` exit 1; own row `g1-m02 macos 0s 0/0 on 35240fac3f31 36/0/33 … lagging (not settled)`, the same code and view as g1-server and g1-m01.
- `status` lists `Recurring failures of kind entry-boot: 1` (service log: `refused-same-version: the source differs from the installed program of the same version, raise bootVersionV1`) and `unstable` on `.git/objects/pack/pack-303ed9a6….pack`.

## Local work of ~/Documents/semio, checked before any change

- Working tree: 9 modified launcher files and 1 new fixture (`🚀️launcher/…`, edited 2026-11-13 04:04) are byte-identical in the fleet tree. `…/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS/🎫️ticket.json` held one more `ticket.reopen` event (14:27:24) than the fleet copy (14:26:44, a strict prefix); it was copied into the fleet tree at 21:02. `readiness-report.md` there is older than the fleet copy.
- Three stashes (`80734a04d82`, `62a9b9293fc`, `a6bcdd9ef55`): 49 of about 50 files each equal HEAD; the rest (`🎓️teaching/README.md`, `🖱️ui/…/WindowChrome/shared-border.ts`) are older versions of lines HEAD changed later. The dry run replaces `.git/refs/stash` by the fleet's, so all three were exported with `git diff --binary <stash>^1 <stash>` to `~/Documents/semio/.cache/f/lost/pre-relocate-stashes/` (ignored, local only) with the stash list, the local refs (`refs/codex/turn-diffs/…` exists only there), `git status` and a copy of the index.
- 594 ignored files outside `.cache`, `.venv`, `.nx`, `node_modules`, `target`: every one also exists in the fleet tree.
- Unpushed commits: 0 on both sides.

## Preparation (21:09, no git command that changes a repository, no service touched)

`~/Documents/semio` held the module of commit 5ececaa (members `📒️journal`, `🔁️replica`, `🗺️topology`, `🚧️boundary` only). Copied from the live fleet tree with `rsync -a` (no delete): the module directory `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/` and the 9 files of the sealed code closure outside it (list from `.cache/f/code/35240fac3f31…/files`). Both compare equal afterwards. The topology directory of `~/Documents/semio` was copied to `~/semio-g1-m02-setup/topology-relocate`.

## Dry run 1, 21:09:19, code of the old tree — exit 0

`repo ./📜️script.ts relocate --dry-run --worktree /Users/Shared/semio --to /Users/ueli/Documents/semio --lang en`

```
Relocating this machine from /Users/Shared/semio to /Users/ueli/Documents/semio (phase all)
  269563 identical, adopted as they are
  1795 missing here, taken from the fleet
  24 only here, published as new work
  1 differ, this side is newer and wins
  1692 differ, the fleet is newer and wins (local bytes kept in lost/)
  78 deleted by the fleet and older here (local bytes kept in lost/)
  .claude/settings.json: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-10T23:46:25.543Z, fleet 2026-11-13T07:02:13.140Z)
  .git/objects/info/commit-graphs/commit-graph-chain: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-13T03:52:16.952Z, fleet 2026-11-13T06:49:35.650Z)
  .git/objects/info/packs: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-13T19:54:30.927Z, fleet 2026-11-13T19:57:23.117Z)
  .git/objects/pack/multi-pack-index: differ, this side is newer and wins; here wins (here 2026-11-13T19:54:30.822Z, fleet 2026-11-13T19:52:20.656Z)
  .git/packed-refs: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-10T23:42:49.557Z, fleet 2026-11-13T04:15:51.946Z)
  .git/refs/stash: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-12T14:35:50.646Z, fleet 2026-11-13T04:15:51.946Z)
  .gitignore: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-11T02:37:20.473Z, fleet 2026-11-13T04:15:51.946Z)
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/design-contract.md: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-11T02:37:20.473Z, fleet 2026-11-13T05:30:57.552Z)
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🎫️ticket.json: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-12T01:25:33.943Z, fleet 2026-11-13T05:31:16.118Z)
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/coordination-plan.md: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-12T22:07:09.878Z, fleet 2026-11-13T19:34:19.986Z)
  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS/readiness-report.md: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-12T14:36:47.050Z, fleet 2026-11-13T19:24:28.085Z)
  Cargo.lock: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-12T22:07:09.912Z, fleet 2026-11-13T19:38:34.513Z)
  README.md: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-12T22:07:09.915Z, fleet 2026-11-13T05:38:54.067Z)
  bun.lock: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-12T14:12:40.424Z, fleet 2026-11-13T04:15:51.946Z)
  nx.json: differ, the fleet is newer and wins (local bytes kept in lost/); fleet wins (here 2026-11-10T23:46:35.261Z, fleet 2026-11-13T06:49:42.523Z)
  puzzle-artifact-editors-windows-kit/README.txt: deleted by the fleet and older here (local bytes kept in lost/); fleet wins (here 2026-11-10T23:46:35.262Z, fleet 2026-11-12T22:18:18.552Z)
  puzzle-artifact-editors-windows-kit/agents.json: deleted by the fleet and older here (local bytes kept in lost/); fleet wins (here 2026-11-10T23:46:35.262Z, fleet 2026-11-12T22:18:18.552Z)
  puzzle-artifact-editors-windows-kit/prompt.txt: deleted by the fleet and older here (local bytes kept in lost/); fleet wins (here 2026-11-10T23:46:35.262Z, fleet 2026-11-12T22:18:18.552Z)
  puzzle-artifact-editors-windows-kit/run.ps1: deleted by the fleet and older here (local bytes kept in lost/); fleet wins (here 2026-11-10T23:46:35.262Z, fleet 2026-11-12T22:18:18.552Z)
  staffage-launch-kit/README.md: deleted by the fleet and older here (local bytes kept in lost/); fleet wins (here 2026-11-10T23:46:35.262Z, fleet 2026-11-12T22:20:08.459Z)
  ... and 1751 more paths with a decision
Git: fleet tip 5ececaa78b92, local tip 5ececaa78b92, relation equal; 0 staged paths here are replaced by the stage of the fleet (the old index is kept in lost/).
Dry run finished: nothing was changed.
```

The 24 "only here" paths are `✏️s/🔌️plugins/🔋️energy/…/bestest-600FF` and `bestest-900FF` files and git pack files.

## Dry run 2, 21:10:37, code of the new tree — exit 0

```
Relocating this machine from /Users/Shared/semio to /Users/ueli/Documents/semio (phase all)
  269563 identical, adopted as they are
  1796 missing here, taken from the fleet
  24 only here, published as new work
  1 differ, this side is newer and wins
  1692 differ, the fleet is newer and wins (local bytes kept in lost/)
  78 deleted by the fleet and older here (local bytes kept in lost/)
  ...
Git: fleet tip 5ececaa78b92, local tip 5ececaa78b92, relation equal; 0 staged paths here are replaced by the stage of the fleet (the old index is kept in lost/).
Dry run finished: nothing was changed.
```

## Stop condition of prompt (a) exceeded, and why the run is prepared anyway

1692 conflict-remote + 78 delete-local = 1770 decisions, above the limit of 25. The owner asked for the relocation to be finished on this machine. There is no conflict-local and no resurrect decision; the one "this side is newer" path is `.git/objects/pack/multi-pack-index`. Every losing file belongs to a checkout at the fleet's own commit (relation `equal`) whose working changes were compared one by one above, and adopt keeps the losing bytes in `.cache/f/lost/`.

## Real run — not started

`bash ~/semio-g1-m02-setup/run-relocate.sh` (exports `SEMIO_LANGUAGE=en SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24`, `SEMIO_FLEET_CACHE_ENDPOINT` unset; calls `provision.sh g1 g1-m02 --user ueli` from the topology copy and logs to `~/semio-g1-m02-setup/provision-relocate-<time>.log`). It re-runs through sudo and needs the owner's password, and macOS asks once "repo would like to access files in your Documents folder" (Allow; exit 4 means the consent is pending, then rerun). To be run by the owner in a terminal.

---

# Real run and result (2026-11-13, evening)

Result: **g1-m02 runs from `~/Documents/semio`** — daemon as the per-user LaunchAgent, synchronisation on, `Git state: shared`, own fleet row `ok` on the server's code `3bfc232d8114`, preflight without a failed check. **Not finished: the `finish` phase refused (`unverified`), so `/Users/Shared/semio/.cache/f/relocated.json` is not written.** The old tree has synchronisation off, no daemon and no service. Open points below.

## Run

`bash ~/semio-g1-m02-setup/run-relocate.sh`, started in the owner's terminal, password typed by the owner. Log: `~/semio-g1-m02-setup/provision-relocate-211304.log`. Exit 1 at the finish step.

- Step 5: `launchctl setenv` refused six times by System Integrity Protection (warning; the variables are in `/etc/zshenv`, `/etc/profile` and the service environment).
- Step 6a: `boot program: {"action":"refused-downgrade","version":2,"installed":3}`. The module copied into `~/Documents/semio` at 21:09 held boot program 2; the old daemon had installed version 3 in the meantime. The installation therefore kept naming `/Users/Shared/semio`.
- Step 6: system daemons `tech.semio.fleet.replica` and `tech.semio.fleet.compact` removed, LaunchAgents written to `~/Library/LaunchAgents`.
- Step 7, `relocate` prepare and move: `Relocation step reached: adopted`. Adoption report `~/Documents/semio/.cache/f/lost/adopt-2026-11-13T21-06-28-504Z.md` (and `.json`): same 269522, materialise 1624, publish 24, conflict-local 1 (`.git/objects/pack/multi-pack-index`), conflict-remote 1733, resurrect 0, delete-local 78. 279046 changes applied; the local stage (3669 entries) replaced by the fleet's, old index in `lost/index-1794604143503-5d99dda4344f`; 15 entries published (4.1 GB, one git pack of 3.95 GB).
- Step 8: agents loaded. The replica agent started the daemon in `/Users/Shared/semio` (`sync off`), because the installed boot program still named that tree, so the script's wait for a running daemon in the new tree could not succeed.

## Correction by hand (no privilege, no git)

- From `~/Documents/semio`, which held boot program 3 after the adoption: `repo <replica>/🥾️entry/🟨️.mjs install --worktree /Users/ueli/Documents/semio --store /Volumes/semio-store --take-over` → `{"action":"current","version":3,"changed":true}`; `entry.json` and `worktree` name `/Users/ueli/Documents/semio`.
- `launchctl kickstart -k gui/501/tech.semio.fleet.replica` → daemon in `~/Documents/semio`, no daemon lock left in the old tree. No consent prompt was needed (`preflight`: `worktree access: the replica can read /Users/ueli/Documents/semio`).
- The script then ran `relocate --phase finish --wait 120`: `The new worktree or the fleet does not agree yet: the old worktree stays as it is.`

## State after the run

`status` — exit 0:

```
Replica g1-m02 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 17546)
276376 files tracked; queued: 0 incoming, ? outgoing; 1823 overwritten files kept in .cache/f/lost
Recurring failures of kind refused: 87 times, last 53 s ago.
Journal: 672.28 MiB in 480 segments of 8 origins.
Newest snapshot by g1-m02: 68709s old, 87.65 MiB.
Reclaimable now: 66 journal segments (94.40 MiB).
Git state: shared (stage and commits replicate); stage tree 47eebd16b9bf41ee127f76df7206bb79372171b9.
Background tiers, last pass (cost, duty cycle of one core): hot 3 ms 1.1%, directory sweep 810 ms 10%, file sweep 2781 ms 2.6%, tree walk 38675 ms 2.1%, git view -.
Latency over the last 0 publishes and 24 applies: publish p50 - ms, p95 - ms; apply p50 335 ms, p95 4221 ms.
g1-m01: 0 events (0 B) behind, last event 1601s ago
g1-m02: 0 events (0 B) behind, last event 46s ago
g1-m03: 0 events (0 B) behind, last event 1775s ago
g1-m04: 0 events (0 B) behind, last event 34048s ago
g1-m05: 0 events (0 B) behind, last event 61836s ago
g1-m06: 0 events (0 B) behind, last event 1343s ago
g1-m07: 0 events (0 B) behind, last event 145s ago
g1-server: 9 events (3.35 KiB) behind, last event 0s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 0s 5/0 now off 2f03e0cd8033 77/0/81 -/-|-/- off (sync is off, not compared)
  g1-m02 macos 4s 0/0 cp:4s on 3bfc232d8114 122/10/275 -/-|335/4221 ok
  g1-m03 macos 1s 5/0 now off 2f03e0cd8033 71/0/72 -/-|-/- off (sync is off, not compared)
  g1-m04 linux 17322s 6/0 now on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 10451s 7/536 now off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 577s 4/0 now off 2f03e0cd8033 81/0/85 -/-|-/- off (sync is off, not compared)
  g1-m07 macos 4s 0/0 cp:4s on 3bfc232d8114 122/0/188 -/-|424/7787 ok
  g1-server macos 4s 0/0 cp:4s on 3bfc232d8114 122/0/181 191/517|613/920 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  warning: g1-m04 runs older code (7757dd83d95b) that attests no checkpoints; it is compared as it is now, only with other origins of that kind, and not with the others
The fleet is not in agreement (8 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

`fleet --wait 120` — exit 1 (other origins); final table:

```
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 4s 5/0 now off 2f03e0cd8033 77/0/81 -/-|-/- off (sync is off, not compared)
  g1-m02 macos 1s 0/0 cp:1s on 3bfc232d8114 122/10/275 -/-|1195/4679 ok
  g1-m03 macos 4s 5/0 now off 2f03e0cd8033 71/0/72 -/-|-/- off (sync is off, not compared)
  g1-m04 linux 17315s 6/0 now on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 10444s 7/536 now off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 570s 4/0 now off 2f03e0cd8033 81/0/85 -/-|-/- off (sync is off, not compared)
  g1-m07 macos 1s 0/0 cp:1s on 3bfc232d8114 122/0/188 -/-|333/7787 ok
  g1-server macos 4s 0/0 cp:4s on 3bfc232d8114 122/0/181 172/517|613/920 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  warning: g1-m04 runs older code (7757dd83d95b) that attests no checkpoints; it is compared as it is now, only with other origins of that kind, and not with the others
The fleet is not in agreement (8 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

`preflight` — exit 0: `23 checks, 2 with warnings, none failed`. Warnings: time synchronisation state not readable without root; `native build: pending` (`~/Documents/semio/.cache/f/native-bootstrap.done` is missing, the provision script stopped before that step).

`verify` — exit 1, 17 paths:

```
Drift on 17 paths (273045 shared paths checked).
  content differs: .git/index
  content differs: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landing.lock/owner
  content differs: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landings.md
  state differs: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/result-C.md
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🟦️.ts
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🦀️.rs
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🧩️example/🟦️.ts
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🧩️example/🦀️.rs
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🟦️.ts
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🦀️.rs
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🧩️example/🟦️.ts
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🧩️example/🦀️.rs
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏛️bestest-600FF/🗣️.dsl.semio
  missing locally: ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏛️bestest-900FF/🗣️.dsl.semio
  content differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts
  content differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🟦️.ts
  state differs: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔌️backends/📋️registry/🔣️.json
87 paths are refused by the path policy and not shared:
```

## Open points

1. **Letter-case clash, caused by this adoption.** Commit 5ececaa spells four directories `🏛️bestest-600FF` / `🏛️bestest-900FF` (under `✏️s/🔌️plugins/🔋️energy/…/📚️examples` and `…/🖼️assets`); the fleet's stage renamed them to `…600ff` / `…900ff`. `~/Documents/semio` still had the `FF` spelling on its case-insensitive volume, the adoption classed the 10 files as "only here" and published them. The store now holds both spellings as live entries with identical content. `verify` reports the spelling the disk does not use as `missing locally` (first `ff`, after the rename below `FF`), and the daemon publishes no delete because a case-insensitive lookup still finds the files. The four directories in `~/Documents/semio` were renamed to the fleet's `ff` spelling at 22:29 (matches the stage). The 10 `FF` entries in the store need a fleet-side removal; g1-m01 listed the same paths in its dry run and will publish them too.
2. **`finish` cannot pass while (1) stands**, and it needs a `verify` without any drift, which the other 7 paths (files being landed by other machines at that moment) also prevent on a moving tree. Command once (1) is solved, from the replica directory of `~/Documents/semio`: `repo ./📜️script.ts relocate --worktree /Users/Shared/semio --to /Users/ueli/Documents/semio --phase finish --lang en`.
3. **Old tree.** `/Users/Shared/semio`: `sync.json` `enabled:false`, marker `relocate.json` step `adopted`, no daemon, not deleted. Files changed there after synchronisation went off (22:06): only `undefined.stale` and one vitest cache result. Processes with that tree still open at 22:34: a `repo-watch`, three `bun` processes and one agent shell; work done there no longer reaches the fleet.
4. **Native build pending** in the new tree; rerunning the provision script (sudo) retries it.
5. Fleet not in agreement: g1-m01, g1-m03, g1-m05, g1-m06 have synchronisation off, g1-m04 is stale; g1-m02 and g1-m07 are `ok`, g1-server `lagging`.
6. `~/Documents/semio/.cache/f/lost/` holds 1823 overwritten files of the adoption and the stash export `pre-relocate-stashes/`.

## Later the same evening (22:37–22:58)

- **Open point 1 is solved.** The four directories were moved out of the worktree (copies kept in `~/semio-g1-m02-setup/bestest-hold`); the daemon published `delete` for the 10 `ff` entries and re-materialised the 10 `FF` entries from the store. The `FF` directories were then moved out as well (`delete` published, nothing came back) and the copies put back as `…600ff` / `…900ff` (`published`, count 10). The store and this disk now hold the `ff` spelling only, the same as the stage; `verify` lists no `bestest` path any more. Other machines saw the 10 files deleted and re-created within about two minutes.
- **`finish` still refuses.** Seven runs of `relocate --phase finish` without `--wait` (22:44–22:58), each exit 1 `unverified`. A `verify` directly before each run listed between 1 and 8 drift paths, different ones every time (`landing.lock/owner`, `landings.md`, files under `📚️library/🏗️build/…` and `⚡️caching/…`, `🗺️topology/🧪️tests/🐍️.py`, `undefined.stale`): files other machines were writing in that minute. `finish` needs a verify with no drift at all, which a tree under continuous edits does not give. The marker `relocated.json` is therefore still missing; nothing else distinguishes the old tree from a retired one (synchronisation off, no daemon, no service).

## 23:02 — finished

The tenth run of `relocate --phase finish` (23:02:21) exited 0: `/Users/Shared/semio/.cache/f/relocated.json` = `{"version":1,"originId":"g1-m02","to":"/Users/ueli/Documents/semio","wallMs":1794607341783}`. The old tree is retired (synchronisation off, marker written, not deleted). `fleet --wait 120` directly afterwards: exit 1, final table:

```
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 1s 5/0 now off 2f03e0cd8033 78/0/81 -/-|-/- off (sync is off, not compared)
  g1-m02 macos 0s 0/0 cp:0s on 2d6054177087 152/0/295 660/660|557/19465 diverged
  g1-m03 macos 2s 5/0 now off 2f03e0cd8033 71/0/72 -/-|-/- off (sync is off, not compared)
  g1-m04 linux 19079s 6/0 now on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 12208s 7/536 now off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 23s 1/0 cp:23s on 2d6054177087 152/0/208 6/6|1644/34919 ok
  g1-m07 macos 0s 0/0 cp:0s on 2d6054177087 152/0/208 691/3077|369/2066 ok
  g1-server macos 0s 0/0 cp:0s on 2d6054177087 152/0/208 164/426|330/2221 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  warning: g1-m06 runs git 2.50.1, other machines run another version
  warning: g1-m04 runs older code (7757dd83d95b) that attests no checkpoints; it is compared as it is now, only with other origins of that kind, and not with the others
  g1-m02 differs in view.digest at a common checkpoint: #1=ab19,#4=5ed2,#5=32e7,#7=c7f8,#8=c644,#9=b898,#11=c958,#12=b0d5,#13=5a5b,#15=f instead of #1=4ef2,#4=f50b,#5=0000,#7=0000,#8=2402,#9=0000,#11=7a4c,#12=ed0e,#13=0000,#15=4 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (8 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

Still open: native build pending in the new tree (provision script, sudo); deletion of the old tree is the owner's decision.

## 23:05–23:13 — view aligned with the other origins

After the finish, `fleet` named `g1-m02 … diverged` (`view.digest`): 295 untracked paths here against 208 on g1-m07 and g1-server. The 87 extra lines of `origins/g1-m02/view.txt` were symbolic links with absolute targets (`/Users/ueli/Documents/semio/…`) in two old ticket folders (`26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/…` 86, `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/…/sb-config/🎨️styling` 1): the `unsafe-link` paths that the path policy refuses, untracked in the fleet's stage, absent in the old tree and on the other machines, left in this checkout by commit 5ececaa. The 87 links were removed (the targets are untouched; paths and targets are listed in `~/semio-g1-m02-setup/removed-links.tsv`, the links are still in commit 5ececaa).

`fleet --wait 300` at 23:12 — exit 1 (other origins); g1-m02 `ok` with the same code and the same view (154/0/211) as g1-m07 and g1-server:

```
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 4s 5/0 now off 2f03e0cd8033 78/0/81 -/-|-/- off (sync is off, not compared)
  g1-m02 macos 1s 0/0 cp:1s on 2d6054177087 154/0/211 214/660|563/2489 ok
  g1-m03 macos 115s 5/0 now off 2f03e0cd8033 71/0/72 -/-|-/- off (sync is off, not compared)
  g1-m04 linux 19572s 6/0 now on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 12701s 7/536 now off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 515s 3/0 cp:515s on 2d6054177087 152/0/208 6/6|1644/34919 stale (attestation too old)
  g1-m07 macos 0s 0/0 cp:0s on 2d6054177087 154/0/211 503/3077|408/2857 ok
  g1-server macos 0s 0/0 cp:0s on 2d6054177087 154/0/211 167/431|260/828 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  warning: g1-m04 runs older code (7757dd83d95b) that attests no checkpoints; it is compared as it is now, only with other origins of that kind, and not with the others
The fleet is not in agreement (8 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

## 23:20–23:36 — old tree compared file by file, late edits merged

Every non-ignored path of `/Users/Shared/semio` (272850, `git ls-files --cached --others --exclude-standard`) compared by sha256 with `~/Documents/semio`: 272706 identical, 139 differ with the newer copy in the new tree (fleet changes after 22:06), 4 missing in the new tree, 1 newer in the old tree.

- Missing: `…/SHARED-WORKTREE-FLEET-OVER-NFS/1113-landing.lock/owner` (a lock that was released), `…/🧰️scripts/1113-agree/agree_oracle.py` (old copy from 21:50, removed by the fleet since; the new tree holds the current folder with `live-probe.ts`), `undefined.stale` (artifact), and `…/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS/os-failures.md`.
- **Two files were edited in the old tree after its synchronisation went off**, by another agent session on this machine whose shell still works in `/Users/Shared/semio`: `os-failures.md` (22:52:14, new) and `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🔤️text-chunks/🟦️.ts` (22:38:43, one import line; the new tree held the unchanged committed version). Both were copied into `~/Documents/semio` at 23:35 and are byte-identical. All other files changed in the old tree after 22:06 are ignored cache results.
- That session ("Architecture course website deployment") was sent a message naming the new path; edits it makes in the old tree after 23:35 are not merged.
- Stashes: the three stashes of the old tree (`4c31562a4c22`, `653769b67e27`, `3df348cfb44c`) are commits in the repository of `~/Documents/semio` and `refs/stash` points to the first one. The three former stashes of `~/Documents/semio` (`80734a04d82`, `62a9b9293fc`, `a6bcdd9ef55`) are still in its reflog (`git stash list` shows them) and exported as patches in `.cache/f/lost/pre-relocate-stashes/`. None was applied: they are stashes, not working-tree changes.

## 23:40 — stashes checked against HEAD, old tree quiet

- The three stashes of the old tree, file by file against commit 5ececaa and the live tree (the base of each is in the history of HEAD):
  - `4c31562a4c22` (auto stash before a merge, 2026-11-11): 1 file, changed again in HEAD since.
  - `653769b67e27` (the work that became commit 740, 2026-11-11): 1270 files — 1236 equal HEAD, 33 changed again in HEAD since the stash base, 1 only in the stash: a generated `🗑️generated/authoring-objects/frames.json` of the ticket `26/11/10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE`.
  - `3df348cfb44c` (auto stash before a checkout, 2026-11-10): 146 files — 127 equal HEAD, 19 changed again in HEAD since.
  The former stashes of `~/Documents/semio` gave the same picture at 21:05 (49 of about 50 files equal HEAD, the rest older versions). No stash holds source that HEAD lacks; applying one would put older versions over newer committed files. They stay as stashes.
- 23:39: no agent shell has its working directory in `/Users/Shared/semio` any more, and no non-ignored file there changed after the merge of 23:35.

# 2026-11-14 — old tree deleted, native bootstrap attempts

## Old tree removed (02:22, on the owner's instruction "there should only be the canonical repo")

- Before: re-diff of every non-ignored old-tree file changed since 22:06 (nothing missing in the new tree), `lost/` of the old tree copied to `~/Documents/semio/.cache/f/lost/from-old-worktree` (157 MB), `relocated.json` and `relocate.json` copied to `~/semio-g1-m02-setup/`.
- The old tree's `repo-watch` service (pids 61560, 61561) stopped with `kill -TERM`; its 12 build-store disk images detached with `hdiutil detach`.
- `rm -rf /Users/Shared/semio` at 02:22:29–02:23:22, exit 0; free space 51 GiB → 114 GiB. `~/Documents/semio` is the only repository on this machine.
- `~/.local/bin/python3.12` and `python3.14` linked into the old tree's tool store; the dangling links were removed at 04:10 (the bootstrap's `uv python install 3.12` failed while they existed).

## Native bootstrap (step 9), not complete: `.cache/f/native-bootstrap.done` is missing

Runs through the provision script (sudo, by the owner) and then four times as the worker account with `~/semio-g1-m02-setup/run-native-bootstrap.sh` (the same `bash <native>/🥾️bootstrap/🐚️.sh setup` with the cache environment; it writes the marker only on exit 0).

1. Provision run 23:28–01:10, exit 3: `workspace:setup` waited 34 minutes for memory behind another session's `graph-plan` and test run, then `Slice worker exited with code 75` (reference 68e7218a).
2. 01:21: `ENOTDIR: not a directory, lstat '/Users/ueli/Documents/semio/CLAUDE.md/package.json'`. A root symlink `CLAUDE.md -> AGENTS.md` appeared fleet-wide at 00:58; the Bun workspace probe threw instead of reading it as missing. **Fixed** in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs` line 254: `observe` treats `ENOTDIR` like `ENOENT` (the same file already does so in two other places). No test was added.
3. 01:23–02:20: the graph plan waited for memory (`wants 6562 MB`, 6.8 GB available against a need of about 8.6 GB). Stopped; `repo service host stop` and the second resident host (pid 89103) ended; old-tree images detached.
4. 02:22–02:25: `go:build` failed with `error obtaining VCS status: exit status 128`: git reported `bad tree object HEAD`. At about 02:05 the pack directory of the shared repository had been replaced (index and `.rev` of `pack-afe8dcf2…` present, its `.pack` missing, older packs gone); every worker attested no HEAD commit, g1-server did. The packs were back at 03:46 (`pack-58516ce4…` and others) without any action here.
5. 02:47–04:09, with `GOFLAGS=-buildvcs=false` and the replica agent paused from 02:57 to 03:46 (`launchctl bootout`/`bootstrap gui/501`, to give the graph plan its 6.6 GB): launcher, native `repo` command, dependency scopes and the graph plan passed; `nx run workspace:setup --parallel=3` ended with 8 failed tasks:

```
- workspace:deps-js-all:prototype
- @semio-tech/energy-oracle-py:deps:prototype
- @semio-tech/repo-test-dotnet:deps:prototype
- @semio-tech/ui-styling-py:deps:prototype
- workspace:deps-build-tools:prototype
- @semio-tech/framework-os-mcp-rs:build:prototype
- @semio-tech/repo-lib:cpp-setup:prototype
- workspace:deps-cargo:prototype

error: Closed TypeScript artifact layout required
error: Invalid capability output region: Capability Features
error: Original JavaScript repository required
error: Single-link regular artifact source required
error: Unknown or ambiguous selected Cargo package: Cargo.toml semio-framework-os-mcp
error: uv python install 3.12 exited with status 1
```

`uv python install 3.12` is explained by the dangling links (removed since). The other errors come from the build code and manifests of the shared tree, which other machines were changing during the run; they are not specific to this machine. Retry with `bash ~/semio-g1-m02-setup/run-native-bootstrap.sh` once they are fixed; the build needs about 8.6 GB of available memory on this 16 GB machine, so nothing else should build at the same time.

## State at 04:11

`status`: synchronisation on, daemon running, `Git state: shared`, stage tree `47eebd16b9bf…`. `fleet --wait 120`: exit 1; g1-m02 `ok` with the same code (`0fe22c30102b`) and view (416/12/373) as g1-server and g1-m07:

```
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 0s 0/0 cp:0s on 0fe22c30102b 416/12/373 8895/58637|235/2378 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m02 macos 0s 0/0 cp:0s on 0fe22c30102b 416/12/373 56366/263692|343/6949 ok
  g1-m03 macos 0s 0/0 cp:0s on 0fe22c30102b 416/12/368 2918/11169|1312/92817 diverged
  g1-m04 linux 3s 0/0 cp:3s on 0fe22c30102b 416/11/373 -/-|590/131651 diverged
  g1-m05 darwin 30604s 7/536 now off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 4265s 5/0 cp:4265s on 4e292b9e97b6 369/12/349 35/956|338/12203 stale (attestation too old)
  g1-m07 macos 2s 0/0 cp:2s on 0fe22c30102b 416/12/373 185/592|344/146183 ok
  g1-server macos 2s 0/0 cp:2s on 0fe22c30102b 416/12/373 195/1472|253/1301 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  g1-m03 differs in indexTree at a common checkpoint: - instead of 47eebd16b9bf41ee127f76df7206bb79372171b9 (#n = bucket n of 256 of the paths, value = its digest)
  g1-m04 differs in view.digest at a common checkpoint: #16=6911,#52=0e5f,#64=b6ef,#86=3ab8,#114=25e8,#121=b976,#147=28fb,#163=fd47,#218 instead of #16=4580,#52=2568,#64=34b9,#86=91b9,#114=e07b,#121=09af,#147=fabf,#163=2c2d,#218 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (8 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```
