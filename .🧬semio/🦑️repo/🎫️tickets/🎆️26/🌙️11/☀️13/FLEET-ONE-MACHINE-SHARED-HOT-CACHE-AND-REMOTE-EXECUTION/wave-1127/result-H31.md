# H31: admission queues (cargo-preparation, compiler slots, dead holders)

STATE 05:10: both fixes LANDED. Cargo preparation 03:01 and 04:06 (lock workspaces); graph yield dead-holder 05:02 (lock graph taken over: owner file H9 04:10 was over 40 min old; H9's owner text restored). Yield tests 6/0 and 8/0 run live, typecheck 0 errors, dangling-imports 0.

Scripts: `T/🧰️scripts/H31/` (`patchlib.py`, `apply-cargo.py`, `apply-yield.py`, `apply-yield-tests.py`). Scratch: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H31/`.

## What `cargo-preparation` protects
`repo cargo` runs a `synchronize` child (`🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts`, `preparedPair`) per `-p <crate>` selection. Under the repo-wide lease `cargo-preparation:<root>` it (1) plans the selected owners (reads every member manifest, executes preparation recipes into a private directory), (2) writes a changed workspace membership, (3) runs `cargo update --workspace` and `cargo fetch --locked` (the only things that mutate `Cargo.lock`, the registry and the manifests) and (4) writes a receipt `<storage>/<sha(root,manifest,sorted packages)>.<suffix>` plus a witness for launchers.
Root cause of the queue: the receipt is per selection, so every new `-p <crate>` has none, and without a receipt the code went straight to the EXCLUSIVE lease and did all of (1)-(4) inside it. A receipt that went stale (any manifest edit by any of the 50 agents) did a shared pass (full plan) and then a second full plan under the exclusive lease. N peers = N serial full plans.

## Before (live, measured 02:10-02:20, load 61-63, swap 12.1 of 13.3 GB)
Two `repo cargo check -p <crate> --lib` of independent no-dependency crates started together (`semio-framework-intrinsic-size`, `semio-framework-io-base64`, both without receipt):
- wall 531 s and 602 s;
- second run: `Waiting for access to cargo-preparation:<root> (exclusive): position 2 of 2, behind process 3922 (exclusive)` for 64 s (log `scratch/before-semio-framework-io-base64.log`);
- other waits in the same runs: `cargo:planning-capture` 29-30 s, machine admission "too many learned reservations are still growing to their peak (need 1333 MB; 1858 MB spare)", `admitted cargo with 1 of 10 jobs: the host cannot carry more right now`.
So at this load the preparation queue was 64 s of 600 s for two runs; the 8-15 min waits of H16 are the same queue with many peers (each holding the exclusive lease for a full plan).

## Fix 1: shared planning, short exclusive section (cargo preparation)
`🛠️preparation/🟦️.ts`:
- every selection (receipt or not) first runs as a SHARED pass: plan, hash, confirm receipt; independent runs overlap;
- a stale or missing receipt without recipes and without membership writes throws `CargoPreparationExclusiveV1` carrying a `commit` closure of the planned pass; the exclusive pass runs `commit` first: re-verify the planned inputs (`current()`), re-read tool identity, accept a receipt a peer renewed meanwhile (same `inputsSha256`, same lock), else run update and locked fetch alone and write receipt+witness. No second plan. A changed input throws the existing replan signal and the exclusive pass plans as before;
- a selection that ran recipes under exclusive leaves a hint `<receipt>.recipes` and goes straight to exclusive next time (recipes run during planning, a shared pass would run them for nothing); the hint is removed when a later exclusive pass has no recipes.
Test `🛠️preparation/🧪️tests/🪜️escalation/🟦️.ts` (new): two runs without receipt plan concurrently (peak identity concurrency 2, failed with 1 against the old code before the patch), exactly one update and one fetch for both, a current receipt runs nothing, a stale receipt is planned while a peer holds the shared lease and the renewal waits behind it, then runs update+fetch once.

## Fix 2: dead holder at the graph front door (H9 finding)
`🕸️graph/🕰️generations/🛌️yield/🟨️.mjs`: `waitGraphQueueYieldV1` now reads the live holders of the door in the ticket every `doorPollMilliseconds` (500) by pid AND start time (`readDoorV1` -> `processRecordAliveV1`, tolerance 3 s, pid reuse counts as dead) and starts again at once when none is left (bilingual `holderGone` text). Before: only a newer generation or the 600 s wake. Policy, schema, `.d.mts` updated. Tests in `🛌️yield/🧪️tests/🪧️holder/🟦️.ts`: ended holder skipped in < 1 s, reused pid skipped, live holder still waited for with the door poll, holder killed while waiting skipped at the next poll. The two existing tests of that file were red before (fake `startedMs` of 10 days ago, now judged reused) and use the real start now.
Other lease/door waits: the lease queue reaps tickets by kernel claim proof (the pid check there is only an order hint, `ticketPidGoneV1`), custody claims by held lease; no other wait judged a holder by document alone.

## Compiler slots (not changed, finding)
The slot count already derives from cores, free memory, pressure and measured unit peaks: `🎟️capacity/🧮️units` `planCompileUnitsV1` (share 50 % of cores, minimum 50 % of that, memory = `(available - 2048 MB reserve) / measured unit footprint`, pressure caps) and the machine admission (`[capacity] cargo: sized at 37 % of the default from 9 earlier run(s)`, 1333 MB reserve per run). `semio.compiler.cpu/v1/<n>` has one slot per core (10 here). At this load every run asks for 1 core, so slots were not the binding queue; the memory broker was ("too many learned reservations are still growing", 5.5 GB reserved not yet resident). A real change (smaller minimum, fair split of free cores among queued claimants) touches the oracle-tested policy (`minimumShareBasisPoints`, literals in `🧮️units/🧪️tests`) and is left open.

## After (live, 03:11-03:25, load ~60, new selections without receipt, patched code)
- `semio-framework-trace`: 363 s wall (cargo-preparation wait <= 33 s); `semio-framework-hash`: 405 s (second run waited 73 s behind the first's shared pass, then exclusive); two runs with a wrong crate name (`-number`, `-expression`) failed after preparation, so their walls (297-317 s) only bound the preparation time. Before: 531 s / 602 s, preparation wait 64 s. Load and peers differ between the two sets, so the wall difference is NOT proof; the proof is the test `🪜️escalation` (old code: peak planning concurrency 1, new: 2, one update+fetch for two runs).
- Remaining queue at this load: planning itself takes 60-70 s of CPU-starved time under the shared lease, and an exclusive renewal queues behind every shared planner ahead of it (FIFO). A cheaper plan (cached listing snapshot) is the next lever.
- Regression found and fixed: `🛠️preparation/🧪️tests/📥️original-control` expects one identity call; the first landing read it twice (fixed 04:06, 13/0).
Baselines unrelated to this change: `🧪️tests/📥️original-control` (cargo) 7 fail before and after, `🏗️build/🧪️tests/🏗️native` 1 fail before and after.
Not verified: `📦️native-dependencies` heavy suite, per-lock lease keys (standalone workspaces skip preparation, so not needed), compiler-slot policy change (not made).
