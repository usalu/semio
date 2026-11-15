# g1-m07 Join

This Mac (`ueli-workstation-mac-mini-m4-16gb-256gb-02-lan`, account `ueli`, addresses `192.168.178.81` and `192.168.178.82`, macOS 27.2.1, 16 GB) is a member of group `g1` as origin `g1-m07`. It syncs and shares git state. It is the machine of `g1-m05-collision-mac-mini-m4-02.md`; it no longer writes to `origins/g1-m05`.

Work ticket: `26/11/13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02` (scripts, audit, summary).

## Summary

| Item | Value |
|---|---|
| Host used | `192.168.178.70` (NFS and cache), network `192.168.178.0/24` |
| Origin | `g1-m07` (`v1/origins` listed `g1-m01` … `g1-m06`, `g1-server` right before the run) |
| Account | `ueli` |
| Mount | `/Volumes/semio-store` from `192.168.178.70:/Users/Shared/semio-store` (nfs, nobrowse) |
| Worktree | `/Users/ueli/Documents/semio`, not a symlink, the only semio tree on this machine |
| Path taken | adopt (`join --adopt`): the tree was the bootstrap archive that the `--plan` run of 20:45 had extracted |
| Adoption | `270833 identical, 1615 taken from the fleet, 6 published, 0 kept here, 1656 replaced by the fleet, 0 resurrected, 64 deleted, 0 still queued` |
| Provision exit | 3 (fleet ready, native build pending) |
| Native marker | `.cache/f/native-bootstrap.done` absent after eight runs (the eighth read the graph and then failed 8 of 30 `workspace:setup` tasks on refusals of the build code, the same pending state as `g1-m01`, `g1-m04`, `g1-m06`): the fourth failed on a stale build closure (a build source changed on another machine during the hour-long run), the fifth to seventh waited for a 6627 MB graph memory reservation that was never granted, also at 7830 MB reported headroom (table in the work ticket report) |
| Services | LaunchAgents `tech.semio.fleet.replica` (running) and `tech.semio.fleet.compact` (scheduled) in `gui/501`; LaunchDaemon `tech.semio.fleet.store-mount` |
| Preflight | exit 0: 23 checks, none failed, 2 warnings (time synchronisation needs root, native build pending) |
| `verify` | exit 0 at 02:53 on 14 November: no drift on 273200 shared paths (on the join day 1 drifting path, `undefined.stale`, see "Findings") |
| `fleet` | verdict `ok` at 02:53 on 14 November: view `369/12/349` and code `4e292b9e97b6`, equal to `g1-server` and `g1-m06` (on the join day view `44/0/22`, code `35240fac3f31`) |

Exports on the run: `SEMIO_LANGUAGE=en`, `SEMIO_FLEET_GROUP=g1`, `SEMIO_FLEET_NFS_HOST=192.168.178.70`, `SEMIO_FLEET_NETWORK=192.168.178.0/24`, `SEMIO_FLEET_CACHE_ENDPOINT` unset. No `--server`, `--first-machine`, `seed` or `--force`. The topology file was not edited. No token was generated.

## Findings For The Fleet Code

- **The adopt path does not rename NFD names to NFC.** `bootstrap-settle` does; `join --adopt` on a tree that macOS `tar` extracted does not. 29 names stayed NFD here (`♻️mit-bestand/🎤️präsentation` and 26 names below it, two ticket files). The path policy refused them (`not-nfc`), `verify` reported 110 drifting paths, the daemon published 90 files below `🎤️präsentation` twice as new files (same bytes as the fleet, `base: null`), and the native bootstrap failed with `The lock file of dependency scope ♻️mit-bestand is out of date … 🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript is a member the lock does not list`. It also rewrote `♻️mit-bestand/package.json` at 21:11:22; `g1-m01` put the fleet version back at 21:14:26. Renaming the 29 names to NFC (`normalize-names.py` in the work ticket) removed all of it.
- **The dependency sync fails once on a plain `node_modules` directory.** `Internal error: Exact privately produced preservation capsule required`: the capsule `.pddc3bcee` held the directory that an earlier `repo` call had created at 20:48, and the recheck of the capsule stamp failed in the same run. The next `repo dependencies sync --all` exited 0.
- **`--plan` extracts the bootstrap archive into an empty worktree** and then stops. The next run therefore sees "a repository" and adopts instead of settling.
- **The adopt path does not carry the executable bit of an identical file.** `🗺️topology/📜️script.ts` was `0755` in the archive and `executable: false` in the fleet (`g1-m02`, 20:24:39). It was adopted as identical and the daemon then published `executable: true` at 21:04:48. Set back to `0644` here (the index mode is `100644`).
- **`undefined.stale` in the worktree root.** The `[DEBUG] hitcost` probe in `🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:1139` appends to `process.env.PROBE_OUT + ".stale"`; without `PROBE_OUT` that is `undefined.stale` in the current directory. The file is 28 MB, not ignored, and `g1-m02` and `g1-m01` republish it every few seconds (533 events in the newest segment of `g1-m02`), each as a new 28 MB blob. `verify` cannot exit 0 on any machine while it changes. Not touched here: it is another ticket's live probe.
- `origins/g1-m05` still shows `darwin`, `sync: off`, 9038 files: the last attestation is the one this machine wrote at 19:41:08. Open decision 2 of the collision note (the events of 05:19 – 05:27 in that journal) is unchanged.
- **A repack is not replicated atomically.** At 02:03:59 on 14 November `g1-server` published the index of a new pack (`pack-afe8dcf2….idx`, `.rev`, `multi-pack-index`) and the deletes of the 20 packs it replaced; the `put` of the 4 GB `.pack` was in no journal 43 minutes later. Every worker lost `HEAD` (`objects-missing`). The deletes of superseded packs must wait until the new pack is in the store, and an index must not travel before its pack. Repaired from the store blobs on `g1-m07` (work ticket report, last section).
- **The learned slice peak can exceed what the capacity gate can grant.** `$TMPDIR/semio-slice-peak-<uid>.json` held 6627 MB after one load; the next plan books that beside the 6144 MB grant of its own run and waits for ever on a 16 GB host (three runs, up to 92 minutes). Details in the work ticket report.
- **`workspace:setup` cannot pass on a Mac worker at the moment** (14 November): `framework-os-mcp-rs:build` is refused by the 256 character path envelope (crate `semio-framework-replication-retained-field-preparation`, 262 characters below `/Users/ueli/Documents/semio`), and `workspace:deps-build-tools` nests `repo tools setup all`, which no parent declares as a launch. Four other causes were fixed and three verified from `g1-m07` (work ticket report, last two sections). A content edit of the root `📋️project.json` does not dirty the root owner of the graph; a mode change does.
