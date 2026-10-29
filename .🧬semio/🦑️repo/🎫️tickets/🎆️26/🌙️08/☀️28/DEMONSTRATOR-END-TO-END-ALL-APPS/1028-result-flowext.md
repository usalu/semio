# 1028 result flowext

Status: RED (not yet at compiler) - blocked before rustc by the Source-boundary admission of shared framework crates being restructured by peers.

Slice: 9 flow extension workspaces under `✏️s/🔌️plugins/🌊️flow/🧩️extensions/<dir>/Cargo.toml` (bim, brep, dictionary, draw, list, logic, math, primitive, text); each is its own workspace, checked with `--workspace` per manifest via `C/1028-tools/flowext/run.sh`.

Files changed: none yet.

Frontier (chk4, all 9 sequential):
- bim: semio-framework-os-store-engine source extent exceeded (bytes) - peer raised its Cargo.toml limit meanwhile.
- brep/dictionary: semio-framework-os-vcs file count exceeded - peer raised limit to 96 files.
- draw/list/logic/math: semio-framework-plugin-decl-views closure over its byte budget (peer mid-restructure: staged-form/, new lifted packages untracked).
- earlier: os-kernel "Unbalanced Rust source delimiter" (transient, gone) and a transient missing composition-publication crate.

No extension-owned compile error observed yet. Rerunning after peers settle.

## Update (chk5-chk9, ~19:30)
Source admission now passes; the wasm32-wasip2 frontier is in shared framework crates that peers are mid-migrating (NativeReceiver/`I: ?Sized` bounds, store engine, durable-group-wire). No error in any of the 9 extension crates has been reached because every check dies upstream:
- chk5: semio-framework-value PagedMapReceiver `terminal_is_empty` (peer fixed with `+Sync` within minutes); replication-edit/ids/mutation-retained, ui-locale, store-blob-identity, store-link-schema (Sized/`?Sized` receiver bounds).
- chk6/7: manifest-view-model, mesh-engine, store-one-item, store-replay (private `new`).
- chk8: store-engine-core (`Mutation: ColdMutation<P>`, `P: Clone`).
- chk9 (latest, primitive): store-durable-group-wire (unresolved StoreMutation, DurableStoreBindingPlanV1, store_post_revision, StorePartsRef).

Files changed: none. SHARED CHANGE: none by me.
Re-run: `C/1028-tools/flowext/run.sh <n> 🏗️bim 📐️brep 📖️dictionary 🖍️draw 📃️list 🧠️logic 🧮️math 🔤️primitive 📝️text` (logs `C/🗑️generated/1028/flowext-chkN-<dir>.txt/.rc`). Status stays RED until the framework closure compiles.

## CHECKPOINT 22:55 state

STATUS: extensions still RED (never reached their own crates). Latest upstream frontier (chk25, primitive): window-config (peer mid-edit, duplicate defs) and flow-document (see DAG/flow below).

### Fixed in shared store crates (free-fix, same pattern: strengthen where-clauses to `Clone+ToValue+FromValue+Send+Sync+'static` / `Clone+ToValue+FromValue+Mutation<P>+ColdMutation<P>+OpBinary+OpText+Send+'static`)
- os-store-engine lifted store (3 files) + replay/native/history/report; history/replay/publication; initialization authority + genesis factory schema; space-member native-vcs + space-member-for-artifact-store; plugin genesis (OriginalStoreGenesisOwner State/Mutation bounds, OriginalStoreSlot), initialization retirement, plugin lifecycle (all weak blocks), tool-run operations + fold; durable-group snapshot capture; engine-command (`ColdMutation` qualified, `ArtifactOriginalVcsFailure::from`); store root dup `ReferenceSpaceMember` re-export removed; transient publication: restored `paged_copy` helper, `SnapshotProjection<Output=P>`, `P: Clone` on disposer.
- codec-factory: already green on wasm32 (peers fixed before I ran).

### DAG member-snapshot migration (assignment 4) - implemented, UNVERIFIED by compile of dag crates
Shape check: dag types derive DslRecord/DslEnum/DslScalar themselves, so the DSL record and persisted model are the same types (no twin); `NativeReceive` reads RecordValue by field ORDINAL, so ToValue wire names are irrelevant. Real mismatches found and handled: (1) `DagNodeKind::AppInstance` DSL keyword was kebab `app-instance`, derive tag is `appInstance` -> added `#[dsl(key = "appInstance")]`; (2) `#[value(default = "path")]` fields (IoPortSpec cardinality/visible, DagSnapshot schema) were rejected by the derive -> derive now treats a path default as natively required (canonical-JSON-only convenience; DSL text always carries the field) - SHARED CHANGE in `value/✨️derive/🛬️receive/🦀️.rs`; (3) `Option<DslValue>` null maps to None natively (Some(Null) not preserved).
Changes:
- NEW shared `value/🛬️decode/🧳️owned/🌳️value/🧱️bulk/🦀️.rs` (mounted `pub mod bulk` in tree): `NativeBulkValue`, `NativeBulkReceiver`, exact-quote one-step dynamic subtree read, `NativeReceive for DslValue`.
- NEW `graph/🛂️manifest/🛬️receive/🦀️.rs`: `NativeReceive`+`NativeBulkValue` for `PropertyValue`/`PropertyBag` (sorted, duplicate-key refusal), `NativeDefault for PropertyBag`; mounted in manifest `🦀️.rs`.
- `♾️infinite/.../presentation`: `NativeReceive` derive on PortShape, EdgeRouteStyle, IoPortSpec, DagMedia, DagMediaKind, DagPreviewContent, DagNodeKind, DagNodeSpec, DagHostSnapshotEdge; `DagExpandedPaths` NativeReceive (mapped from Vec<String>, sort+dedup) + NativeDefault.
- dag vcs: `NativeReceive` derive on DagSnapshot; Cargo dep `semio-framework-value-native-owned`. dag retained: `SnapshotOpen = OriginalRecordMemberSnapshotOpen<DagSnapshot>`; unit test type-name assertion updated; new test `dag_member_receives_every_node_kind_natively_from_its_original_text_record` (all 11 node kinds, properties, DslValue ports) in `🧵️retained/🧪️tests/🔬️unit/🦀️.rs`; dev-dep `semio-framework-job` added to the dag crate.
- Wasm check `value-native-owned + graph-manifest + presentation` compiled green (flowext-dag1: only dag-vcs failed on the missing dep, now added). Check `flowext-dag2` (dag-vcs + dag, wasm32-wasip2, detached pid unknown, queued ~50 min) is the verification of the last edit; its result lands in `C/🗑️generated/1028/flowext-dag2.txt/.rc`.

### NOT done
- Dag stdio-semio `semio_member_owner!` macro, db hash-projection fixture, plugin test fixtures and `🌿️vcs/🧪️tests` still name `ProducedMemberSnapshotOpen` for dag (grep it).
- Flow side (FlowSnapshotPackProducer) still blocked on its DSL-native decision; it keeps `artifact-flow-flow-document` red and blocks every extension check.
- The new dag test is not yet compiled/run (native: `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/.../🕸️dag/📦️packages/🦀️rust/Cargo.toml -p semio-framework-artifact-infinite-dag --lib dag_member_receives_every_node_kind`). Expect first-run fixes in the test (ArtifactDialect values, frame helper) and possibly in ordinal alignment of DslEnum variant payloads.

### Next step
1. Read `flowext-dag2.txt`; fix dag/dag-vcs errors. 2. Run the dag test above. 3. Fix remaining `ProducedMemberSnapshotOpen` references for dag. 4. Rerun `C/1028-tools/flowext/run.sh <n> 🏗️bim 📐️brep 📖️dictionary 🖍️draw 📃️list 🧠️logic 🧮️math 🔤️primitive 📝️text` once window-config and flow-document compile.
