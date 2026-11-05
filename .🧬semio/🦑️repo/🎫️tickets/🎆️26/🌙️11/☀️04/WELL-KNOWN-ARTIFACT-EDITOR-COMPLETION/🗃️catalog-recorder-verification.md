# 🗃 Catalog Recorder Integration Verification

New tests exercise actual useCatalogState and actual handleLocalCatalogEventMessage, using controlled worker transport only. Native request wire is decoded, archive bytes use real pack/archive codecs, and schema-first strict status cases are independently validated with Ajv.

Five passing tests cover strict persisted/pending/failure statuses, stale-client exclusion, authoritative ref update before observer callback, optimistic and remote projections never acknowledging writes, failed status not issuing additional probes, sequential mutations completing only on specific disk archive readback while preserving unrelated concurrent docs, and cancellation preserving a successor observer.

Focused command: SEMIO_LANGUAGE=en repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/vitest.catalog-recorder.config.mjs. Result:exit0,1file5tests passed,duration12.00s,tests277ms. No product edits.

Review hazard sent to root: recordLocalCatalogRef currently captures generation inside queue continuation, so a queued old request can migrate to a new catalog generation. Capture generation on invocation and check again when dequeued. Worker identity should be included where replacement does not necessarily change catalog client.

## Queued generation regression

Added a real-hook regression queuing two admits before replacing localDataDir. Both old promises resolve null and the queued operation sends no write to the successor catalog. Corrected the harness to retain the actual mocked worker in backboneWorkerRef, matching production exact-worker ownership. Focused single-worker Vitest: 6/6 pass, 12.13 seconds. The earlier 3 failures were the obsolete null worker mock; no product weakening was needed.
