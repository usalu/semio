# Result S1 (wave 1121): serve test and ByteStream test

Root cause of the ByteStream flake/hang: `google.bytestream.ByteStream/Write` in `remote-execution/🖥️server/🟦️.ts` answered a short upload (fewer bytes than the digest declares) by ending the queue gracefully and awaiting the store PUT. That PUT declared `content-length` N but the body ended early, so the fetch never settled and the call hung until the test limit (reproduced: 60 s with a 60 s limit, stuck exactly at the "short" upload). Fix: the handler now owns an `AbortController`; any failure aborts the store upload before it is awaited. The test now finishes that step in about 5 ms.

Test change: the large blob is `maximumBatchBytes + 1 MiB + 17` (read from the policy) instead of 24 MiB; it still exceeds a batch and spans several 1 MiB read chunks, and the test passes with bun's default 5 s limit (no timeout raised).

Serve test: not reproducible as a hang. It passes in 1.2 s alone (also with the transpiler cache off, 3-6 s) under load average 30-45. One cold run of the file took 48 s wall for 2 s of test time (module load under load), the likely source of the earlier 120 s report; the serve path itself prints readiness, honours cancellation and releases handles (worker exit 0, store and service stopped). No product change was made in `⌨️command/🟦️.ts`.

Stability: `🧪️tests/🟦️.ts` 17 pass x3 (18-26 s), ByteStream alone with default timeout 1 pass x3, `⌨️command/🧪️tests/🟦️.ts` 7 pass x3 (2-4 s).
