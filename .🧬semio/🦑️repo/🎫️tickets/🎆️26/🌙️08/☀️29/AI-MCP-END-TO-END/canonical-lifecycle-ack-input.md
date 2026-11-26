# Canonical Lifecycle Acknowledgment Input

The published Note component's compact 318-byte acknowledgment used 180,202,126 metered VM instructions during input admission, before the kernel dispatched the acknowledgment. The canonical host Poll writer uses one event object with a Captured, Accepted or Retired receipt. The generations are quoted unsigned decimal, the instance ID is a bare unsigned 32-bit integer and the request sequence is a bare nonzero JSON-safe integer. The actor output receipt uses its own existing wire representation.

The typed actor input now recognizes the canonical single acknowledgment alongside the empty Poll and otherwise uses the existing JSON receiver. Both specialized paths share checked unsigned-number and positional limit decoding. The acknowledgment path pays for each byte it examined, the event owner and its single exact vector allocation under the existing Source. It observes the original cancellation and deadline before allocation, keeps custody in the original input receiver until its authenticated return, and uses ordinary paid retirement. The fast path checks JSON depth, per-container item and byte limits, the native ownership ceiling, allocation and release headroom and its local work and copy grants.

The neutral corpus has 14 exact byte strings and compares against typed serde JSON. It exercises every receipt variant and maximum integers, plus valid whitespace, reordered and escaped forms, multiple events and malformed scalar forms. One direct regression audits an exact two-item vector birth and its paid release. A second verifies that short grants and cancellation produce no allocation or typed input. The integration regression compares the same typed result, admission work and paid teardown against the generic route for each receipt variant.

The production receiver compiled in the post-turn relay dependency graph. The first standalone ActorInput test build reached a shared Store edit after compiling the older `MemoryBackbone` dependency. A retry resolved that compile failure and ran the standalone suite. The canonical admission and ordinary paid closure matched the same event decoded by the generic path on all three receipt variants.

| Exact receipt | Typed work | Generic work | Typed capacity | Generic capacity | Typed and generic admission steps |
| --- | ---: | ---: | ---: | ---: | ---: |
| Captured | 311 | 3,928 | 512 B | 371,670 B | 5 and 81 |
| Accepted | 334 | 4,016 | 512 B | 371,919 B | 5 and 81 |
| Retired with maximum integers | 392 | 4,227 | 512 B | 371,974 B | 5 and 81 |

The generic comparison adds one legal leading space to the identical wire. These figures come from the native Source accounting fixture. The published component's interpreter instruction baseline remains a separate measurement.

The first standalone suite found that the direct exact-close law counted the retirement controller's released scaffolding but omitted its matching birth. The corrected test audited both actual effects and verified the remaining release is exactly one 512-byte event. Exact native relink against the fresh 540-dependency Cargo graph passed all four focused acknowledgment tests. Two separate InstanceOpen fixture expectations were still red in the preceding full-suite run.

The next full suite reported five heap-audit failures. The same executable passed all 16 with standard Rust test output capture disabled. An independent relink against the exact 540 Cargo dependencies also passed all 16 in that mode, including both corrected InstanceOpen laws. The failing captured output showed 160 unconditional factory-ticket debug lines emitted inside paid retirement. The test harness allocated storage for those lines while the fixtures were observing retirement heap traffic. Two unconditional diagnostic prints were removed from the shared retirement path. The full standalone default-capture suite was queued against the patched dependencies.
