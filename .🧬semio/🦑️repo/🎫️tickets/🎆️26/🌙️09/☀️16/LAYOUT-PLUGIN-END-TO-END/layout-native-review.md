# Layout Native Evidence Review

2026-10-29. Independent read of the completed first integrated log. No compiler/test rerun or source edits.

## Actual Outcome

The request failed before compiler execution during source admission, at unit658/906, package semio-s-artifact-stdio-idml. The files quota was exhausted at its IO admission/change/part Rust source: observedBytes7081, maximumBytes126026, remainingFiles0. These byte fields describe the refused read and remaining admission allowance; they are not a measured whole-crate byte count. OwnedCommandFailure reports status1, signal null.

No Cargo Running unittests lines, individual test outcomes or test-result summaries exist in this log. Actual executed tests:0. Passed/failed/ignored/filtered test counts are unavailable rather than a successful zero-count test result. No new media, drawing-child, metadata, private font or transform case has execution evidence here. Compiled dependencies would not execute their cfg(test) suites in any event; this attempt did not reach compilation. Explicit typed-operations8 remains its separate future gate.

## Intended Selection

Current router and native-layout-selection.md describe all15 explicit package selectors, owning Layout manifest, --lib --features=testkit -- --nocapture. The log does not print an independent full argument vector, so selection is corroborated by retained request documentation/router, not inferred from source-admission package counters.

- semio-s-artifact-layout-layout-editor-transform-tests
- semio-s-artifact-layout-layout-engine
- semio-s-artifact-layout-layout
- semio-s-artifact-layout-layout-editor-tests
- semio-s-artifact-layout-layout-engine-scene-tests
- semio-s-artifact-layout-layout-engine-flow-tests
- semio-s-artifact-layout-layout-editor-commands-tests
- semio-s-artifact-layout-layout-mutations-content-tests-1
- semio-s-artifact-layout-layout-mutations-content-tests-2
- semio-s-artifact-layout-layout-mutations-frames-tests-1
- semio-s-artifact-layout-layout-mutations-frames-tests-2
- semio-s-artifact-layout-layout-diff-plan
- semio-s-artifact-layout-layout-sqlite-member-tests
- semio-s-artifact-layout-layout-engine-export-unit-tests
- semio-s-artifact-layout-layout-editor

A fifteen-owner selection is not fifteen executed suites. In particular private editor and engine tests require their selected own library roots, and the transform companion cases require its actual test binary. Exact per-case coverage and17-case expectation must be checked on a successful later run rather than predicted from declarations.

## Evidence

Log SHA256: 00a04cc984d27357b5fd7fcef511ee86cca393645d3152bc1b6f5b972ccc8acb

~~~text
error: Rust crate Source refused before compiler: semio-s-artifact-stdio-idml (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"files","path":"/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🔺️change/🧳️part/🦀️.rs","observedBytes":"7081","maximumBytes":126026,"remainingFiles":0,"expectedBytes":null})
~~~
~~~text
error: Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"files","path":"/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🛂️admission/🔺️change/🧳️part/🦀️.rs","observedBytes":"7081","maximumBytes":126026,"remainingFiles":0,"expectedBytes":null}
~~~

Rust and coordinator were notified of the concrete precompiler blocker. No rerun was launched by this review.
