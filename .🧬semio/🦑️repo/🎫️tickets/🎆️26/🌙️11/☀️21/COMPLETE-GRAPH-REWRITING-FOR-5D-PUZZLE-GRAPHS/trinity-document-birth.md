# Trinity Document Birth Audit

## Verified from source

- Editor and viewer reference birth use `empty_rule_state`; it now constructs a Jack graph with a local Semio child owner. Their `genesis_child_pack` delegates to `genesis_working_child_pack`, which requires that owner and encodes its graph. Initial instance construction seeds derived children from the in-memory snapshot.
- Blank/reset and demo selection publish `Effect::LoadDocument` containing only the encoded parent and an empty document SPR. The demo selector parses `PRIMARY_TEXT` directly. The shipped DSL declares only the Nakagin child reference and root, with no graph content. The test support separately materializes the graph from a child JSON fixture; the production selector does not call it.
- The archive loader calls `complete_document_archive_genesis` for missing members on the hydrated candidate parent, then calls the app's genesis function. Existing archive members are preserved. A parent-only load therefore depends on deriving the member from the decoded parent.
- Rewriting's pack codec encodes its DSL record and reconstructs from that record. Jack's binary codec law explicitly asserts that decoding retains the child address but drops its local owner. Rewriting's genesis function calls `jack_content_for_handle`, which requires a local owner. The existing registered-demo test only counts the effect and compares parsed parent values; it does not apply the load.
- The mounted test log inspected during this audit was still compiling and did not yet contain a test result. No build was launched by this audit.

## Inference and uncertainty

The source path indicates that parent-only blank/reset and demo loads cannot reconstruct the required child after pack decoding, even though in-memory boot can seed the new empty owner. The demo additionally needs its actual graph member, not just an empty child, to preserve the referenced root and useful rewrite input. This conclusion has not been confirmed by executing a mounted load through the registry; any separate hydration that reinstates the local owner before archive completion would change it. A mounted test should apply both effects through the actual registry and assert the restored child contents and authoring work afterward.


## Changes made after the audit

The editor and viewer now share a blank state with the canonical empty Jack child. The archive genesis path can recreate an exact empty child or the authored Nakagin member when a parent-only load has no in-memory owner. It preserves a supplied archive member and rejects a mismatched live owner. The Nakagin child JSON moved from the test fixture tree to the demo assets so the mounted app and fixture consume the same file.

A native regression asserted the initial owned graph and its empty published state. The original full archive fixture later reached all 119 records and waited for terminal settlement. The focused export and parent-only load tests exercise those paths separately.

The mounted parent-only archive test passed for both the blank rule and the demo. It compared the authored rule fields and bindings and verified that each reloaded graph had the expected content.
