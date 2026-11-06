# Remodel Metadata Probe — 2026-11-05

One normal owned repo cargo metadata --no-deps probe for the remodeling workspace, shared prototype and one requested job, ended exit 1 (handle 76497). The repository ingress attempted to refresh Cargo.lock and its Cargo subprocess returned status 101 before metadata: semio-framework-ui-declaration was not found at the configured path. The requiring package is semio-framework at 🧰️framework/📦️packages/🦀️rust/Cargo.toml:21; its target declares semio-ui-declaration, while the framework source already uses semio_ui_declaration. This is an actual selected resolution error, not a test or compiler result. Captured output is 🗑️generated/remodel-metadata-2026-11-05.txt until consumed.

An active peer task is already addressing shared package-name mismatches, so this shared binding was not edited here. No additional metadata probe or native build was launched.

The subsequent reread showed a peer had already changed framework root declaration and locale keys to their canonical owner names; its Rust facade already imports those names. No edit was made there. Retained 🧪️framework-bindings.mjs independently parses the manifest and 33 direct path owners with two TOML parsers, verifies canonical source imports and reports zero binding mismatches.

One fresh normal owned metadata probe, handle 82486, exited 1 during Cargo.lock refresh. Cargo status 101 now reports the feature durable-group-testing at 🏪️store/📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml:25 refers to semio-framework-os-store-one-item-publication/durable-group-testing, but that dependency name is absent. Captured output is 🗑️generated/remodel-metadata-second-2026-11-05.txt. This is the next actual selected error, not a compiler or test result; no retry was launched.

Before the next edit, current source showed the peer corrected the one-item-owners feature forward reference to semio-os-store-one-item-publication. The prior Cargo refusal is the actual RED; the retained neutral five-case 🧫️feature-refs.json and 🧪️feature-refs.mjs using two TOML parsers now pass. No production change was made in this turn.

One fresh normal metadata probe, handle 80964, finished exit 0. The Cargo.lock refresh reported current, and captured output contains valid Cargo metadata version 1 with 56 packages and 56 workspace members. Full output is 🗑️generated/remodel-metadata-third-2026-11-05.txt pending cleanup. This verifies metadata resolution for the requested workspace, not native compilation, tests, component publication or browser acceptance.
