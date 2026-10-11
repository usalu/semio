# 2026-10-10 stdio-a migration report (source-level, awaiting `GREEN os`)

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🧿️semio,📖️pdf,🧊️gltf,📜️docx,🎒️zip,🗜️deflate,🏗️ifc,📐️step}`. Nothing below is compiler-verified: the plugin crate (`semio-framework-plugin`) is red, so no crate in scope can be checked yet.

## Done (source-level)

- Derives, scripted (`scan.py`/`closure2.py`/`apply.py` in the scratchpad): every type reachable from the 48 `dsl::Mutations` enums and from every `*Snapshot` of the scope derives `RetireOwned` (348 + 42 types) and `CanonicalJsonTree(owner = semio_framework_pack_json)` (1007 + 20 types). Extra requests landed: image snapshot, mesh snapshot/mutation, drawing/value mutations, kit/text snapshots, `OpcRelationship`/`OpcTargetMode`.
- Hand-written canonical trees where the derive refuses the wire: gltf (26 records through `canonical_record!`, plus `GltfJson`), pdf (`PdfObject`, `PdfStreamFilter`, `PdfEmbeddedFile`).
- Externally tagged named variants restructured to newtype payload records with a differential wire test vs a mirror of the former enum: `IfcValue::TypedValue(IfcTypedValue)`, `StepValue::TypedValue(StepTypedValue)` (41 sites).
- Paged one-item routes (docs/mesh/brep/zip) on the grant contract: shared generic `StructuralPreparation<S, M, E>` (sealed clone source via `SnapshotRead::admit_retained_clone_source`, `S::retained_clone_cursor()` traversal, per-crate `StructuralEdit`, sealer path, `begin_batch_digest`, three-generic request, four close demands, terminal-asserting `Drop`). Lives in `🧿️semio/✏️editor/📬️preparation` with a twin in `🎒️zip/✏️editor/📬️preparation` (docx re-uses the zip one). Edits: `MeshVertexEdit`, `BrepVertexEdit`, `ZipTextEdit` (rename entry / archive comment), `DocxXmlEdit`. Snapshots derive `RetainedClone` (mesh/brep/docx/zip).
- Deflate encoder jobs moved to the borrowed-outcome job protocol (`step -> Result<Option<JobOutcomeBorrow>>`, `borrow_outcome`, `📤️publication` module modelled on wfc, grant-quoted close). Codec tests rewritten for the new protocol.
- docx command works: grant-based close + four demands + `work_demands`; mesh/brep works: `work_demands`.
- zip OPC retained cursor test ported to `RetainedCloneStep`/`admit_borrowed`/`admit_owned_retirement`.
- docx drops its `document_store_owners:` override (default bounded owners).

## Pending until the plugin crate compiles

- Compile-check `cargo check --lib --tests` native + wasm32-wasip2 per crate; fix drift (sqlite snapshot native hooks, `StepContext` signatures, store API).
- Tests still on the old store/job API: semio `🧪️tests/🔬️unit` (4 store close-cursor tests), docx `retained_opc_and_xml_store_route…` lifecycle test (must keep asserting success for >1 MiB owners), zip `archive_registered_factory_resumes…`, binary/sqlite snapshot tests under `🧿️semio/…/📸️snapshot/🧪️tests`, `🧾️original` drawing tests.
- Limitation: `Vec<u8>` fields (e.g. `SemioTexture.bytes`, `ZipEntry.data`) still need one contiguous capacity grant per turn; owners larger than the store's per-turn capacity need paged field types (schema change, not done).

## Out-of-scope needs reported to main

See corrections #15, #16, #31 in `📓️2026-10-10-migration-corrections.md`.

## Kit-style editors that emit no document mutations (preparation factory = trait default `None`, explicit fail-closed denial; approved by main)

Their `reduce` only emits the `SetActiveExample` load effect. The override `build_artifact_store_one_item_preparation_factory` (and the dead `member_preparation_birth_bytes` pricing fns of model/brep) was removed:

- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🦀️.rs`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🦀️.rs`
- `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🦀️.rs`
- `🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🦀️.rs`
- `🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🦀️.rs`
- `🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🦀️.rs`
- `🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🦀️.rs`
- `📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/✏️editor/🦀️.rs`
- `📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/✏️editor/🦀️.rs`
- `📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/✏️editor/🦀️.rs`
- `📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/✏️editor/🦀️.rs`
- `📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/✏️editor/🦀️.rs`
- `📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/✏️editor/🦀️.rs`
- `📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/✏️editor/🦀️.rs`
- `🧿️semio/…/🏛️model/✏️editor/🦀️.rs` (also lost its operation-wire wrapper override)

Editors that really edit (docx, zip, semio mesh, semio brep) use `store::PagedOneItemPreparationFactory`; stdio-B's snapshot-details macro supplies the diff-apply fallback. Any structural editor not on the paged factory moves onto the store generic document-lane edit (#37) when it lands.

## PARKED 11:5x (usage limit) — exact state

- No file is mid-edit; all 746 changed `.rs` files in scope parse (`rustfmt --emit stdout` syntax pass, no errors) as of the last edit.
- Generic paged route now lives once in the OS store: `🏪️store/📬️paged-one-item/🦀️.rs` (`store::PagedOneItemPreparationFactory<S, M, E>`, `PagedOneItemEdit`, `PagedOneItemEditStep`, `paged_one_item_factory_birth_bytes`); the os-kernel crate checked clean with it (slot-gated, 11:0x). The semio/zip twins are deleted; mesh/brep/zip/docx mount it via `NativeEditPreparationRoute::new(E::recognizes, Arc::new(store::PagedOneItemPreparationFactory::<S, M, E>::default()))`.
- Last compile round (slot-gated `cargo check --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/Cargo.toml --lib --tests --message-format short`, log `.🧬semio/🦑️repo/⚡️cache/play-fleet/stdio-a/deflate-check.log`): contract (stdio-B) and binary now compile; the run stops in `semio-framework-plugin` (not mine): `🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18728:307 E0308 expected Option<Arc<[u8]>>, found Option<Arc<Vec<u8>>>` (1 error, 1304 warnings). Nothing of mine has been type-checked yet.

## Next steps after the reset

1. Retry the deflate check above until the plugin lib compiles; then fix deflate (new `📤️publication` module, borrowed-outcome jobs, rewritten codec tests).
2. Same command for zip, docx, step, ifc, gltf, pdf, semio (`-p`/manifest under `🗿️artifacts/<crate>/📦️packages/🦀️rust`), native `--lib --tests`, then `--target wasm32-wasip2 --lib` with the component features (`component-app-assembly` and the crate's `[features]`); fix every error whatever its cause (expected: `RetainedClone` derives needing leaf impls, `CanonicalJsonTree` derive refusals, sqlite snapshot native hook signatures, `StepContext` signatures, store API drift).
3. Port the tests still on the old store/job API: semio `🧪️tests/🔬️unit` (close-cursor tests), docx `retained_opc_and_xml_store_route…` (must keep asserting success for >1 MiB owners via the paged route), zip `archive_registered_factory_resumes…`, semio binary/sqlite snapshot tests, drawing `🧾️original` tests.
4. Run the 2 MiB texture fixture; if the store refuses the contiguous capacity grant, switch `SemioTexture.bytes` to `PagedBytes` schema-first (Rust + TS + schema JSON/proto) and report (main's decision #2).
5. Unit tests per crate (`cargo test … --lib --no-fail-fast -- --test-threads=4`), report counts honestly.
6. Pending upstream: store generic document-lane edit (#37) for any structural editor not on the paged factory; stdio-B items none open from me.

## Release-path status (post-resume)

Native `--lib` and `wasm32-wasip2 --lib` with `--all-features` (semio: every feature except `conversion-model-bcf` and the umbrella `conversion-model`), 0 errors:

| crate | native | wasm32-wasip2 |
|---|---|---|
| deflate | green | green |
| gltf | green | green |
| pdf | green | green |
| zip | green | green |
| step | green | green |
| semio | green (no bcf) | green (no bcf) |

- semio `conversion-model-bcf`: red because the bcf crate does not compile (stdio-B: `BcfMutation` leaf roster, `🖊️markup/🧬️schema/🧬️mutations/🦀️.rs:114`).
- docx and ifc moved to stdio-c. Before the move I changed: docx (`retire_sqlite_snapshot` via `admit_owned_retirement`, `DocxXmlPartRows::into_rows`, pack/text codec `encode_standalone`, `decode` takes `NativeSnapshotDecodeOwner`), ifc (removed duplicate manual `artifact_retire_struct!` lines, `print_text` owner). ifc `component-app-assembly` native was green; docx had 22 errors left, all in zip OPC types fixed afterwards (needs a re-run by stdio-c).
- Cross-scope edit: `SnapshotEditEvent` and `SnapshotEditingCommand<C>` in `📇️registry/🧬️contract/✏️editing/🦀️.rs` gained `value_derive::RetireOwned` (stdio-B informed through main).
- Conversions adapted to B's current schemas: tiff (sample blocks, `Vec<String>` ASCII), jpg (`JpgSnapshot.image`), gltf accessor enums, docx/pptx `build_minimal_*` and pdf `text_document` paths.
- Zip: `RetireOwned` on OPC row/patch/package/diff types and `OpcRelationshipOwners`; `ArchiveTextWork` ported to the new `ArtifactCommandWork` trait (`work_demands`, `checkpoint_byte`).
- Not done yet (budget rule): `--tests` for every crate, unit tests, 2 MiB texture fixture. deflate test target still has 18 old-API errors in the sqlite snapshot tests.
