# Document Load Performance (setActiveExample) - manifest interned by id, removed from the snapshot

Status: implemented across Rust, TypeScript, schemas and fixtures. `semio-s-artifact-trinity-jack-model` (snapshot, records, sqlite, diff, memory graph, ast, executor-facing types) and `semio-s-artifact-trinity-jack-lsp` pass `cargo check`. The rest of the Jack workspace (editor, viewer, executor, shell, tests), the Rewriting artifact and the Puzzle 5D rewriting bridge could not be compiled because the shared framework (`semio-plugin`, `semio-plugin-instance-render`, `semio-plugin-instance-commands`) was mid-edit by other agents and failed to build (not related to this change), and the shared disk filled twice. No test was run to completion; see "Verification state".

## Root cause

- `JackSnapshot` owned an inline `manifest: Manifest` next to `manifestId`; for Nakagin that is 42 node kinds plus edge and port kinds, each with property lists.
- Every retained clone of a snapshot copied it field by field in a hand-written step machine (`🔨️modules/🏠️host/📑️snapshot-clone`), and every retirement walked it field by field through the paced owned-value cursor (about 10 steps per second). One load retires several snapshots: about 10 s each, about 8 min per `setActiveExample` (recorded in `🌙️09/☀️17/TRINITY-PLUGIN-END-TO-END/📓️status.md`).
- `Graph` (memory graph) copied the same manifest again.

## Design (schema-first, no compat layer)

- `JackSnapshot`, `JackArtifact`, `JackDiff`, `JackQueryGraphSnapshot` (the query-scene document) and `Graph` no longer carry a manifest. `manifestId` is a required string (default `nakagin`); in the diff it is `string | null` (null = unchanged).
- The manifest is an interned registry resource in `🛂️manifest/🦀️.rs`: `Manifest::shared(id) -> Option<Arc<Manifest>>` (the built-in `nakagin` resolves lazily once per process), `Manifest::register(id, manifest)` for custom ids, `Manifest::intern(manifest) -> id` for derived manifests (content-addressed id, equal manifests share one allocation). `JackSnapshot::manifest()` and `Graph::manifest()` return the shared `Arc`.
- Wire/schema: json, graphql, proto (fields renumbered), ts of artifact, snapshot, diff and ast drop `manifest`; the `Manifest` type remains only as the registry-entry shape. SQLite: three tables (`jack_document` with `manifest_id TEXT NOT NULL`, `jack_camera`, `jack_content_child`) instead of thirteen; Rewriting's embedded Jack tables followed (30 to 20 tables).
- LSP: the host interns each manifest once per session (`JackLanguageSession::register_manifest`, wasm `registerManifestJson`), snapshots name it by `manifestId`; an unregistered id is refused (`unknown manifest id`).
- Binary receiving: the field-by-field manifest allocation phases and `JackRecordReconstructionPolicy` (rows/type-depth ceilings, meaningless without a manifest) are gone.
- Puzzle 5D rewriting derives a per-document manifest: it now interns it (`Manifest::intern`) and the graph names it by id.

## Before / after

- Before: about 10 s per retired snapshot and about 8 min per `setActiveExample` (status.md, 2026-09-17). A baseline build on current HEAD could not be run (disk full, then the framework build break), so the "before" is that record plus the quantity the new test measures for the old shape: retiring one `Manifest::nakagin_default()` through the same funded owned-retirement pump (`manifest turns`).
- After: not measured yet. The test `nakagin_snapshot_clone_and_retirement_do_not_walk_the_manifest` (`🔨️modules/🏠️host/🧪️tests/🔬️unit/🦀️.rs`) prints `[DEBUG] manifest retirement N turns ...; snapshot clone ...; snapshot retirement M turns ...` and asserts `M * 8 < N` plus `Arc::ptr_eq` of the shared manifest between source and clone. Snapshot retirement now has seven short fields instead of seven fields plus about 100 manifest steps, so at 10 turns per second the per-snapshot cost drops from about 10 s to well under 1 s.

## Verification state

- `cargo check -p semio-s-artifact-trinity-jack-model -p semio-s-artifact-trinity-jack-lsp` (shared target `C:\cargo-nr`, `-j 6`): finished, one pre-existing unused-import warning.
- Workspace `cargo check --workspace --all-targets --keep-going`: blocked by framework crates that other agents are editing (`semio-plugin-instance-commands`: `original_press_selected`, `ReservedCommitMetadata`; `semio-plugin-instance-render`: `store::Revision`; `semio-plugin`: `assert_latest_history_preview`), and by `os error 112` (disk) twice.
- TS: the fork ran the Ajv oracle (8 pass), sqlite snapshot TS tests (26 pass), `mutate-jack-1` and query inference TS tests (17 pass) and the dev trinity graph tests (3 pass) after its edits. Rewriting TS/SQL/fixtures were ported by another agent concurrently; the Rewriting Rust was aligned here (not compiled).

## Tests added or changed

- `🧪️tests/🪪️manifest-id-only/🟦️.ts` (new, TS, third-party validator Ajv over every committed snapshot and diff fixture against the JSON Schemas; asserts no inline `manifest` and a registered `manifestId`).
- Rust: manifest interning (`manifests_are_interned_once_and_shared_by_id`), empty and unknown id errors, `fixture_loads_manifest_id_only`, host clone/retirement timing test above, sqlite tests (three tables, three rows, exact row admission, `serde_json` oracle asserts the JSON snapshot has no `manifest` and the expected `manifestId`), LSP unit tests (register manifest, unknown id, both row shapes, Nakagin from `🛂️manifest.json`), operations test registers its custom manifest, executor fixture test drops the `inlineManifest` case.

## Files changed

Under `C:\semio\✏️s\🔌️plugins\🔱️trinity\🗿️artifacts\🔌️jack` (path prefix `🏅️standards\🔖️1\🪆️subsets\✳️any` abbreviated as `…any`):

Rust production: `🛂️manifest\🦀️.rs`; `🔨️modules\🔺️graph-model\🦀️.rs`; `🔨️modules\🏠️host\📑️snapshot-clone\🦀️.rs`; `🔨️modules\🧠️memory-graph\🦀️.rs`; `🔨️modules\📡️json-transport\🦀️.rs`; `🔨️modules\🧠️lsp\🔱️engine\🦀️.rs`, `🦀️.rs`, `🧩️component\🦀️.rs`; `…any\🧬️schema\🦀️.rs`; `…any\🧬️schema\📸️snapshot\🦀️.rs`; `…any\🧬️schema\🔺️diff\🦀️.rs`; `…any\🧬️schema\🌳️ast\🦀️.rs`; `…any\🧬️schema\⚙️operations\🦀️.rs`; `…any\🧬️schema\🧮️executor\🪜️execution\🦀️.rs`, `⚖️owned-bytes\🦀️.rs`, `🧱️execution-preparation\🦀️.rs`, `🔁️pattern-delete-return\🦀️.rs`; `…any\🧬️schema\🗣️language-service\🦀️.rs`, `🧩️example-fixture\🦀️.rs`, `🩺️completion-and-lint\🦀️.rs`; `…any\🚪️io\📝️text\📸️snapshot\🦀️.rs`, `🧬️records\🦀️.rs`; `…any\🚪️io\📝️text\🔺️diff\🦀️.rs`; `…any\🚪️io\💾️binary\📸️snapshot\🫴️receiving\🦀️.rs`; `…any\🚪️io\🪶️sqlite\📸️snapshot\🦀️.rs`; `…any\✏️editor\🎮️commands\▶️run-query\🧵️job\🦀️.rs`; `…any\✏️editor\🎭️modes\✏️edit\🪟️windows\📊️results\🫧️transient\🦀️.rs`; `🐚️shell\📦️packages\🦀️rust\📦️bin.rs` (doc line).

Schemas (json, graphql, proto, ts, sql): `…any\🧬️schema\🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🟦️.ts`; `…any\🧬️schema\📸️snapshot\🔣️.json`, `🔗️.graphql`, `🛰️.proto`; `…any\🧬️schema\🔺️diff\🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🟦️.ts`; `…any\🧬️schema\🌳️ast\🔗️.graphql`; `…any\🚪️io\💾️binary\📸️snapshot\🫴️receiving\🧬️schema\🔗️.graphql`; `…any\🚪️io\🪶️sqlite\📸️snapshot\🗄️.sql`, `🟦️.ts`; `🔨️modules\🧠️lsp\🧪️tests\🔖️protocol\🔣️.json` (doc string).

TypeScript (by the TS fork): `…any\🚪️io\🪶️sqlite\📸️snapshot\🟦️.ts`, `🧪️tests\🟦️.ts`; `…any\🚪️io\📤️export\…\🔖️rfc8259\✳️any\🟦️.ts`; `…any\🚪️io\📥️import\…\🔖️rfc8259\✳️any\🟦️.ts`; `…any\🧪️tests\🔌️mutate-jack-1\🟦️.ts`, `🐍️.py`; `…any\🧬️schema\💡️inferences\🧪️tests\🪆️query\🟦️.ts`; `…any\🧬️schema\🧬️mutations\🔎️set-query\🟦️.ts`; `…any\🧪️tests\🪪️manifest-id-only\🟦️.ts` (new); `🔨️modules\🧠️lsp\🧪️tests\📡️worker\🪞️session-double\🟦️.ts`.

Fixtures (hand-edited): `…any\🧪️tests\🧫️fixtures\🧬️mutations\🎯️set-root\🧹️clears-the-root\{📸️snapshot\⬅️before,📸️snapshot\➡️after,🔺️diff}\🔣️.json`; `…any\🧪️tests\🧫️fixtures\🧬️mutations\🔎️set-query\🔎️replaces-the-query\{…}\🔣️.json`; `…any\🧪️tests\🧫️fixtures\🧮️executor\🪜️resumable-query\🔣️.json`; `…any\🚪️io\🪶️sqlite\📸️snapshot\🧪️tests\🧫️fixtures\🔣️.json`; `…any\🚪️io\💾️binary\📸️snapshot\🫴️receiving\🧪️tests\🧫️fixtures\🔣️.json`; `…any\🖼️assets\🎬️demo\🔣️.json`; `🐚️shell\🧪️tests\🧫️fixtures\🔱️mini-tower\🔣️.json`; `🔨️modules\🧠️lsp\🧪️tests\🔖️protocol\🔣️.json`; `🔨️modules\🧠️lsp\🔖️protocol\🔣️.json`.

Rust tests: `🔨️modules\🏠️host\🧪️tests\🔬️unit\🦀️.rs`; `🔨️modules\🧠️lsp\🧪️tests\🔬️unit\🦀️.rs`; `🧪️tests\🧰️support\🦀️.rs`; `🐚️shell\🧪️tests\🔬️bin-unit\🦀️.rs`; `…any\🧬️schema\🧮️executor\🧪️tests\🔬️unit\🦀️.rs`; `…any\🧬️schema\⚙️operations\🧪️tests\🔬️unit\🦀️.rs`; `…any\🧬️schema\💡️inferences\🧪️tests\🔬️unit\🦀️.rs`, `…\🧭topology\🧪️tests\🔬️unit\🦀️.rs`, `…\🎛️flat-position\🧪️tests\🔬️unit\🦀️.rs`; `…any\🧬️schema\🧪️tests\🔬️empty-document\🦀️.rs`; `…any\🚪️io\📝️text\📸️snapshot\🧪️tests\🔬️unit\🦀️.rs`; `…any\🚪️io\🪶️sqlite\📸️snapshot\🧪️tests\🦀️.rs`; `…any\✏️editor\📌️panels\🗿️artifact\🧪️tests\🔬️unit\🦀️.rs`.

Dev trinity (`C:\semio\✏️s\🧑‍💻dev\🔱️trinity\🪆️graph`): `🦀️.rs` (authored scene uses `manifest()`); `🧪️tests\🦀️.rs` (native receiving test without policy and manifest); `🧪️tests\🪆️content\🦀️.rs`; `🖼️assets\🎬️demo\🔣️.query.json`; `🧪️tests\🧫️fixtures\🪆️retained.json` (manifest definitions under `manifests`, TS parse only).

Rewriting (`C:\semio\✏️s\🔌️plugins\🔱️trinity\🗿️artifacts\♻️rewriting`, same `…any` prefix): `…any\✏️editor\📄️document\🦀️.rs`; `…any\✏️editor\🎮️commands\🕸️node-graph-edit\🦀️.rs`; `🔨️modules\▶️rule-application\🦀️.rs`; `…any\🧬️schema\🧬️mutations\🖼️edit-working-graph\🔺️diff\🦀️.rs`; `…any\🚪️io\🪶️sqlite\📸️snapshot\💰️backing\📤️projection\🦀️.rs`, `📥️reconstruction\🦀️.rs`; `…any\🚪️io\🪶️sqlite\📸️snapshot\📏️encoding\🦀️.rs`; `…any\🚪️io\🪶️sqlite\📸️snapshot\🧪️tests\🦀️.rs`; `…any\🧪️tests\🧰️support\🦀️.rs`; `…any\✏️editor\📌️panels\🗿️artifact\🧪️tests\🔬️unit\🦀️.rs`. (Its SQL, TS and JSON fixtures were changed by another agent in the same ticket.)

Puzzle 5D: `C:\semio\✏️s\🔌️plugins\🧩️puzzle\🗿️artifacts\🖐️5d\🔨️modules\♻️rewriting\🦀️.rs`.

## Notes and risks

- The TS fork's own notes are in `document-load-ts-fork-notes.md` (ticket root). All one-off edit scripts and logs of this work were deleted; every changed Rust file passes a `rustfmt --check` parse.
- Last attempt to compile the whole Jack workspace (after the disk recovered): `semio-plugin` itself fails at `🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🦀️.rs:108` (it re-exports `assert_latest_history_preview` and `assert_mounted_history_overwrite` that no longer exist in `semio_plugin_testing_acceptance`), which is another agent's in-flight framework change. To finish: rerun `cargo check --workspace --all-targets -j 6 --keep-going` in the Jack, Rewriting and Puzzle workspaces with `CARGO_TARGET_DIR=C:\cargo-nr\t`, fix leftovers, then run the host test and record the `[DEBUG]` line here (and delete that `eprintln!` afterwards).

- A stray `taskkill` matched another agent's `cargo check ... --keep-going` (pid 46896) while I was cleaning up; I killed it by mistake. Whoever ran it must restart that check.
- `C:\cargo-dl` (about 1.7 GB, left by my first killed build) still needs deletion by a person; the shared `C:\cargo-nr` target was used for all later builds.
- Unverified until the framework builds again: editor/viewer/executor/shell/test crates of Jack, Rewriting and Puzzle (possible leftovers: unused imports, call sites of `Graph.manifest`).
