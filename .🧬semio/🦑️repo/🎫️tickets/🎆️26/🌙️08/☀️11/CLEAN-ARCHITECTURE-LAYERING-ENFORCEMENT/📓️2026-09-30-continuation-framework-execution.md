# Framework Document HTTP Transport Execution

## Scope

Completed source implementation of the bounded document HTTP transport boundary selected with the root executor. This does **not** claim the complete GIS preview/worker/rendering extraction from the preceding audit.

The directory Rust HTTP client no longer imports GIS types, constructs a GIS route, or exposes four GIS methods. It retains exact scope encoding, request/response byte limits, cancellation before/after transport, credential isolation and closed HTTP outcomes. Response bytes are returned only through the supplied typed/validated decoder.

A neutral compiled port in the existing first-party schema module admits owner-bound schema declarations, validates request and response JSON using OwnedJsonSchemaValidator, encodes every declared route segment within the supplied document scope and fails closed on missing operations, missing schemas and foreign owner/schema identities. No new external runtime dependency was introduced.

GIS owns the concrete route metadata, request/response schema documents and typed transport wrapper. Its existing PluginBuilder production composition publishes that declaration through TopicContribution. The existing ProgramBridgeEntry manifest bridge carries it to native wgpu; Shell resolves an unambiguous service from installed owner manifests and receives an injected compiled neutral port. Removing the contribution therefore removes its transport and produces an unavailable outcome. Wgpu acquires no GIS package dependency.

Existing ContributedInferenceMetadata/InferencePayloadContract discovery and job tracking remain untouched; this is a scoped document transport contribution, not a replacement discovery or inference payload protocol.

## Evidence

- `cargo test -p semio-framework-schema --lib -- document_http::tests:: --nocapture`: 2 passed, 0 failed. Runtime DEBUG observations: install neutral (1 owner), install secondary (2), remove secondary (1), reinstall secondary (2), remove secondary (1); all five neutral replies identical `{schema: neutral.reply/v1, value: 7}`.
- `cargo check -p semio-s-plugin-gis -p semio-framework-os-renderer-wgpu --no-default-features`: completed successfully in 2m39s after correcting import/dependency wiring.
- Registered Nx checks and owner descriptor regeneration are in progress; no claim of completion for these checks yet.

## Files

Updated (attributed source paths):

- `Cargo.lock` (first-party dependency graph entries for schema, GIS and wgpu)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🦀️.rs`
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

Removed source paths: none.

Owner descriptor pair is pending canonical component/descriptor producer verification; it will be added only after successful regeneration.

## Remaining Domain Ownership

GIS preview and undo Rust structs, finite ring validator and reducer still live in general directory schema. TypeScript directory schema, OS root worker envelopes, renderer protocols, SVG projection, localized GIS status/history controls and MCP's GIS approval gateway remain general-owned. This slice changes their Rust directory transport consumer atomically but does not establish whole-shell or whole-framework GIS/Hub removability.

Native wgpu's pre-existing verified execution-target lease admission is preserved; the source comment says native lease filling is currently unavailable. The new owner transport injection is exercised through actual GIS production plugin assembly and finite client calls, separately from this pre-existing admission limit.

No Git mutation, worktree, AGENTS change, external runtime dependency, migration/compatibility alias or new script filename was used. Temporary logs are confined to the ticket generated directory and are scheduled for cleanup after final evidence is retained here.
