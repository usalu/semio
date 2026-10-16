# Same-Scan Compact Recency Provenance

## Purpose and scope

Actor's collected prototype 82098 stopped at a serde retirement row whose aggregate maximum moved backwards by 1000 ms while byte count stayed equal. Its narrowly authorized io-base64 observer could not identify serde's original maximum source. This change captures bounded provenance during the actual planning and recensus scans themselves. It does not repair or accept any row, infer writer authority, change timestamp policy, or explain that historical event without a future authorized observation.

## Actual owner integration

`⚡️caching/🧹️pruning/🟦️.ts` now optionally carries a read-only `recencySource` on actual scanned units. The existing default remains off: the original number-Stats walk and returned aggregate shape remain available without diagnostic records. Actual inactive compiler retirement explicitly requests the diagnostic flag for both its planning scan and its recensus scan. No extra post-hoc traversal supplies the printed source.

Inside `measureUnit`, diagnostic measurement performs one bigint lstat at the same measurement point. The record and numeric byte/timestamp fields derive from that single result. The numeric timestamp is reconstructed from seconds plus fractional nanoseconds, then the original `Math.max(recencyMs, atimeMs, mtimeMs)` remains unchanged. File contents are never opened. The source records relative path, clock field, exact device/inode, logical file bytes, and atime/mtime/ctime nanoseconds. Equal maxima select one deterministic path/field in lexical order; this is explicitly one source, not an inventory of all ties.

Incremental sessions include their actual sibling lock's same-scan observation. Crate aggregation adjusts that source path to the crate namespace and chooses a source from the actual selected newest finalized/working sessions. Temporary per-session records are funded before accumulation. Regular unit and incremental logic preserve all previous byte, recency, working-session and lock calculations. Directory timestamps remain excluded.

The existing retirement condition is unchanged: missing row, changed bytes, changed recency, or held lock still refuses before deletion. Only its error formatting calls `describeCacheRecencyChangeV1`, which includes both retained records. The formatter is ordinary diagnostic data and issues no storage, lease, deletion or writer authority. Existing real Original, reservation, compiler writer, Cargo-store, physical locks and descriptor checks remain the authority boundary.

## Finite bounds

| Diagnostic quantity | Limit |
| --- | ---: |
| Retained maximum sources per unit | 1 |
| Source path | 256 UTF-16 units and 1024 UTF-8 bytes |
| Encoded source record | 2048 bytes |
| Cumulative source allocation per actual scan walker | 1048576 bytes |
| Serialized mismatch JSON body | 8192 bytes |

A planning or recensus stage has separate build/incremental walkers, so new retained provenance totals at most 2 MiB per stage and 4 MiB across both live stages, plus bounded current measurement facts. Incremental temporary/final copies are charged conservatively before accumulation. These limits govern the new diagnostic metadata; they are not claims about the pre-existing unit arrays or the entire repository.

Unavailable or over-limit data is explicitly printed as `recencySource:null`, `sourceStatus:"not-retained"`. Records are never truncated into misleading source identities. The mismatch formatter drops source records if necessary to preserve its final JSON limit while retaining all original aggregate/changed fields. The fixed log prefix is separate from this JSON-body bound. The existing diagnostics output lifetime/enforcing store is retained; this adds no new output namespace or writer API.

## Test-driven evidence

Before implementation, the actual finite carried-Original recency prerequisite produced a genuine RED because the diagnostic scan returned no source (`recency-provenance-red.log`). Separate missing-function REDs were retained for the fixed metadata budget decision and mismatch formatting (`recency-provenance-funding-red.log`, `recency-provenance-report-red.log`).

The language-neutral corpus now covers access and modification maxima, equal ties, backwards maxima, fractional timestamps, malformed/duplicate/oversized metadata, and exact byte-budget boundaries. Actual private filesystem scans are checked against independent Python lstat and SQLite reductions. The diagnostic aggregates must equal the ordinary number-Stats scanner, including fractional timestamps; actual maximum-source fields must equal the independent query. A real incremental sibling lock is independently selected as its unit's maximum. Captured earlier records remain frozen and unchanged after later physical timestamp updates.

The same actual received Original owns the finite private fixture and awaited Python children. No structural control is minted by the test, no canonical namespace/data is mutated, and the fixture is removed before its Original settles. Pure oversized-formatter inputs test diagnostic fallback only and carry no physical authority.

The current counterpart results and exact Source freeze are appended after collection. No repeat Actor cohort or release is authorized or launched by this diagnostic change.

## Files

- `pruning/🟦️.ts`: same-scan optional source capture and bounded retention.
- `pruning/🏪️budget/⚙️execution/🟦️.ts`: enable capture in the two real scans and print bounded before/after provenance only after refusal.
- `pruning/⏱️recency/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/🔣️.json}`: neutral diagnostic summary/budget/report contracts and implementation.
- `pruning/🧪️tests/⏱️recency/{🟦️.ts,🔣️.json}`: actual carried-Original filesystem/Python/SQLite laws.
- `build/📥️caller/📤️orchestration/🟨️.d.mts`: one precise declaration for its already-authored `requireBuildOrchestrationContextV1` runtime export; no orchestration runtime edit.

The existing `issuer-bootstrap recency prototype|release` and `types prototype` editor routes run these laws and expanded ES2022/noUnchecked strict. The Actor prerequisite remains registered but is held pending Root review and explicit next-cohort authorization.


## Collected final checkpoint

The final Source is frozen. The actual recency prerequisite is BOTH GREEN, one test / 75 assertions per mode: prototype 1.71 s and release 2.98 s (`recency-provenance-frozen-prototype.log`, `recency-provenance-frozen-release.log`). Expanded ES2022/noUnchecked strict is collected exit 0 (`recency-provenance-frozen-types.log`). Final counterparts include the temporary incremental-record funding fix; no implementation changes followed them.

The actual logs print same-scan maximum-source records and an original-versus-current mismatch using the real earlier/later private scans. The test demonstrates byte equality with a genuine backwards timestamp update while preserving the original record's inode/clock facts. This is deliberate fixture behavior, not attribution of Actor's serde event. The actual production mismatch route is wired for a separately authorized next cohort, which has not run. No unknown file was retired, no historical witness changed and no Source/storage cap increased.


## Reviewed authorized actual cohort

Root reviewed the implementation and accepted the complete BOTH 1/75 plus strict checkpoint. One new actual Actor prototype Original 49501 is live on that frozen Source, through the same `actor-current/📜️script.ts test prototype` route and unchanged 600000/240000 ms budgets (`actor-provenance-prototype.log`). Pack has completed and RetainedTurn has reached actual compiler execution. The actual production retirement scans request same-scan provenance. No recensus mismatch has been observed yet at this writing. The narrow image-held observer remains bounded. A separate release Original is authorized only after a full six-owner prototype pass; otherwise runtime stops at the collected exact failure.


## Collected cohort 49501

Original 49501 is collected terminal exit 1 at 289116 ms. Pack passed 0/0. RetainedTurn reached compiler execution but its preserved Cargo child stopped at `timeout 134716ms`. The launcher creates its 240000 ms admission before store/input/Source/capacity preparation and passes only the remaining time to the child. Thus approximately 105284 ms had already been consumed before this child spawn. Existing logs do not split that interval into individual waits. No recensus mismatch occurred, so no changed-row source cause was collected and none is inferred from the absence of recurrence.

Both actual launches admitted ten of ten cores and a 26 GiB rustc memory budget. Finite automatic retirement completed, physical compiler/descriptor checks passed, and RetainedTurn's initial physical headroom was 4297216000 bytes against the unchanged 4294967296-byte floor. Output advanced through dependency compilation into RetainedTurn test Source warnings; no Rust semantic/compiler error preceded the enforced timeout. A read-only single process snapshot showed a Cargo/rustc chain, which identifies ancestry but does not establish CPU or I/O progress rate. SDK impact is not attributed without timing evidence.

The narrow observer drained 516 samples / 26 events / 28367 encoded record bytes and closed its image before terminal. Release did not run. Runtime is stopped; the next Source-only task is fixed actual phase timing and bounded owned-root CPU-time/elapsed/state/RSS observations for a future separately authorized cohort. Authored 240000/600000 ms limits remain unchanged.
