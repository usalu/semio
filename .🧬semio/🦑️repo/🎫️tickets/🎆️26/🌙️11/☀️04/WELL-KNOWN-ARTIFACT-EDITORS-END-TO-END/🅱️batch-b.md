# 🅱️ Batch B (T08): csv, tsv, xlsx, docx, pptx, idml

## Architecture findings (verified)

- csv, tsv live in family plugin `stdio` (`🌎️hub/🧩️compositions/🗄️stdio`), xlsx, docx, pptx in `🧩️extensions/💼️office`. Each has per-subset `#editor` and `#viewer` apps built from window kits (csv/tsv: table kit, docx/pptx/xlsx: document kit plus shared snapshot-details window). All five declare a natural-file codec (`natural_file_codec` / `encode_natural_file` / `decode_natural_file`).
- docx, pptx, xlsx decode every XML part into the authoritative DOM and keep unknown XML parts and binary OPC parts; existing unit tests `unmodeled_parts_survive_decode_encode_verbatim` (docx, xlsx) and the pptx custom-part fixture cover preservation.
- idml had no surface and no family mount. It is a headless package (`IdmlSnapshot{domVersion, parts[]}`, mutations `replacePackage`/`replacePart`).

## idml work

- `📚️idml/📖️story` is its own crate `semio-s-artifact-stdio-idml-story` (`📖️story/📦️packages/🦀️rust`), because the idml crate hit the Source admission file-count limit (`maxSourceFiles`) when the module was added inside it. `📖️story/🦀️.rs`: addressed story `Content` runs (`story_contents`, `story_pages`, `set_story_content`) with byte-exact splice, per-part FNV revision guard, XML escaping, control-character refusal, empty-element expansion. Tests `📖️story/🧪️tests/🦀️.rs` use the language-neutral fixture `📖️story/🧪️tests/story-content.json` and cross-check with `quick-xml` (dev-dependency).
- New surface crate `semio-s-artifact-stdio-idml-any` at `📚️idml/🏅️standards/🔖️8.0/🪆️subsets/✳️any`: `#editor` (document kit, `set-page`, snapshot details, natural file codec `.idml`, birth snapshot) and `#viewer`.

## Status per artifact (as of this write; no cargo result is in yet)

| Artifact | Surface | Natural file codec | State |
| --- | --- | --- | --- |
| csv | `#editor`/`#viewer` in plugin `stdio` (table kit; set-cell, add/remove row/column, set-header, snapshot details) | `.csv` text/csv | Present. Not yet re-verified in this session: my csv editor test run was cancelled to respect the one-queued-cargo rule. Known lossy point from the data audit: line endings normalized. |
| tsv | `#editor`/`#viewer` in plugin `stdio` | `.tsv` | Present, byte-exact per data audit. Not yet re-verified. |
| xlsx | base/strict/transitional editor+viewer in `💼️office` | `.xlsx` | Present. Preservation test `unmodeled_parts_survive_decode_encode_verbatim` exists in the base schema unit tests. Not yet re-run. |
| docx | base/strict/transitional editor+viewer in `💼️office` (set-page run text, set-run-formatting) | `.docx` | Present. Preservation tests `unmodeled_parts_survive_decode_encode_verbatim`, `unmodeled_run_properties_survive_round_trip`, `canonical_xml_authority_edits_nested_run_without_losing_unknown_markup` exist. Not yet re-run. |
| pptx | base/strict/transitional editor+viewer in `💼️office` | `.pptx` | Present. Preservation: custom part fixture `typed-construction-fidelity`, `unmodeled_slide_master_survives_decode_encode_logically`. Not yet re-run. |
| idml | NEW surface crate written, not yet compiled; not yet mounted | `.idml` in the new editor | In progress (see below). |

## OOXML unknown-part preservation (data-audit risk)

docx, xlsx and pptx decode keep binary OPC parts as raw bytes and unknown XML parts as authoritative DOM parts that re-serialize. Existing tests assert the unknown part survives decode/encode. Remaining gap to prove: custom XML with comments/processing instructions in unknown parts (DOM printing) and part ordering. Planned: one extra case per format in a shared fixture, after the idml compile result.

## idml wiring still required after the crate compiles

1. `📚️idml/🧩️composition/🔣️.json` (owner `stdio-office`, apps `IdmlEditor`/`IdmlViewer`, package `semio-s-artifact-stdio-idml-any`, playground `stdio-idml`), and run `composition-prepare` plus `native-codec-projection` through the T06 heavy queue so the office `🦀️.rs`, `Cargo.toml`, component manifest, hub workspace dependency list (`🌎️hub/Cargo.toml` has no idml entries yet), `🔌️plugins.json` and `🚀️playgrounds.json` are regenerated. No generated file is hand-edited.
2. Stdio `idml` kind must then be hosted by the plugin (`host_artifact` of the idml declaration) and added to the shipped-fleet census, editor-catalog fixture (`snapshotEdits` entry `idml`), natural-file-lifecycle fixture row (count 28 -> 29).

Logs of my runs: the running idml cargo job writes `/tmp/b08-idml.log` (started before the log rule); later runs use this folder `🗑️generated/`.

## Update: demo sample and run

- Deterministic sample generator `generate-idml-sample.py` (this folder; validates with `zipfile` and `xml.dom.minidom`, checks `mimetype` stored first, checks two builds are byte-identical). Output: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/📚️examples/🎬️demo/🖼️assets/🧪️example/📚️.idml` (mimetype, designmap, one Spread, one Story with two text runs). Python validation passed. Rust admission (`IdmlPackage::decode`) is exercised by the test `the_demo_package_opens_edits_saves_and_reopens` in the surface crate.
- One combined cargo run queued: `-p semio-s-artifact-stdio-idml-story -p semio-s-artifact-stdio-idml-any --lib`; output goes to `🗑️generated/b08-idml-run.txt`. Result will be appended below.

## Transcribed evidence (T20, before the `🗑️generated` deletion)

- `b08-csv.log`, `b08-idml.log`: 0 bytes.
- `b08-idml-run.txt`: capacity-wait lines only; no result.
- Contract (offline): **idml FAILS** 2 of 2 tests (`B/idml > ships an editor and a viewer …`; `B/idml > has a natural-file sample accepted by its third-party oracle`). Verbatim lines in `✅️validation.md` E1. csv, tsv, xlsx, docx, pptx pass in E1.

## Update: tsv scaffold markers

`tsv/…/🚪️io/🦀️.rs:1` and `🟦️.ts:1` carried the "scaffolded by W1b" markers; the io is real (derived composition/analyzer in the file, plus `📝️text`, `💾️binary`, `🪶️sqlite` leaves with diff, mutations, inferences and snapshot). Markers removed (doc header rewritten, TS twin reduced to `export {};`). No code change.

## Run state

The first combined cargo run was cancelled by the capacity gate (SIGTERM while waiting at load ~49). Re-queued once (single invocation, `-p semio-s-artifact-stdio-idml-story -p semio-s-artifact-stdio-idml-any --lib`, 2h limit), output `🗑️generated/b08-idml-run.txt`. No test result exists yet: story and surface code are uncompiled until that run returns. Not done: office mount wiring (composition json, generators) and csv/tsv/xlsx/docx/pptx re-runs.

## Update: wiring sources authored (not generated, not compiled)

Authored by hand per the coordinator; T06's generator run is expected to regenerate and normalize them.
- `📚️idml/🧩️composition/🔣️.json`: owner `stdio-office`, apps `IdmlEditor` (order 18) and `IdmlViewer` (19), order 36, playground `stdio-idml` (react 6276, wgpu 6376), mutation descriptor `semio_s_artifact_stdio_idml::schema::{snapshot::IdmlSnapshot,mutations::IdmlMutation}`, coordinate owner is the idml artifact root (its schema lives there, not under a subset folder), `nativeFactories: []` (idml has no `📡️.protocol.semio`; epw/gif/bmp/semio are the same).
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🦀️.rs`: enum variants `IdmlEditor`/`IdmlViewer`, `host_artifact(idml declaration)`, `editor`/`viewer`, `OnArtifactKind` activation. Office `📦️packages/🦀️rust/Cargo.toml`: deps `semio-s-artifact-stdio-idml`, `-idml-any`, `-idml-story`.
- `🌎️hub/Cargo.toml`: three workspace entries. `🔌️plugin/📇️catalog/📜️definition/🦀️.rs` (authored roster) now ends `…html, idml`; its Cargo.toml has the idml dependency. The stdio package test-dependency block and the bridge Cargo.toml also gained idml entries (generator-owned blocks).
- Fixtures: `✏️editor-catalog` (editorCount 89, formatCount 37, `editorApps`, `snapshotEdits` entry replacing `/parts/1/bytes` with a valid designmap), shipped-fleet sqlite census (`artifactKinds`, `dialectBindings`), natural-file-lifecycle (`formats` row and the `BATCH_B_NATURAL_FILE_APPS` named set). The shipped-fleet Rust census match arm (`typed_snapshot_owner!` for `s.stdio.idml`) is not added yet.

## BLOCKER found while wiring: idml has no SQLite snapshot owner

`assert_sqlite_snapshot_editor` (law `sqlite_snapshot_idml_editor`) and the shipped-fleet census require `IdmlSnapshot` to implement `ArtifactSqliteSnapshot` plus the `ArtifactSqliteSnapshotHost` native hooks, and the census requires an authored `typed_snapshot_owner!` arm. Today `IdmlSnapshot` only has `impl ArtifactStoreSnapshot {}`. Until that owner exists, these laws and the census will fail for idml (the census failure also fails the shared test, so T06 should expect red there). Plan: reuse the zip semantic tables by projecting parts as ZIP rows with schema `stdio.idml`, but the native encode/decode hooks need a DSL record spec for the idml snapshot; this needs its own focused task.

## Update: OOXML preservation cases (test-first, not yet run)

Added `custom_xml_with_comments_and_instructions_and_part_order_survive_round_trip` to the docx, xlsx and pptx base schema unit tests: an unknown XML part containing a comment, a processing instruction and CDATA plus two binary parts around it; asserts markers survive, text stable across saves, binary parts byte-exact, and package part order stable and in insertion order. Runs sequentially via `run-batch-b-tests.sh` (this folder; one queued cargo at a time), outputs `🗑️generated/b08-{idml,docx,xlsx,pptx}.txt`.

## Update: idml SQLite owner, native receipt inputs (authored, uncompiled)

- `📚️idml/🚪️io/🪶️sqlite/🦀️.rs` (one file, SQL inline, so the idml crate stays within the Source admission file budget): schema `idml_package(id=1, schema, dom_version)`, `idml_part(id, package_id, ordinal, path)`, `idml_part_byte(id, part_id, ordinal, value)`. `impl ArtifactSqliteSnapshot for IdmlSnapshot` (project/reconstruct with contiguous-ordinal and identity checks, re-admission through `IdmlSnapshot::package()`), and `impl ArtifactSqliteSnapshotHost` with a row/byte census preflight and native hooks that reproduce the exact native wire (`decode_pack`/`parse_dsl`, `encode_pack`/`print_dsl`). No DSL record spec was needed: idml's pack wire is its own value record (`native_record_spec`), so the hooks reuse that codec rather than a second representation. `ArtifactStoreSnapshot::sqlite_snapshot_codec()` for `IdmlSnapshot` now returns the derived codec (`🚪️io/🦀️.rs`).
- shipped-fleet census: `typed_snapshot_owner!` arm for `("s.stdio.idml","8.0","*","stdio.idml")` added after html.
- Native receipt: `📚️idml/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio` (zip's generic frame with identity `stdio.idml`) and `nativeFactories` entry `stdio.native.idml.v1` in the idml composition file. The definition's `native_factory`/`pack_schema_hash` (`32afdf…`, the record-spec hash) is unchanged: csv shows the projection does not use sha256 of the protocol file for the hash.
- Known unknowns for the first compile: exact SQLite control API names copied from zip's owner; the idml crate Source file budget (limit is per-closure file count; this change adds one file); `declaration()` uses `document_codec_bare_with_owners`, whose owner type `IdmlCodecOwners` may also need a SQLite hook.
- Test run state: `run-batch-b-tests.sh` (sequential, one cargo at a time) was started before these edits; its idml step may compile the earlier tree. It will be rerun for idml after it finishes.
