# Same-generation Note provenance recovery design

The warm single-component pass already delays staging until its selected Cargo invocation exits. The failure retry in `component-pass/🟦️.ts` validates the replacement graph on either side of source admission, checks its footprint against the held lane, and invokes Cargo again with the same target and build directories. It currently handles compiler errors and skips successful links.

## Minimal source capture

Add a source-only observation at the warm pass's compiler-artifact announcement, independent of the existing streaming output prefetch. Use a bounded attempt-local queue and drain it before retry or disposal. Backpressure the stdout reader if needed without acquiring a jobserver token in the capture worker. Use the active compiler invocation's issued control and the existing source witness's finite file and byte storage.

For a newly compiled workspace artifact, read the two existing candidate dep-info neighbors as independent physical inputs. Resolve and validate their paths under the generation's target or build directory. Take only checksum rows for covered workspace paths. If the witness does not already hold that checksum and extent, read the current source body through a bounded physical observation. Retain it only when its exact length and BLAKE3 match rustc's record. Keep this capture out of the permanent session observer so it cannot pin an old output, dep-info file or source claim. Do not register consumed-version selections until final provenance. A later or stale candidate cannot authorize publication. Final receipt discovery must independently select the observed dependency record and verify the same witness.

Test using the existing isolated component-driver fixture and source witness corpus. Admit version A, have the fixture compile B, verify that B is captured at artifact announcement, then write C before delayed final provenance. The expected output identifies B. A second fixture advances the edit to before capture and still expects provenance refusal. Add a queue-pressure row or assert the configured input bound when adapting the fixture.

## Same-generation retry

For a complete link, diagnose exact source-witness disagreement before registering the first one-shot stage or provenance task. Factor dep-info discovery and checksum resolution out of `CargoProvenanceSession.writeInvocation` so preflight and final provenance use the same artifact and source identities. The preflight can record an observed intermediate body only after exact checksum verification. Record each attempt's mismatch independently of the final source witness's selected-consumption state.

Feed the first distinct typed disagreement to the existing `runCargoPassAttemptsV1` diagnose hook and its held-lane `admitWarmCargoRetryGraphV1` callback. Do not enqueue the component pipeline until that hook accepts one successful link. Run the original authoritative receipt session, recheck, seal and ordered model/browser publication for that chosen outcome. Leave multi-component and partial passes on their existing path.

Retrying the permanent receipt session in place after failure would reuse a session marked failed and physical caches that already pin old paths. A direct retry after registration of post-link tasks would also need attempt-private stage paths, distinct completion keys, typed retirement of old stage tasks and a fresh provenance session and custody. The preflight approach avoids those lifecycle changes. No source, build, or lease state was changed during the review.
