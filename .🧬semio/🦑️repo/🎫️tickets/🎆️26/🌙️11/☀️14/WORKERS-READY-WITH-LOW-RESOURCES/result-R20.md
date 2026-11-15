# R20: why no serve completed on the group server, and what changed

All evidence observed on the live system (no graph command of my own). Times are UTC, 2026-11-15.

## Cause 1 (the idle read): an exclusive hold of the native image by `repo tools setup all`
- 05:54 `ps`: read `repo show projects` pid 36489/36610, 1.8 s CPU in 52 s. `sample` shows the main thread in `kevent64` (idle). No child, no network socket.
- Lease registry (`~/Library/Caches/semio/lease-registry/<dir>/36610-*.json`, same for the earlier read 32508): `waits: native-fixed-image:/Users/Shared/semio/.cache/b/d, shared, queued, 583 s`. The holder, pid 30229 (`tools setup all`, phase `javascript-installed-paths`, 98 % CPU, parent 30192), held `.../b/d` **exclusive** since 05:37:12 (736 s at the sample, 1441 s at 06:07). A second `show projects` (pid 31523, another agent) waited behind it for 701 s.
- Host capacity was NOT the wait: `grants/xiCJ...json` shows the read admitted (3346 MB reserved) at 05:46:20, `queue/` empty, pressure normal, 17.8 GB free. Not a deadlock, not a farm wait, not a lost wake-up.
- This is systematic, not a one-off: `lease-registry/.../history.jsonl` holds of `native-fixed-image ... tools` since 02:49Z: 27.0, 14.6, 28.7, 15.1, 15.7 min, and the next one (pid 63446) from 06:10Z. Every `repo` command attaches the image shared, so every graph read waits out each setup run. The hold is not preemptible (leases only order tickets; `interactive` advances a ticket by 1 h inside the queue but cannot take a resource from a holder).
- The read was also silent to the log: the graph child only pumped stderr into its progress watch.

## Cause 2 (every claim lost after 5 minutes): the claim could never be renewed
Log: serving 05:27:16 -> "held by another machine" 05:32:44 (5 m 28 s); serving 05:37:43 -> lost 05:43:03 (5 m 20 s); restored 05:46:19 -> lost 05:51:21 (5 m 2 s). No other machine took anything.
- The wanted policy claimed with `ttlMilliseconds` 1 200 000 and renewed every `renewMilliseconds` 300 000, but the cache server clamps a claim's time to live to `maximumTtlMilliseconds` 300 000, and a claim's total lifetime is `max(ttl, 120 s, expected*4)` capped at 30 min. The claim therefore expired exactly when the first renewal was due (and lived at most 300 s even if renewed). `renew` then answered `lost`, and the server printed "held by another machine", a guess that was wrong.
- A lapsed claim made the serving machine drop the request while its read went on for nobody ("reading: request" with `held: []` in `serve-state.json`).

## Cause 3 (who may serve): the gate was 15 000 MB
`serveTotalMegabytes` was 15 000 (and the generation head rank `servingTotalMegabytes` 15 000), below the 16 384 MB of the workers, so a 16 GB worker counted as a serving host. The memory plan of the graph says 24 000. In this window no worker took a claim (all lost claims were this machine's own lapses), but the gate allowed it.

## What changed (landed 07:05 and 07:13 local, `T/🧰️scripts/R20/land.py`, `T/landings.md`)
1. Claim: `claimTtlMilliseconds` 180 000 (below the clamp), `claimRenewMilliseconds` 45 000 (three renewals per ttl), `claimExpectedMilliseconds` 450 000 (asks for the 30 min lifetime). A lost claim is claimed again at once ("had lapsed; this machine holds it again"); only when the cache server says another machine holds it, the server says "held by <machine>" and leaves the request; the lifetime cap of the server is told as such. Both languages.
2. Gate: `serveTotalMegabytes` and `servingTotalMegabytes` 24 000 = the memory plan's `serveHostMegabytes` (a test cross-checks the two schemas). A claim can only be granted to a machine that passes `mayServe`.
3. Class: the read of a request runs with `SEMIO_LEASE_PRIORITY=interactive` (highest lease and capacity class), a background refresh with `background`.
4. No silent wait: new member `🕸️graph/🔬️stall`. Before a read starts, the lease registry is looked at; while another live command holds `native-fixed-image:` exclusively the start is held (no 3.3 GB reservation idling behind it), with a notice every 60 s naming resource, pid, command, phase, hold time and last progress, bounded by `leaseWaitMilliseconds` 20 min (then the read starts anyway, queued in the highest class, and says so).
5. Silence dump: a running read without progress for 120 s writes a dump to the build-agent log (repeated every 5 min): lease waits with holder pid/command/phase/time, the capacity grant or queue place with pressure and free memory, and the last stderr line of the read.

## Proof
- Tests first, red on the pre-landing tree (5 of 5 red: 15 simulated minutes lost the claim at 300 s), green after:
  - `🪪️graph-state/🙋️wanted/🧪️tests/🛠️server/🎟️claim` 5 tests: policy fits the cache server clamp and lifetime (against `CLAIM_POLICY_V1`); gate equals the memory plan; 180 rounds (15 min) of a running read with the real claim table on a fake clock, never lost, served once; a stolen claim is told with the thief; a lapsed claim is taken again and said so.
  - `🛰️sidecar/🧪️tests/🕸️graph/🔬️stall` 8 tests: blockers (shaped from the three real registry entries above), dump lines in both languages, queue place, nothing registered, the start wait (notice every minute, free at once, bounded at the limit, cancellation, no line when nothing blocks), class per reason, and a real fake-`repo` process that prints nothing: exactly one dump at 300 ms with pid and last output.
  - Changed existing tests: the "16 GB machines serve" test now states the opposite; the resume test expects "taken again" instead of "given up" for a claim nobody holds.
- Suites green after the landing (land.py ran them under the lock): sidecar (9 files), wanted (11 files incl. cost), remote-exchange (4 files). Overlay run with `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, parse proof of all edited files, dry run clean.
- Live after the landing (build-agent.log): `06:07:52 the read waits to start: native-fixed-image:/Users/Shared/semio/.cache/b/d is held exclusively by pid 30229 (... tools setup all, javascript-installed-paths) for 1441 s, last progress 1 s ago`; `06:08:31 ... is free; the read starts after waiting 40 s`; `06:09:23 the claim for g1-m02 had lapsed; this machine holds it again`; `06:11:19 the read (pid 45064) made no progress for 121 s; what it waits for:` and at 06:13:53 the dump names the lease behind pid 63446 and `last output of the read: [script] repository:command remains active on native-fixed-image ... (shared, 115 s)`.

## NOT solved / not verified
- **No serve completed within my watch.** After the claim is stable, the read still waits behind each `tools setup all` (14 to 29 min every 20 to 60 min). That is the owner of the setup sharing (R18): the setup must not hold the data image exclusive for its whole JavaScript install (short exclusive windows, or an install into a private directory that is swapped in), otherwise no read can ever finish between two runs. I did not touch setup code.
- The graph child is also restarted by other agents' edits inside its closure (06:07, 06:11 agent, 06:14, 06:16 cost module): every restart cuts the wave in progress and restarts the 20 min wait counter.
- The R17 serve state resumes the held request, but I did not measure how many projects a cut read re-infers.
- `tsc` not run (Bun parse proof and tests only). Nothing observed on other machines.

## For the other packages
- R18: this is the blocker of every serve; see above. R5c: the stall member is new in the graph closure; no change in your files.
- `edits.py` is incremental: the first landing is archived as `edits-r20a-landed.py` and `new-r20a/` (re-running it would duplicate anchors that contain their own old text).

## Coordinator follow-up (06:28Z): short exclusive window for the dependency funding
My findings agree: the holder is a `tools setup all` run (pid 63446; its 14 to 29 min holds are in `history.jsonl`), the same defect as R1 in reverse. Not landed, because the lease acquisition happens inside every `repo` command boot (`📤️publication/⚙️setup`, `💰️funding`: `withFundingParentV1`, `retryWhileDependencyStoreBusyV1`, `dependencyStoreShelteredByAncestorV1`), i.e. outside the graph server, and changing which hold a funding takes on the shared image cannot be tested safely today against processes that other agents have running (pid 63446 must not be disturbed).
Prepared design for the package that owns the funding (needs the funding producer, storage request builder and fleet-fetch install files):
1. Funding installs into a staging directory beside the image with no hold on `native-fixed-image:<dir>/d` beyond `shared` (it already holds `.../o` shared).
2. Exclusivity only for the atomic publish/swap (rename of the staged tree, journal row), bounded by a schema value (proposal 20 s) and logged with start, duration and holder list.
3. Readers keep their shared hold on the old tree through the swap (snapshot semantics), judged current by the existing marker/journal check (`📤️publication/⚙️setup/💰️funding`, "whether the dependency funding is current"); a graph read never waits for a funding whose published state is current.
4. Tests: a funding of 60 s with a reader started in the middle: the reader starts within 2 s and the exclusive window is under the bound; the registry shows no exclusive hold during staging; a crash during staging leaves the old tree intact.
What my side already gives meanwhile: the start of the serving read is held without a reservation, names the holder every minute, is bounded at 20 min and then queues in the highest class; the silence dump explains the wait. No serve can finish while a funding holds the image exclusively; none completed within the 25 minutes (last observed: `06:27:22Z the read waits to start: ... held exclusively by pid 63446`).
