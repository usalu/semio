# Direct `objc2` Metal Runtime Leaf

## Exact scope and result

This checkpoint covers only the former direct macOS Metal declaration `objc2 = "0.6"`. QuartzCore and the Foundation-family leaves are outside this checkpoint and are not re-claimed here.

The production Metal manifest now has zero exact `objc2` declarations. Its root lockfile package block directly lists `bytemuck`, `objc2-metal 0.3.2`, `raw-window-handle`, and `semio-framework-ui-render`; it does not directly list `objc2`.

The replacement is the backend-private `🦀️objective_c.rs` interface. It owns the exact runtime ABI used by Metal: class and selector lookup, typed `objc_msgSend` trampolines, retain/autoreleased-return retain/release, autorelease-pool push/pop, non-null `Owned<T>`, and the opaque Objective-C object wrappers and selectors reached by current backend callers. A token-level crate census found no declared public wrapper function or opaque object type with no second reference in the Metal crate.

## Schema, fixture, and oracle

- `🧬️schema/🔣️objc2-runtime-abi.schema.json` is the language-neutral schema.
- `🧫️fixtures/🔣️objc2-runtime-abi.json` fixes the pointer layout, clone retain delta, drop restoration, null rejection, autorelease-pool result, and the empty/one/4096/4097 boundary vocabulary.
- `🧪️objc2-runtime-oracle` is an isolated ticket-only Rust workspace with a locked `objc2 0.6.4` dependency. Keeping it under `.🧬semio` prevents the test oracle from becoming a live repository/product dependency declaration.
- The oracle executed on macOS, asserted retain `+1`, drop restoration, and null rejection, emitted `🔣️actual.json`, and `diff -u` against the language-neutral fixture exited 0.
- `cargo fmt --manifest-path <ticket-oracle>/Cargo.toml -- --check` exited 0. Both schema and fixture parse as JSON.

## Executed native and Wasm gates

All build artifacts were directed to Phase-9 ticket targets; `🧪️target-p0-current` was not used.

1. `cargo test --locked -p semio-framework-ui-backend-metal --lib -- --nocapture` — exit 0, 5 passed, 0 failed. The owned-runtime test logged `empty=ok single=ok max=4096 maxPlusOne=rejected hostileNull=rejected retainDelta=1 restored=true pool=drained` and byte-compared its generated contract with the JSON fixture.
2. `cargo check --locked -p semio-framework-ui-backend-metal --target wasm32-unknown-unknown` — exit 0, finished the dev profile.
3. The earlier `--features backend-testing --no-run` probe remains red on five pre-existing dependent-crate calls to `Scene::finish`, which is compiled only under `cfg(test)` in `semio-framework-ui-render`. That broader feature-cohort failure is not represented as a pass; the exact featureless Metal library tests and Wasm consumer check above are the accepted leaf evidence.
4. `cargo fmt --package semio-framework-ui-backend-metal -- --check` remains red on broad pre-existing formatting drift across `backend.rs`, `objective_c.rs`, `pipelines.rs`, and other Metal sources. No unrelated whole-package formatting rewrite was made.

## Hostile re-add and official census

The hostile manifest mutation inserts exactly `objc2 = "0.6"` immediately before `objc2-metal = "0.3"`. The same exact direct-declaration predicate reports baseline `0`, hostile `1`, delta `+1`; therefore a direct re-add cannot be mistaken for the retained transitive graph.

Fresh official command:

`bun ./📜️script.ts verify dependencies list rust --raw`

Filtering its JSON by exact identity `name === "objc2"` produced `exactDirectRows: 0`. A separate product-manifest census excluding `compose`, `temp`, and `.🧬semio` likewise found no exact direct declaration. Global totals were deliberately not used because concurrent manifests are changing during Phase 9.

## Concrete checkpoint and non-claim

The exact direct Metal `objc2` leaf is closed at production manifest `0`, Metal lock-package direct edge `0`, official exact Rust census row `0`, hostile delta `+1`, native `5/5`, oracle parity exact, and Wasm exit 0.

This is not transitive zero. `cargo tree --locked -p semio-framework-ui-backend-metal --edges normal` still reaches `objc2 0.6.4` through `objc2-metal 0.3.2`; those transitive rows are truthful and remain outside this exact direct-owner leaf.
