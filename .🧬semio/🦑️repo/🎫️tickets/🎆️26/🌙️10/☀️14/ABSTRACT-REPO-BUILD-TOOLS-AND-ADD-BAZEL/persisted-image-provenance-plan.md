# Persisted Image Provenance Follow-Up

The completed normal-custody reader diagnostic found 789 of 794 candidate NP paths present. Fifty bounded records had valid header/result integrity; eight complete Source checks yielded one current record and seven refusals whose first stale file was the process build capacity module. Presence does not establish that ALL9 wrote those records, and exact production key eligibility remains unknown.

## Existing Publication Semantics

`observeInferenceComputationV1` preserves an existing NP record unless a new optional publication actually succeeds. Computation can produce an in-memory project/publication without replacing the persisted record. A replacement requires mutually consistent observations, a non-racy computation, exactly JSON-representable result and currently valid facts. It then supplies the **entire new header plus result** as temporary peak bytes and new-size-minus-old-size as retained delta. Optional refusal leaves the prior file intact; `usedInference` is null for the unpersisted computation.

`withBuildGraphOptionalPublicationV1` returns false for insufficient actual physical headroom, an invocation-local peak reservation refusal, or a false shared ledger publication result. The ledger also returns false on a contended exclusive SQLite transaction, independently of capacity. On its admitted transaction it reconciles physical optional bytes and checks full temporary peak plus retained bytes and physical headroom. Thus a false result is not by itself a capacity diagnosis, and the current implementation retains no per-reason historical denial counter.

Pressure maintenance runs at genuine writer admission, not every project. It may remove only whitelisted recomputable leaves under actual exclusive image custody; a live reader can defer it. The fixed pool is 1 GiB, low water 512 MiB and minimum physical headroom 1 GiB. No policy increase or raw removal is proposed.

The earlier ALL5 sample at 00:12:44 UTC reported 537,118,608 optional bytes and 2,671,648,768 available physical bytes: 247,696 bytes above low water, 536,623,216 below the hard pool maximum. It had 3,788 NP-family leaves, including selected-namespace images around 14 MiB. This is historical ALL5 evidence only. It does not establish ALL9 usage, pending peaks, maintenance outcome or denial, and an aggregate value below the hard maximum does not guarantee every temporary peak fits.

## Source Timestamp Finding

A bounded read of current Source and existing log metadata found:

| Item | Observed value |
| --- | --- |
| ALL9 log birth | 2026-10-15 02:13:18.051 UTC |
| ALL9 log final write | 2026-10-15 02:33:19.008 UTC |
| Current capacity module mtime and ctime | 2026-10-15 02:43:38.430 UTC |
| Current capacity module bytes | 8,581 |
| Current capacity SHA | `fe2119d86f93d4cafe346cdec1bf7b252544e6ba16550f045f871902f108de82` |

The current capacity stamp is after ALL9's terminal log write. It is not evidence of an in-pass edit. These timestamps alone do not prove a byte change; the next bounded reader must compare the seven exact recorded capacity SHA/stamps with current bytes. Log birth/final-write times bound the observed log lifecycle rather than inventing an unavailable original envelope epoch.

## Next Diagnostic Design Only

Reuse the same genuine shared graph-store custody and original finite control, with no FactStore, maintenance, writer or observer fallback. Limit the read to the seven already identified exact record paths; preserve the same complete-record byte and total-byte bounds. Capture each record's no-follow regular identity, inode, mtime/ctime, complete byte SHA, header digest and recorded capacity file fact. Compare the actual header/result integrity and capacity fact SHA, size and stamps to the current Source. Recheck record and Source bytes/stamps before leaving custody. Do not infer publication ownership solely from an NP timestamp.

Report separately: record older than ALL9; record timestamp within ALL9; record newer than ALL9; current capacity bytes equal or unequal to its recorded fact; identical bytes with changed stamps; source or record changed during the diagnostic. An older record plus a current Source change can explain refusal without any ALL9 Source conflict. A record current at publication can become stale after ALL9 terminal. Neither case by itself proves quota denial.

There is no existing read-only ledger opener. `openBuildGraphOptionalLedgerV1` opens/creates its database, configures it, begins an exclusive transaction, inventories physical files and updates reconciled usage. The returned `observe()` reads state only after that mutation-capable opening. Therefore the proposed shared reader must not call this opener or claim its returned observation is a no-write status API. A bounded physical optional-leaf inventory under the existing shared image can describe current usage only if separately authorized; it still cannot reconstruct historical pending peaks or denial causes. No such inventory is run by this plan.

Decisive historical capacity-denial proof is unavailable from the retained counters and files alone. If Source-current sample comparisons and exact producer identity still leave unexplained misses, any future diagnostic must explicitly observe the actual publication decision at its existing authority boundary and distinguish quota, headroom and transaction contention. Changing a revision-bearing Source module to add that observation would invalidate existing image identities and must be reported as a new experiment, not unchanged-source eligibility evidence.

Core owns the next bounded reader implementation only after Root release. No additional diagnostic, production Source edit, cache write, quota change, peer action or ALL retry is performed here. Existing evidence: `🗑️generated/all9-persisted-image-diagnostic.json`, `final-all-current9.log`, and `all-current5-optional-cache-sample.json`.
