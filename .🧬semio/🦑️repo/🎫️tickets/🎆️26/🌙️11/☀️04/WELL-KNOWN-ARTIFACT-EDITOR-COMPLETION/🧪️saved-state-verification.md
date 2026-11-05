# 🧪 Focused Saved State Verification

Command: SEMIO_LANGUAGE=en repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/vitest.saved-state.config.mjs

Bounded configuration: jsdom, one worker, no parallel test files, two saved-state suites only (derivation and lifecycle). Result: exit 0; 2 files passed; 18 tests passed; duration 9.49 seconds (transform 5.22s, import 8.10s, tests 107ms, environment 1.09s).

Hook lifecycle setup uses real useCloseDocumentState and real memory storage/config mutations. Keep cancellation, concurrent prompt supersession and stale-entry retirement pass production-hook tests. Accepted-close paths are not covered by this setup: fake entries intentionally omit lower-level retirement ports, so new accepted-close scenarios will need complete stubs or a production close-port seam.

No product or test harness repairs were necessary. Config uses process.cwd() to remain cross-platform and is ticket input retained for reproducibility. Full host/UI browser suite remains outside this focused evidence.
