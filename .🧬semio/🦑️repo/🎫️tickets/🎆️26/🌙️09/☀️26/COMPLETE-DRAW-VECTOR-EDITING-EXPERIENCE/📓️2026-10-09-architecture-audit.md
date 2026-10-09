# Draw Architecture Audit — 2026-10-09

Read-only source inspection; no tests or browser journeys were run in this audit. Root AGENTS.md and ✏️s/AGENTS.md were read. Prior implementation and end-user acceptance records were read; their historical test counts are not new validation.

## Authoritative Source Map

Base: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets`.

- `✳️any/✏️editor/🦀️.rs`: command enum, adapters, manifest, framework selection, mounted command admission, bounded/resumable operation owners.
- `✳️any/✏️editor/🎮️commands/🎛️edit-selection/🦀️.rs`: atomic grouping, ungrouping, duplication, delete, stack order, alignment, equal-gap distribution, shape-to-path conversion.
- `✳️any/✏️editor/🎮️commands/✏️edit-path/🦀️.rs`: typed path editing, geometry mutation emission, point identity rebinding.
- `✳️any/🧬️schema/🧮️geometry/✏️editing/{🦀️.rs,🟦️.ts}`: shared path editing including anchor/handle deletion and translation, numeric coordinates, exact split, reverse, contour opening/closing/joining, line/cubic conversion. JSON schema and fixtures already exist.
- `🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/{🔺️diff,↩️inverse}/🦀️.rs`: existing persistence primitive to reuse for path algorithm results.
- `✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs`: localized structured property controls, gradients/stops, stroke caps/joins/dashes, path controls.
- `✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs`: virtualized hierarchy and framework-owned selection; preserve these patterns.
- `✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🦀️.rs`: persisted local viewport/framing, typed diff and inverse. Appropriate owner for future window-local grid/snapping preferences.

## Missing and Incomplete Capabilities

Current non-test editor vocabulary contains no clipboard copy/cut/paste, grid/snapping/rulers, or authored-path simplify/smooth/offset/outline operation. Existing trace simplification is distinct from authored-path simplification. Existing path/paint/arrangement work should be extended, not replaced.

`🎮️commands/🔀️combine-boolean/🦀️.rs` currently silently no-ops with fewer than two ids, accepts a string operation and raw ids without command-level existence/lock/visibility checks, creates a root Boolean, and does not select the result. Lower persistence or scene validation may reject unsupported input; that is not a substitute for useful semantic command validation. Test those lower contracts before claiming an invalid value can persist.

`🎮️commands/📤️export-document/🦀️.rs` exposes SVG/PDF only. Faithful resumable PNG export and actual browser/native rendering acceptance are already recorded as open in the prior end-user report.

## Recommended Clipboard Integration

Use the existing framework clipboard/host effect contract if present; inspect its actual API first. Define a schema-owned selection packet with typed layer subtrees, relevant assets, coordinate context, and bounded/versioned decoding. Avoid arbitrary JSON snapshots as the end-user input. Copy should publish a clipboard effect without document mutation. Cut should publish the same packet and semantic deletes atomically only after successful preparation. Paste should validate every subtree, generate all new identities, remap internal Boolean references, resolve required assets, and emit create-layer/asset mutations in one history entry. Preserve world placement across differently transformed destination groups; explicit paste-in-place and offset paste must have shared fixtures. Select newly pasted roots through the framework interaction effect.

The existing `schema::clone_drawing_layer_node` already recursively assigns descendants and remaps Boolean references inside one cloned root. Multiple copied roots need one shared old-to-new mapping to preserve cross-root Boolean links, and collision checks must cover every generated descendant, not only the top root. Do not reuse the root-only collision loop in `edit-selection::plan` as a complete clipboard identity policy.

## Recommended Boolean Integration

Extract a pure semantic plan before touching manifest/UI: whitelist operation, require two distinct existing geometric operands, establish ancestor lock/visibility and destination rules, reject cycles and invalid operands, and return mutations plus result selection. Keep Boolean operands by reference and the existing resumable scene algorithm rather than flattening synchronously. Use localized concrete errors and language-neutral fixtures for stale ids, duplicate ids, locked ancestor, unsupported operation, nested coordinate spaces, and inverse restoration.

## Cancellation and Performance

Main editor routes already distinguish bounded and resumable tools. `drawing_bounded_extent` currently counts only top-level layers/assets against 4,096 items, while some handlers traverse descendants or segment buffers; this is insufficient evidence that any newly added heavy path or clipboard handler is safely bounded. Inspect operation-owner hooks and attach expensive preparation to resumable work with progress, cancellation, source revision checks, and atomic final mutation publication. No partial copy/cut/paste or path edit should become visible on cancellation. Reuse existing mounted geometry and retained mutation authority lifecycle; do not invent a synchronous fallback that bypasses it.

Window-local gestures/settings belong in utilities/actions and window config/transient effects. Cross-window document operations belong in commands/tools, with existing manifest audience and admission patterns. Selection belongs in framework interaction state, not persisted artwork.

## Verification Entry Points

Artifact package router: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/📜️script.ts`.

Existing Nx targets: `@semio-tech/draw-drawing-rs:test`, `:check`, `:verify-drawing-canvas-window-ownership`. The ownership verification accepts its existing native subcommand. Extend existing script/launch registration for new permanent verification commands. Follow schema-first neutral fixtures, Rust/TypeScript parity, independent third-party oracle and runtime debug evidence. Actual browser creation, clipboard, algorithm cancel, Undo/Redo, pane focus and exported-file reopening remain required acceptance; this audit does not claim they pass.
