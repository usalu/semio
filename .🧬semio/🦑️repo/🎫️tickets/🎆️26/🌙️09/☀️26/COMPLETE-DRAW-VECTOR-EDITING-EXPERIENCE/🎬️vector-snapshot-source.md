# Vector Snapshot Source Ownership

The native document vector producer now retains a genuine immutable store snapshot read. Its preparation cursor stores bounded source indices instead of references into the document, allowing the producer to own the source without cloning the drawing or manufacturing a lifetime. The same cursor serves borrowed documents and store-owned documents.

The source read remains retained through preparation, image tracing, and Boolean resolution. It can be recovered only after work completes, fails, or is cancelled. Publication still requires the editor to compare its captured instance, revision, and generation against current authority; the producer's authority query does not replace that comparison.

The integration tests construct actual artifact stores with Draw's document owners, capture their real snapshot reads, compare the retained root pointer, resolve the shared traced-Boolean fixture, and check captured generation/revision queries. A second test cancels at preparation, tracing, algorithms, and completion, and exercises sticky work-limit failure. Each case returns the exact read with its store retirement witness and drives store close to terminal. The language-neutral vector fixtures and their third-party oracle remain unchanged.

## Verification

- The initial native run reached the Draw compiler and rejected the missing owned-source constructor, providing the expected red result. The test harness's inaccessible store constructor was corrected to the public constructor.
- The first verification attempt exhausted its 600,000 ms build allowance before tests. It is a failed build observation, not a passing verification.
- The replacement full native run completed successfully: **516 tests passed, zero failed or skipped**, in 7.163 seconds of test execution after an 8 minute 21 second build. Both ownership tests were included.
- The focused uncaptured run completed successfully: **2 tests passed**, with the other 514 intentionally filtered out. Actual `[DEBUG]` output confirms exact read return and store terminal after preparation, tracing, algorithms, completion, and failure. Both test handles are terminal; no native Draw test process remains live.
- Scoped whitespace checking passed for the native implementation and its test.
- The prior producer stage passed 551 TypeScript tests and 514 native tests. Those results predate this native source refactor.

## Remaining Work

The editor still needs actual scheduled geometry activation, complete-plan publication, and shared published geometry for rendering, picking, bounds, and conversions. This source refactor does not by itself change mounted editor behavior. Bounded retirement, fonts and outlines, image/export IO, and full browser and multiuser acceptance remain unfinished. The full editing goal and ticket remain open.
