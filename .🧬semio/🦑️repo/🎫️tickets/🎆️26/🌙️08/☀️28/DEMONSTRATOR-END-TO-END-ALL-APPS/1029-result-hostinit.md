# 1029 Result: hostinit (plugin host crates on the pre-10/28 store-initialization API)

STATUS (10:50 10/29): EDITS DONE FOR ALL SEVEN HOSTS, VERIFICATION PENDING (process3d host lib+tests already compiled GREEN natively `hostinit-p3d1`; process3d workspace check `p3d2` queued; no other host compiled yet). Runner `C/1029-tools/hostinit/{c,w}.sh`, logs `C/🗑️generated/1029/hostinit-*.{txt,rc}`.

## Decision: framework-default initializer instead of per-plugin ports
Every bespoke `ArtifactStoreInitializationAuthority` (process3d, gismap, generation2d, generation3d, raster, draw, jack) is a copy of the framework replay (`semio_framework_os_store_initialization_replay::ArtifactStoreReplayInitializer`, same phase names). The 10/28 framework rewrote that replay around original-construction admission (`admission_demands`/`admit_original_construction`, `retire_step` instead of `clear`, `Pending` admission, `OriginalTurnFailure`) and its doc says an app implements the trait only when initialization is more than a ledger replay. Porting seven copies of that accounting is both error-prone and duplicate code, so each host now drops its authority and the editor/viewer falls back to the default `build_document_store_initialization_job` (cad, puzzle, energy, flow, 200+ editors already do). The decode owner catalogs (`build_envelope_decode_owner_bundle`) stay.

## SHARED CHANGE
- `ArtifactEditor` (`🧰️framework/…/🔌️plugin/📜️declaration/🪟️surface/✏️artifact-editor-trait/🦀️.rs`): new provided hook `admit_document_store_publication_authority(operation, generation)` (no-op default) called by the default `build_document_store_initialization_job`, so domains with `REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY` (process3d, generation2d, generation3d) keep their lease/ABA gate (`validate_document_store_publication` consumes and releases it) without a bespoke initializer.

## Per host
- process3d: deleted `🚀️store-initialization`, `🧮️census-copy-timeline`; value factories moved to `🪪️decode-owner-catalog`; `process3d_demand_grant` moved to the host root; editor overrides `admit_document_store_publication_authority`; unused imports/consts pruned; `🧪️retained-laws` lost the initializer/timeline/census laws, `owned_store_measured` now builds candidates with `ArtifactStoreReplayInitializer<…, BoundedDocumentStoreOwnerProducer>`; decoder-backing laws use a `Vec<ProcessStep>` owner.
- gismap: deleted `🏁️initialization-authority`, `🔚️initialization-close`, `GisMapSnapshotCloneAuthority`; editor override removed; host unit laws re-pointed at the framework initializer (alias).
- generation3d: deleted crate `semio-s-artifact-procedural-3d-initialization` (members, workspace dependency, 2 Cargo.toml deps, Cargo.lock of the workspace and of `🌀️procedural/🏭️bridge`); editor uses the lease hook; `🔬️retained-authority-laws` initializer laws re-pointed at the framework initializer.
- generation2d: deleted `🏗️store-initialization-authority`, `🔁️store-initialization-phases-and-job` (publication hostiles kept as `🎭️publication-hostile`), snapshot copy cursor (credits kept as `🧮️publication-credits`); editor uses the lease hook.
- raster: removed authority + job from `🚀️store-initialization` (owner-catalog fns kept for editor/viewer), editor override removed, owned-host laws and mounted-boot law re-pointed.
- draw: deleted `🚀️store-initialization`, `🔄️initialization-turn`; editor and viewer overrides removed.
- jack: deleted `🏗️initialization-authority`, `📜️initialization-contract`; editor override removed; unit laws re-pointed; the two laws of the deleted authority's close internals (typed refusal / borrowed fault) and their fixture `🧫️fixtures/🚫️close-refusal` removed.

## Open risk to verify by tests
History-bearing documents (raster demo archive replays one edit) now go through the default replay, which needs `owners.replay_preparation`; catalogs built by `bounded_artifact_store_owners`/`admit_source_constructor` install none. Law `a_demo_edit_archive_loads_back_through_the_document_archive_door` (raster editor) is the probe.
