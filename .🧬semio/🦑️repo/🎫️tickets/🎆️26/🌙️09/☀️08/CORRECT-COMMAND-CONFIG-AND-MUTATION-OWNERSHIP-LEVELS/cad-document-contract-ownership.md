# CAD Document Contract Ownership

## Result

The CAD parent document now has one canonical shape in JSON Schema, TypeScript, GraphQL, protobuf, and Rust: four optional `s.stdio.semio@v1/model` child slots, a `s.stdio.semio@v1/drawing` child collection, references keyed by model-definition id, and nodes. The old inline object/geometry fields and the persisted `activeModelDefinitionId` selector are rejected.

Every committed child handle now uses the native family kind `s.stdio.semio` and has `childId == target.artifactId`. The five child-create mutations call typed model/drawing constructors which reject malformed URIs, identity mismatches, and the wrong standard/subset before producing a diff. Rejection is returned as a fatal mutation outcome, so no candidate is partially admitted.

`activeModelDefinitionId` was UI focus state. `saveSelected` and `saveCurrent` now resolve the pane from the host-supplied concrete `ViewModel.window_id`, confirm that exact instance in `window_instances`, and map only its registered CAD window kind. Missing, closed, or foreign instances fail closed; there is no first-pane fallback and payload window/surface ids do not grant authority. Modelspace export requires no pane and emits no active selector.

The retired `focusModelDefinition` app command had no lawful host effect: the plugin `Effect` vocabulary has no focus effect. Its only non-test bindings were object/primitive rows that are unreachable while composed child content is unresolved; those rows now have no fabricated action. The existing host boundary is `ui.window.focus` / `ShellCommand::FocusWindow` in `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell`; a future resolved-child tree must invoke that host capability with a concrete instance id rather than restoring a CAD document mutation.

## Evidence

- `🗑️generated/cad-document-contract-ticket-green-1.log`: terminal green ticket production-parser + Ajv oracle and strict TypeScript compilation. It covers 38 committed snapshots and 19 diffs after retiring the selector mutation.
- `🗑️generated/cad-document-contract-root-script-green-1.log`: terminal green root permanent `📜️script.ts verify cad-document-contract` command with the same oracle and strict typecheck.
- `🗑️generated/cad-child-identity-corpus-green-1.log`: terminal green Bun scan of all 95 committed CAD mutation JSON files and 192 child handles.
- `🗑️generated/cad-document-contract-green-7.log`: terminal green direct oracle before permanent-command validation.
- `🗑️generated/cad-document-contract-nx-root-2.log`: no-result root Nx attempt; it waited on another process's graph construction and was interrupted. The root and ticket Nx targets are registered, but this is not counted as green.
- Native test source is registered as `cad_document_contract_round_trips_exact_child_identities`. No Cargo command was launched here; the root task owns the shared `cargo-trinity` queue.

## Exact file ledger

### Updated or created

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/cad-document-contract-ownership.md`
- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `✏️s/🔌️plugins/📐️cad/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/🔬️testkit/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️.proto`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-cad-1/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-cad-1/🐍️.py`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-cad-1/🥒️.feature`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️io/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️model-definition/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧬️schema/🧬️mutations/🏢️create-building-storey/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧪️tests/🔬️unit/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-shape-model/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢create-building-model/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡create-energy-model/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-structure-classic-model/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️create-drawing/🔺️diff/🦀️.rs`

### Rewritten committed corpus

Every file below was rewritten directly; no migration or compatibility reader remains.

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles-the-occupied-energy-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles-the-occupied-energy-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles-the-occupied-energy-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles-the-occupied-energy-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles-the-occupied-energy-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏛️create-structure-classic-model/🏛️rehandles-the-occupied-structure-classic-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏛️create-structure-classic-model/🏛️rehandles-the-occupied-structure-classic-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏛️create-structure-classic-model/🏛️rehandles-the-occupied-structure-classic-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏛️create-structure-classic-model/🏛️rehandles-the-occupied-structure-classic-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏛️create-structure-classic-model/🏛️rehandles-the-occupied-structure-classic-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles-the-occupied-building-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles-the-occupied-building-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles-the-occupied-building-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles-the-occupied-building-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles-the-occupied-building-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides-the-shape-reference/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides-the-shape-reference/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides-the-shape-reference/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides-the-shape-reference/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides-the-shape-reference/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates-the-structure-classic-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates-the-structure-classic-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates-the-structure-classic-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates-the-structure-classic-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates-the-structure-classic-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates-the-building-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates-the-building-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates-the-building-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates-the-building-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates-the-building-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves-the-shape-reference-off-origin/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves-the-shape-reference-off-origin/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves-the-shape-reference-off-origin/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves-the-shape-reference-off-origin/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves-the-shape-reference-off-origin/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps-the-shape-reference-list/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps-the-shape-reference-list/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps-the-shape-reference-list/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps-the-shape-reference-list/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps-the-shape-reference-list/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens-the-shape-reference-plane/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens-the-shape-reference-plane/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens-the-shape-reference-plane/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens-the-shape-reference-plane/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens-the-shape-reference-plane/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates-the-energy-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates-the-energy-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates-the-energy-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates-the-energy-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates-the-energy-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks-the-shape-reference/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches-the-shape-reference-to-a-new-plan/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches-the-shape-reference-to-a-new-plan/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches-the-shape-reference-to-a-new-plan/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches-the-shape-reference-to-a-new-plan/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches-the-shape-reference-to-a-new-plan/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles-the-occupied-shape-slot/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles-the-occupied-shape-slot/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles-the-occupied-shape-slot/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles-the-occupied-shape-slot/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles-the-occupied-shape-slot/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🎯️outcome/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🦠️mutation/🔣️.json`

### Removed

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-active-model-definition/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-active-model-definition/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-active-model-definition/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-active-model-definition/↩️inverse/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-active-model-definition/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-active-model-definition/🧪️tests/🏗️switches-the-active-pane-to-the-building-model/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎯change-active-model-definition/🏗️switches-the-active-pane-to-the-building-model/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎯change-active-model-definition/🏗️switches-the-active-pane-to-the-building-model/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎯change-active-model-definition/🏗️switches-the-active-pane-to-the-building-model/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎯change-active-model-definition/🏗️switches-the-active-pane-to-the-building-model/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🎯change-active-model-definition/🏗️switches-the-active-pane-to-the-building-model/🎯️outcome/🔣️.json`
