# Shooting Persistence Audit

The Shooting dev entry `✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🚀️entry/🟦️.ts` boots the generic framework host using the catalog. The generated catalog registers the Shooting composition component WASM. The Shooting editor `✏️editor/🌉️wasm/🦀️.rs` explicitly states that the old wasm-bindgen editor was deleted and contains store aliases only. Its scene TypeScript twin is a view-model, not the browser renderer.

All paths below are relative to `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any`.

`🧬️schema/📸️snapshot/🔣️.json`, `🧬️schema/📸️snapshot/🦀️.rs`, and `🧬️schema/📸️snapshot/🟦️.ts` define the snapshot. `emblem` is an owned child, but its Rust type is `ArtifactChild<SemioImageSnapshot>` in `✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs`; it cannot type-safely hold a drawing. A distinct typed drawing field or child is required. The scene export already constructs a real `SemioDrawingSnapshot` and converts it through the registered drawing-to-SVG IO bridge in `🚪️io/📝️text/📸️snapshot/🦀️.rs`.

`🧬️schema/🔺️diff/🦀️.rs` implements typed collection patches, apply, and inverse; optional emblem assignment gives a precedent for a child. The generic binary snapshot codec at `🚪️io/💾️binary/📸️snapshot/🦀️.rs` uses DSL-derived records. The SQLite snapshot codec at `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` explicitly preflights data, reconstructs every `ShootingShot` at line 73, and writes every shot at line 85. Schema changes must also update its SQL schema, TypeScript counterpart and admission limits. Extend the relevant mutation, command and request schemas and their implementations for event-sourced changes; validate both the native and textual/binary/SQLite round trips.

## Integration follow-up

The SVG inspector creates fixed sections of 5, 9 and 12 rows. `ui_node_list` uses fallible `UiFixedList`; `PanelTreeBuilder.section` explicitly excludes these rows from the window ledger and fixed-node headroom must cover them. Verify that headroom and child capacities admit all 26 controls plus their input nodes in a rendered SVG inspector. SQLite drawing uses four binary64 companion columns and validates `ShootingDrawing` on both reconstruction and encoding. Sparse mutation and package reexports are being edited concurrently and should be verified after those changes settle.

## Dev Host Activation

`repo run @semio-tech/framework-os-dev:dev-shooting-react-prototype` is the supported single build plan for a fresh host: graph inference defines `dev` as depending on `activate`, which owns component, materialization, support, fonts, renderer, session, and prepare dependencies. Plain `serve` depends only on session; an existing activation receipt allows it to start with stale components. The freshness pass only reports staleness, and the watcher responds to receipt changes; lazy builds cover missing components. The activation implementation itself publishes already-prepared modules, so invoking it outside its declared build target skips its dependency closure. No supported graph-bypass ingress was found.

## Camera Event Integration

The `patch-shot-camera` leaf and diff/inverse are mounted by mut-any, reexported by aggregate and editor mirrors, and declared in Rust and TypeScript mutation unions, JSON schema, oracle manifest, feature scenarios, independent Python adapter, and binary protocol tag 32. Rust unit coverage exercises text and binary round trips, sparse camera edits, inversion and invalid poses. No definite native receiving compilation mismatch was found by source inspection. The separate normative `🚪️io/📝️text/🧬️mutations/📖️mutations.grammar.semio` contains unrelated canvas operations and omits this event, despite the event metadata declaring a text surface; actual `OpText` uses generic JSON encoding.

## Native SVG Pane Audit

The preview raster uses a 1024-pixel maximum extent, but pending raster admission allows at most 1 MiB. A square raster above 512 pixels exceeds that limit; admission rejects it without consuming pixels and the Publishing state retries indefinitely. The owner was notified. The pane registry is thread-local, but native winit currently drives both frame build and preview pump on the original caller; the worker-oriented frame comment is stale.
