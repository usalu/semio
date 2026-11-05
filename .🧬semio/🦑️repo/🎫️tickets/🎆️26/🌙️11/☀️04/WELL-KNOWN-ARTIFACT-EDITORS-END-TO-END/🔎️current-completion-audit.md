# 🔎 Current Completion Audit

Read-only audit of current acceptance source and current ticket logs. Existing batch reports are historical claims, not proof of current completion. No builds or browser runs were launched by this audit.

## Requirements and Completion Criteria

The current matrix enumerates 38 artifacts. Completion requires every declared editor and supported subset to be registered, buildable and reachable in the OS; opening valid natural files; a visible useful document; a concrete supported edit; undo and redo; save acknowledged for the exact document revision; close/reopen and browser reload preserving the edited content and identity; export independently parsed with edited semantic content retained; invalid import refused without changing the document; accessible keyboard interaction and localized labels; progress and cancellation of expensive work; and honest multi-format preservation/refusal. The repository additionally requires language-agnostic fixtures and third-party comparison. Palette-opening an example document alone does not establish natural-file support.

## Current Harness Blockers

| Priority | Current code | Gap | Required proof |
| --- | --- | --- | --- |
| P0 | browser drive/openProgram | Opens palette program only; valid row sample is never imported. | Open actual sample through shell file route and assert resolved editor and content. |
| P0 | browser bodyRendered | Checks every global pane for any characters/SVG/canvas; unrelated windows can satisfy it. | Scope to opened document window and verify expected content. |
| P0 | browser save | Global check-in count decrease is accepted as save; no exact document/revision or storage receipt. | Observe exact save acknowledgment and persisted edited content. |
| P0 | browser reopen and reload-reopen | Calls palette open again, checks painting only; no identity or edited-content assertion. | Reopen saved artifact by identity and assert edit after close and full reload. |
| P1 | browser program selection | Ignores ownerApp and picks first matching kind/editor, fallback ignores plugin. IDF owner is energy.model. | Resolve ownerApp plus correct subset deterministically. |
| P1 | browser invalid step | Any new notice/refusal suffices; document preservation not checked; filechooser promise unawaited. | Require import-specific refusal and unchanged identity/content/history. |
| P1 | browser render fault handling | render uses cumulative faults, while entry.pass only checks step statuses; late faults need not fail a row. | Attribute and fail all faults within the current journey. |
| P1 | browser journey coverage | One row per artifact, selecting first app; subset/standard editors and multiple natural formats not exhaustively covered. | Explicit per-supported-editor and per-format journeys. |
| P1 | oracle validation | Many formats validate signatures only (DWG AC, GIF GIF, PDF header, PLY ply, STL any >=84 bytes). CSV/TSV only UTF-8. | Independent structural parser and assertion of edited semantic value. |
| P1 | contract invalid samples | expected list may be empty and equality 0=0 passes; missing invalid sample files are silently filtered. | Require existing non-empty required fixture lists. |
| P1 | contract sample selection | Chooses any accepted candidate; metadata agreement does not exercise codec/editor. | Required fixture-specific decoder round trips with independent oracle. |

Current browser source: ✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts. Offline tests: ✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts. Oracle implementation: ✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🔬️oracles/🟦️.ts.

## Current Evidence Matrix

| Artifact | Current configured edit | Latest p3 workspace evidence | End-to-End |
| --- | --- | --- | --- |
| html | textEdit | rc=1; error[E0425]: cannot find function `demo_diff_cases` in this scope | Unproven |
| json | set-node | rc=1; error: failed to run custom build command for `blake3 v1.8.5` | Unproven |
| md | textEdit | rc=1; error: failed to run custom build command for `onig_sys v69.9.3` | Unproven |
| txt | textEdit | rc=1; error[E0599]: no method named `absorb` found for struct `mirror::standards::v_utf_8::subsets::any::schema::diff::component::TxtDiff` in the current scope | Unproven |
| xml | set-node | rc=1; error[E0425]: cannot find function `xml_escape_attr` in this scope | Unproven |
| csv | set-cell | rc=1; error: test failed, to rerun pass `-p semio-s-artifact-stdio-csv-io-registry --lib` | Unproven |
| docx | scan | rc=1; error: failed to run custom build command for `ring v0.17.14` | Unproven |
| idml | none | rc=1; error[E0252]: the name `FromValue` is defined multiple times | Unproven |
| pptx | scan | rc=1; error: failed to run custom build command for `zstd-sys v2.1.2+zstd.1.5.7` | Unproven |
| tsv | set-cell | rc=1; error[E0599]: no method named `unwrap` found for struct `RetainedWorkOutcome<T, E>` in the current scope | Unproven |
| xlsx | scan | rc=1; error[E0599]: no method named `diff` found for enum `strict::schema::mutations::XlsxStrictMutation` in the current scope | Unproven |
| bmp | scan | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-bmp-mutations (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"files","path":"/home/deck/semio/✏️s/🔌️plugi | Unproven |
| gif | setSnapshotValue | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-gif-editor (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"files","path":"/home/deck/semio/✏️s/🔌️plugins/ | Unproven |
| jpg | setSnapshotValue | rc=1; error[E0432]: unresolved import `semio_framework_ui_contract` | Unproven |
| png | scan | rc=1; error[E0425]: cannot find function `demo_mutation_cases` in module `mutations` | Unproven |
| tiff | setSnapshotValue | rc=1; error[E0432]: unresolved import `semio_framework_ui_contract` | Unproven |
| dwg | setSnapshotValue | rc=1; error[E0599]: no method named `inverse` found for enum `component::DwgMutation` in the current scope | Unproven |
| dxf | setSnapshotValue | rc=1; error[E0432]: unresolved import `crate::standards::v_r12::subsets::any::io::text::snapshot::preview` | Unproven |
| pdf | scan | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-pdf-schema-4 (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"files","path":"/home/deck/semio/✏️s/🔌️plugin | Unproven |
| svg | setSnapshotValue | rc=1; error[E0425]: cannot find function `demo_diff_cases` in this scope | Unproven |
| gltf | setSnapshotValue | rc=1; error: failed to run custom build command for `blake3 v1.8.5` | Unproven |
| las | setSnapshotValue | rc=1; error[E0433]: cannot find `engine` in `crate` | Unproven |
| obj | setSnapshotValue | rc=1; error[E0433]: cannot find `engine` in `crate` | Unproven |
| ply | setSnapshotValue | rc=1; error[E0433]: cannot find `engine` in `crate` | Unproven |
| step | setSnapshotValue | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-step-snapshot (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"files","path":"/home/deck/semio/✏️s/🔌️plugi | Unproven |
| stl | setSnapshotValue | rc=1; error[E0433]: cannot find `engine` in `crate` | Unproven |
| bcf | scan | rc=1; error[E0599]: no associated function or constant named `KIND_ID` found for struct `TableWindowKit` in the current scope | Unproven |
| epw | scan | rc=1; error[E0599]: no method named `print_diff` found for struct `EpwDiff` in the current scope | Unproven |
| idf | scan | rc=1; error: cargo test --workspace --no-fail-fast --profile prototype --config profile.prototype.inherits="dev" --config profile.prototype.opt-level=0 --config profile.prototype.debug=1 --config profile.prototype.codegen-units=256 --config | Unproven |
| ifc | setSnapshotValue | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-ifc-2x3-base (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"bytes","path":"/home/deck/semio/✏️s/🔌️plugin | Unproven |
| avi | setSnapshotValue | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-avi-tests (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"bytes","path":"/home/deck/semio/✏️s/🔌️plugins/� | Unproven |
| mp3 | setSnapshotValue | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-mp3-inferences (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"bytes","path":"/home/deck/semio/✏️s/🔌️plug | Unproven |
| mp4 | setSnapshotValue | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-mp4-viewer (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"bytes","path":"/home/deck/semio/✏️s/🔌️plugins/ | Unproven |
| wav | scan | rc=1; error[E0425]: cannot find function `close_with_self_funded_context` in crate `semio_framework_job` | Unproven |
| binary | scan | rc=1; error: Rust crate Source refused before compiler: semio-s-artifact-stdio-binary-snapshot (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"version":1,"reason":"files","path":"/home/deck/semio/✏️s/🔌️plu | Unproven |
| deflate | scan | rc=1; error[E0576]: cannot find method or associated constant `diff` in trait `protocol::MutationKind` | Unproven |
| semio | scan | rc=1; error[E0432]: unresolved import `semio_s_artifact_stdio_ifc_geometry` | Unproven |
| zip | scan | rc=1; error[E0599]: no method named `diff` found for enum `standards::v2_0::subsets::base::schema::mutations::component::ZipMutation` in the current scope | Unproven |

Log rows are historical run evidence, not claims that the exact same failure remains in current source. Source edits require a fresh correctly scoped passing run. The current [DEBUG]p3-contract.log records 192 pass / 2 fail / rc=1: IDML shipped-editor/viewer and valid-sample checks fail. dev-s.log ends in build:graph-plan killed by SIGKILL. These supersede older runtime report claims about capacity waiting; neither proves a running application.

## Required Verification Order

1. Finish current source compilation defects and source-admission/toolchain blockers, then rerun the affected workspace suite.
2. Make the journey assert exact artifact identity, persisted revision and semantic content before relying on green output.
3. Bring up the actual app and run real valid-file import/edit/save/reopen/reload/export/refusal for each editor and format.
4. Run keyboard/localization, cancellation, failed-save/stale-load, repeat-open/disposal and multi-document isolation checks.
5. Transcribe final command, exit, counts and failures before generated-log cleanup; keep this audit and input scripts.
