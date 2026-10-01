# Framework Document HTTP Transport Execution

## Scope

Completed source implementation of the bounded document HTTP transport boundary selected with the root executor. This does **not** claim the complete GIS preview/worker/rendering extraction from the preceding audit.

The directory Rust HTTP client no longer imports GIS types, constructs a GIS route, or exposes four GIS methods. It retains exact scope encoding, request/response byte limits, cancellation before/after transport, credential isolation and closed HTTP outcomes. Response bytes are returned only through the supplied typed/validated decoder.

A neutral compiled port in the existing first-party schema module admits owner-bound schema declarations, validates request and response JSON using OwnedJsonSchemaValidator, encodes every declared route segment within the supplied document scope and fails closed on missing operations, missing schemas and foreign owner/schema identities. No new external runtime dependency was introduced.

GIS owns the concrete route metadata, request/response schema documents and typed transport wrapper. Its existing PluginBuilder production composition publishes that declaration through TopicContribution. The existing ProgramBridgeEntry manifest bridge carries it to native wgpu; Shell resolves an unambiguous service from installed owner manifests and receives an injected compiled neutral port. Absent or ambiguous contributions fail unavailable during new port admission. Wgpu acquires no GIS package dependency. This does not claim cancellation of an already retained runner when a contribution is removed.

Existing ContributedInferenceMetadata/InferencePayloadContract discovery and job tracking remain untouched; this is a scoped document transport contribution, not a replacement discovery or inference payload protocol.

## Evidence

- `cargo test -p semio-framework-schema --lib -- document_http::tests:: --nocapture`: 2 passed, 0 failed. Runtime DEBUG observations: install neutral (1 owner), install secondary (2), remove secondary (1), reinstall secondary (2), remove secondary (1); all five neutral replies identical `{schema: neutral.reply/v1, value: 7}`.
- `cargo check -p semio-s-plugin-gis -p semio-framework-os-renderer-wgpu --no-default-features`: completed successfully in 2m39s after correcting import/dependency wiring.
- Supplementary invocation of the Cargo-built native binaries: exact `schema_vectors_match_owned_validator` and `typed_owner_transport_preserves_proposal_binding_and_closed_geometry` each passed (1 passed, 0 failed). GIS runtime DEBUG: `gis-owner transport submit/events/approval validated; foreign job and open ring refused; calls=7`. The scripted first-party transport exercised seven finite HTTP replies, rejects foreign jobs and proposal digests, rejects an open ring and unapplied approval, and rejects an out-of-contract cursor before HTTP. Existing `geo::LineString` and `serde_json` supplied independent geometry and JSON oracles. This is local runtime evidence, not a claim of a live Hub server session.
- `bun nx run @semio-tech/gis-plugin:inference-transport-check`: exit 0, uncached, 29m39s including shared Cargo lock waits and dependency recompilation. Ajv independently accepted/rejected all 15 authored schema vectors. Both exact Cargo laws passed (1 passed, 0 failed each): `inference_client::tests::production_manifest_installs_owner_transport_and_schema_vectors` and `inference_client::tests::typed_owner_transport_preserves_proposal_binding_and_closed_geometry`. The latter compiled and ran after temporary DEBUG source removal.
- `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false bun nx run @semio-tech/gis-plugin:describe`: exit 0, 32m24s, canonical owner target plus 11 dependencies (one prerequisite cache hit). Actual WASIp2 component build and first-party owned engine extraction completed; live engine compiled 145,520,979 core bytes and completed export execution at fuel 1,096,769,769. The canonical producer checked binary/readable identity and hashes before replacing only the GIS descriptor pair. Independent post-run projection confirms one GIS-owned document port for `s.gis.gismap.inference`, four operation schema pairs byte-identical to authored files, and removal of this run's `.🛂️descriptor-staging-WwMh48` and `.semio-describe-core-KqikEb` temporary directories.
- Producer hashes: component `d2e4fd6f802fca783ae3035f09e11c6b95c9976965f3b562170b363c27d013bb`; core `2dd5dc7b959b09a9f2c327d0ea34f6b1250538a09028b384dbe774476d3bc9af`; descriptor `548e01047ada651b7e18b3d62d9c25ffbdef0f32680040146dfe62268e53790c`. No hash or manifest projection was manually patched.
- `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false bun nx run @semio-tech/framework-schema:document-http-check`: exit 0, uncached, 33m49s including shared Cargo lock waits. Ajv independently matched all four neutral request vectors and admitted the authored neutral declaration. Three exact Cargo laws each passed (1 passed, 0 failed): `document_http::tests::owner_removal_preserves_neutral_document_transport`, `document_http::tests::schema_vectors_match_owned_validator`, and `os_directory::client::tests::document_http_transport_preserves_scope_bounds_and_owner_decode`. The exact kernel law confirms scoped URL encoding, private bearer propagation, output caps, denied status and cancellation without an additional HTTP call.
- Launch registration is present for `⚖️inference-transport-check🌍️gis🦀️` and `⚖️document-http-check🧬️schema🦀️` (root executor owns `.vscode/launch.json`).
- Source scan over the complete directory-client subtree and new compiled document-http module: zero GIS type/route names, ring/latitude/longitude vocabulary and temporary DEBUG statements.

## Files

Updated (attributed source paths):

- `Cargo.lock` (first-party dependency graph entries for schema, GIS and wgpu)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🛂️.descriptor.semio`
- `✏️s/🔌️plugins/🌍️gis/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📋️project.json`

Created (attributed source paths):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🧬️schema/🌐️document-http/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/🌐️document-http/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/🌐️document-http/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧬️schema/submit-input.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧬️schema/events-input.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧬️schema/cancel-input.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧬️schema/approve-input.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧬️schema/submit-output.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧬️schema/page-output.json`
- `✏️s/🔌️plugins/🌍️gis/💡️inference/🔌️client/🧬️schema/approve-output.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️2026-09-30-continuation-framework-execution.md`

Removed source paths: none.

The owner descriptor pair was regenerated by the existing first-party canonical component/descriptor producer. Other owner descriptors were not edited by this executor.

## Remaining Domain Ownership

GIS preview and undo Rust structs, finite ring validator and reducer still live in general directory schema. TypeScript directory schema, OS root worker envelopes, renderer protocols, SVG projection, localized GIS status/history controls and MCP's GIS approval gateway remain general-owned. This slice changes their Rust directory transport consumer atomically but does not establish whole-shell or whole-framework GIS/Hub removability.

Native wgpu's pre-existing verified execution-target lease admission is preserved; the source comment says native lease filling is currently unavailable. The new owner transport injection is exercised through actual GIS production plugin assembly and finite client calls, separately from this pre-existing admission limit.

No Git mutation, worktree, AGENTS change, external runtime dependency, migration/compatibility alias or new script filename was used. The native checks ran on macOS arm64; no Windows or Linux runtime pass is claimed.

## Cleanup

All useful owned checks completed and their exact results were retained above. Duplicate owned retry sessions were stopped through their individual handles; no external process was stopped and no Nx graph or shared Cargo state was reset. Both temporary DEBUG source statements were removed. The canonical producer removed this run's temporary core and descriptor staging directories. The unrelated existing `.🛂️descriptor-staging-SYGCm2` directory was preserved.

This executor removed only the ticket's `🗑️generated/continuation-framework` subtree after retaining the evidence here. Other workers' ticket outputs and all source/input/audit/report files were preserved. Normal build assets created by the existing canonical producer remain in its registered package output directory; shared Cargo artifacts were preserved.
