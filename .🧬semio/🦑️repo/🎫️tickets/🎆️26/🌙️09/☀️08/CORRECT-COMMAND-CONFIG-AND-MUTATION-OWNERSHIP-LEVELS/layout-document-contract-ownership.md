# Layout Document Contract Ownership

## Result

Layout now exposes one exact persisted parent shape across Rust, JSON Schema, TypeScript, GraphQL, and Protobuf artifact/snapshot/diff facets. Artifact and snapshot own `schema`, `name`, `grid`, paragraph and character styles, stories, image links, parent pages, spreads, pages, nullable `printTarget`, optional `dataFieldsJson`, optional `backgroundDrawing`, and optional `referencedModel`. Diff owns `artifact` plus nullable changes for those fourteen fields. The obsolete camera field is absent.

`backgroundDrawing` keeps the existing `LayoutDrawingChild { handle, content }` shape. The handle is a shared `ArtifactChild`, uses `s.stdio.semio@v1/drawing`, and has `childId == target.artifactId`. Its constructor now derives that ID only from canonical drawing content, so equal content imported through different source formats has one identity. Inline drawing content remains because the live exporter reads it and no child-store/read-view path exists yet. `referencedModel` is a shared non-owning `ArtifactLink`.

All closed native document, nested record, and diff decoders deny foreign fields. The native contract test covers JSON canonicalization, text and Pack round trips, artifact/snapshot projection, diff decoding, parent and nested unknown-field rejection, valid child projection, wrong child identity rejection, and source-independent child construction.

## Validation

- PASS — permanent direct ticket route: `bun validation/📜️script.ts layout-document-contract`.
- PASS — permanent root route: `bun 📜️script.ts verify layout-document-contract`. Log: `🗑️generated/layout-document-contract-root-route.log`.
- PASS — independent Ajv validation and production parser round trips over 50 committed mutation snapshots and 25 committed diffs.
- PASS — strict TypeScript compilation of the artifact, snapshot, diff, and oracle sources.
- Runtime traces:
  - `[DEBUG] Layout exact document contracts matched 50 native snapshots, 25 committed diffs and independent owner/child rejection vectors`
  - `[DEBUG] Layout document contract preserved drawing content and exact child/link authority`
- PASS — the integrated independent TypeScript compiler/AST, GraphQL metadata AST, Ajv, and fast-glob field-parity report discovered 192 owners and reports zero disagreements. Log: `🗑️generated/layout-artifact-field-parity-report.log`.
- INFRASTRUCTURE — two Nx target invocations remained in bootstrap/project-graph construction and were interrupted without reaching the registered task; `🗑️generated/layout-document-contract-nx.log` records the non-interactive attempt. The underlying permanent root and ticket routes both passed.
- PENDING — native test `layout_document_contract_json_text_pack_projection_and_identity` is authored and registered but was not run because the parent explicitly deferred new Cargo work while the shared `cargo-trinity` queue is active. The command is `bun nx run abstraction-ownership-validation:layout-document-contract-native`; it uses the single ticket target and `CARGO_INCREMENTAL=0`.
- PENDING — Protobuf source was audited for the exact typed field structure but was not compiled because no Protobuf compiler was available in this source milestone.

## Remaining integration

A real child-store/read-view path must materialize `SemioDrawingSnapshot` by the shared child handle before `LayoutDrawingChild.content` can be removed from the parent wire. Removing it now would discard data used by live export. This remaining integration belongs with shared child persistence and plugin assembly; this subtask did not change the shared plugin registry.

## Registered commands

- `workspace:layout-document-contract`
- `workspace:layout-document-contract-native`
- `abstraction-ownership-validation:layout-document-contract`
- `abstraction-ownership-validation:layout-document-contract-native`
- VS Code launch orders `311.168` and `311.169` in both launch files.

## Exact file ledger

Layout-owned source and fixtures:

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`

Shared registration files, limited to the Layout routes/targets/launch entries:

- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`

Ticket evidence:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/layout-document-contract-ownership.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/layout-artifact-field-parity-report.log`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/layout-document-contract-nx.log`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/layout-document-contract-root-route.log`

The concurrently modified live exporter file `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs` was inspected to establish payload use and was not edited by this subtask.
