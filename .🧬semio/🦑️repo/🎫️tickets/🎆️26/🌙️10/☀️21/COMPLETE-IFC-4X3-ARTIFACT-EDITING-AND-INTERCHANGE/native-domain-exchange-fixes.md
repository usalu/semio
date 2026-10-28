# Native Domain Exchange Fixes

Clean source checkpoint for independent audit. The FEM owned receiving, projection and pack publication bindings are implemented and connected to registered native artifact entries. Native compilation and execution remain unverified. The ticket and end-to-end infrastructure goal remain open.

## Review Findings and Bindings

The production `fem_reference_pack` authoring-record/default-document/envelope bridge was removed. No such bridge runs before original authority. The retained test fixture helper alone creates explicit fixture authority.

The actual FEM snapshot receives nodes/materials through `NativePersistentListReceiver`, preserving sealed persistent rows. `Fem3dReceiving` retains original authoring data, child row/page receivers, source aliases, produced lists and actual output. Capture, advance, take and close have separately funded frontiers. Original plain rows remain retained until funded retirement. `receive_authoring` uses the caller's `NativeSnapshotDecodeOwner` and original physical step grant; it creates no replacement decoder or guessed grant. An incompatible installed construction recipient refuses explicitly.

`receive_exchange_record` binds strict unloaded canonical fields into retained typed slots before persistent receiving. `ArtifactPackReceiving::receive_pack` drives `RetainedRecordReceiving` on original bytes and borrowed declaration, keeping parser, record, authored rows and persistent output in original receiving/publication frames. Cold actual construction explicitly refuses. Artifact conversions and relational reconstruction now name their explicit authoring reference outputs instead of assigning Vec rows into persistent fields.

`native_fem_owned.rs` funds and retains the IFC identity index, flat relationships, canonical shared material/section keys, vertex table, row backing, generated identifiers and partial rows. Cancellable in-place heapsort avoids allocating HashMap/BTreeMap scratch. Bounded allocation-free value/header/reference validation uses those funded indexes. `native_fem_build.rs` receives every ADD2 header, value, aggregate, entity, root identifier and scratch vector through original snapshot authority; partially built graphs remain retained on refusal.

The actual FEM `FieldProjectionSource` and `encode_exchange_pack` stream the existing canonical Pack document cursor into one final envelope buffer. No whole authoring clone, RecordValue, owned RecordSpec, encoded middle body or wrapped-body copy is constructed. Cursor scratch and final backing retain original encoder custody through cancellation. The ordinary default Pack encoder remains a test/reference interface and is not the production bridge.

Entity indexing, BIM child aggregates, canonical material properties and analysis membership now checkpoint inside their loops. The BIM callback adapter preserves the original terminal ValueError; the new FEM owner routes propagate original typed refusals directly.

## Admitted Mapping

| Direction | Retained data | Admission and refusal |
| --- | --- | --- |
| ADD2 to FEM native pack | Node coordinates; bars/zero-roll frames; shared canonical material and section identities; E/G/nu/rho; A/Iy/Iz/J/name; boolean supports | Exact ADD2 identity/header, bounded simple instances, positive unique IDs and valid references, finite values, one unloaded global LOADING_3D analysis, exactly one membership per item, metre project coordinates, identity context, matching full SI engineering dimensions and scale. Loads/cases/combinations/solids, releases, specialized relationships, factored membership, placements/local support axes, nonzero roll, non-SI scale, georeference and binary properties refuse atomically. |
| FEM native pack to ADD2 | Actual persistent rows, supported members, canonical shared engineering definitions and boolean supports | Strict unloaded canonical record, valid scoped namespaces/references, finite physical properties, one support per node, zero frame roll and bounded output backing. Authors new IFC root IDs, SI/derived units, reference topology/axes and analysis membership. Loads/combinations/solids and nonzero roll refuse. |

Successful results declare lossy fidelity and warning diagnostics. Source artifact IDs, unreferenced definitions on reimport, unrelated IFC metadata and solver/display settings are not roundtrip guarantees. This is not full IFC/MVD interchange or solver equivalence. The route returns an artifact payload; it does not apply or publish a document diff.

## Actual Host Route and Control Fix

Normal artifact exchange uses `reactor/💼️jobs::abi::step` -> `run_io_run` -> `original.receive_nested` -> `io_run_with_snapshot_control` -> `resolve_run` -> registered `IoEntry.run`. The same original paired IoRunControl reaches `IfcIntoFem`/`FemIntoIfc`; no companion is manufactured. The exporter calls owned analytical receiving then the owned FEM writer. The importer calls FEM ArtifactPackReceiving then the funded ADD2 builder.

`plugin/io/authority/🪶️snapshot::with_snapshot_authority` narrows native direction only for codec.sqlite-export/import. `io_run_with_snapshot_control` restricts those calls to one exact SQLite endpoint transfer; they are not the normal artifact exchange route. That narrow lane remains narrow. Native FEM SQLite construction still refuses until an original receiving wallet is bound; its reference reconstruction is explicitly separate.

The minimal shared fix in `🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs::select` checks whether the borrowed pending slot actually owns a frontier. Empty slots permit a transition with both original controls; occupied slots block without replacement. The original wallet and control identities remain unchanged.

## Tests and Evidence

New mounted Rust tests cover every persistent receiving prefix and funded closure; denied capture/frontiers including zero copy; actual pack receiving; borrowed writer equivalence to canonical reference Pack; long-name cancellation; registered native entry roundtrip with shared definitions; original cancellation during mapping/target packing; invalid inertia dimensions and missing analysis membership. They are unexecuted source tests.

- IFC `io/🧪️tests/native_fem_owned.rs` is mounted by native_exchange.rs and belongs to semio-s-artifact-stdio-ifc-4x3 lib tests. Existing native_fem.rs/native_bim.rs include interior scan regressions.
- FEM `schema/📸️snapshot/native-tests.rs` is mounted by native.rs and belongs to semio-s-artifact-fem-3d-model lib tests.
- Framework control nested-io.rs adds empty-slot, occupied-owner and actual nested transition tests; its existing dual-control case now permits empty pending slots. The feature and JSON describe this neutral behavior.
- `bun test './🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/nested-io/🟦️.ts'`: **2 pass, 0 fail, 9 assertions**, using Ajv, independent JSON and SQLite decisions.
- Native oracle file: **5 explicit skips, 0 failures** without SEMIO_IFC_NATIVE_ORACLE_OUTPUT, including the new native-fem-owned case. No reference output substitutes for native output.
- New Rust leaves pass rustfmt syntax parsing, which is not type checking or execution.
- Current `cargo check --manifest-path <IFC>/Cargo.toml -p semio-s-artifact-stdio-ifc-4x3` exits 101 before compiler -vV: **Original compiler ingress requires the repository launcher and authentic enforcing store**. Current log: `🗑️generated/native-domain-fixes-cargo.txt`. No enforcement/build bypass was attempted.
- Bundled Python `C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe` runs retained native-domain-fix-oracle.py with test-only **IfcOpenShell 0.8.3**. Generated native-domain-fix-oracle.json checks the retained independent reference source: two members, one shared material/section, no EXPRESS validation errors; it explicitly records nativeExecuted:false. This reference has different node/property values from the new Rust fixture and is source-schema/sharing evidence, not native differential proof. Supply real native-fem-owned.ifc to that script and run the standard ifc-oracle.py into its native oracle folder before enabling native-output tests.

The unchanged broader TS exchange suite was previously 62 pass, 4 native-output skips, 0 fail. It was not rerun for Rust-only changes; the separately checked native oracle file now has five skips.

## Remaining Infrastructure Boundary

The newly owned FEM projection/receiver/writer is not the remaining cold bridge. Generic serializer_entry/deserializer_entry still hold/drop their generic decoded IFC source and use cumulative ArtifactNativeSnapshot spec/record/body helpers for source-pack decoding and final IFC pack encoding. ArchiveChildren lacks controlled retirement. Fixed warning metadata and the general route wrapper are outside the new FEM owned frame. These shared boundaries need the actual OriginalArtifactPackReceiving and retirement interfaces in the next scoped IFC route change; this checkpoint does not claim their full physical custody or failure atomicity is repaired. BIM reference projection has improved cancellation but has not adopted the owned FEM construction protocol.

Runtime/type verification, real native-output differential oracles, generic IFC source/target pack receiving/publication and separate native SQLite binding remain open. Generated inputs and outputs are retained for concurrent work and review.

## Touched Files

IFC any IO: native_exchange.rs, native_graph.rs, native_bim.rs, native_fem.rs, native_domains.rs, new native_fem_owned.rs, new native_fem_build.rs; tests native_fem.rs, native_bim.rs, new native_fem_owned.rs, native-domain.feature, native-oracle.test.ts. Minimal IFC dependency rows: root Cargo.toml glTF-model workspace path and 4x3 package Cargo.toml native-owned test dependency.

FEM any schema: 🦀️.rs explicit reference conversions; 📸️snapshot/🦀️.rs mounts; new snapshot native.rs, native-tests.rs, record.rs, receive-record.rs, projection.rs, write-pack.rs; binary/text/SQLite snapshot codecs use actual receiving/explicit refusal interfaces instead of nonexistent Vec/FromValue construction.

Framework control: ⏱️control/🦀️.rs; tests nested-io.rs, nested-io.json, new nested-io.feature, nested-io/🟦️.ts. Ticket: this report and native-domain-fix-oracle.py. No AGENTS, git state, worktree, enforcement or build-system edits.