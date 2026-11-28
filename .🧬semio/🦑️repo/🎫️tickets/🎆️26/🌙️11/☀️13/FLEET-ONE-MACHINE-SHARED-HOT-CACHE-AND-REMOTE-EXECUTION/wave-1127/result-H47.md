# H47: m12 ENOSPC in the compiler image `r` after H34/H41, live lane images

## Why m12 died again at 05:46 (measured 11-27)
- m12 ran the new code (stage started 05:22 CET by supervisor 15756, adopted 05:08 by 74454; H34 04:02 and H41 05:05 were in). The code was blind, not old:
  1. The failing writer is `.cache/b/r/m/cargo/home/.semio-ready/f3|f8.fetched.json.stage` (and the lane file `.cache/b/r/m/u/l/q90be4b/lane.json.*` on an earlier try). `cargo` is a ROOT SHARD of the Rust partition (own image, about 16 GiB, born by relayout), and lane `u/l/<id>` can be a shard too. The in-run guard, the start gate and the collector measured only the HOME image `r/m` (15.6 GB free of 40959 on the same minute), so a full shard was invisible. H41's note "no worker has a shard" was wrong for m11/m12: after landing, the guard named `…/r/m/cargo has 0 MB free, floor 648 MB` (m12) and `…/r/m/u/l/pa7be67 has 79 MB free, floor 1180 MB` (m11).
  2. The ENOSPC line scrolled out of the 20-line failure tail (the stage printed thousands of `[repository:command] still running` lines after it), so `readyNoSpaceV1(result.tail)` was false: no sweep, no `image-full`, only a generic `stage-failed` with growing back-off (ready.json said `stage-failed`, not `image-full`).
  3. Host free never fell below 15 GB in the metrics ring (`host.disk.free`, 05:00-05:48), so this was not host exhaustion; thin images keep freed blocks (oversize images cannot unmap), inner used swings 21 -> 40 GiB while host free stays flat.
  4. The collector did not run for it: nothing measured the shard as short, and the unit collector never touches `cargo/home`.
- Guard floor was not too low for a burst in the observed case (a 5 s tick and a floor of 1.6 GB of 40 GiB); but a 5 s tick can miss a burst, so the image tick is now 1 s.

## Landed 06:00-06:07 (landings.md "Wave 1127 H47")
- Guard, gate and relief now cover every mounted shard image of `r` (`readyImageFactsV1`), the no-space line is found in the whole log (`readyNoSpaceEvidenceV1`) and added to the failure tail, image guard tick 1 s, low-water mark (20 % of the image, 1-8 GiB, never below the need) with in-place collection before each stage (never a refusal) and while a stage runs (background, one at a time, every 2 min), `readyImageStoreV1` maps a shard to the home mount for the unit collector. Typed refusals and `image-full` back-off unchanged.
- Live lane images: `🏊️pool/🧩️image` (`ensureLaneImageV1`, `retireIdleLaneImagesV1`, `repairLaneImagesV1`) wired into `acquirePooledLaneV1` and `acquireProductLaneV1`: a lane without files gets an image of its own by the existing shard rule (capacity `min(32 GiB cap, 66 % of host free)`, refused when the host cannot hold 16 GiB), an existing lane image is mounted again (reboot), a lane with files stays in the home image (no relayout while builds run), the image of a vanished idle unclaimed lane is retired whole (blocks return to the host at once), `removeLaneV1` keeps the mount point of a lane image. Every refusal or failure leaves the lane in the home image.
- Tests: image-guard 12/0 (Python models for floor/need and low-water mark, evidence log with 150k later lines, shard rows), loop 6/0 (relief before start without refusal), lane images 6/0 (Python reads the ledger: aligned, <= cap, path `u/l/<id>`); pool dirs pass (`pool-model` flaked once in about ten runs under load, passes alone); typecheck 37 and 76 files 0 errors; dangling-imports 0.

## Not done / unverified
- A full CARGO-HOME shard (m12): the start gate collects below the mount (`cargo-incremental` writer, units) and refuses typed `image-short` if that does not free the need; there is no in-place growth of a shard (the home-image grow of H34 does not know shards) and no writer for `cargo/home/registry`. If incremental sessions are not what fills it, m12 stays on a typed refusal. Needs the contents of m12's `cargo` shard (not visible from the store).
- Lane births need host free >= about 24 GB (16 GiB want / 66 %); m12 (15 GB) and m11 stay in the home image until the host has room. Nothing was created live by me; the first birth happens on the next new lane of a worker with room.
- Relief of a SHARD runs the unit collector only (lanes' incremental, units); it does not run the `cargo-incremental` writer, only the start gate does.
- Main ready test 55/58 with 3 failures and 2 errors, not caused by this lane (`named holder` fixture ENOENT writing `code/a.ts`); the full list is in `🗑️generated/H47/main-ready.txt` while this file is open.
- The hour of proof (m12/m11/m01 r free, no ENOSPC): see the watch table below.

## Watch (`🧰️scripts/H47/watch-r.py`)
Window 06:08-06:17 only (landing 06:00-06:11; the full hour is NOT proven, rerun `python3 T/🧰️scripts/H47/watch-r.py g1-m01 g1-m11 g1-m12`; it prints state, `r/m` free of total from the newest disk-facts block and every non-source-excerpt ENOSPC line per log):

| origin | 06:08 | 06:12 | 06:16 | ENOSPC lines |
|---|---|---|---|---|
| m12 | r 15365 of 40959, guard stopped the stage on `r/m/cargo` 0 MB free | 20950 free, stage running | 19391 free, reading graph slice | none after landing |
| m11 | r 29446, guard stopped on lane `r/m/u/l/pa7be67` 79 MB free of 29.5 GB | 33855 | 33811, reading graph slice | none |
| m01 | r 15335 of 98304 (host-limited) | 15496 | 14243 | none |

- First proof the new guard is right: within a minute of landing it named the two real full images (m12 cargo shard, m11 lane shard) that H34's guard could not see.
- Open for the next hour: whether m12's cargo shard and m11's lane shard get enough room from the in-place collection (watch for `image-short`/`image-full` instead of `stage-failed`), and the first live lane birth (needs host free >= about 24 GB).
- Main ready test after landing 56/2: `runs too long ... timed out` (5 s timing test under load 30-40) and `named holder` fixture ENOENT; neither touches this lane's code.
