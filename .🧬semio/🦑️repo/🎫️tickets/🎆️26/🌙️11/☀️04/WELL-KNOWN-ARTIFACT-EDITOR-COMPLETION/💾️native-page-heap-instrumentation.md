# Native Page Heap Instrumentation

Read-only source audit. Observing trace counters does not install an allocator; absent HeapWitness/ObservedAllocator can make zero-heap assertions falsely green.

| Native target | Allocator finding before root repair |
|---|---|
| pending-original-page integration4 | Missing binary global |
| borrowed-publication-wire integration3 | Missing binary global |
| mounted-ui-copy integration2 | Missing binary global |
| borrowed-json-sink integration2 | Missing binary global |
| actor --lib backing2 | Actual actor root cfg(test) global ACTOR_TEST_HEAP:HeapWitness |
| native-decoding --lib encoding1 | Actual value-testing-allocator normal devcrate global ObservedAllocator; do not add another |
| tests16 original-emission1 | No global in five explicit mounted source files or inspected normal testing companions; root notified to install binary witness |
| peer --lib metadata3 | Root found missing global and added test-only PEER_HEAP before fresh red |
| native-identity nativeclose1 | Actual native-window test root 🧪.rs global HEAP:HeapWitness |

The actor witness is cfg(test), so linking actor normally does not instrument integration binaries. Likewise native-decoding's allocator devdependency does not propagate into consumers. The four integration package manifests contain no normal allocator dependency; job testkit caller/testing fixtures contain no global. No conflicting allocator was found in the inspected normal paths. This is source inspection, not a full Cargo-resolved allocator uniqueness proof; actual compiler rejects duplicate globals and remains authoritative.

Root owns adding the four integration globals and tests16 witness. No source edits or native jobs were performed by this audit. New results must be recorded after instrumentation; uninstrumented zero counters cannot establish physical receipt parity.


## Tests16 normal dependency follow-up

Canonical binary root is plugin/companions/tests16/🦀.rs, package manifest at the same owner's packages/rust/Cargo.toml. Its five explicit mounted source files contain no global allocator. A recursive TOML dependency walk resolving workspace paths, normal dependencies and root dev dependencies found409first-party packages, zero unresolved workspace bindings and zero allocator packages; value-testing-allocator is not linked through this graph. Inspected normal testing companions02/05, fixture/law owners and job testkit caller have no global. Actor's known witness remains cfg(test) and inactive as a normal dependency. Root was advised to add the test-only binary HeapWitness; no active conflicting allocator was found. Manifest resolution is not a full Rust cfg evaluator, so final compiler allocator uniqueness remains authoritative.


## Instrumentation freeze marker

Root confirms six native roots now have actual test-only instrumentation: peer, four integration binaries (pending, wire, mounted UI, borrowed sink), and tests16. All six AST checks pass; this marker changes test instrumentation only, with no production change. Native runs must use this snapshot or later; no physical pass is inferred from parser success. Existing actor/native-identity witnesses and lower decoding ObservedAllocator remain single owners, without an added duplicate.
