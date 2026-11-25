# Result F40: publish cost and cross-origin lookup misses (19:40-20:10)

## 1. "Publish as expensive as compile": measurement artefact (fixed)
`unit:publish` in the phase ledger sums the `publish` events (`phase-ledger` groups `publish` -> `unit:publish`). `📜️script.ts` emitted `event("publish",key,Date.now()-compiled,...)` where `compiled` is taken BEFORE the compile (offload/compile), so every publish event held compile + publish. Proof from the numbers: m02 compile 431 s, publish 451 s (difference 20 s over 117 units = 0.17 s per unit); m12 402 s vs 419 s (17 s over 127 = 0.13 s). The real publish (local store write, hashing) is about 0.15 s per unit, not 3.6 s. Upload to the cache server was never on the path: `shareFleetUnitAwaitingV1` only queues a pointer in the spool and starts the uploader; it waits only when the 4096-entry spool is full (bounded by `spool.waitMilliseconds`). So nothing needed to be made asynchronous; the phase line was wrong.
Fix: `published=Date.now()` taken after the compile and the miss event; both publish events use it. Test `the publish event measures the publication, not the compile before it` (1.5 s slow compile, publish must be > 1 s below the miss time). unit-cache test file 29/0.
Effect: from the next build of each machine the `unit:publish` line will read about 20 s instead of 450 s; publish is NOT a wall-time problem.

## 2. "Every lookup misses": not resolved; evidence and what is excluded
What the server data shows (AC `provenance.jsonl` + AC entries on the coordinator, crate/slot parsed from each entry; scripts in `T/🧰️scripts/F40/`):
- 88,576 unit AC keys since 11-25 morning; only 2,467 were put by more than one origin.
- Keys of one registry crate with an immutable source differ per origin and per time: itoa has 9 keys in 6 slots; inside one slot (`b7beb5aa`) m02, m08 and m11/m13 each have a different key AND different rlib/rmeta bytes (so the compile inputs really differ, the key is not merely over-sensitive). futures_core key `8a0309a7` was put by m14 (15:18), m02 (17:39) and m13 (19:37): the same key does match across origins and across hours when inputs are equal, so a general machine-specific input is NOT proven.
- All 14 workers are 16 GB M4 Mac minis, same toolchain (nightly-2026-07-20, `commit-hash 9f36de77`), same path `/Users/ueli/Documents/semio` (m02 logs), so absolute-path, user, CPU and OS-version differences do not apply to this fleet; `platform.linker` (darwin major) only enters for linked crate types.
- The dominant explanation for workspace crates is time: the replicated tree and the wrapper code changed all day (F11 landing 16:30, F16 17:20, trains 19:00), the Ready machines built din4108 before 17:40 and m02/m12 at 19:30, so source digests, `--remap`/OUT_DIR arguments and wrapper versions differ. Of 40 min of m02/m12 new keys, 631/633 (m12) and 534/881 (m02) have a same-slot key from a Ready origin with different bytes.
- Components are named digests in the local `lookups.jsonl` (`args.*`, `extern:*`, `env:*`, `source`, `toolchain`, `platform`), so two keys can be diffed only when both lookup logs are at hand. Peer lookup logs are not in `S/`, and the server entries' `unit.json` blobs answered 404 over `/main/cas/<digest>` with the coordinator's credentials (unresolved; the server index shows the digests), so a same-crate two-origin component diff could NOT be produced.

## Not verified / next step
- Same key on two origins for one din4108 crate: not proven. Recipe: export `SEMIO_UNIT_CACHE_TRACE_KEYS=1` on two workers (the lookup log then also stores `trace`: portable args, env, platform), build one registry crate set from the same tree state, then `repo` unit-cache `churn` / `traceDifferencesV1` on the two logs.
- Cheapest structural improvement not landed: carry the key `components` into the AC `unit.json` so the server can diff two keys of one slot without access to the machines (changes the entry payload, needs a design decision, not landed blind).
- Tree churn is a legitimate miss source for workspace crates; only a quiescent tree (no landings during a build) gives a clean cross-origin comparison.

# Round 2 (20:10): zero-touch key evidence (landed; live diff NOT yet possible)

## 404 on `unit.json` blobs: not a server bug
Old keys 404 because retention removed them (AC answers only while every blob it names is stored). A fresh key (`c889c4aa`, put 19:5x) answered 200 for the AC and for its `unit.json` blob (8,730 bytes); the earlier probe used superseded keys. Nothing to fix there.

## What landed (schema first, en/de)
- Schema (`🛰️fleet/🧬️schema/🔣️.json`): `KeyComponents` (origin, component digests, small path-free values), `SlotKeys` (server answer), optional `keyComponents` in `RemoteUnit` and `SpoolJob`; `Messages.missNearest` en/de in the unit-cache schema.
- Every uploaded unit manifest (`unit.json`, stored next to its AC entry, already served by the cache) now carries `keyComponents`: digests of toolchain, source, platform, one `extern:<dep>` per dependency (the deps' keys), `env:*`, `args.*` and the OUT_DIR/generated env, computed from the same portable inputs as the key (`keyComponentsOfV1`); strict parse in `parseRemoteUnitV1`.
- Server: `GET /<instance>/v1/slots/<slot>` lists the action results of a slot newest first (`RemoteCacheStoreV1.keysOfSlot`, slot index only); client `slotKeys`.
- Wrapper: on a fleet miss, `explainFleetMissV1` runs while the unit compiles (no critical-path time, 1.5 s cap), reads up to four stored units of the same slot, picks the nearest (other origin preferred) and prints `[units] miss <crate> nearest:<key12> differs in <inputs> / <de>` to the build log (visible in `ready-log-*`).
- Tests: fleet 20/0 incl. components equal under two checkout paths (and naming exactly the differing inputs), real-server slot listing + explain; unit-cache 29/0; remote-cache 27/0; dangling-imports 0. Script for a manual diff: `T/🧰️scripts/F40/diff-slot.ts <crate>` (needs units uploaded by new code).

## NOT done / unverified
- No same-slot pair carries components yet: workers must restart on the new code and rebuild; the coordinator's `itoa` slot (`57fa5422`) lists 0 keys, so I could not diff two origins on `itoa` and no divergent input was found or fixed. Whether the live cache server already runs the new route is unconfirmed (no server process visible from here).
- Next: after the next worker builds, grep `[units] miss` in `S/origins/*/ready-log-workspace*.txt` or run `diff-slot.ts itoa`, then fix the named component (candidates in order: `args.*` values, `platform`, `extern:*` of dependency bytes, `source`).
- `origin` is the host name, so m10/m13/m14 (all `Uelis-Mac-mini.fritz.box`) are indistinguishable in the record.
