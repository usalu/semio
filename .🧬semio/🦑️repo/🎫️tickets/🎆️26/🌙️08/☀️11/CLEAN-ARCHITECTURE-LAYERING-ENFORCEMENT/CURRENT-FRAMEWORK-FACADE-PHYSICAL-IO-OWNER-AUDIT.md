# Current Framework Facade and Physical IO Owner Cuts

Read-only source audit against the live tree on 2026-10-02. No production/Cargo edits, native runs, test skips, fixture hash resets, or whole-framework absence claim. Prior facade/kernel-boundary, seven-owner-cuts and residual-framework reports were read first. Their ownership conclusions remain useful, but current source epochs below govern this proposal.

## Epoch Authority

Full current sources and inputs are preserved in `CURRENT-FRAMEWORK-IO-FULL-SOURCE-EPOCH-CAPTURE.md`; unchanged root facade and IO vocabulary reuse the complete prior `📓️2026-10-02-framework-facade-original-source-capture.md`. The generated source-epoch JSON is a convenience output, not the only preservation authority. There are 113 complete source/input captures and 49 actual direct assembly/codec caller files. This is a bounded chosen dependency/caller slice, not all transitive Framework source.

Framework facade is unchanged from the prior capture. Cargo gained the actual neutral Pack JSON dependency. Physical IO changed exactly its two wire JSON operations: encode uses `semio_framework_pack_json::to_json_string`; decode uses `from_json_str(..., JsonMemberPolicy::Reject)`. Do not restore the old OS JSON call spelling or lose explicit duplicate-member policy during an import inverse.

## Eight Current Normal Cargo Edges

A live Bun TOML parse of framework Cargo manifests, normal/dev/build sections and root workspace-key path resolution found the following eight direct normal dependencies to physical products. This independent bounded census agrees with the parent-provided eight-edge count; target-specific sections were not traversed in this check, so it is not a complete workspace absence receipt.

| General owner | Actual physical target | Current source frontier |
|---|---|---|
| Framework root | OS Kernel | root lines 3–6 aliases dsl/protocol/protocol_core/store; lines 22–34 IO/macros/vocabulary/snapshot exports |
| Graph | OS Kernel | package root line 11 dsl_core alias; manifest conversion hook and engine value traits use neutral Value through product identity |
| ToolRun | OS Kernel | package root line 4 dsl alias; root lines 8–10 actual Record schema and Pack record APIs; lines 1401–1403 controlled RecordSpecProducer |
| Editor | OS Kernel | package root line 3 store alias; root PackError and lines 701–708 pack_rt record/JSON bridges |
| Editor | OS Infinite Canvas | root line 8 publicly exposes whole canvas product crate; camera/text runtime use |
| Surface | OS Kernel | package root lines 5–7 aliases dsl/store and reexports os_dsl; node-graph pack_rt bridge calls |
| Surface | OS Infinite Canvas | paint line 17, tiled-map line 19, node-graph lines 21–22 canvas/board exports |
| Surface | OS Infinite DAG artifact | node-graph line 25 DagCamera/DagHostSnapshot/Edge/NodeKind/NodeSpec/IoPortSpec |

Graph Value-only identity is the smallest independent package cut: select canonical Value/derive directly, retaining the manifest hook's accepted fixture spellings and every original engine/manifest law. ToolRun and Editor/Surface OS store removals depend on the real controlled Record/Pack/JSON contracts below. Infinite Canvas/DAG require an actual neutral camera/text/geometry runtime extraction plus a higher DAG binding; rebadging the product crate or exporting domain DagHostSnapshot from neutral Surface cannot resolve those edges.

## Genuine Neutral Contracts Versus Product Responsibilities

| Actual definition or path | Responsibility and coherent destination |
|---|---|
| store lines 2458–2485 ArtifactAssemblyTransaction/error/lock/begin | std-only process-wide publication barrier; canonical lower IO assembly owner. Keep one real lock identity consumed by every registry. |
| physical IO budget/resource/codec contexts, routing/composer/subset/format registries, vocabulary and thunk macros | general IO mechanism. Canonical lower IO owner can depend on Value, Diagnostic, neutral IO vocabulary, SQLite engine and canonical assembly barrier. Macros must be defined there so `$crate` becomes lower. |
| store line 5831 ArtifactDsl | text representation contract uses Diagnostic TextError plus extension/envelope identity; neutral document encoding contract. Existing body must retain parse/print fixpoint and exact extension/envelope identity. |
| store line 10740 ArtifactPack | document encoding contract currently includes RecordSpec and optional relational capability. Split at real canonical Record and Snapshot capability identities; do not copy RecordSpec or reach OS through an alias. Plain encode defaults still have the original panic/verification semantics; controlled paths remain required separately. |
| store lines 10873–10953 ArtifactSqliteSnapshot and ArtifactSqliteSnapshotCodec | neutral handwritten semantic schema, concrete TypeId, export/import function identity, owner-controlled decode/encode/preflight/subset validation and retirement. Move complete contract plus actual ownership helpers, not just raw callbacks. |
| store line 11612 DialectMigration | neutral exact dialect-pair transform metadata and conflict policy. Actual registration implementation currently rejects cross-kind migration, despite the stale nearby comment saying it is merely convention. Retain real validation. |
| store line 10957 full ArtifactCodec | product document-history runtime: compile_dsl/print_mirror/edit_text/apply_ops_binary/replay_envelopes use VcsError, ArtifactPackFiles/TextFiles, op-log/envelope replay. Keep constructor and those callbacks above the lower IO boundary. |
| IO lines 1884–1968 ArtifactAssemblyRegistryPlan/Error/commit | product aggregate assembly, because the plan contains full store ArtifactCodec and commits the store registry. Move exact aggregate orchestration to OS artifact composition owner, using lower locked registry contracts. |
| IO lines 2111–2167 native snapshot registry | currently stores full product ArtifactCodec but only needs extension, pack_schema_hash and actual snapshot_sqlite provider identity. Replace its canonical payload with a neutral snapshot capability record containing exactly those fields; upper assembly constructs it. Do not retain full ArtifactCodec under a generic alias. |
| IO lines 2616–2680 serializer/deserializer constructors | typed document conversion depends on canonical neutral ArtifactDsl/Pack traits; native record generation remains with canonical Record/Pack. Product-specific reconstruct/replay implementations stay upper. |

## Smallest Acyclic Extraction

1. Select one lower IO vocabulary owner and the existing lower SQLite snapshot engine. Extract the real std-only assembly barrier from store first. Both lower IO registries and upper store registries must accept this same guard; no second mutex or independent registry path.
2. Publish lower locked IO batch guards and immutable validated proposals for composers/subsets/formats/native snapshot capabilities. Upper aggregate acquires store guards, then lower IO guards in the current order; it validates every candidate under every held lock, then performs only infallible insertion/extension. Moving aggregate composition higher avoids dragging VCS/history/ArtifactCodec into lower IO. A callback that commits store after separately releasing/reacquiring lower guards violates the original atomic proof.
3. Move exact neutral DSL text / document Pack / SQLite capability contracts only after canonical Record/Pack is available. Move SQLite retirement and exact bounded native encoding/decoding helpers with their true owner dependencies. Store implements/exposes actual product types through those lower contracts; no lower→store callback accessor or compatibility facade remains.
4. Lower native snapshot capability contains explicit extension, pack schema hash, TypeId-bearing SQLite provider and original function-identity comparison. Retain schema parsing, reserved semio_snapshot refusal, nonempty domain tables, exact duplicate admission and conflict rejection. Typed snapshot IO retains concrete TypeId+SQLITE_SCHEMA matching and bypasses native conversion where the original typed path does so.
5. Rebind facade generic consumers directly to Value/Replication/Record/JSON/Snapshot/IO. Product users of full assembly/document codec select upper OS registration owner. Move actual thunk definitions and all their callers to canonical lower IO; a Kernel macro reexport leaves `$crate` identity above.

## Original Atomic, Retirement and Cancellation Laws to Retain

The complete captured IO unit cohort preserves composer conflict without replacement, preflight without publication, format MIME/alias claims, all resource budgets and shared cancellation, resolved-resource cancellation, opaque ordering/span admission and vocabulary corpus. Keep IO fidelity at physical IO line 1426, unit mount at 2008 and mechanism-law include at 2689 registered through their actual package owner after movement.

`commit_artifact_assembly_registry_plan` currently acquires store document/migration guards first, then composer/subset/format/native-snapshot locks; builds and validates all proposed snapshots, composers, subsets, format indexes and store candidates; then performs composer insertion, subset insertion, format insertion, store commit and snapshot extension with no new Result-producing operation after mutation starts. Preserve this order and every lock-poison failure. Exact duplicates remain accepted and conflicting owners never replace the first row.

The app-declarations-fixture cohort has real headless aggregate success/refusal at lines 717–745 and typed snapshot identity/bypass cases at 683 onward. Preserve both document-codec absence and IO-row absence after refusal. Its named `sqlite_snapshot_missing_document_owner_capability_is_rejected_before_publication` at line 712 is already ignored in the current source with an explicit incomplete provider-roster reason. Do not count it as a passing law or add more ignored cases; resolving that preexisting proof gap requires the real semantic provider roster.

Retain full snapshot-capability original native decoding, encoding, retirement, fixtures and schemas. Native-retirement law `sqlite_snapshot_native_retirement_covers_success_cancellation_and_refusal` covers erased owner retirement on success/cancel/refusal. Native-encoding original laws prove exact output admission rather than approximate preflight, rejection before either encoder, cancellation inside binary/text output, one control through both envelopes, typed collection projection, escaped UTF-8, intrinsic octet emission, chunk/index/frame/caller ceilings, expression names and literal/numeric tags. Native-decoding preserves compressed aggregate admission before projection, inline empty collections, and final-file metadata admission before database mutation. `OwnedSqliteSnapshot` cleanup and `retire_sqlite_snapshot` must remain real owner-driven retirement on every exit, not plain dropped substitutes.

The original bounded store retirement law retains zero grants, sub-page liveness and exactly one-page accounting. The app fixture `the_framework_owns_every_bounded_close_lane_an_app_declares_nothing_for` retains all ten editor/viewer document/config/draft/presence/transient disposer lanes plus peer retirement. Those are higher concrete app integration laws and must stay compiled at the higher owner even if low-level retirement primitives move.

## Actual Callers and Compile Queue

The complete direct assembly/codec caller roster is preserved below. OS plugin builder lines 727–777 constructs and freezes actual declarations/runtime before `begin_artifact_assembly` and `commit_artifact_registration_plan`; plugin root lines 4385–4386 forwards to the current IO aggregate. Both callers move to the real upper aggregate, retaining original assembly failure mapping. Concrete S native codecs, SQLite fixtures and OS MCP workspace callers must be rebound without losing registration/route tests.

Compile order: first High's real Pack JSON and Record/schema producer/native control cut, including accepted/refused duplicate JSON members and full original controlled encoding/decoding/retirement laws; then Graph Value identity and ToolRun Record/Pack rebinding; Editor/Surface record↔JSON calls can follow once that real neutral bridge is available. Then std-only assembly/vocabulary ownership, neutral snapshot contract and lower IO locked batches; upper store/aggregate composition and real callers; generic facade import cut last. Infinite Canvas and DAG-neutral-runtime/higher-binding cuts are separate required work and cannot be inferred complete by IO progress.

Do not add record wrappers that build ordinary schemas before a controlled callback, duplicate JSON implementations, alias Kernel as neutral DSL, or compile against only synthetic Vec-valued stand-ins. Current IO already uses canonical Pack JSON with explicit Reject policy; preserve it and wait for High's current source authority instead of replaying a historical facade capture.

Before native jobs: recheck these source epochs after parallel changes, exact import/body inverses and all registered mounts, current locked metadata and physical edge census. Then parent-approved compile queue and ordinary full package cohorts. Whole product/S/Hub physical deletion plus actual generic package source/manifest/macro/exports and all original laws remains terminal proof. No such run was performed here.

## Important Current Source Hashes

- `🧰️framework/📦️packages/🦀️rust/🦀️.rs`: 8282 bytes, SHA-256 `510d428fdd9359648198a3e7023e9bffc407db8665cd7a8bcbeb8259a3d4c28d`.
- `🧰️framework/📦️packages/🦀️rust/Cargo.toml`: 3961 bytes, SHA-256 `20a2668486f4ab31d4dc475722021d01e35935726b81f73d6a0e40d2f949ecb7`.
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs`: 151854 bytes, SHA-256 `99c58efb8292c39c17a68c2c5b827479d753400b56a83bdfd8ce3401cfe6000e`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: 1500851 bytes, SHA-256 `823f795f9f4eb30914d533202948c052821fedeb421539330bbe4269b21f7da7`.

## Direct Assembly/Codec Caller Roster

- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs`.
