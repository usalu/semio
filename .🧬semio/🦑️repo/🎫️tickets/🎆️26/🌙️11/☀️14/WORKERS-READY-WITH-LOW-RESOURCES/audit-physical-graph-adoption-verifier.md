# Read-Only Physical Graph Adoption Verification

From the repository root, run the ticket verifier through the light tools route.

```sh
repo tools exec --light bun -- "$ticket/🧰️scripts/verify-graph-adoption.ts" "$fleet_store" "$producer" "$destination" "$expected_head"
```

The first three arguments select the replicated store and the two enrolled origins. The optional expected-head file holds the exact `origin`, `scope`, `generation`, `manifest`, `identity`, `platform` and `implementation` tuple. Pass it to verify a particular producer generation. Without it, the signed receipt supplies the lookup context and an independent producer metadata read still has to match. Use `-` to omit that file when passing a fifth argument for an existing trusted registry pins file. The default is the coordinator's local cache server pins. An optional sixth argument supplies the fleet group when it differs from the producer's prefix.

The verdict contains only status and a reason code. Exit 0 verifies the destination signature against the pinned enrollment, a matching complete producer metadata record, fresh replicated Ready, and a fresh pool report of exactly 16,384 MiB. The telemetry is an observed pool fact and does not prove a physical host identity beyond the enrolled keys or measure a restore's actual peak. Exit 2 is pending. A missing receipt returns before resolving or contacting a cache. Exit 1 refuses contradictory, malformed, incomplete or stale evidence.

For a present receipt the verifier uses the existing complete-only generation metadata function behind the exchange's look interface. It resolves from the existing local signed fleet facts copy without initiating a refresh. The read port permits one producer action key over the existing sealed or TLS-authenticated cache. All blob and mutation operations are disabled. The verifier opens no graph store and writes no files.

The private signed fixture exercised absent and tampered evidence, stale telemetry, the wrong memory class, mismatched and absent producer heads, and an exact enrolled restore observation. SQLite independently checked the decisions. The private sealed cache observed one producer ActionResult GET through the verifier and no blob or mutation request. The focused test passed 28 assertions. The read-only physical peer check returned `receipt-missing` with exit 2. No worker was started and no physical adoption is claimed.
