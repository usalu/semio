# Local Interaction Owner and Query Review

## Reviewed Source and Native Scope

The coordinator read the complete capture, retirement and query modules, including all three query tests. Capture holds an exact Store SnapshotRead under the shared canonical reader, using native borrowed map/vector iterators and fixed-width identity formatting. Its close cursor returns the read and waits for the same registry's reclamation witness. Domain retirement separately drains initialized UTF-8 bytes, empty allocations, map entries and immutable shared roots; the final Arc owner is selected atomically.

The query owns one 256-byte page. It compares request, ordinal, instance, generation and all three full revisions before accepting ACK; a wrong or repeated ACK leaves authority unchanged. ACK hides the page immediately and byte retirement precedes reuse. Cancel hides the page and returns the frozen read through its exact close path. Each concrete encoder invocation writes at most one byte, avoiding ambiguity about a prefix written before a later source error. The hostile native source really errors after a Unicode first field and the second key, and cleanup counts equal the actual encoded prefix.

The coordinator read the native local_interaction_ output: nine passed, zero failed, 429 filtered, 0.06 s runtime after 10.46 s compilation. The live-Drop panic is expected and caught. The independent expanded source target also passes and is retained in `📓️coordinator-local-interaction-source-r2-2026-08-27.md`.

## Shared Reader Error Defect

The shared multi-byte canonical reader still returns only a String error after the encoder may have written earlier bytes to the caller buffer. Its completed_bytes counter then omits that prefix. The new query avoids that path by requesting one byte, but generic reader consumers remain affected. The publication executor owns a typed progress-plus-error repair with actual borrowed/indexed source laws and explicit caller accounting; this must land before the next expensive CAD build. No failure credit is fabricated and no Store code was changed mid-compile.

## Next Live Authority Contract

The interaction executor is mounting fixed read/ACK/cancel commands and response pages through the existing continuation scheduler, with one per-instance query slot and backpressure. This is not mounted at the reviewed checkpoint.

A fixed input-authority token may hash the exact document and config content revisions plus a checked UI-topology revision. It is explicitly not a canonical topology-content hash. Every topology cache insertion, replacement, removal or reset must advance the checked revision before changing the cache, with overflow failing before mutation. Captured immutable inputs, app/request lifetime, late ACK rejection, config-only/document-only/cache-only changes and current-token publication require tests. Restore must still build/read an actual retained topology index and atomically validate current authority; no complete app topology callback is admitted by the token.

The six old reserved semantic tails and full/sparse tutorial restore consumers remain separate unfinished work. Passing isolated page ownership tests does not make those application interactions complete.

