# Shared Strategy POSIX EOF Attribution

Current W runtime is SHA c823a977d7644c46898e9789246e96db4a863608fb690a5d35d2ee795ffdcf5d. It is still frozen while nonbenchmark Source57465 and refreshed strict qualify the baseline. No EOF correction or new benchmark has executed.

## Evidence And Limits

Saved controlled4119 reports a complete no-op capture with113,174 observer calls. Current36536 reports a complete no-op capture with117,270 calls and an incomplete publisher capture at the existing55-second phase deadline. The difference is exactly4,096 =2 censuses ×2,048 files ×1 additional observation. Current36536 has custody0/0, cleanup0 and Native0; it does not establish complete publisher equality or performance qualification. Old4119 wall time is historical, and external load/cache/order prevent a causal wall-time claim.

The current Source EOF segment performs `io(EOF read)`, `check()`, `sync(io(final FD stat))`, `exact(named stamp)`, then ancestor verification. There is no await or IO between the post-EOF `check()` and the opening check inside the following `sync`. In contrast, the retained bounded-IO contract requires the full sequence initial FD stamp, complete stream, one-byte EOF probe, final FD stamp, exact named stamp and all ancestors, followed by close/postclose proof. The proposed POSIX correction restores one synchronous EOF/stat/named bundle with its existing outer callback pair. It does not remove any actual IO, IO work debit, Original/child phase guard, named comparison or callback failure boundary. The Native awaited EOF path stays unchanged.

The current Git HEAD contains an older asynchronous implementation and is not the immediate pre-Batch1 byte baseline. It is not used as exact provenance for the proposed hunk. Exact old aggregate counts, current Source order, and new actual Source/independent trace counterlaws establish the correction; no missing Source bytes are inferred from a digest.

## Schema First RED Plan

Add a closed constant POSIX EOF trace contract inside the existing Windows WitnessStrategyProtocol and matching authored corpus protocol. This adds no root definition or production configuration. Cases are an ordinary empty file, a frozen callback failure at the pre-EOF observer, and an undefined failure at that same boundary. Each executes the actual extracted W stream/check/IO/sync/exact/verify helpers inside the real synchronous descriptor owner with a genuine issued Original phase and an actual empty Generated file.

The ordinary trace must show the initial observer/FD-stat/observer pair, then the outer pre-EOF observer, EOF read, final FD stat, exact-name observer/named-stat/observer, and outer completion observer. The four actual metadata/read operations preserve four work debits and their before/after Original-first then child guards. The ordinary stream retains six observers; current Source has seven. Both failure cases must preserve the exact arbitrary primary and refuse before the EOF read; current Source performs EOF before that callback. All accepted descriptors close and custody returns0/0. Independent Python computes the trace/hash expectations and SQLite checks observer/work/IO arithmetic. Current Source must produce actual semantic RED before a narrow branch-only correction.

After the RED and baseline gates close, make only the POSIX EOF/stat/named synchronous bundle correction, leaving Native code and X/C/H/P/F untouched. Run the new trace laws, existing bounded IO/fault/ancestry/boundary/progress laws, full Windows, all nonbenchmark P/W Source and refreshed strict. Independent Low reviews the exact hunk and Source holds. A new unchanged55/65/120 complete benchmark requires Root CPU-quiet release after semantic qualification; no budget or fixture-width change is proposed.
