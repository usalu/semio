# Procedural Lane Eviction

Procedural's 11:04 retry entered headroom admission before taking a pooled lane. The prior attempt in `p3430e9` had recorded useful warm work. Its replacement, `w2pN2J2`, received `p9fc596` after the only observed idle lane was reclaimed.

| Local time | Evidence |
| --- | --- |
| 11:04:40 | Free measured at 14,345,412,608 bytes (13.36 GiB). The build log reserves 14.4 GiB and keeps a floor of 2.9 GiB. It reports one other admitted command, with 0.0 GiB still forecast. The request triggers collection for about 4.0 GiB. |
| 11:04:59 | The unit-stage fill sample reports 15,368,577,024 bytes free (14.31 GiB). It contains Energy's live `p29efda` with 9,296,928,768 profile bytes and idle matching `p3430e9` with 5,338,173,440 profile bytes. The recorded gain before lane collection is 1,023,164,416 bytes. Roughly 3 GiB is still required. |
| 11:05:06 | `p9fc596/lane.json` gives birth to the new Procedural lane. |
| 11:05:10 | The following free-space sample is 20,604,096,512 bytes (19.19 GiB), an increase of 5,235,519,488 bytes from the post-unit sample. The build proceeds in the new lane. |

The production Cargo launcher peeks at warm availability for the estimate, then takes headroom before acquiring the lane. GC first scans incremental sessions and older build units, measures the remainder and plans lane retirement. It rechecks the lane claim before removal. An admitted Energy command retains its lane even though its byte reservation has aged to zero. That claim is independent of the admission forecast. The retained profile lists one idle compiler lane, and the earlier reclamation stage left a shortage.

The image is 96 GiB. The headroom estimate uses the warm component class fallback because the current history lacks matching product-specific samples. The nearest-rank 90th percentile of its six samples is 16,546,082,816 bytes. The 15% margin reaches the 15%-of-volume cap, resulting in the observed 14.4 GiB. These values are conservative, and the retained data cannot establish a smaller safe demand for the retried graph. The current component caller already supplies exact declared package names, and the launcher derives the exact crate-set class. Historical warm and cold data predate the current retry and include a wildcard class. Recent publication failures below the reservation do not add samples.

The deterministic replay uses the production estimate, headroom decision and GC planner over the recorded byte counts. SQLite independently computes the capped estimate, floor, shortage and only eligible lane. This replay runs without touching the compiler image. One test passed with nine assertions. Source evidence came from the retained Procedural build log, the compiler cache's fill journal and headroom history, and the replacement lane's birth marker.

The available records establish neither a false estimate nor a safely reclaimable idle alternative sufficient for this attempt. An earlier lane claim would reserve cache space while queued, inhibit the collector, and change the existing headroom-before-lane contract. The audit leaves admission, ordering and GC unchanged.
