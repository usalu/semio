# Flow26 same-byte document reload transient audit

## Scope and evidence

This is a source-only audit. No production file was changed and no command was run by this audit.

The recorded Flow26 run has 251 passes and two failures. The scoped window failure is `flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows`, which reaches the exact message `Flow generation transient survived same-byte document reload` after all four configuration pages and the one transient page settled ([`run.log`](🗑️generated/astra-runtime/flow-native26-full/run.log)). The current test source still contains that assertion at [`window-ownership/🦀️.rs:129-131`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:129).

## Confirmed result: the assertion is stale, not a transient-retirement failure

`VcsArtifactApp::load_document_pack` parses the immutable document, prepares a replacement transient registry, resets the document store, and commits that replacement in sequence ([`plugin/🦀️.rs:31440-31446`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31440)). The preparation advances the document generation; commit swaps registries, retains the displaced registry for bounded retirement, renews the tool cancellation scope, and clears cache ([`document-replacement/🦀️.rs:3-24`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs:3)). This is the expected production reset boundary.

The failing read is not a non-allocating observation. `window_transient_generation` calls `window_transient_store.capture` ([`plugin/🦀️.rs:22762-22768`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22762)). A typed capture calls `partition`, whose `BTreeMap::entry(...).or_insert_with(...)` constructs the absent per-window transient store, then returns its generation ([`transient/🦀️.rs:189-205`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs:189)). Therefore the query after replacement necessarily returns `Some(0)` for the new Flow-generations partition. `Some` proves that the observation mounted a current default owner; it cannot prove that the previous transient survived.

The Flow transient starts as an empty `FlowWindowTransient` ([`transient/🦀️.rs:5-16`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🦀️.rs:5)). The production behavior demonstrated by these paths is exactly a reset to that default state.

## Minimal fail-first replacement law

Keep the existing real Flow app, real `AddGeneration`, and `load_document_pack` route. Before load, retain the snapshot's `document_generation` and require that its Flow transient is non-default. After load, read one `window_transient_snapshot` and require all of:

1. its `document_generation()` differs from the pre-load snapshot;
2. it contains `FlowGenerationsWindowTransientOwner`; and
3. its typed value equals `FlowWindowTransient::default()`.

The current test already uses `window_transient_snapshot` to obtain this typed owner at [`window-ownership/🦀️.rs:117`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:117), so no new fixture or synthetic path is required. The matching Draw ownership oracle checks the typed snapshot against its default after document load ([`window-ownership/🦀️.rs:84-90`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:84)).

This replacement detects both real regressions: a missing reset leaves the pre-load document generation and non-default state, while a failed remount lacks the typed owner. It accepts the intended lazy recreation of the post-reload partition.

## Flow26 configuration receipt update

The same test now settles every configuration command before issuing the next one ([`window-ownership/🦀️.rs:73-85`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:73)) and states the resulting expected count as four `WindowConfig` plus one `WindowTransient` publication ([`window-ownership/🦀️.rs:120-127`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:120)). Per the Flow26 receipt update, camera and grid assertions pass under that sequential dispatch. The source explicitly retains a separate observed back-to-back same-window amend loss as an unresolved production defect; this audit neither classifies nor masks it.

## Confidence

High. The failure's `Some` condition, the allocative capture implementation, and the registry replacement path establish the causality directly. The test replacement should be applied before assigning any production reset repair.
