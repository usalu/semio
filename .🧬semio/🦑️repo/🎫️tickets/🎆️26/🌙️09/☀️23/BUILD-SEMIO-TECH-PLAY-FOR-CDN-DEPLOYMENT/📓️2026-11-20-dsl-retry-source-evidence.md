# OS DSL receiving edits during descriptor retries

`wvuq3J7` descriptor attempts 2, 4, 5 and 6 report a typed input change on `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs`. Attempts 5 and 6 have independent evidence of distinct byte-changing writes. The logs for attempts 2 and 4 identify the path but record neither the expected digest nor the observed digest.

| Attempt | UTC log file birth–last write | Cargo phase | First semio-os-dsl admission |
| --- | --- | ---: | --- |
| 2 | 15:44:17.112–15:47:43.710 | 85.302 s | Memo recall, 21 files and 119,283 bytes |
| 3 | 15:47:44.975–15:50:29.027 | 52.984 s | 36 read operations, 22 files. Fails on the separate converted source |
| 4 | 15:50:33.298–15:55:17.045 | 181.579 s | 36 read operations, 22 files |
| 5 | 15:55:29.310–15:56:34.541 | 21.020 s | 36 read operations, 22 files |
| 6 | 15:57:04.845–16:01:02.929 | 118.201 s | 36 read operations, 22 files |
| 7 | 16:01:33.195–ongoing at 16:03:46.332 | Ongoing during review | 36 read operations, 22 files |

Within each attempt's log, two pre-spawn rechecks of the same unit use its memo. Their memoized closure byte totals for attempts 3 through 7 are 131,483, 131,732, 131,942, 132,378 and 132,378 bytes. Those are package-closure totals, which can change through other files and cannot establish this path's exact previous digest. The first admission in attempts 4, 5 and 6 went through source reads, which excludes a pure recall of the original attempt-2 memo by those attempts.

## Physical evidence

- The matching unit-admission memo under `.cache/b/r/m/u/a/f2/f23e5ce73c4b12253c9271e25f8c88aefe6097a189eb5090d2b4eeb7b14460b7.json` was created **15:57:00.692 UTC**. It records receiving source modification **15:56:16.002 UTC** and change time **15:56:16.460 UTC**, within attempt 5. Inode **37107839**, length **28,070 bytes**, SHA-256 `b915042693ae27e6d7a3d3b2bc539a7a528edb11bdb40861df17bf59015dd028`.
- The live file was independently hashed between matching stat observations at **16:02:46 UTC**, and a second read around **16:03:46 UTC** produced the same digest. Its birth and modification times were **16:00:31.745** and **16:00:31.767 UTC**, with change time **16:00:33.038 UTC**, inside attempt 6. Inode **37125693**, length **28,049 bytes**, SHA-256 `d39b4fa801cecd64be205f8cf39e28f52babcf0d9a065a96b8ef73bdb51f211c`. The later inode, size and bytes differ from the attempt-5-period version.
- Three older memo variants found in the scan still hold the November 14 timestamp, inode **783750**, length **27,343 bytes**, and SHA-256 `270006c31cff4187c5ad28fcd4366ef1116e09e641b86fa1a4bc3d1680d5aae2`. The logs do not identify the memo key used by attempt 2. The exact attempt-2 and attempt-4 version pairs were unavailable.

Within any one failed process, the same path is reported again during the two internal backoffs at 2 and 8 seconds. Those lines show the existing convergence check, rather than two additional writes. The subsequent new job process starts fresh source admission. For attempts 5 and 6, the live and cached observations rule out the explanation that one unchanged physical file disagreed with a single retained digest across both processes. The record establishes no source-memo defect. No Cargo job or production source was changed.
