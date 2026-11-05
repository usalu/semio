# Public Reload Observers

The ticket-only browser journey now requires authored surface equality across a real attached-folder reload, in addition to history membership. The schema declares the actual public surface/window pair for each editor: bitmap input, grid2d grid, grid3d grid and both graph author canvases. Bitmap also compares the kept output surface, so the pin-preview action has an observable persisted rendering consequence.

Each observation activates the same public dock tab, closes framework panels, moves the pointer away, then records SHA256 of the surface screenshot and its public role/ARIA/leaf-text projection. Reload must restore both within a bounded 30-second observation. A missing surface fails rather than comparing two empty observations. Bitmap's no-argument pin-preview remains a direct ActionPane action: the real framework dispatch bypasses staged forms when actionRequiresStagedForm is false, including destructive markers.

Before reload the journey must witness an actual tool-run projection. After boot it opens the public tool-run panel and requires both no run projection and absence of the exact previous scope. This is an explicit ephemeral UI reset check, independent of successful history replay. It does not infer the contents of an inaccessible transient cache from the tool-run panel.

Folder attachment uses the native FolderEventLogStorage. Its latest snapshot consists of pack plus SPR, and may arrive in an archive. Reading only the artifact base pack would omit replayed edits. No new decoder, guest-state read or invented DSL mirror was added. Exact hidden seed and unrendered authored fields remain outside these browser observers; the canonical snapshot/SPR and revision/render native suites still need to run.

Changed retained inputs: wfc-browser-schema.json, wfc-browser-cases.json and wfc-browser-scenario.ts. The existing Ajv schema check, Bun import of all five scenarios and both-locale public workload expansion passed after the change. The browser journey itself has not run; no runtime reload or persistence result is claimed.

Related: [Browser Workloads](run-r2-browser-workloads.md), [Browser Audit](run-r2-browser-audit.md).

Independent read-only audit found no concrete source-level blocker in the public surface/reset assertions. It confirmed the observer remains narrower than complete canonical snapshot replay semantics, as stated above.

## Surface Dependency Review

The author-surface comparisons do not require solve-cache persistence. Both graph adapters wrap only their canonical Snapshot and map slot rectangles plus authored pinned_tile_id (graph2d editor/🕸️graph-view-args-and-gestures/🦀️.rs:16–32; graph3d editor/🔗️graph-view-and-editor/🦀️.rs:11–27). Their render dispatch passes that adapter and empty selection, while only the separate preview branch receives transient solve data. The selected graph surface is wfc.graph.

Grid2d grid scene/render accepts Snapshot, window config and utility only (windows/🔲️grid/🦀️.rs:193–199). Grid3d grid scene projects grid_meshes_json(document) and grid_instances_json(document), with selection/hover supplied separately; it has no solve payload parameter (windows/🧱️grid/🦀️.rs:244–269). Bitmap input accepts Snapshot and input config only (windows/🖼️input/🦀️.rs:155). The pointer is outside the surface for both observations.

Bitmap output is intentionally an additional mixed renderer, but the scenario first keeps the complete output as authored pins. That command changes canonical revision, so the pre-keep cached solve is ineligible; complete_pinned_indices supplies the authored fallback (windows/🧩️output/🦀️.rs:185–194). The same fallback must render after reload. Partial/failed keep cannot manufacture a pass because its preceding one-edit verdict and matching output witness remain required.

## Selection, Camera and Hover Baseline

No actual baseline-neutralization blocker was found in this scenario. The authored setup uses only action forms for change-seed, tile creation/color, slot creation and resize, followed by public framework tool controls. These branches emit canonical document mutations, not camera or selection updates. Both graph author dispatches explicitly supply an empty selection slice. Neither the setup nor reload witness drags, zooms, picks or points inside an author canvas. The witness focuses the same dock tab and moves the pointer to the page origin before capture. Grid3d has selection/hover render parameters, but the scenario never invokes its pick/worldSelect/camera routes. Bitmap input/output and grid2d cameras therefore remain at the same form-unmodified window configuration. No speculative reset action was added that could mask a real persistence defect.

## Exact Native Persistence Coverage

Read-only inspection found a concrete coverage gap: artifact-native pack+SPR replay is instantiated for graph3d, while the inspected bitmap/grid2d/graph2d/grid3d store-fixture modules exercise snapshot codecs and/or direct mutation/inverse ladders. This does not demonstrate a production persistence failure. It means the browser surface observer must not be described as complemented by an already-established all-five SPR replay suite.

Each artifact workspace declares its root package as a member, and the root Rust entry explicitly mounts the following modules under cfg(test); full workspace --lib selection therefore executes these laws rather than merely compiling them as dependencies.

| Artifact | Selected root package | Exact tests and actual scope |
| --- | --- | --- |
| Bitmap | semio-s-artifact-wfc-bitmap | Artifact-root 🧪️tests/🔬️store-fixture/🦀️.rs: every_example_survives_the_pack_envelope compares complete snapshots. a_committed_example_takes_a_whole_edit_session_and_inverts_it applies seed4242, a pin and input pixel edits directly with vcs::apply_mutation, then undoes to the full original. Despite its module documentation, it does not instantiate ArtifactStore or serialize/replay SPR. |
| Grid2d | semio-s-artifact-wfc-grid2d | Artifact-root 🧪️tests/🔬️store-fixture/🦀️.rs: every_example_survives_the_pack_envelope and a_decode_in_place_retires_the_displaced_document_in_bounded_steps compare full example snapshots. Artifact-root unit test the_document_round_trips_through_its_own_dsl_and_pack_codecs covers both snapshot carriers. No edit-history replay in those laws. |
| Graph2d | semio-s-artifact-wfc-2d | Artifact-root 🧪️tests/🔬️store-fixture/🦀️.rs: a_mounted_document_round_trips_through_both_envelopes compares complete pack/DSL snapshots; an_edit_ladder_applies_and_inverts directly applies seed4242 and room-a pin then undoes. No ArtifactStore/SPR replay. |
| Grid3d | semio-s-artifact-wfc-grid3d | Artifact-root 🧪️tests/🔬️store-fixture/🦀️.rs: a_mounted_sequence_of_edits_lands_on_a_document_that_still_round_trips directly applies seed4242, tile, pin, mask and removal edits, then compares whole pack/DSL snapshots. undoing_the_whole_sequence_returns_the_document_to_the_committed_example checks inverse restoration. No serialized history replay. |
| Graph3d | semio-s-artifact-wfc-3d | any/🧪️tests/🗄️store-fixture/🦀️.rs: a_store_opens_on_a_bundled_example_applies_an_edit_and_closes opens a real funded-owner ArtifactStore, dispatches change_seed(99), checks seed99, and runs actual document text+pack round-trip helpers. the_tower_example_round_trips_through_a_live_store verifies full geometry through the same helpers without an authored edit. |

The graph3d helper is substantively stronger than an ArtifactPack round trip. Framework 🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs:631 assert_document_pack_round_trip calls print_document_pack(envelope), then parse_document_pack(&pack_files.pack, &pack_files.spr, genesis). It compares the parsed snapshot with the live snapshot and preserves viewer heads; it also compares canonical-trunk pack+SPR and DSL+ops results. The text helper at line448 compares shared edits, mutation metadata, transitions and the full reconstructed canonical snapshot.

Snapshot-only hidden-field codecs additionally run in their actual component owners: bitmap semio-s-artifact-wfc-bitmap-snapshot (the_pack_codec_round_trips_the_document fixture has seed3,2×2 geometry and a pin; the_dsl_round_trips_the_whole_document uses seed42 and a pin), grid3d semio-s-artifact-wfc-grid3d-snapshot (encoding_then_decoding_is_a_fixed_point and printing_then_parsing_is_a_fixed_point_for_every_example), graph3d semio-s-artifact-wfc-3d-snapshot (every_example_round_trips_through_the_pack_codec / every_example_round_trips_through_the_dsl). Graph2d snapshot codecs are mounted by semio-s-artifact-wfc-2d-schema; its root artifact store-fixture independently exercises both full carriers. Grid2d snapshot codecs are mounted by semio-s-artifact-wfc-grid2d-snapshot; its root artifact laws above provide explicit whole-document equality.

These are source coverage mappings only. None is reported as passing a fresh native run here. No speculative persistence test or runtime change was added during diagnostic standby.


## Grid2d/Grid3d Minimal Live Store Fixture Mapping

Auditor read-only constructor/import review for root's new replay laws. Grid2d root already aliases kernel as store and protocol. Existing root store-fixture has examples() parsing bundled sources; use first valid returned Grid2dSnapshot. Import crate::mutations::{change_seed,Grid2dMutation}. Grid3d use crate::examples::blocks::snapshot and crate::mutations::{change_seed,Grid3dMutation}, retaining its existing snapshot import.

Use actual document.schema.clone() for envelope schema rather than copying graph3d fixture's hardcoded /v1 spelling. Canonical create_document_envelope(schema,id,document,None,&os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), then ArtifactStore::new::<os_store::ArtifactSnapshotPublication>(envelope,protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await. Install document_store_owners::funded_bounded_artifact_store_owners::<Snapshot,Mutation>() exactly before dispatch; a bare store refuses edits. Dispatch ArtifactCommand::Apply{mutations:vec![change_seed(99)],transaction:None} with artifact_app_laws::fixture_identity. Assert initial seed differs from99, edited full snapshot/seed, then canonical assert_document_pack_round_trip (and text helper if wanted). Close through close_owned_step(1,ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) until terminal with bounded loop.

Both root manifests already dev-enable kernel protocol-laws/mutation-testing,plugin artifact-app-testing and asyncmacros; no new runtime library required. Root store-fixture modules are included by root library unit tests, so proposed allworkspace --lib gates execute the new laws. Direct existing codec/mutation tests remain distinct.

Independent reference options: existing Bun Ajv validates shared language-neutral fixture/schema; Python stdlib sqlite3 or existing Bun SQLite can independently replay a seed event into a small state row and compare seed99, preserving initial/genesis seed separately. Such an oracle validates expected event semantics, not the native SPR binary format. Canonical store helper is the actual pack+SPR decoding/replay evidence; do not claim an independent JSON/SQL fixture is a second SPR implementation.

## Bitmap and Graph2d Replay Acceptance Laws Added

The identified gap is now addressed in source for bitmap and graph2d by authored_seed_pins_and_geometry_survive_actual_pack_spr_replay, mounted in each existing artifact-root store-fixture module and selected by its root package. Root independently owns the corresponding grid laws. Both consumers load the schema-first shared WFC 🧪️tests/🗄️store-replay corpus and verify the exact example row, an initially different seed and an initially free pin target. They open a real ArtifactStore using the actual document.schema, install its exact funded bounded owner catalog, dispatch seed99 plus geometry and pin mutations through ArtifactCommand::Apply, and compare observed native fields with the portable expected projection. Bitmap resizes output to5×4 and pins(0,0)color1; graph2d moves room-a to(7,9) and pins it to room.

Both then invoke the proven full DSL+ops and pack+SPR document helpers, comparing reconstructed entire snapshots with the edited live store, and drain the real bounded store owners before drop. Existing root dev dependencies supply all required APIs, so no manifest/runtime dependency was added. Pinned rustfmt parser checks passed for both files. Actual native execution remains pending; authored coverage is not a test-pass claim.

## Independent Authored Projection Reference

At 2026-11-05T02:36:56.861Z, Ajv validated the shared store-replay schema and exact four-artifact roster. Existing third-party jsonc-parser independently applied each authored seed/pin/geometry projection recipe; all4 outputs equal the same portable expected projections consumed by actual native store laws. Grid dimensions remain authored baseline; bitmap size/periodicity and graph location are actual edits. This oracle checks expected event/field semantics, not the SPR binary codec. Native laws use canonical store pack+SPR replay and remain unexecuted until their actual suites run.

Follow-up validation: Root independent Ajv+jsonc-parser reference oracle passed4/4 portable replay cases. Read-only audit found no concrete API/custody issue in the four new laws. The public cfg=[test] root-source preflight measured bitmap777active lines/6files and graph2d1255/13files; these are explicitly non-authoritative and not upper bounds, with actual native Source Admission still required. Scoped diff --check also passed for the two owned files.


## Four New Live Store Replay Laws Reviewed

Read actual bitmap,grid2d,graph2d,grid3d root store-fixture additions after authoring. No concrete API/lifetime/custody mismatch found. Each uses a valid bundled example, seed!=99 and an absent target pin precondition, actual document schema, funded exact store owners, actual authored mutation dispatch, a portable expected seed/pin/geometry projection, canonical text and pack+SPR roundtrip helpers, then bounded paid owned-store close. Graph2d decimal7.0/9.0 coordinates intentionally preserve floating geometry. Shared expected-field equality precedes full canonical snapshot replay helper equality; this is substantive editedstate coverage, not a genesis codec-only roundtrip.

Public readRustSourceClosureV1 diagnostic cfg=[test] for actual root library entries artifact/🦀️.rs with package manifestDirectory:bitmap777lines57451bytes6files;grid2d604/48371/6;graph2d1255/106205/13;grid3d836/63562/7. No4000line risk appears in this sample. This is not authoritative native admission and not an upper bound. Retained input root-source-preflight.ts; detail generated/root-source-preflight.json. Initial nonexistent package-local source entry was corrected to actual manifest lib path before these results. No source edits or native queue; execution of new laws remains pending.

Graph3d proven fixture follow-up: any/🧪️tests/🗄️store-fixture/🦀️.rs now builds its envelope using document.schema instead of the stale hardcoded s.wfc.wfc3d/v1 string. This aligns the existing real replay law with its actual canonical snapshot and the four new fixtures. Pinned parser passed.


## Replay Test Mounting and Filters Confirmed

All four new corpus cases are reached by root artifact library cfg(test) mod store_fixture. Each root package manifest has lib path ../../🦀️.rs and is an explicit artifact workspace member. The async_test annotation wraps the named law; these are executable native tests, not merely dependency helper code. Fourcase corpus entries are explicitly selected by bitmap,grid2d,wfc2d,grid3d IDs in their own tests.

After full suites, a focused rerun may use the same workspace commands with test filter actual_pack_spr_replay before the final test-harness separator:

```powershell
repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- test --manifest-path '✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.toml' --workspace --lib --locked actual_pack_spr_replay -- --test-threads=1
repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- test --manifest-path '✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.toml' --workspace --lib --locked actual_pack_spr_replay -- --test-threads=1
repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- test --manifest-path '✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.toml' --workspace --lib --locked actual_pack_spr_replay -- --test-threads=1
repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- test --manifest-path '✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.toml' --workspace --lib --locked actual_pack_spr_replay -- --test-threads=1
```

Fifth graph3d coverage: root 🦀️.rs282 mounts standards/1/any/tests/🗄️store-fixture. Exact law store_fixture::a_store_opens_on_a_bundled_example_applies_an_edit_and_closes dispatches seed99 on real funded store then invokes text and pack+SPR roundtrip helpers. Separate tower law checks a complex example without a newly authored edit. Focused exact selector:

```powershell
repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- test --manifest-path '✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.toml' --package semio-s-artifact-wfc-3d --lib --locked store_fixture::a_store_opens_on_a_bundled_example_applies_an_edit_and_closes -- --exact --test-threads=1
```

Commands are mapped only, not executed. Full workspace suites already include these laws; do not rerun merely to duplicate a passing verdict unless source changes/failure justify it. Graph3d is not in the newly authored fourcase corpus and must not be claimed as its fifth fixturecase.
