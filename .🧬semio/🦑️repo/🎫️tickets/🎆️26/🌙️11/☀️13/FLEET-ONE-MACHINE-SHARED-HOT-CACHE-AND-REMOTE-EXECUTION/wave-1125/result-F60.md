# F60: memory grant growth, beyond-host and refusal in the cargo unit admission broker

## Design (schema-first, consistent with F13)
A unit whose predicted peak is above the parent grant used to be refused outright (`parent-memory`), even on the 32 GB coordinator. Now, when the launcher gives the broker a growth port (`startAdmissionBrokerV1({growth})`), the broker takes this decision (`decideGrantGrowthV1`, pure, language-neutral corpus + independent Python oracle):
1. `fits`: the grant carries the unit.
2. `grow`/`wait`: this host can ever hold it (holdable = (total - floor - interactive reserve) x budget share, the host-capacity arithmetic). The broker sends the early "queued" verdict, logs en/de "asking to grow by N MB (at most 15 min)", and books the difference as a supplementary reservation in the fair queue of the host gate beside the grant (`createGrantGrowthPortV1`: new grant of its own, so the warden thresholds, floor, interactive reserve, other runs and the queue order all apply; the holder's own grant row is never edited). Bounded by `Policy.growth.waitMilliseconds` (15 min), progress every 30 s in both languages, cancelled at once when the compiler wrapper disconnects (abort signal withdraws the waiter). The allowance grows by the booking, the booking is given back when the unit is done or its connection closes. Timeout ends as the old `parent-memory` refusal plus an en/de timeout line.
3. `beyond-host`: this host can never hold it but another fleet host can (or the fleet is unknown): refusal reason `beyond-host` (never compiled here; Rust decodes it en/de). F13's queue is reached upstream: the unit-cache offload gate's `beyondHost` now also holds when the learned peak is above this host's holdable memory (`unitBeyondHostV1(..., holdableMegabytes)`), so such a unit is queued on the fleet by F13's `awaitBeyondHostV1` before it ever reaches the broker; the broker refusal is the second line.
4. `refused` only when no machine of the fleet can hold it: reason `no-host-fits`, bilingual log naming unit, need and the largest host (origin, holdable MB); the Rust wrapper message carries need and largest host. Fleet = `<store>/v1/origins/*/memory.json` (fresh mirrors, `SEMIO_FLEET_STORE_ROOT`); an unreadable store is "unknown" and never counts as small.
Unchanged: no port, unknown host facts, or a parent that grants nothing (quiet/lent/missing allowance 0) keep the strict old refusal.

## Files
Schema `Policy.growth`, `Messages.growth`, `RefusalV1` (admission `🧬️schema/🔣️.json`); `🏔️ceiling/🟦️.ts`; `🟦️.ts`; `🦀️.rs`; product host-capacity `🟦️.ts` + `📖️texts/🔣️.json`; launcher `🟦️.ts`; unit-cache `📜️script.ts`. Exact shas in `T/landings.md` (section F60). Scripts: `T/🧰️scripts/F60/`.

## Proof
- New `admission/🧪️tests/📈️growth` 8/0: corpus validated by its own schema; law equals the Python oracle on 10 cases; big host (32 GB, 20 GB free) gets a grown grant, runs, booking released on done; small host (16 GB) is refused `beyond-host` with no booking and en/de text; no machine holds it -> `no-host-fits` naming `g1-server` and 23961 MB; timeout -> `parent-memory`; client vanishes -> abort signal; no port / unknown facts / zero parent unchanged; all growth texts en+de with equal placeholders.
- New `host-capacity/🧪️tests/📈️unit-growth` 6/0: the port books exactly the difference as its own grant and gives it back; gate off books nothing; unanswered wait ends at its bound (timeout) and by abort (cancelled), no waiter left; fleet store reader (fresh only, unknown is unknown).
- Rust: `rustc --test` of `🦀️.rs`: `decodes_terminal_admission_refusals` ok (new arms).
- Neighbours: parent 4/0, beyond-host 2/0, sibling 2/0, probe 3/0, farm 6/0, cancellation 2/0.
- LIVE: `repo cargo --store=shared --configuration=prototype -- check -p semio-ui-contract --lib` (output `W/🗑️generated/F60/uic-check.txt`, kept as evidence because the run was still compiling when this was written; the coordinator may delete it; final exit not seen): the unit `syn` (cold estimate 2686 MB, grant 1333 MB; exactly the refusal that failed the F54 check) logged "asking the host to grow the grant by 1353 MB" and "the memory grant grew to 2686 MB after 0 s" in both languages and compiled on.

## F54 proof file (`W/🗑️generated/F54/check.txt`, `dwg-editor-ac1018`)
The check FAILED (exit 101): `semio-build-scheduler: syn (host): ... estimated 2617 MiB but its parent memory grant allows 1333 MiB` (syn has no measured peak under profile prototype, cold estimate 2617 MB; not a sibling case of F54's fix). That is exactly the grant-below-need case solved here. Not re-run to the end (F46 measured 23m55s for `dwg-editor` alone); only the syn-sized refusal was re-proven live above.

## The 2 failing tests of the main admission file
Not related. Both come from `real Cargo through the wrapper` (tiny-workspace helper builds the scheduler through the broker): under fleet load 40-45 it hit its 600 s timeout (F54/F13 saw exit 101 earlier), and the test-runner then reports `Build test receiver closed with active callback` as the second failure. Result 17/2, same as before my change. Separate failure in `🧢️unit-ceiling` (9/2): the carving list now says `semio_hub_sourcing` 4000/4400 MB while the corpus expects 4500; another lane changed the carving list, not touched here.

## Not verified / notes
- Fleet-wide behaviour: no real fleet beyond-host queue run; the gate integration is covered by unit tests only (`unitBeyondHostV1` holdable arm in the corpus).
- The supplementary booking is a separate grant: the process tree under the parent grant is observed by the parent's grant, the booking observes nothing (`observe: 0`), so the extra memory counts as reserved, not as measured, until released.
- Concurrent grown units each book their own difference and the pool allowance is the sum, so the broker admits the same total memory the host gate granted.
- `localHoldableMegabytesV1` and `growthHostV1` use the build policy's host-capacity numbers; a remote host's holdable is computed with the same policy at its reported total.

## Coordinator note
- Live proof: `repo cargo check -p semio-ui-contract --lib` exited 0 in 7m 23s; syn got a grown grant (1333 → 2686 MB).
