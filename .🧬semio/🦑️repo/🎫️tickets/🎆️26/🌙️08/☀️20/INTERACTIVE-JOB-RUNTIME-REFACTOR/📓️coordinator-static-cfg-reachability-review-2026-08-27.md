# Static Test Reachability Review

The canonical static gate still reports nineteen standalone-test thread findings. The fix must not merely suppress these names: the coordinator read three current projections in the root `📜️script.ts` that can also hide production code.

| Projection | Inspected behavior | Required hostile law |
| --- | --- | --- |
| `interactivityCfgTestItemSpans` | Treats any cfg text containing the word test as test-only | Keep `not(test)` and `any(test, feature="x")` production-visible |
| `policyTestModSpans` | Same word test predicate, or any module named tests, with only a two-line attribute lookback | A name alone is not test-only authority; handle stacked/multiline attributes without hiding live code |
| `toolJobProductionProtocolSource` | Strips all(...) if its text contains test | Keep `all(not(test), feature="x")`; only provable test-only conjunctions may be removed |

One sound lexical cfg/test-item projection should drive these interactivity and command census consumers. It must handle nested all/any/not, ordinary unknown target/feature predicates conservatively, stacked and multiline attributes, cfg_attr, comments/raw strings, nested blocks and standalone test functions. A module filename or the word tests is not a reachability proof. Third-party proc-macro attributes are only test-only when their actual expansion establishes that fact; the owned async_test expansion must be inspected rather than assumed.

Strict language-neutral accepted/rejected fixtures and an independent existing Rust syntax/expansion oracle are required. Fake test attributes in comments/strings and live functions named test must remain visible. The nineteen real diagnostics need exact provenance through the improved projection; other genuine production findings must remain failures. The renderer executor owns this queued verifier repair and must coordinate its root-script edits with the worker-budget executor.

No static-green claim or source admission increase follows from this audit. No production file was changed by the coordinator.
