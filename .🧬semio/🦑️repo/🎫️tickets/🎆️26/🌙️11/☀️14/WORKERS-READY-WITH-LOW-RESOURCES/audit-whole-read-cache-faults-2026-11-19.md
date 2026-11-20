# Whole-read cache fault audit

The graph read began at 13:12 UTC on 2026-11-19. The cache sidecar log records restarts from 14:28:55 to 14:31:06, 14:32:09 to 14:35:34, and 16:29:34 to 16:31:26 UTC. These outages overlap the read and can cause transport faults. The logs do not timestamp individual graph fault counts or tie any particular lookup to an outage, so they do not establish how many faults the restarts caused.

At the inspected tail, 1,353 fleet records had been fetched, showing reuse succeeds. The adoption counter separately showed 23,414 faulted lookup attempts; that is not a count of distinct owners. Fetch faults combine unavailable transport, invalid metadata or blob delivery, and unpacking failures; the pre-instrumentation read does not distinguish them. Earlier GET errors and disk-pressure PUT refusals in the same historical sidecar log precede the current read and must not be attributed to it.

No request, cache file or process was changed for this audit.
