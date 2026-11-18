# Owner pool failures and the g1-m05 dependencies timeout

Read-only investigation, 2026-11-16 00:26 to 00:40 UTC. No builds, no tests, no signals, no git writes, no signed fleet requests.
Paths are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/` (abbreviated `B/`) unless shown absolute.
"Inferred" marks conclusions drawn from code or logs, not observed directly.

---

## Q1. Why the fleet cache refuses owner records

### 1.1 Samples of `pool.json` (what the fleet reports)

Method: 110 samples, 11 rounds, 30 s apart, 00:26:23 to 00:31:23 UTC, of `/Users/Shared/semio-store/v1/origins/*/pool.json`. Each sample is the single `lastError` string the pool currently holds, so the counts are snapshots, not failure events. g1-m09 has no `pool.json`.

| Class (from `lastError`) | Samples | Meaning (see 1.3) |
|---|---|---|
| refused (generic fallback text) | 48 | `the record was not stored: the fleet cache refused the entry (9 documents, ~6.5 MiB)` (44 samples), `(9 documents, ~5 MiB)` (3), `(29 documents, 96 MiB)` (1) |
| no answer | 20 | `owner pool: the fleet cache did not answer, the waves of this machine are kept` |
| child ended | 11 | `owner pool: a child ended (exit 0)` (all on g1-m01) |
| claim lost | 5 | `the record was not stored: the claim was lost before the upload` |
| too large | 4 | `the record was not stored: the entry is 521 MiB in 110 documents` (3, owner `repo-policy`), `the entry is 1524 MiB in 290 documents` (1, owner `repo`) |
| none (empty) | 22 | g1-m05 (11), g1-m06 (11) |

Per origin (samples): g1-m01 child-ended 11; g1-m02 refused 10, claim-lost 1; g1-m03 refused 11; g1-m04 no-answer 6, claim-lost 4, refused 1; g1-m05 none 11; g1-m06 none 11; g1-m07 refused 5, no-answer 3, too-large 3; g1-m08 no-answer 11; g1-m10 refused 10, too-large 1; g1-server refused 11.

Failed counters in `pool.json` at 00:26: g1-m01 20, m02 43, m03 9, m04 54, m07 46, m10 7, server 41, others 0 (total 220).

Owners named in the failure samples (57 owner-named failures): `semio-s-artifact-procedural-2d-editor-commands` 11, `semio-s-artifact-norm-din16798-io-text` 11, `semio-s-artifact-energy-model-native-derive` 10, `semio-s-artifact-bim-model-spaces` 4, `semio-s-artifact-procedural-3d-geometry` 3, `semio-s-artifact-raster-raster-examples` 2, `semio-s-artifact-bim-model-mut-any-61` 2, and 1 each for `trinity-rewriting-v1`, `norm-en1995-inferences`, `norm-en1996-mut-walls`, `norm-en1997-mut-layer-water-more`. Cargo and Go owners (`@semio-tech/stdio-obj-rs` 2, `os-db-compact-rs`, `framework-plugin-host-imports-rs`, `repo-hooks-go`) appear 1 to 2 times each. Owner-level breakdown: `semio-s-artifact-*` 47 of 57 (about 82 %). Owner `repo` and `repo-policy` (the too-large cases) 4. The `@semio-tech/plugin-registry` 1046 MiB case from the brief is not in these samples.

### 1.2 Does the pool keep a failure log?

No. `pool.json` holds one `lastError` string; `flush` rewrites the whole file (`B/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/📊️status/🟦️.ts:12-21`). The schema keeps `lastError` only (`…/owner-pool/🧬️schema/🔣️.json:344`).

The R49 "failed with cause" notes are not on disk. They are written to the fleet cache under the state `ownerPoolFailureStateV1` (`…/owner-pool/🟦️.ts:35-40`, `publishOwnerPoolFailureV1`) and read back with `readOwnerPoolFailureV1` (`…/owner-pool/🟦️.ts:43-51`), kept for `failureMaximumAgeMilliseconds` 1 200 000 (20 min, schema line 47). Only a failure with `deterministic=true` and a known digest is written: `…/owner-pool/🔬️inference/🟨️.mjs:44`.

Consequence (inferred from the code, important): the deterministic flag is set only on the missing-record path (`B/🪪️graph-state/📦️project-records/🟦️.ts:98-100`, `sure=!missing` in the `documents===undefined` branch). The size refusal and the generic refusal leave `sure=false`, so no failure note is written and every machine re-infers and re-uploads the same refused owner. That explains repeated failures on `semio-s-artifact-*` across all pullers.

### 1.3 Where each failure text comes from

- `the record was not stored: <why>`: `…/owner-pool/🟦️.ts:158` (`claim.why()` is the reason).
- `the fleet cache refused the entry (N documents, K KiB)`: the fallback at `B/🪪️graph-state/📦️project-records/🟦️.ts:100`. It is used whenever `publishGraphEntryV1` returned `false`, the hold was not lost, the document-size check passed, and upload rights exist. It is a catch-all, not a server refusal reason.
- `the entry is N MiB in M documents, above the limit of …`: `B/🪪️graph-state/🛰️exchange/🟦️.ts:83-85` (`graphDocumentProblemV1`, added by R59).
- `the fleet cache did not answer`: `…/owner-pool/🟦️.ts:55` (pointer read returned `"fault"`), text at `…/owner-pool/🧬️schema/🔣️.json:227`, written by `…/owner-pool/🏃️runner/🟨️.mjs:46`.
- `the claim was lost before the upload`: `project-records/🟦️.ts:100` (`hold.lost()` before `publishGraphEntryV1`).
- `owner pool: a child ended (exit 0)`: `…/owner-pool/🏃️runner/🟨️.mjs:82` (child exited before it drained; backoff 30 s when it started less than 10 s earlier).
- `… a child held N MB, above … MB, and was stopped`: `🟨️.mjs:70` (not seen in the samples).

### 1.4 Why a 9-document, 6.5 MiB entry is refused

Not the size limits: `entryMaximumBytes` is 512 MiB (`B/🪪️graph-state/🧬️schema/🔣️.json:32`, 536870912) and `documentMaximumBytes` 256 MiB (line 31). `publishGraphEntryV1` (`B/🪪️graph-state/🛰️exchange/🟦️.ts:158-173`) returns `false` for: client not usable (line 161; breaker, `🧬️schema/🔣️.json:38` down 15 s, denied 60 s, no-claims 600 s), upload off (161), lost hold (161), more than 4096 blobs (164), any `hasBlob`/`putBlob` failure (166-167, blob timeout 20 s + size / 2 MiB/s, `🔣️.json:37`), or a `putResult`/`swapHeadV1` refusal (170). The pool cannot tell these apart, so all show the same text.

Candidates, in order of my belief (inferred, none observed):
1. Breaker open after no-answer faults: `usable()` false, so the publish is refused at once. Matches the timing of the no-answer samples on m04 and m08.
2. Server-side refusal of a blob or result (store outcomes `too-large`, `capacity`, `digest-mismatch`, `aborted`, `B/🌐️remote-cache/💾️store/🟦️.ts:239-281`), mapped to `false` by the client. Capacity is unlikely on this host: the local cache-server directory `/Users/ueli/Documents/semio/.cache/f/cache-server` is about 787 MB on disk (mirror 523 MB) against a 2.5 GiB bound (`B/🔣️policy.json:37`). The g1-server store was not measured.
3. Blob upload outlasting its 20 s + size time while the parent event loop is blocked (RSS 4 to 7.7 GB per pool, host load up to 86; R59 section 6 candidate (b)).

### 1.5 The limits that refuse an entry (current values)

| Limit | Value | Where |
|---|---|---|
| Document bytes (pre-compression) | 268 435 456 (256 MiB) | `B/🪪️graph-state/🧬️schema/🔣️.json:31`, checked `…/exchange/🟦️.ts:84` |
| Entry bytes (pre-compression, sum of documents) | 536 870 912 (512 MiB) | `…/graph-state/🧬️schema/🔣️.json:32`, checked `…/exchange/🟦️.ts:85` |
| Documents per entry | 4 096 | `…/graph-state/🧬️schema/🔣️.json:33`, checked `…/exchange/🟦️.ts:164` (and `:96`) |
| Claim TTL / heartbeat (graph state) | 30 000 ms / 8 000 ms | `…/graph-state/🧬️schema/🔣️.json:27-28` |
| Blob timeout | 20 000 ms + 2 MiB/s | `…/graph-state/🧬️schema/🔣️.json:37` |
| Breaker | down 15 s, denied 60 s, no-claims 600 s | `…/graph-state/🧬️schema/🔣️.json:38` |
| Owner pool claim TTL / failure note age | 300 000 ms / 1 200 000 ms | `B/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧬️schema/🔣️.json:44, 47` |
| Cache server total store | 2 684 354 560 (2.5 GiB) | `B/🔣️policy.json:37` |
| Cache server entry (policy) | 536 870 912 (512 MiB) | `B/🔣️policy.json:38` |
| Cache server entry (effective) | min(policy entry, floor(total / 2)) = 512 MiB | `B/🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts:37` |
| Store write: entry too large | declared or streamed > entry bound | `B/🌐️remote-cache/💾️store/🟦️.ts:239, 246, 258, 272` |
| Store write: capacity | total + reserved + extra > total bound (after collect) | `…/store/🟦️.ts:261, 265, 273` |
| Action result size | 4 194 304 (4 MiB), HTTP 413 | `B/🌐️remote-cache/🖥️server/🟦️.ts:80, 176-178` |
| Claim body | 4 096 bytes | `B/🌐️remote-cache/🎟️claims/🟦️.ts:159` |

R59 added the explicit document and entry limits (`graphDocumentProblemV1`, bilingual text, used by `publishGraphEntryV1` line 162 and by the pool's failure path). R59 section 6 states the refusal cause was "Not identified"; the limits it added are the 256 MiB and 512 MiB pre-compression checks, which do not explain the 9-document refusals.

### 1.6 Status endpoint (read-only, GET /status)

- Addresses from `/Users/Shared/semio-store/v1/fleet/cache.json`: cache at `http://192.168.178.70:6094` (publisher `ueli-mac.fritz.box`, directory `/Users/ueli/Documents/semio/.cache/f/cache-server`), execution `http://192.168.178.70:8980`, scheduler `…:8981` (both held by one bun process, pid 86571, per `lsof`).
- `127.0.0.1:8980`, `:8981`, `:6090`: no listener on the loopback address (curl code 000).
- `GET http://192.168.178.70:6094/status` and `…:8981/status`: HTTP 401 "Unauthorized". The fleet gate requires a signed request (`B/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts:364-383`). I did not sign anything and did not read keys or tokens.
- Result: the divergence and refusal counters by class were not obtained. To get them, use the signed fleet status path (the `🚢️fleet-status` module), not a hand-made request.

### 1.7 Why a 1 046 MiB owner is in 227 documents (R59, as far as the files show)

- An owner entry is one `record` document plus one `closure-<key>` document per tool closure the owner consumed (R59 section 1). Owners with many tool closures have many documents.
- R59 section 3: the owner's own data is 0.003 to 0.13 % of the bytes; 99.9 % is shared tool-closure material. So the size is set by the number of distinct closures, and owners that consume many closures (`repo`, `repo-policy`, `@semio-tech/plugin-registry`) cross 512 MiB.
- Inconsistency (inferred): R59 reports closure documents of at most 6.5 MB raw and 0.7 MB packed. 227 documents at 1 046 MiB average 4.6 MiB each, which the R59 figures do not describe. So either those entries were produced by a build without the packed-row form, or the 1 046 MiB number counts something other than the packed documents. The files do not settle which. Check the build on the producing machine.
- Note `🪪️graph-state/📦️project-records/🗜️packed-rows` (R59) is the form the limit is meant to apply to.

### 1.8 Fix for Q1 (concrete, with location)

1. Set `sure=true` when `graphDocumentProblemV1` returns a problem, in `B/🪪️graph-state/📦️project-records/🟦️.ts:100`, so the failure note is written (`…/owner-pool/🔬️inference/🟨️.mjs:44`) and the other pullers stop retrying. Same for refusals known to be permanent.
2. Replace the boolean in `publishGraphEntryV1` (`B/🪪️graph-state/🛰️exchange/🟦️.ts:158-173`) with a reason (breaker open / blob PUT status / store outcome `too-large`, `capacity`, `digest-mismatch`). Remove the catch-all fallback text at `project-records/🟦️.ts:100`.
3. Keep failure counts by class in `…/owner-pool/📊️status/🟦️.ts` and its schema, so `pool.json` shows `refusedCapacity`, `refusedTooLarge`, `breakerOpen`, `claimLost` counters instead of one string.
4. Split owners with more than 512 MiB of closure documents (chunk into several entries, or keep closure documents out of the record), at `…/owner-pool/🟦️.ts` publish path and `project-records`. Check first why the producing build emits 4.6 MiB documents.
5. Claim lost: pass the owner-pool claim TTL (300 000 ms, schema line 44) to the record claim (`project-records/🟦️.ts:94`, `ttlMilliseconds`), instead of the 30 s graph default, or renew the hold from a timer that is not blocked by the child (inferred cause).

---

## Q2. g1-m05 Windows `dependencies` stage (stage-timeout, try 8)

### 2.1 State and tries (from `ready.json`, `ready-timings.json`, `ready-log-dependencies-failed.txt`)

- `ready.json` (01:25 local file time): state `backoff`, stage `dependencies` (3 of 5), attempts 8, refusal `stage-timeout`, next attempt 1794789569702 ms = 00:39:29 UTC, `reason: adopted` (a running stage was adopted, not restarted).
- `ready-timings.json` `dependencies` runs (all times UTC, 2026-11-16):

| Try | Start | End | Seconds | Outcome | Gap before (s) |
|---|---|---|---|---|---|
| 1 | 19:17:23 | 19:32:39 | 916 | stage-failed (exit 1) | |
| 2 | 19:50:16 | 20:06:37 | 981 | stage-failed (exit 1) | 1057 |
| 3 | 20:07:42 | 20:23:21 | 939 | stage-failed (exit 1) | 64 |
| 4 | 20:41:12 | 20:57:04 | 952 | stage-failed (exit 1) | 1071 |
| 5 | 21:01:22 | 21:17:24 | 962 | stage-failed (exit 1) | 257 |
| 6 | 21:25:58 | 21:42:20 | 982 | stage-failed (exit 1) | 514 |
| 7 | 21:59:28 | 22:59:29 | 3600 | stage-timeout | 1028 |
| 8 | 23:19:29 | 00:19:29 | 3600 | stage-timeout | 1200 |

- Tries 1 to 6 died about 950 s in, right after `Checking declared package providers: 7748 edges`, with the Windows relative-open error `ERROR_ACCESS_DENIED` at `C:\` (R64, fixed in the landing before try 7).
- Tries 7 and 8 passed that point and ran into the cap.

### 2.2 What the stage was doing when the cap hit (try 8, `ready-log-dependencies.txt`)

The file is a rolling 124-line tail. Its first line is `still working after 3410 s`, its last `still running after 3590 s`. Earlier lines of try 8 are not in it, and the provider lines may come from an earlier try (inferred).

- Phase marker `[repo] still working after N s; last phase started: source.served` is repeated 6 times between 3410 s and 3590 s. The phase never advanced in the tail.
- Lock activity dominates: `bun:dependency-storage remains active on dependencies:store-ownership:frontier:2080320 (exclusive, 3461 s)`, `…store-ownership:readText:1984320 (exclusive, 3456 s)`, and 50 `…store-input-replay` lines with `mode: shared, elapsedMs: 0` (the replay is waiting, not working).
- Progress counters: `Checking declared package providers: N edges` reached 7748 and stayed there; the last 8 lines are all 7748.
- Ownership index (resource name suffix): readText 1887744 at 3338 s, frontier 2075648 at 3461 s, frontier 2080320 at 3461 s. Rate between 3338 s and 3461 s is about 1 560 index units per second (inferred, the index is not documented as a progress count and the total is not logged).
- Stage cap: `still running after 3600 s` then the kill (SIGTERM, SIGKILL after the grace period), `🚦️ready/🟦️.ts:333`, outcome `stage-timeout` (`🚦️ready/🟦️.ts:241`).

### 2.3 Stage caps (`B/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json:65-69`)

| Stage | Cap | Line |
|---|---|---|
| native | 7 200 000 ms (2 h) | 65 |
| repo-mcp | 3 600 000 ms (1 h) | 66 |
| dependencies | 3 600 000 ms (1 h) | 67 |
| tools | 3 600 000 ms (1 h) | 68 |
| workspace | 14 400 000 ms (4 h, raised by R62) | 69 |

The cap is wall-clock from the stage start (`🚦️ready/🟦️.ts:333`), and it is not in the stage digest, so changing it does not re-run a stage (R62 note).

### 2.4 How long the stage would need on this machine

- Macs (`ready-timings.json` `dependencies`, outcome ok): g1-m01 20 s and 23 s; g1-m02 9 s; g1-m03 45 s; g1-m07 11 s; g1-m08 21 s; g1-m10 126 s and 139 s. (The brief says 9 to 900 s; in these files the 900 s values are the g1-m05 failures, and the successful Mac runs are 9 to 139 s.)
- g1-m05 has been past 3 600 s in two tries and has not finished. It needs more than 3 600 s, which is at least 26 times the slowest successful Mac run (139 s). The total is not computable from the files: the log has no total, and the lock counters have no known length.

### 2.5 Is the work resumable across tries?

Partly (inferred from the code and the log):
- Completed generations are verified and reused: `verifyJavascriptDependencyCompletedExportsV1` (`B/📦️dependencies/💾️storage/🟦️.ts:510-517`) and the ready descriptor (`writeJavascriptDependencyReadyDescriptorV1`, line 503). So a finished generation is not rebuilt.
- The provider check (`Checking declared package providers`) and the store-ownership frontier walk restart in each try: the same 7748-edge line appears after each restart (R64 saw it at about 950 s in tries 1 to 6), and each try holds the exclusive lease from the beginning. No checkpoint of the frontier position was found in the files.
- Each timeout kills the whole stage process group, so an in-flight generation is lost and rebuilt.

### 2.6 Fix for Q2 (concrete, with location)

1. Immediate, for g1-m05 only: raise the `dependencies` cap in `B/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json:67` (3 600 000 to 14 400 000, like `workspace`). The digest does not include it, so the stage is not re-run.
2. Real fix, the Windows path: the store-ownership walk takes an exclusive lock per frontier step and the provider check walks 7748 edges on every try. Find why Windows holds `dependencies:store-ownership:frontier` exclusively for an hour (`B/📦️dependencies/💾️storage/🟦️.ts`, the `frontier` and `readText` resource calls, lines 180-220 and 510-530) and make the walk shared or batched.
3. Resumability: persist the provider-check result and the frontier position under the stage's `.cache` directory (checkpoint written from `control.publish`, `…/storage/🟦️.ts:524`), so try N+1 starts at the last position. Log the total number of frontier steps at the start, so the remaining time can be read from the log.

---

## Not done, and why

- Q1 counters from the cache server (item 3 of the brief): blocked by the signed-request gate (401). Not worked around.
- The 1 046 MiB `@semio-tech/plugin-registry` entry: not present in the samples; only the `repo` (1 524 MiB) and `repo-policy` (521 MiB) entries were seen.
- Sampling covered 5 minutes, so rare classes may be missing.

Scratch files (sample log and sampler output) were written to the session scratchpad, outside this ticket, and removed after this file was written.
