# Reachable bounded store disposers

Ticket 26/09/22. Follows 26/09/18 slice TC3e §7, which counted the omission and fixed only `NoteViewer`.

## What fails

`ArtifactEditor::build_document_store_disposer` and `ArtifactViewer::build_document_store_disposer` default to `None` (`🔌️plugin/🦀️.rs`, the `ArtifactEditor` and `ArtifactViewer` defaults). `VcsArtifactApp` captures that value at construction and `drive_artifact_owned_disposer` faults `interactive-job.close-owned-disposer-missing` ("app owner did not provide the required bounded disposer for <lane>") when the close ladder reaches a lane whose disposer is `None`. The store then hits `Drop` without its terminal-empty witness. On a `panic = "abort"` wasm32 guest that assert is an abort.

The close ladder drives document, config, draft, presence, and transient. `ViewerApp` supplies the draft lane itself. A viewer therefore declares document, config, presence, and transient. The worked shape is `🗒️note` editor `🦀️.rs` (the five lanes) and `🗒️note` viewer `🦀️.rs` (the four viewer lanes), plus the mounted-side note on `🧱️block` 3d viewer `🦀️.rs`.

## Census

Every `impl ArtifactEditor for` / `impl ArtifactViewer for` under `✏️s/🔌️plugins`, including the two path-qualified impls (`PlaybookViewer`, `Generation3dViewer`):

| | impls | missing `build_document_store_disposer` |
|---|---|---|
| measured 2026-09-22 | 296 | 228 |

TC3e §7 recorded 229 of 298. This pass is two impls and one missing surface smaller; the reachable set below does not depend on that remainder. `🗒️note` is 2/2 declared. `🗄️stdio` is 9 declared and 167 missing. `🌍️gis` is 2 declared (both editors) and 2 missing (both viewers).

## What "reachable" means

A missing disposer fires only when something constructs the app and then closes it.

1. **Mounted.** The hub's trusted catalog (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `TRUSTED_BOOTSTRAP_PACKAGES`) opens documents only for packages with `opensDocuments: true`, and `trustedBootstrapDescriptorOpenTargetsV1` keeps only the **editor** of each kind. That is the note editor and the two GIS editors. All three already declare disposers. Stdio has `opensDocuments: false`.
2. **Guest codec.** `plugin_artifact_codec_app` constructs every app of the bundle, keeps the editor, and closes every other app through `close_artifact_codec_app`. The hub calls that guest interface only when `linkedCodecRegistry` is null. In the selectable closure that is **note only**. Stdio and GIS carry linked Rust codecs, so the hub never asks their guests.
3. **The staged sweep is the other caller.** `owned_codec_answers_every_call_on_every_staged_component` drives all four `codec` exports on `STAGED_CODEC_COMPONENTS` (note, gis, stdio) whenever `plugin_wasm` finds a component under `.🧬semio/🦑️repo/⚡️cache/cargo/target*`. All three wasm files are present, so the sweep constructs every app those components register, including the viewers the hub itself never opens.

A surface that is not in a staged component, and is not an open target, is never constructed. It never closes.

## What the staged components actually register

**GIS** (`🌍️gis/🦀️.rs`): `GisMapEditor`, `GisMapViewer`, `GisTerrainEditor`, `GisTerrainViewer`. The two editors already declare disposers. The two viewers did not. Both viewers are closed by every `codec` call on `semio_s_plugin_gis.wasm`.

**Stdio** (`🗄️stdio/🔌️plugin/🦀️.rs`): the shipped component (`not(feature = "full-app-catalog")`) registers 18 apps — csv, tsv, txt, json (any and i-json), xml (any and valid), md, html, each as editor and viewer. The nine editors already declare disposers. The nine viewers did not. `full-app-catalog` names the other ~70 subsets (the rest of the 167) and the crate documents that fleet as library-only: it cannot be linked into a component, because it blows `wasm-component-ld`'s function ceiling. Those apps are not in `semio_s_plugin_stdio.wasm`, so the sweep does not construct them, and the hub does not open them.

**Note** is already declared. Its newest staged component is the TC3e wasm-release build.

Other plugins (norm's 30, the draw viewer, the playbook viewer, and the rest) are not in `STAGED_CODEC_COMPONENTS` and are not hub open targets. Left as they are.

## Declarations added

The nine shipped stdio viewers and both GIS viewers. Each is `NoConfig` / `NoPresence` / `NoTransient`. Each now declares:

* `build_document_store_owners` and `build_config_store_owners`
* the four disposers `bounded_document_store_disposer`, `no_config_store_disposer`, `no_presence_store_disposer`, `no_transient_store_disposer`
* `no_presence_local_root_retirement_factory`, `no_presence_peer_retirement_factory`, `no_transient_local_root_retirement_factory`

Document owners match the sibling editor. The nine stdio viewers and `GisTerrainViewer` use `bounded_document_store_owners`. `GisMapViewer` uses `crate::spr::gis_map_document_store_owners` (the editor's catalogue, which lives in the schema module, not the editor module).

`NoteViewer` already had the four disposers (TC3e). The first sweep of a component built from that source still died in presence close with `presence close requires its installed local-root retirement factory` (`🏪️store/👥️presence/♻️retirement/🦀️.rs`). The same three factories were added there. `ViewerApp` still supplies the draft lane.

The other 158 stdio apps, and every other plugin's missing surfaces, were not edited.

## Verification

`CARGO_INCREMENTAL=0`, private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-bounded-disposers`.

`cargo check --features component-app-assembly` of `semio-s-artifact-stdio-{csv,tsv,txt,json,xml,md,html}`, `semio-s-artifact-gis-gismap`, and `semio-s-artifact-gis-gisterrain`: exit 0 in 1m 22s. The gismap fingerprint records `features: ["component-app-assembly", "default"]`, and that unit's dep-info names `👁️viewer/🦀️.rs`.

Wasm components, built one package at a time (a combined `stdio`+`gis` link duplicates the WIT exports, because gis embeds stdio with `default-features = false` and a joint `cargo build` unifies stdio's default `plugin-entry` back on):

| component | profile | bytes |
|---|---|---|
| `semio_s_plugin_note.wasm` | wasm-release | 14 948 073 |
| `semio_s_plugin_stdio.wasm` | wasm-release | 49 771 679 |
| `semio_s_plugin_gis.wasm` | wasm-release | 47 884 925 |

`cargo test -p semio-framework-plugin-host owned_codec_answers_every_call_on_every_staged_component` against those three release components (the freshest mtime `plugin_wasm` will select): **failed, 3/3, 287s**.

| package | first failure |
|---|---|
| `semio:note` | `codec.print-mirror(note.document) lost the minted identity` |
| `semio:gis` | `codec.pack-schema-hash(gis.map): DeadlineExceeded` |
| `semio:stdio` | `codec.pack-schema-hash(stdio.txt): DeadlineExceeded` |

Note's row is past the close fault. `codec_sweep_one_component` runs `pack-schema-hash`, then `genesis` by schema and by kind, before `print-mirror`. Those three returned. The mirror came back, and its dsl text does not contain `artifact-0123456789abcdef0123456789abcdef`. That is a document-text assertion, not `close-owned-disposer-missing`.

The run before the retirement factories, on the same release profile, is why the factories were added: note and stdio answered `pack-schema-hash` with `presence close requires its installed local-root retirement factory` inside the 120s budget, and gis hit `DeadlineExceeded`. After the factories, stdio no longer returns that fault; the call now runs until the law's 120s deadline (`codec_budget().deadline_ms`). The wasm-dev components (stdio 252 MB, gis 216 MB) miss that same deadline. The law therefore does not yet show a terminal-empty close of the stdio or gis guest. It does show they no longer fail closed in the first seconds with `interactive-job.close-owned-disposer-missing`.
