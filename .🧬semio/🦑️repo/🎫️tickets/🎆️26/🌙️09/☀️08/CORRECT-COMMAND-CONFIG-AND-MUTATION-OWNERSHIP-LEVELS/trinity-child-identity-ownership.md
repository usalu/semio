# Trinity Child Identity Ownership

## Correction

Wires and Jack compose the `graph` subset of the canonical `s.stdio.semio` artifact kind. Their child-slot metadata now declares that artifact kind. Each constructed or committed owned child now uses one exact identity for both `childId` and `target.artifactId`; the dialect retains `standard=v1` and `subset=graph`. The Store restore gate remains unchanged. Jack query-result construction updates both identity fields when it mints its result child.

## Source and contract files

- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🪜️execution/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`

## Wires committed snapshots (20)

- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects-cutting-an-edge-the-board-never-carried/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects-cutting-an-edge-the-board-never-carried/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✏️edit-node-text/🧪️reports-a-no-op-when-the-label-is-retyped-verbatim/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✏️edit-node-text/🧪️reports-a-no-op-when-the-label-is-retyped-verbatim/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌱create-node/🧪️rejects-a-node-id-the-board-already-holds/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌱create-node/🧪️rejects-a-node-id-the-board-already-holds/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-node-kind/🧪️reports-a-no-op-when-the-kind-already-reads-topic/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️change-node-kind/🧪️reports-a-no-op-when-the-kind-already-reads-topic/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐resize-node/🧪️reports-a-no-op-when-the-radius-already-matches/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐resize-node/🧪️reports-a-no-op-when-the-radius-already-matches/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔷change-node-shape/🧪️reports-a-no-op-when-the-shape-already-reads-circle/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔷change-node-shape/🧪️reports-a-no-op-when-the-shape-already-reads-circle/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🧪️rejects-deleting-a-node-the-board-never-held/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🧪️rejects-deleting-a-node-the-board-never-held/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🚩set-node-root/🧪️reports-a-no-op-when-an-unflagged-node-is-set-to-not-root/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🚩set-node-root/🧪️reports-a-no-op-when-an-unflagged-node-is-set-to-not-root/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🤝️connect-nodes/🧪️rejects-an-edge-whose-source-node-is-absent/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🤝️connect-nodes/🧪️rejects-an-edge-whose-source-node-is-absent/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧭move-node/🧪️reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧭move-node/🧪️reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero/📸️snapshot/⬅️before/🔣️.json`

## Jack committed snapshots (16)

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✂️delete-edge/🚫️rejects-cutting-an-edge-the-scene-never-had/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✂️delete-edge/🚫️rejects-cutting-an-edge-the-scene-never-had/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✏️rename-node/✏️keeps-the-name-a-node-already-carries/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/✏️rename-node/✏️keeps-the-name-a-node-already-carries/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕️create-node/🚫️rejects-a-node-id-the-scene-already-holds/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕️create-node/🚫️rejects-a-node-id-the-scene-already-holds/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌉️create-edge/🚫️rejects-an-edge-whose-endpoints-are-absent/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌉️create-edge/🚫️rejects-an-edge-whose-endpoints-are-absent/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📍️move-node/📍️keeps-a-node-at-the-point-it-already-occupies/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📍️move-node/📍️keeps-a-node-at-the-point-it-already-occupies/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔧️change-data-property/🏷️keeps-a-node-property-at-the-value-it-already-holds/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔧️change-data-property/🏷️keeps-a-node-property-at-the-value-it-already-holds/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️rejects-deleting-a-node-the-scene-never-had/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️rejects-deleting-a-node-the-scene-never-had/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹️remove-data-property/🧹️keeps-an-edge-without-the-property-it-never-had/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹️remove-data-property/🧹️keeps-an-edge-without-the-property-it-never-had/📸️snapshot/⬅️before/🔣️.json`

## Validation

- `trinity-child-identity-static.log`: PASS. Wires 20/20 and Jack 16/16 snapshots carry `artifactKind=s.stdio.semio` and equal child/target IDs; Rust, TypeScript, and JSON artifact/snapshot/diff metadata agree; both constructors and the Jack query-result remint preserve paired identity.
- `wires-child-identity-oracle.log`: registered Wires document oracle queued through Nx; it validates all 20 committed snapshots and the three metadata facets.
- `jack-child-identity-oracle.log`: registered Jack document oracle queued through Nx; it validates all 16 committed snapshots and the three metadata facets.
- Native domain tests `wires_child_restore_projection_accepts_the_exact_owned_content` and `jack_child_restore_projection_accepts_the_exact_owned_content` call the unchanged `ChildRestoreProjection` gate directly. They are authored and await the shared Cargo queue.
