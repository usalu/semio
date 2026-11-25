# Result F6 (graph seal), 16:55

## Verdict: NOT sealed, nothing landed. Root cause is the graph cache image being 100 % full, not closure-missing.
- `current.json` gen 260 `closure-missing` is only the quarantine pointer of an old failed load (manifest null). Whole reads since 11-22 are all dead ends of the same kind.
- Machine rebooted 01:16 on 11-25 (uptime 15 h), so the 11-22 reads are long gone. Since then six whole reads ran (`.cache/f/graph-server/read.<ms>.log`, 13:30-16:45): every one reached fragments (5234/5235), then died with `The disk is full: partition / has 83-87 GiB free` (wrong partition named; the real full volume is the graph image `.cache/b/g/m`, `/dev/disk11s1`, 18 GiB fixed image `fixed.dmg` 19,326,345,216 B, 65 MB free, 100 %). One read (11-25 16:20) was in `facts` 507904/841640, another worker read (pid 97384, 16:43) in `closures 4608/5101` while I wrote this; both will die at the patch/seal step for the same reason.
- No OOM this time (E54's fix holds: heavy child 3.5-3.9 GB, grant 3 GB).

## What fills the 18 GiB image (du, 16:40)
| path | size | note |
|---|---|---|
| generations/j | 8.8 GB | build journal of key 113bea42 (state waves, written 15:16 by the failed read): 878 `.rows` files of ~10 MB, header lists only 719 (159 orphan chunks of retries, ~1.6 GB); only `clear(key)` removes it and only a resume that passes the header checks or a finished seal reaches that |
| inference/selections | 5.9 GB | 120 selections, one (8d8f21d7, 14:24 today) holds 4.1 GB in `ce` |
| generations/o | 1.6 GB | objects of earlier generations |
| derived-nodes 0.8, members.jsonl 0.3, part-ids 0.3, build-input-closures 0.24 | | |
The seal needs new space for the manifest/index/objects while the journal (8.8 GB) is still alive, so the last step fails at 100 %.

## Why the image does not grow (elastic rule)
`B/🔣️policy.json`-sibling `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` `elastic.graph`: automatic, maximum 24 GiB, `hostFreeReservePercent: 92`. Growth is capped by `floor(hostFree * (100-92)/100)`, host free is 74 GB of 926 GB (volume 92 % used, 827 GB), so the cap is about 5.9 GB, far below the current 18 GiB: the image can never be recreated bigger. (`elasticGrowthV1` in `📐️elastic/🟦️.ts`; growth also means the image is recreated, i.e. the store is lost.)

## Why I did not free it
- `repo cache gc` (and `cache status`) take the exclusive graph area; with load 40-46 their source admission alone takes 6-11 min and then dies (`resource loan ran out` / exit 124, my two runs and a peer's `cache gc --area=graph-inference` pid 79696 all stuck). Reads keep (re)claiming the graph area, so the collector never gets it.
- The only things that would free space are the live journal (needed for resume) and the 4.1 GB selection memo (loses hours of work); removing them by hand is forbidden (no rm) and would be a guess. Policy edit (reserve % or capacity) replicates to 14 machines and recreates the image: needs the coordinator's decision, not landed.

## Options for the coordinator (in order)
1. Free host disk (827 GB used of 926 GB; three test-sandbox images are mounted under `.cache/test-sandbox/rJTEOQy`), then let the elastic rule grow the graph image to the 24 GiB cap (raises `hostFreeReservePercent` headroom: with 200 GB free the cap is 16 GB). This is the clean fix.
2. Or lower `elastic.graph.hostFreeReservePercent` 92 -> 80 (cap becomes ~14.8 GB at 74 GB free) and let the next opening recreate the image; the store (journal + memo) is rebuilt, fleet records are fetched from peers.
3. Code fix worth a ticket: the journal should write rows compressed or per-owner de-duplicated (9 GB of JSON rows for one read is the dominant consumer), delete orphan chunks on every `save`, and the disk-full text should name the image partition not `/`.
4. Stop the competing read launches (worker.ts, `repo show projects`) until space exists; each failed read re-writes the journal.

## Not done
No seal, no `repo-mcp:build`/`install` (they need the graph); MCP server restart not yet applicable. Nothing landed, no landing.lock taken.

## Round 2 (17:05)
Landed (details and test names in `T/landings.md` "Wave 1125 F6"; locks `caching-storage` and `graph` taken and released):
1. **Elastic rule**: the 92 % host share no longer limits growth. New schema fields `growHostFreeFloorBytes` (24 GiB) and `growHostFreeFloorPercent` (5) in every elastic rule; `elasticGrowthCeilingV1` = host free + the image's own allocation - max(floor, percent), capped by `maximumBytes` (graph 24 -> 32 GiB). `hostFreeReservePercent` stays only as the birth share (sum of images born one after the other). Python model in the budget test updated (independent check).
2. **In place**: the primitive existed (`growNativeImageV1`, used only by the doctor `storage-grow` repair, whose target is the declared 6 GiB, so it never grew the elastic image) but the regrow path only knew "recreate". `growElasticImageInPlaceV1` (reset module) now runs first on macOS: exclusive non-waiting lease, detach, `hdiutil resize`, attach, receipt rewritten; new regrow outcome `grown`; held or failed falls back to the recreation. NOT exercised live: the graph image was never idle (a read always held it); the doctor `storage-grow` target is still the declared capacity (finding).
3. **Failed read retires its journal**: `build()` clears its journal key when the failure is disk-full (`diskFullFailureV1`); test builds a world, injects ENOSPC and sees an empty `j/`. After my landing the 8.8 GB journal of the dead read was gone (image 9.7 of 18 GiB used, 8.2 GiB free at 16:58).
4. **Compact encoding**: journal rows files are Brotli-packed (`<key>.<n>.rows.br`, schema pattern, pack/unpack in the journal module, cross-checked against the `brotli` CLI, 4x or better smaller on test rows). Landed, schema-first.
Tests: journal 9/0, journal-memory 1/0, budget elastic test passes (2 failures in that file are a peer's open mount-contract edit: "Actual Native platform image required"), reset tests 5 files/0, shards 4/0.

Read: one detached whole read (holder pid 36744, "show projects", started 17:01) holds the graph claim, facts 352256/841640 at 17:03, image free 8.2 GiB. `repo show projects` itself does not start a whole read when none is sealed (it refuses and points to `repo graph --gr...`), the sidecar read was already started by another caller; I started nothing else and stopped starting reads. Pending: seal, then `repo run repo-mcp:build` + `repo-mcp:install` (result below when done).

### Doctor storage-grow (17:25)
`repo doctor --repair` `storage-grow` now plans a pressed elastic image (mounted free below `pressureFreeBytes`) and grows it to `elasticGrowthV1` (new `elasticGrowTargetV1`), not the declared 6 GiB. Test in the doctor probes file (python model), 23/0. Landed without an area lock (none held, no concurrent doctor edit).

### State at 20:28 (not sealed)
- Reads since the landing: (a) 17:01-17:58 stopped by the 5-minute no-progress watch at 47/48 owners (`framework-os-dev` silent, host load 50-87); (b) 18:04-19:09 died with `Internal error: Observed TOML parse refused` for a dwg `Cargo.toml` a peer was rewriting at that moment (file valid again seconds later; a transient peer edit kills a 2 h read: finding, the `BUILD_SOURCE_UNPARSEABLE` path of the cargo-owners phase is not tolerant like the project-owner path); (c) 19:12-20:24 (my `repo graph --graph-policy fresh`) stopped by the same no-progress watch at 188/192 owners (`framework-os-host-rs` 390 s silent, load 85-108). Each run keeps its owner records, so the next run starts further; no disk-full any more (image 6.1 GiB free, journal now small/Brotli).
- Blocker is host load 85-108 from the fleet (gate 40): in-process cargo owners are time-sliced and look silent for > 5 min. Not started again while load > 40. Next steps for whoever owns the load: when load < 40 run `repo graph --graph-policy fresh` once (seal expected within about 30 min: owners nearly all stored), then `repo run repo-mcp:build` and `repo run repo-mcp:install`, then restart the repo MCP server. Option: scale the no-progress watch with load like E83's stretched budgets.
