# Point Selection Publication Review

Reviewing the new node marquee path found that retained query publication still serialized candidate IDs with layer granularity and then discarded that string. The merged point selection was subsequently serialized all at once through a separate helper. That made the query output check apply to a different payload than the one actually sent, and could cause unnecessary refusal after an additive/toggle merge.

The retained serializer now prioritizes the merged `node_selection` vector, emits `point` granularity, and checks its actual escaped JSON output against the byte budget. The editor routes that exact string to the point domain. Ordinary layer and hover queries retain their existing route. Direct point-selection effects share the same final effect constructor.

## Verification and Pending Checks

- Added a language-neutral fixture containing quoted, backslash-containing, and Unicode layer IDs with geometry-bound point references.
- Added a native regression test requiring one pending publication step per merged point, exact point-domain targets through independent serde_json parsing, and refusal when JSON escaping pushes a raw string that fits under the limit beyond the actual output byte limit.
- Added a native node-query regression for result-capacity overflow before publication and exclusion of hidden/locked ancestors.
- Changed-file whitespace checks passed.
- No new native pass is claimed. Native handle 17529 and component handle 93519 were polled after these edits and remain live. The current source includes these regression tests, but the active run's terminal roster must establish whether they were included.
- The previous TypeScript run (54316) passed 275 tests, with one PDF raster skip. It predates this Rust-only publication fix and is not evidence for this fix.

The existing result caps (256 hits and 8192 serialized target bytes) remain. Raising or replacing those caps requires correct retained-memory accounting and responsive publication/cleanup; full large-selection usability remains outstanding. Current browser focus, marquee cancellation, keyboard deletion, and end-user history acceptance also remain unverified until a current component is activated.

## Changed Files

Relative to `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/`:

- `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs`
- `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🧫️fixtures/🎯️point-publication/🔣️.json`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

This note and the acceptance ledger also changed. The ticket and full editor goal remain active.
