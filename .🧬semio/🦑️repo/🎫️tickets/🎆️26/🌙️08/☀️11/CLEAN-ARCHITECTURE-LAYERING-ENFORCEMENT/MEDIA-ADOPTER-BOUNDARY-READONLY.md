# Media Adopter Boundary Read-Only Audit

Observed 2026-10-14. No source edits, test execution, runtime verification, deletion experiments, copied artifacts, Git mutations or ticket lifecycle calls. Repo MCP unavailable. This report is the only authored output. Findings describe a concurrent-work snapshot; hashes permit drift checks.

## Ownership finding

The current defining path is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🟦️.ts`, not the abbreviated framework/media/turn path. Line 42 resolves the workspace and reads S Layout's artifact editor Rust, then checks `LayoutMediaExportJob::admit_original(request,body,cx)`. General Media Source and its native subclass therefore require S Layout merely to validate a generic producer preparation law. The assertion belongs to the Layout adopter, while the generic mutable-slot signature and paid publication oracle belong to General.

## Call and law chain

General `🧵️instance/🦀️.rs:6137` invokes `A::build_media_export_job(&mut original.request,&mut original.body,cx)` and refuses readiness unless the exact request was consumed and an actual body remains. General ArtifactApp declaration (`📜️declaration/🧩️app/🦀️.rs:262`) owns the generic signature. General editor surface declaration (:350) and EditorApp forwarding (:1325–1326) carry it to the adopter; viewer forwarding (:1751–1752) is also General. S Layout editor (:1440–1443) owns its concrete override and delegates to `LayoutMediaExportJob::admit_original`. S export implementation (:2619 onward) owns actual page/authority checks, paid body allocation, incremental page-id/revision preparation, exact request donation, completion/output/snapshot-close custody. That concrete call is an adoption contract, not a generic requirement.

Generic native laws remain in `🎞️media/⚠️turn/🧪️tests/🦀️.rs`; the Cargo `media-turn` integration target delegates to those production-library laws. The source oracle uses Ajv strict schema validation, fast-json-patch and independent SQLite transaction rollback; preserve them.

## Neutral laws to preserve

Keep every existing source test and all fixture sections: exact original Unicode/business identity through pending publication and cancellation; caller-funded output birth before allocation; exact producer request-field roster; installed supplier/snapshot root through denial; paired request/body through foreign-root and publication-phase denials; result retention on unfunded/busy/occupied completion slots; custody commit before handle receipt; every terminal download root through registry denial; original chunk custody before paid EOF. In producer preparation keep the generic SDK mutable body-slot assertion and SQLite phase oracle. Remove only its concrete S editor source read when the held General lane is released and the adopter assertion has its owned route.

## Correct adopter route and bounded next batch

Place source adoption coverage beside S export at `✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts` (currently absent), with its own schema/fixture if extending beyond the existing source assertion. Route via Layout artifact package `📜️script.ts` OwnedVerifyScript (`verify layout-media-adoption`) and its `📋️project.json`, then register a matching ordered `.vscode/launch.json` entry. This is a recommendation, not an existing command. Do not reuse General's fixture as a concrete adopter roster or invent a dummy artifact.

Existing S native law `exact_layout_out_media_factory_dispatches_a_real_reserved_job` lives in `✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs:361`; export Rust includes that module at :4758–4759. It checks the actual ActionBus factory and dispatch, not the newer mutable-slot `admit_original` path, so alone it does not replace a concrete admission law. Existing helper `request`, `fixture_export_job`, real snapshot and LayoutMediaExportJobFactory are the appropriate owned fixture context. Add native coverage of the actual override/admission with real Layout request and paid caller grants if runtime adoption is part of the batch; retain unrelated export fixtures and laws.

Bound batch to the S source adopter test (and owned schema/fixture), Layout script/project/launch registration, then the single held General line only after coordination. Do not touch commands/frontend/init/native/signals or selected graph/Root member validator work. No deletion checks or artifact copies. All proposed filenames must be checked against the 256-character absolute-path limit.

## Authored commands and execution limits

Existing General source launch :93708: `bun repo run @semio-tech/framework-plugin:test-media-turn-source --skip-cache --exclude-dependencies --parallel=1 --configuration=prototype`.
Existing General native launch :93724: `bun repo run @semio-tech/framework-plugin:test-media-turn-native --skip-cache --exclude-dependencies --parallel=1 --configuration=prototype`.
Source handler (:110 onward) strictly typechecks Media Turn and Cancellation Issuer TypeScript, then runs both using Bun. Native handler runs source first, then Cargo `--offline --test media-turn --features media-turn-testing -- --nocapture` through repository Cargo infrastructure. Native target's nine wrapper laws are enumerated below.
S native export test can be selected through existing artifact package test target: `bun repo run @semio-tech/layout-layout-rs:test --skip-cache --exclude-dependencies --parallel=1 --configuration=prototype -- long --offline --lib -E 'test(exact_layout_out_media_factory_dispatches_a_real_reserved_job)' -- --nocapture`. This selected command was not found as an exact launch entry and was not executed. New executable coverage needs launch registration before completion.

No test was run; no feature/runtime behavior is claimed working. General source is held by parent coordination. Schema validation here means JSON was parsed to inventory fixture fields, not Ajv execution.

## Fixture and native rosters

Fixture keys: version, sourceIdentity, cases, dispatchSources, artifactSources, closing, mediaSources, birth, requestFields, snapshotPreparation, producerPreparation, completionPublication, handleReceiving, downloadReceiving, chunkReceiving

Request fields: source_identity, retained, operation, tool_id, parent_document_id, canonical_base_revision, snapshot, snapshot_close, port, output_credit, output_chunks, completion

snapshotPreparation phases: sourceLoan, snapshotSupplierAliases, closeFlag, closeOwner, ready; cases: 4

producerPreparation phases: body, request, frame, publication; cases: 6

downloadReceiving phases: handleMetadata, statusMetadata, outputReceiving, registryPublication; cases: 7

Native wrapper roster: original_media_pending_domain, original_media_complete_projection, original_media_physical_publication_close, original_media_cancellation_issuer, original_media_output_cold_birth, original_media_installed_snapshot_source, original_media_completion_result_receiving, original_media_handle_receiving, original_media_download_receiving

## SHA-256 source snapshot

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🟦️.ts`: `4335c92fa3c9a911d50d0ed9eb06ddcf838173f6ffb25832159a92543cb1ec47`; absolute path length 103
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧬️schema/🔣️.json`: `20aeb5d0c1da8dc2d91d89992418f2e4deb8c75aca79ea79e686153316ffa8da`; absolute path length 106
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🧫️fixtures/🔣️.json`: `1b1b35fc5620f47a478d70cc74b48f51cbf39587bff1e948dd68b2a0af2a702c`; absolute path length 116
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🦀️.rs`: `4991dd9d678dcfbf8239d3e6036c253dd3cc5758f4daf9482aa7b5823376912e`; absolute path length 103
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🎟️native/🦀️.rs`: `b21a74b08f7d434987fcb974174559a753bcde6331d75c34b1c0af211936ae5e`; absolute path length 112
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🦀️.rs`: `b8d80b79ff1d265fef43b2d9a3b49bf4cfe1b32c48859bbbe64b06ee9345e17e`; absolute path length 100
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🦀️.rs`: `737817f1b0e1a50b0fa74a0f36165ca96ed37890abf0846fd4155d3721e03f12`; absolute path length 104
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🦀️.rs`: `a4eefc278203127fa2b6f6235c0d6e2459b94346e5578cc8ea6f444c0aba906b`; absolute path length 102
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs`: `b9b80dd8aca1d8f63399e965819ebe9ce5b990138c1886e034bb9cc59bebbe19`; absolute path length 91
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`: `315bbd7bf06ad889c72ac54b9fc817363e5a74cfe1a8b2f8ee290a0d42832e06`; absolute path length 104
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📋️project.json`: `aef1eb0d2acc9def08f5994176adbd8f643891a57b9fd100b6c4db9f8be42e06`; absolute path length 107
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`: `a8b2e9a39c6f6e7e2dcdbdc083ca584c27819498c3d652023433d53218f1af3c`; absolute path length 103
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: `26778fced263372f06403bbe19d859be6bca9a3207f9d9c59bdbfa19a5f74ad7`; absolute path length 118
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs`: `140fdb40f01dc932d362a53bc8dd18e9ed1196255ee36dd5a344f59f14d8eeed`; absolute path length 136
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs`: `fda71a7d46f16c9a13f1a5bbe2dffa2c2644a9d67e40eab77dd740af22a8828c`; absolute path length 151
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📜️script.ts`: `3887a28910bb759739b6a4b66ecbb3919b2708ef76e9015d5ec7bd9cf2e30545`; absolute path length 101
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📋️project.json`: `3122e3a623da5fd100131e59c7130a1b0ce56e8a69300e72a5a4f90c6c6c2042`; absolute path length 104
- `.vscode/launch.json`: `c73a6fcf37c7e7077e771b49c10c5d8e7c449b3a4d3bfbcf6ab1d17164852981`; absolute path length 47
