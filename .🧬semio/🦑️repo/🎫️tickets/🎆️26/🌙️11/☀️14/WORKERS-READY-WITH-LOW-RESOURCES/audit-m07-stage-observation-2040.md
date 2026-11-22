# m07 Stage Observation at 20:40 UTC

The m07 replica attested executing `e3e9e32d044406facd0c95ae39f1dedc00100821a7fa4643e6e1d7a3e903b67e` with null head and null index tree at 20:39:02 and 20:39:54 UTC. Its incoming queue was zero and server cursor was 186:429042. This is an unavailable Git observation, not an observed alternative stage tree. No Git checkpoint or index-mode cause was present in those mirrored records.

At that same cursor the server had a Git checkpoint for tree `7d8c0b059bcaed05be881279fe59b4fa8305d514`. The server next committed event, ending at 186:429423, was an unrelated ticket Markdown PUT rather than a Git index event. The differing cursors do not establish why m07 could not observe Git.

A fresh m07 attestation at 20:40:46.293 UTC restored the exact tree and normal branch/head; its Git checkpoint was observed at 20:40:37.243 UTC at the same server cursor 186:429042. The evidence shows recovery of observation, not a cause, an index mutation, a detached head, or complete Ready. No index, process or service was changed.

At 20:48:36.200 UTC a later m07 attestation was fresh again with exact tree and `e3e9e32d` code, server cursor 186:438709. Its Ready receipt at 20:48:37.553 was independently current, workspace running and not Ready. At 20:49:06.544 the attestation still had the exact tree and advanced to server cursor 186:440465, matching the sampled head. This records recovery of freshness and cursor progress; it does not establish why the intervening freshness gap occurred or that the later Git-observation implementation was running.
