# R90: g1-m10 `unexpected failure (reference 3a9fba88)` named and fixed

## 1. What the failure is (read from the store at 18:47 UTC)
`origins/g1-m10/service-failure.txt` (new, written by the resident service itself):

```
Error: Closed graph phase and finite total required
    at begin (…/🕸️graph/⏱️phases/🟨️.mjs:19:121)
    at <anonymous> (…/🕸️graph/🕰️generations/🔌️load/🟨️.mjs:274:92)
```

The graph phase clock accepts only phase names of lower-case letters (`^[a-z]+$`), begun once. R86 landing b reported the closure settle's part decisions and supports as `flush parts` and `flush support` (with a space; three emitters in `closure-kinds/🟨️.mjs`: `holdParts`, the support loop, the end of the settle). The first settle that decides a part ends the whole slice read with this error: a cosmetic counter killed the read. Only a machine whose slice has stale closures reaches that code, which is why only g1-m10 failed this way; the 18:33 and 18:47 failures were this error (same reference every time: the stack holds no frame of a file that landings changed).

Reference computation: `sha256(failureDetailV1(error))[0..8]`, i.e. name, message, code and the full stack with the absolute paths and line:column of the machine. Hashing candidate messages without the stack is not possible; I hashed 274 856 string literals of the product tree with 9 prefixes (4.9 M hashes): no hit, as expected. The block in the store made it moot.

## 2. Why R86's mirror wrote nothing
`readyFailureBlockV1` (R86 landing d, `ready/🟦️.ts`) is not in the live tree (grep over `🧰️framework` finds it nowhere; landings.md lists it as landed 18:29 local): a later write of that file, or its restore after a red, removed it. Independent of that, a client-side copy cannot work on a machine whose `ready` child runs old code, and it depends on the service log path being readable by the child.

Replaced by a service-side mirror (landing a, 18:33 UTC, `resident/🏛️service/🧯️failure/🟨️.mjs`):
- Every refusal that carries a reference (internal, consent, relative-call) writes `<store>/v1/origins/<origin>/service-failure.txt` and `service-failure-<reference>.txt` (newest 16 references kept) from the service process, inside the request, from `SEMIO_FLEET_STORE_ROOT` / `SEMIO_FLEET_ORIGIN` of its environment. Content: reference and time, name, code, English and German refusal text, message (at most 4000 characters), then the full stack; at most 64 KiB.
- The refusal line itself now carries the message: `… unexpected failure (reference 3a9fba88): Error: Closed graph phase and finite total required; try again …` (EN and DE; `fields.message` is name, code and message on one line, at most 300 characters, code before the message so truncation keeps it).
- Seen live: m10 produced the file 11 s after its failure at 18:47:59 UTC.

Laws: `resident/🧪️tests/🧯️failure` (3 new: message in fields and both texts and block in the folder; 64 KiB and 300-character bounds; no store/origin/path-origin writes nothing, newest 16 kept) and the real-process law `resident/🧪️tests/🌐️host`: `real process: a resident service that throws refuses with its message in both languages and writes the whole block into the origin folder of the store within seconds` (separate service process from the fixture `💥️throwing-host-main.mjs`, a 90 000-character message, stack, EN/DE, bytes between 60 000 and 65 536). Red on the old module (import error), green after. The old test that required the message NOT to appear in the refusal was changed to require it (you asked for it).

## 3. The fix
Landing b (19:0x UTC window, see landings.md "Package R90 landing b"): phases renamed `decisions` and `supports`; law `closure-kinds/🧪️tests/🪪️part-ids`: `every phase of a settle is accepted by the graph phase counters` (cold, warm and stale settle through the real `createGenerationProgressV1` and `createGraphPhaseClockV1`; on the old module the phases are `verification,flush,flush parts,parts,invalidation,rederivation` and it is red; green after). The former test that pinned `flush parts` now pins `decisions`.

## 4. Not done / not verified
- Step 3 (reproduce the exception in a harness): not needed once the block named the throw site; the law above reproduces it with the real clock. I did not verify what runs after `rederivation N of N` beyond that (support loop, flush, manifest save, `markIndexed`, then the take).
- Whether the 17:10, 17:38 and 18:01 failures were this same error cannot be proven (same reference, the stack carries no landed file, but their code versions are not recorded).
- The host suite has 3 environmental reds on this machine before and after (two `slice memory guard` tests, `full disk`); the landing script allows exactly these.
- g1-m10 after the fix: see section 5.

## 5. g1-m10 watch
Landing b went live 19:02 UTC. m10's next try (19:08:02 start) passed the former failure point: `rederivation 171 of 171` at 283 s, then `stage inference 0 of 0` at 345 s, then `taken 0 of 477` from 526 s. No `unexpected failure` any more, no `service-failure` file newer than the 18:47 one.

m10 is NOT `ready: true` (19:26 UTC: stage workspace 5/5, attempt 20). That try was stopped by the stall rule at 1004 s (`ready-timings.json`: `stalled 1004`): the take showed `taken 0 of 477` for 700 s. `slice-memory.json` (owners source) says why: the owners are derived in the `warm` lane (`warm.total 476, settled 9, lanes 1`, ledger `missing 477`, cpu 458 s user + 89 s system, guard heapUsed 1.5 GB + external 1.0 GB, rss 2.2 GB): 9 owners in 15 minutes on one lane, so 476 would need about 13 hours. That is the take loop (R89's shard processes), not touched here. Two cheap leads for R89: (1) the progress line counts only finished takes, so a lane that is working shows `taken 0 of 477` and the stall rule kills it; `warm.settled` should be told as progress; (2) m10 runs `lanes 1` (`limit 4`): the lane count on a 16 GB machine and the 100 s per owner are the real limit.
