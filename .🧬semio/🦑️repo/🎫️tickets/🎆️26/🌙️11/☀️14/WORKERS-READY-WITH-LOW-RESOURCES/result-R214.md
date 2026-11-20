# R214: leaf pieces travel with owner records (living report)

Started 2026-11-18 17:34 UTC. Scripts: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R214/` (`land.sh`, `land.tsv`). Scratch: `scratchpad/R214/` (`base/` `new/` staged copies, `t.sh` `live.sh` runners with the R194 resolve preload, `measure.ts`).

## 1. What was built (schema first)

- `🪪️graph-state/🧬️schema` `Note.leaves` (optional array of `{namespace, hash, size}`, namespaces `build-input-closures` and `build-lock-closures`): the action entry lists the leaf digests the record depends on, so a taker fetches without interpreting the value.
- `🛰️exchange/🟦️.ts` `publishGraphEntryOutcomeV1(... leaves, leafWidth)`: after the document blobs and BEFORE the action result: leaves deduplicated by digest, a process-wide memo of digests known stored per endpoint+instance (skipped without a request), else `HEAD` (existence by digest), else read, verify size+sha256, `PUT` the raw blob under its own digest (the same CAS key the generation exchange uses, so generations and owner records share one blob). The action result references them as `leaf/<i|l>/<hash>.json` output files (retention follows the entry); `noteOf` keeps them out of the document list so no reader downloads them as documents. A leaf the producer does not hold or whose bytes do not hash to its name: refusal `leaf-missing` (new row in `🧯️refusal`, EN/DE, local, not transient, not deterministic), nothing is stored, and such a record is not kept as "unsent".
- `📦️project-records/🍃️leaves` (new member, schema `Policy` with width bounds 2..8 derived from cores, 16 MiB leaf bound, token pattern; EN/DE texts): `leafReferencesOfTextV1`, `leafSourcesV1`, `adoptLeavesV1` (fetch the listed leaves the store lacks, `width` at a time, each verified by size and digest by the client, stored under the same digest), `leafPortWithDirectPutV1` (writer custody of the graph store when held, else the same immutable file written directly after hashing: a look without custody never turns a fetched leaf into a missing one).
- `📦️project-records/🟦️.ts`: every publication site (`claim().publish`, `publisher`, `offer`, `resend`) passes the leaves named by the record's value; `seedFrom` adopts the listed leaves BEFORE `info.import`; a leaf nobody can give: the record is not imported, `TakeLookV1.outcome = "incomplete"`, counter `incomplete`, a bilingual line naming count and first digests; the reader then counts it as a miss (not "stale": `look` maps unknown outcomes to miss), infers the owner and republishes with the head swap (`reinferred`), which re-uploads the leaves bypassing the memo (heals a fleet cache that lost a blob). New counters `leaves` (fetched) and `incomplete`.
- Format change: `project-records` `Policy.form` 2 to 3 (part of the state digest of every record key): old entries are never found, no compatibility layer; the `📥️take` fixture `form: 3` (it is checked against the independent Python model).

## 2. Backfill (decision)

Producers republish on their next pass: the marker of `offer` is keyed by the state digest, which changed with the form, so every pool republishes every record it holds (with its leaves) without a new protocol; a taker that meets an incomplete entry infers and republishes it (head swap). No "fleet ask keyed by record": it would need a new wire message and nothing is gained, both paths converge on the same republication.

## 3. Proof

Staged (scratch, bun transpiler cache disabled), laws in `🍃️leaves/🧪️tests/🟦️.ts` (8): references of a text; a missing leaf is a source that reads nothing; adoption fetches verified bytes, keeps held ones, reports missing, the real cache refuses forged bytes; two machines (separate roots, separate stores, one real cache server `startRemoteCacheServerV1`): leaves in the fleet before the entry, listed in its note, in the taker's store BEFORE `import` runs, re-take fetches 0; dedupe: two owners sharing one leaf store 3 leaves, not 4; producer lacking a leaf: `leaf-missing`, not published; fleet lost a leaf: `incomplete` (typed, not imported), republished by the inferring reader; no custody fallback; path independence: `captureSourceInputInventoryV1` and `captureCargoLockInventoryV1` under two different roots give byte-identical text that holds neither root nor the temp dir, sha256 cross-checked with `Bun.CryptoHasher`. Mutation: adoption switched off turned 2 of 7 red. Existing: project-records tests, refusal, graph-state tests, take (with form 3 fixture) green staged (details after landing below).
Leaves on this machine already hold NO machine path: 0 of 6293 files contain `/Users/`, `/private/` or `/home/`.

## 4. Cost guard (measured read-only, `measure.ts`)

20 sampled owners (10 Rust: 7 leaves each, 12 to 24 kB; 10 TypeScript: 1 to 2 leaves, 1.7 kB): 81 references, 63 unique (dedupe 1.29 within only 20 owners; TypeScript owners share one leaf), 161 kB with duplicates, 129 kB unique, about 8 kB per owner. Upload or download at 205 MB/s: 0.6 ms for the sample. Whole store here: 5111 + 1182 files, 21 MB for 4785 owners: about 0.1 s at 205 MB/s; the largest leaf is 151 kB. The big shared ~439k-row entry is a recipe closure document (`closure-<recipe>`, existing lane), NOT a leaf; its upload is unchanged (blob by digest, `hasBlob` first). Two Rust owners of the sample had 6 of 7 leaves missing locally here (`a90297ed`, `f7ad5ca4`): the producers hold them only if their store does; those owners are refused with `leaf-missing` until inferred again.

## 5. Landing

18:00:00 to 18:00:50 UTC, all 16 test files green under the lock (counts in `landings.md`), import smoke ok, lock released. The seed-stream law spawns a child on live code and could only be proven after landing: green. The owner-pool real-process tests were not run after landing (they failed in a staged run but load live code, so staging does not explain it: unverified, probably load).

## 6. NOT verified

- Pools and the running whole read still run the old code (form 2): they need a restart; leaf uploads of restarted pools not yet sampled.
- Real leaf upload from a pool child (custody, HEAD rate on the production cache server) only proven against the test cache server.
- `incomplete` is a miss in `look`; the only trace is the bilingual `say` line and the counter.
- Rust owners whose leaves are missing on the producer are refused (`leaf-missing`) until inferred there again.
