# wave-din report (din4108, din16798, din18599)

Result: `check -p` of all 39 non-leaf din crates (plus their `mut-*` leaf dependencies) via `T/wave-din-check.sh` (log `T/🗑️generated/wave-din-2.log`): 0 errors, exit 0.

## Crates, errors before -> after
- din16798-viewer 33 -> 0, din4108-viewer 32 -> 0, din4108-io 16 -> 0, din16798-io-text 4 -> 0, din18599-mutations 2 -> 0.
- Dependents that surfaced and are now clean: din16798/din4108/din18599 editor, din18599 viewer/io/io-text, io, schema, root crates.

## Root causes and fixes
1. din18599-mutations E0080: variant `ChangeZoneInternalGainsWM2`/`ChangeZoneLightingPowerWM2` kebab to `...-wm2`, leaf kinds were `...-w-m2`. Renamed kind strings (and fixture/schema folder names) to `-wm2` across the din18599 tree plus the four generated catalogs that embed them (`🧰️framework/.../🗂️schema-document-index.json`, `🦑️repo/.../🔣️schema-catalog.json`, `📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`, `🌎️hub/.../📕️norm/🧩️component/🔣️.json`).
2. Orphan impls (E0117/E0116): io facets compiled in io crates now compile in the owner crates, the en1991 pattern. Model crate mounts `io/🪶️sqlite` and new `io/🪞️records` (`impl_norm_artifact_record!` with sqlite codec); mutations crate mounts `io/📝️text|💾️binary/🧬️mutations`; diff crate mounts `io/📝️text|💾️binary/🔺️diff`. io-text/io roots re-export (`pub use ..._mutations::text as mutations`, `..._diff::text as diff`, `..._model::sqlite`, `..._mutations::mutation_bridge::*`). din18599 hand-written ArtifactDsl/ArtifactPack/ArtifactStoreSnapshot impls replaced by the macro. `OpBinaryReceiving` added next to the binary mutation codec (`original_pack_value_operation_receiving!`). Manifest deps added to model/mutations/diff (pack, native-owned, contract, diagnostic, compliance, pack-json, dsl-record).
3. Viewers/editors: `norm_viewer_commands!` takes the full model path; din16798 viewer `reference_birth` uses `default()` like its siblings; editors import `viewport_act`; `norm_live_acceptance!`/`norm_latency_bench!` invocations are `#[cfg(test)]` (macros exist only under test/compliance-testing); `semio-framework-job` + `semio-framework-action-bus` added to viewer and editor manifests; din18599 viewer gets `semio-framework-dsl-record`.

## Shared files changed outside the three artifacts (affect every norm viewer/editor; other waves need no further fix for these)
- `📇️registry/🧬️contract/🖥️app-surface/👁️viewer-surface/🦀️.rs` and `🏭️command-and-tool-job-macros/🦀️.rs`: generated `admit_job` now has the 8-parameter signature (`admission` parameter, `OriginalConstructionStatus` result).
- `📇️registry/.../🫧️live-lanes/🧱️lane-stores/🦀️.rs`: `__norm_lane_store!` rebuilt on `semio_plugin::transient_root!` (`retained: true`) with `by_value_projection`, `RetainedClone`, `SnapshotProjection`, `CanonicalJsonTree`, hand `ArtifactCanonicalJsonTree`, `OpBinaryReceiving`; each store in its own private module (the macro hard-codes `retained_root_fields`). `NormLane*Mutation::Set` is now `Snapshot { transient }`. Re-exports `__pack_json`, `__plugin`.
- `🪟️results/🎚️config` and `🪟️viewport/🎚️config` binary mutation files: `OpBinaryReceiving` for the window-config mutations.

## Left / notes
- Test targets not checked (moved io files keep `#[cfg(test)]` mounts and tests that reference `crate::text`/`crate::binary` paths of the old io crate; `--all-targets` was out of scope).
- Cargo.lock files of the three artifact workspaces were updated by cargo for the new edges.
- Other norm artifacts still on the old io layout (en1995 diff in io-text, vdi/iso, ...) need the same owner-crate move, `OpBinaryReceiving`, `viewport_act` import, `#[cfg(test)]` on the two test macros and job/action-bus deps.
