# IFC Format Audit

Read-only source audit of existing IFC4, IFC2x3, STEP and layout architecture. No tests executed and no source files changed.

## Ownership

- IFC root: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc
- IFC4 owner: 🏅️standards/4️⃣4/🪆️subsets/✳️any beneath that root.
- IFC2x3 owners: 🏅️standards/🔖️2x3/🪆️subsets/{🧱️base,🤝️cv20,🧮️sav,🏢️cobie}.
- BIM semantic model import/export is separate: ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📥️import,📤️export}/🏗️ifc. It has domain modules for walls, spatial, structures, MEP, ramps and components, with unit tests; it is not the raw IFC artifact codec.

## IFC4 Gaps

1. 🚪️io/🦀️.rs decode_ifc4_document validates UTF-8, parses shared Part-21 and requires FILE_SCHEMA to contain IFC4. encode_ifc4_document requires IFC4 and unique entity ids. These are syntax/header laws, not complete EXPRESS/schema/arity validation.
2. ✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs renders fallback box meshes. entity_count derives a maximum array length from snapshot values and clamps to 1–6; instances sit at index*2 on X. This does not display imported IFC geometry.
3. No official buildingSMART compliance corpus was found within the stdio IFC tree. Existing committed real assets establish mutation/round-trip evidence, not comprehensive official compliance.

## Existing Editable Details Integration

IFC4 ✏️editor/🦀️.rs already registers snapshot_details_window_definition, renders SNAPSHOT_DETAILS_BODY_KEY through editing::render_snapshot_details, handles SnapshotEditEvent, implements SnapshotEditingEditor, and uses the bounded snapshot-edit tool factory/publication authority. ✏️editor/🎭️modes/✏️edit/🦀️.rs uses snapshot_details_split_layout. The same integration exists for IFC2x3 base/CV20/SAV/COBie.

IFC4 🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs maps /header/fileDescription, /header/fileName, /header/fileSchema, /entities/*/name and /entities/*/args/* to concrete mutation kinds. Entities are selected by id and arguments by position. Inserts/removes cover entity and argument arrays. Complex constituents, entity ids and document schema edits have no mapping and are refused. Its 🧪️tests/🦀️.rs verifies addressing, replay, insertion/removal and refusal.

## Fixtures And Independent References

Relative to IFC4 owner:

- 🧪️tests/🧫️fixtures/🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc — IfcOpenShell 0.8.4.post1 export, 2,496,437 bytes and 24,792 entities according to feature contract.
- 🧪️tests/🏗️mutate-ifc-4/🥒️.feature with Rust subject 🦀️.rs.
- 🧪️tests/🔺️differential-ifc-4/🥒️.feature with Python reference 🐍️.py and Rust subject 🦀️.rs.
- 🧪️tests/🧫️fixtures/{➕️insert-entity-applied,➖️remove-entity-applied,🎛️set-entity-arg-applied,🏷️set-entity-name-applied,📛️set-file-name-applied,🗒️set-file-description-applied,🧩️insert-entity-arg-applied,🧬️set-file-schema-applied,🧹️remove-entity-arg-applied}/{⬅️before.ifc,➡️after.ifc}.
- 🧪️tests/🧫️fixtures/🧬️mutations/🎛️set-entity-arg/✏️renames owns mutation, before/after snapshot, diff and outcome JSON.
- 🔮️oracles/🔣️.json registers ruststep 0.4 as cross-semio implementation/independent reader, IfcOpenShell 0.8.4.post1 as third-party producer and steputils 0.1 as schema-agnostic reader/writer.

IfcOpenShell differential feature claims six schema-expressible kinds. It intentionally excludes set-entity-name, insert-entity-arg, remove-entity-arg and remove-entity because schema-bound arity/type validation or cascading reference repair differs from raw Part-21 mutation semantics. ruststep has no Part-21 writer, and its local oracle contains an apostrophe escape workaround; it must not be called a fully independent mutation producer.

IFC2x3 real fixture is 🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🧪️tests/🧫️fixtures/🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc. Base and specialized subsets have mutation features; base has differential IFC2x3 Python/Rust cases.

## Precise Test Routing

IFC 📦️packages/🦀️rust/📋️project.json declares project @semio-tech/stdio-ifc-rs. Its test/check/build targets execute repo:script from that package and route to bun ./📜️script.ts test|check|build. That script invokes runArtifactRustPackageMain(import.meta.dir, semio-s-artifact-stdio-ifc). The script includes IFC4 and IFC2x3 source/native SQLite snapshot suites with 120000ms budget.

Additional project targets: test-snapshot-sqlite, test-snapshot-sqlite-native (script test-snapshot-sqlite native), test-snapshot-sqlite-source (script test-snapshot-sqlite source). Use repo binary to invoke the project targets; direct bun/cargo execution was not performed during this audit.

IFC 🧪️tests/🔬️io-laws/🦀️.rs includes each standard's 🚪️io/🧪️tests/🔬️unit/🦀️.rs. IFC4 test names include empty_snapshot_matches_schema, codec_round_trip, and the_ifc4_document_codec_round_trips_an_ifc4_file_and_refuses_other_schemas.

IFC 📦️packages/🟦️typescript/📜️script.ts routes @semio-tech/stdio-ifc through runArtifactTypeScriptPackageMain and includes both standards' SQLite snapshot 🧪️tests/🟦️.ts.

## STEP And Schema Conventions

STEP root ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step has AP214 base and CC1–CC6 owners. Each has 🔮️oracles/🔣️.json and language-agnostic mutation feature. Base feature: 🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧪️tests/📐️mutate-step-ap214/🥒️.feature. CC paths use 🧪️tests/🔬️N-mutate-step-ap214-ccN/🥒️.feature.

Schema-first taxonomy co-locates 🧬️schema JSON/GraphQL/proto, TypeScript/Rust schema implementations, snapshot, diff, inferences and mutation leaves with fixture/test siblings. IFC4 and IFC2x3 model/descriptor/mutations/inferences are separate Rust packages in IFC Cargo workspace.

## Layout Artifact Architecture

Layout owner: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any. 🧬️schema/📸️snapshot/🦀️.rs defines LayoutSnapshot with pages, spreads, text stories/styles, grid, image links, parent pages and print target. backgroundDrawing is a genuine s.stdio.semio/v1/drawing child; referencedModel is ArtifactLink with model role. 🧬️schema/🔣️.json records matching child/link annotations. Editor ✏️editor/🦀️.rs dispatches commands to taxonomy modules, routes window/panel renders, and uses bounded interactive export jobs. This is publication layout, distinct from IFC geometric placement.
