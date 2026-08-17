# Mutation Diff Result Contract

## Contract

The crate-owned public application contract is frozen as:

```rust
pub struct MutationApplyError {
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
}

pub type MutationApplyResult<P> = Result<P, MutationApplyError>;

pub trait MutationDiff<P> {
    fn apply(&self, base: &P) -> MutationApplyResult<P>;
}
```

`MutationApplyError::at` sets an outermost-first target and `MutationApplyError::under` prefixes one or more outer segments while retaining an inner target. The error has no external types. There is no infallible adapter, fallback success, or compatibility API.

The protocol-owned wire shape is camel-case JSON with required `code` and `message`, plus optional `target` because Rust omits an empty vector. TypeScript mirrors it with `MutationApplyError`, `MUTATION_APPLY_ERROR_SCHEMA`, and `MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR`. Both Rust's round-trip test and the TypeScript vector use exactly:

```json
{"code":"mutation.apply.invalid-index","message":"index 4 exceeds length 2","target":["slides","4"]}
```

The schema deliberately accepts empty strings exactly as Rust construction and deserialization do; non-empty codes remain a semantic producer expectation, not a mismatched wire constraint.

## Atomic Rejection Boundary

- Generic named and indexed collection helpers preflight before committing. They reject missing, duplicate, conflicting, or invalid targets, including duplicate final insertion indices.
- Presence, transient, and interaction stores stage a complete batch candidate and advance snapshots/generations only after every diff succeeds.
- VCS replay stages snapshot and clock state; malformed persisted diffs propagate a typed rejection without committing either candidate.
- Framework DAG, workflow, run, space, configuration, store, testkit, and planner boundaries consume the typed result. DAG indices no longer clamp, reorders must be complete permutations, and missing node/edge/binding targets reject. Workflow references are resolved before mutation. Non-empty run diffs reject sealed documents.
- The repaired `space_history_op_round_trips` fixture now declares `sa-other` before activating it; strict missing-target rejection was not weakened.

## Nonstdio Plugin Closure (104)

- All **104/104** nonstdio `MutationDiff` implementations return `protocol::MutationApplyResult`; paired implementation/signature scan residual: **0**.
- All **54/54** nonstdio full-artifact `apply_to_artifact` boundaries return `protocol::MutationApplyResult`; bare-return residual: **0**.
- All **54/54** nonstdio `ArtifactBuilder` implementations stage mutation and absorb candidates. Their **108** apply sites commit `self.snapshot` only from `Ok`; rejection records a fatal `mutation.apply` diagnostic. Direct Result-to-snapshot assignment residual: **0**.
- Production wrappers for sequence, raster, remodel, playbook, forms, note, block, procedural, assembly, FEM, GIS, DAG, writer, Present, Trinity Rewrite/Jack, and the Norm host now propagate the typed rejection. The Wires example constructor is also fallible; its command boundary retains the mutation code and rendered target in its `Fault`.
- Named collection families reject missing, duplicate, conflicting, incomplete, and invalid-order targets before commit: CAD, GIS, VCS, FEM, Architect, Block, Puzzle, Layout, Shooting, Curate, Rewrite, and Jack.
- Indexed/nested families stage cloned candidates and reject out-of-range indices, missing parents, duplicate identities, invalid JSON patches, and wrong variant targets: Assembly, Note, Draw, Raster, and Lowpoly. Nested errors retain inner addresses via `MutationApplyError::under`.
- Puzzle 2d/3d/5d JSON bridges reject invalid base/result documents and prefix nested typed errors under `document`; they do not default malformed persisted apply input.
- Adversarial coverage is present for a missing named CAD node, an invalid indexed Assembly insertion, and a missing nested Note parent; every test also asserts that the borrowed base remains unchanged.
- Source parser/formatter emission succeeded for **185** nonstdio Result/error-bearing Rust files with **0** failures. No Cargo command was run in this lane; serialized package checks remain parent-owned.

## Contract-Bearing Framework Files

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔗️causal/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs`

## Static Evidence

- Framework `MutationDiff` implementations: **31**.
- Plugin `MutationDiff` implementations inventoried for follow-on: **162** = **58 stdio** + **104 nonstdio**.
- Focused Rust parser/formatter emission completed for command, DAG, and workflow sources.
- TypeScript transpilation completed for the kernel component.
- Static scan found no framework `fn apply(...) -> P` implementation; its only textual hit is an explanatory store doc example.
- Parent-owned framework gates are green: serialized kernel no-run passed and the full kernel suite passed **955/955**. Nonstdio package gates remain pending after this source freeze.

## Framework Implementation Inventory (31)

- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️component.rs:36:impl protocol::MutationDiff<OpeningPreferences> for OpeningPreferences {`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️component.rs:31:impl MutationDiff<MergePolicySetting> for MergePolicySetting {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:7830:impl MutationDiff<DagSnapshot> for DagDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:179:impl MutationDiff<FlowFixture> for FlowDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2175:    impl MutationDiff<DemoSnapshot> for DemoDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:2044:        impl ::protocol::MutationDiff<$ty> for $ty {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6668:impl MutationDiff<SpaceHistorySnapshot> for SpaceHistoryDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8272:    impl MutationDiff<DemoSnapshot> for DemoDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9542:        impl MutationDiff<DemoSnapshot> for LossyDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:1243:    impl MutationDiff<i64> for AddDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:1660:    impl MutationDiff<MiniDoc> for MiniDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔗️causal/🦀️component.rs:446:    impl crate::os_spr::command::MutationDiff<i64> for CausalAddDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs:1063:    impl crate::os_spr::MutationDiff<i64> for AddDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:1303:impl protocol::MutationDiff<WorkflowSnapshot> for WorkflowDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:2199:impl protocol::MutationDiff<RunArtifact> for RunDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:447:    impl protocol::MutationDiff<DependencyTestSnapshot> for DependencyTestDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:14014:        impl protocol::MutationDiff<WireTestSnapshot> for WireTestDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:15701:        impl MutationDiff<TestSnapshot> for TestDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:15842:        impl MutationDiff<TestConfig> for TestConfig {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:18688:    impl protocol::MutationDiff<ChildrenTestSnapshot> for ChildrenTestDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6230:            impl MutationDiff<DummySnapshot> for DummyDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6465:            impl MutationDiff<TxnSnapshot> for TxnDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6833:            impl MutationDiff<SurfaceSnapshot> for SurfaceDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8078:    impl ::protocol::MutationDiff<NoConfig> for NoConfig {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8174:    impl ::protocol::MutationDiff<NoPresence> for NoPresence {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8275:    impl ::protocol::MutationDiff<NoTransient> for NoTransient {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8367:    impl ::protocol::MutationDiff<protocol::InteractionState> for InteractionConfigMutation {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:325:    impl protocol::MutationDiff<HashProjection> for HashDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs:1180:        impl protocol::MutationDiff<Counter> for AddDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:247:impl protocol::MutationDiff<SpaceSnapshot> for SpaceDiff {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:859:impl protocol::MutationDiff<CollectionSnapshot> for CollectionDiff {`

## Stdio Follow-On Inventory (58)

Every implementation under `✏️s/🔌️plugins/🗄️stdio` is listed exactly once as of this source freeze:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:684:impl MutationDiff<LasSnapshot> for LasDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:501:impl MutationDiff<PlySnapshot> for PlyDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:158:impl MutationDiff<HtmlSnapshot> for HtmlDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:196:impl MutationDiff<EpwSnapshot> for EpwDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:130:impl MutationDiff<ZipSnapshot> for ZipDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:410:impl MutationDiff<GifSnapshot> for GifDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:649:impl MutationDiff<GifSnapshot> for GifDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:1099:impl MutationDiff<PptxSnapshot> for PptxDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:421:impl MutationDiff<Mp4Snapshot> for Mp4Diff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:156:impl MutationDiff<SvgSnapshot> for SvgDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:27:impl MutationDiff<Mp3Snapshot> for Mp3Diff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:47:impl MutationDiff<Ifc2x3Snapshot> for Ifc2x3Diff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:503:impl MutationDiff<IfcSnapshot> for IfcDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:284:impl MutationDiff<BcfSnapshot> for BcfDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:55:impl MutationDiff<BinarySnapshot> for BinaryDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:239:impl MutationDiff<TxtSnapshot> for TxtDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:43:impl MutationDiff<PdfSnapshot> for PdfDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:978:impl MutationDiff<PdfSnapshot> for PdfDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:249:impl MutationDiff<CsvSnapshot> for CsvDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:498:impl MutationDiff<StepSnapshot> for StepDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:179:impl MutationDiff<TsvSnapshot> for TsvDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:690:impl MutationDiff<XlsxSnapshot> for XlsxDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:1170:impl MutationDiff<DocxSnapshot> for DocxDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:235:impl MutationDiff<MdSnapshot> for MdDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:163:impl MutationDiff<XmlSnapshot> for XmlDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:905:impl MutationDiff<JpgSnapshot> for JpgDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:838:impl MutationDiff<PngSnapshot> for PngDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:284:impl MutationDiff<AviSnapshot> for AviDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:23:impl MutationDiff<WavSnapshot> for WavDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:128:impl MutationDiff<JsonSnapshot> for JsonDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:60:impl MutationDiff<DwgSnapshot> for DwgDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:1449:impl MutationDiff<DxfSnapshot> for DxfDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:271:impl MutationDiff<BmpSnapshot> for BmpDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:367:impl MutationDiff<TiffSnapshot> for TiffDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:43:impl MutationDiff<DeflateSnapshot> for DeflateDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:301:impl MutationDiff<StlSnapshot> for StlDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧭️mutation-dispatch/🦀️component.rs:236:impl MutationDiff<GltfSnapshot> for GltfMutationDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:1273:impl MutationDiff<GltfSnapshot> for GltfDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:820:impl MutationDiff<ObjSnapshot> for ObjDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🔺️diff/🦀️component.rs:607:impl MutationDiff<SemioAnimationSnapshot> for SemioAnimationDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:67:impl MutationDiff<SemioSnapshot> for SemioDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🔺️diff/🦀️component.rs:308:impl MutationDiff<SemioAudioSnapshot> for SemioAudioDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🦀️component.rs:409:impl MutationDiff<SemioBrepSnapshot> for SemioBrepDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🔺️diff/🦀️component.rs:213:impl MutationDiff<SemioCadSnapshot> for SemioCadDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🔺️diff/🦀️component.rs:1030:impl MutationDiff<SemioDocumentSnapshot> for SemioDocumentDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🔺️diff/🦀️component.rs:663:impl MutationDiff<SemioDrawingSnapshot> for SemioDrawingDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🔺️diff/🦀️component.rs:341:impl MutationDiff<SemioFlowSnapshot> for SemioFlowDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🔺️diff/🦀️component.rs:53:impl MutationDiff<SemioGraphSnapshot> for SemioGraphDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🔺️diff/🦀️component.rs:419:impl MutationDiff<SemioImageSnapshot> for SemioImageDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🔺️diff/🦀️component.rs:78:impl MutationDiff<SemioKitSnapshot> for SemioKitDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🔺️diff/🦀️component.rs:229:impl MutationDiff<SemioMeshSnapshot> for SemioMeshDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🔺️diff/🦀️component.rs:200:impl MutationDiff<SemioModelSnapshot> for SemioModelDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🔺️diff/🦀️component.rs:43:impl MutationDiff<SemioObjectSnapshot> for SemioObjectDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🔺️diff/🦀️component.rs:911:impl MutationDiff<SemioPresentationSnapshot> for SemioPresentationDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔺️diff/🦀️component.rs:54:impl MutationDiff<SemioTableSnapshot> for SemioTableDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🔺️diff/🦀️component.rs:42:impl MutationDiff<SemioTextSnapshot> for SemioTextDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🔺️diff/🦀️component.rs:120:impl MutationDiff<SemioValueSnapshot> for SemioValueTreeDiff {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🔺️diff/🦀️component.rs:389:impl MutationDiff<SemioVideoSnapshot> for SemioVideoDiff {`

## Nonstdio Implementation Inventory (104)

Every implementation elsewhere under `✏️s/🔌️plugins` is listed exactly once as of this source freeze:

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:33:impl protocol::MutationDiff<WriterPresence> for WriterPresence {`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:73:impl MutationDiff<WriterSnapshot> for WriterDiff {`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:13:impl protocol::MutationDiff<MathematicalPresence> for MathematicalPresence {`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:49:impl MutationDiff<MathematicalSnapshot> for MathematicalDiff {`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:40:impl protocol::MutationDiff<Procedural2dPresence> for Procedural2dPresence {`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:141:impl MutationDiff<Procedural2dSnapshot> for Procedural2dDiff {`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:45:impl protocol::MutationDiff<Procedural3dPresence> for Procedural3dPresence {`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:158:impl MutationDiff<Procedural3dSnapshot> for Procedural3dDiff {`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:69:impl protocol::MutationDiff<AssemblySnapshot> for AssemblyDiff {`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:33:impl protocol::MutationDiff<FlowPresence> for FlowPresence {`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:78:impl MutationDiff<FlowSnapshot> for FlowDiff {`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:33:impl protocol::MutationDiff<Gis3dPresence> for Gis3dPresence {`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:38:impl MutationDiff<GisTerrainSnapshot> for GisTerrainDiff {`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:25:impl protocol::MutationDiff<Gis2dPresence> for Gis2dPresence {`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:101:impl MutationDiff<GisMapSnapshot> for GisMapDiff {`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:15:impl protocol::MutationDiff<VcsDemoPresence> for VcsDemoPresence {`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:76:impl MutationDiff<VcsSnapshot> for VcsDiff {`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:25:impl protocol::MutationDiff<PresentPresence> for PresentPresence {`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:42:impl MutationDiff<PresentSnapshot> for PresentDiff {`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:31:impl protocol::MutationDiff<ShootingPresence> for ShootingPresence {`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:203:impl MutationDiff<ShootingSnapshot> for ShootingDiff {`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:31:impl MutationDiff<PlaygroundSnapshot> for PlaygroundDiff {`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:29:impl protocol::MutationDiff<SequencePresence> for SequencePresence {`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:46:impl MutationDiff<SequenceSnapshot> for SequenceDiff {`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:14:impl protocol::MutationDiff<Fem2dPresence> for Fem2dPresence {`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:144:impl MutationDiff<Fem2dSnapshot> for Fem2dDiff {`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:14:impl protocol::MutationDiff<Fem3dPresence> for Fem3dPresence {`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:143:impl MutationDiff<Fem3dSnapshot> for Fem3dDiff {`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs:105:impl MutationDiff<ArchitectConfig> for ArchitectConfig {`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:32:impl protocol::MutationDiff<ArchitectPresence> for ArchitectPresence {`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:272:impl MutationDiff<ProgramSnapshot> for ProgramDiff {`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:35:impl protocol::MutationDiff<Process3dPresence> for Process3dPresence {`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:86:impl MutationDiff<Process3dSnapshot> for Process3dDiff {`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:33:impl protocol::MutationDiff<LowpolyPresence> for LowpolyPresence {`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:180:impl MutationDiff<LowpolySnapshot> for LowpolyDiff {`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:29:impl protocol::MutationDiff<WiresPresence> for WiresPresence {`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:88:impl MutationDiff<WiresSnapshot> for WiresDiff {`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:38:impl protocol::MutationDiff<FormsPresence> for FormsPresence {`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:109:impl MutationDiff<FormsSnapshot> for FormsDiff {`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:39:impl protocol::MutationDiff<LayoutPresence> for LayoutPresence {`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:155:impl MutationDiff<LayoutSnapshot> for LayoutDiff {`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:42:impl protocol::MutationDiff<CadPresence> for CadPresence {`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:119:impl MutationDiff<CadSnapshot> for CadDiff {`
- `✏️s/🔌️plugins/📕️norm/🎚️config/🦀️component.rs:76:impl protocol::MutationDiff<NormConfig> for NormConfig {`
- `✏️s/🔌️plugins/📕️norm/👥️presence/🦀️component.rs:16:impl protocol::MutationDiff<NormPresence> for NormPresence {`
- `✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs:522:impl<D: Clone + Default + Serialize + DeserializeOwned> MutationDiff<D> for ArtifactDiff<D> {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:52:impl MutationDiff<Iso16757Snapshot> for Iso16757Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:55:impl MutationDiff<Vdi3805Snapshot> for Vdi3805Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:82:impl MutationDiff<Din4108Snapshot> for Din4108Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:214:impl MutationDiff<Din16798Snapshot> for Din16798Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:43:impl MutationDiff<En1990Snapshot> for En1990Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:124:impl MutationDiff<En1991Snapshot> for En1991Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:133:impl MutationDiff<En1992Snapshot> for En1992Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:250:impl MutationDiff<En1993Snapshot> for En1993Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:94:impl MutationDiff<En1994Snapshot> for En1994Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:88:impl MutationDiff<En1995Snapshot> for En1995Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:94:impl MutationDiff<En1996Snapshot> for En1996Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:94:impl MutationDiff<En1997Snapshot> for En1997Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:175:impl MutationDiff<En1998Snapshot> for En1998Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:106:impl MutationDiff<En1999Snapshot> for En1999Diff {`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:64:impl MutationDiff<Din18599Snapshot> for Din18599Diff {`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:42:impl protocol::MutationDiff<PlaybookPresence> for PlaybookPresence {`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:55:impl MutationDiff<PlaybookSnapshot> for PlaybookDiff {`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs:255:impl MutationDiff<ModuleRenderPayload> for ModulePayloadDiff {`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:40:impl protocol::MutationDiff<ImperativePresence> for ImperativePresence {`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:47:impl MutationDiff<ImperativeSnapshot> for ImperativeDiff {`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:37:impl protocol::MutationDiff<RemodelPresence> for RemodelPresence {`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:77:impl MutationDiff<RemodelSnapshot> for RemodelDiff {`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:43:impl MutationDiff<EnergyModelSnapshot> for EnergyModelDiff {`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:32:impl protocol::MutationDiff<RewritePresence> for RewritePresence {`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:79:impl MutationDiff<RewriteSnapshot> for RewriteDiff {`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:36:impl protocol::MutationDiff<JackPresence> for JackPresence {`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:98:impl MutationDiff<JackSnapshot> for JackDiff {`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:27:impl protocol::MutationDiff<DagPresence> for DagPresence {`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:55:impl MutationDiff<DagSnapshot> for DagDiff {`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:30:impl protocol::MutationDiff<DrawPresence> for DrawPresence {`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:166:impl MutationDiff<DrawSnapshot> for DrawDiff {`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:34:impl protocol::MutationDiff<RasterPresence> for RasterPresence {`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:248:impl MutationDiff<RasterSnapshot> for RasterDiff {`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:34:impl protocol::MutationDiff<NotePresence> for NotePresence {`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:143:impl MutationDiff<NoteSnapshot> for NoteDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:31:impl protocol::MutationDiff<Puzzle2dPresence> for Puzzle2dPresence {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:121:impl MutationDiff<Puzzle2dSnapshot> for Puzzle2dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:233:impl MutationDiff<Value> for Puzzle2dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:317:impl MutationDiff<Puzzle2dPlaySnapshot> for Puzzle2dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:37:impl protocol::MutationDiff<Puzzle5dPresence> for Puzzle5dPresence {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:113:impl MutationDiff<Puzzle5dSnapshot> for Puzzle5dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:260:impl MutationDiff<Value> for Puzzle5dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:339:impl MutationDiff<Puzzle5dPlaySnapshot> for Puzzle5dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:33:impl protocol::MutationDiff<Puzzle3dPresence> for Puzzle3dPresence {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:123:impl MutationDiff<Puzzle3dSnapshot> for Puzzle3dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:279:impl MutationDiff<Value> for Puzzle3dDiff {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:358:impl MutationDiff<Puzzle3dPlaySnapshot> for Puzzle3dDiff {`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:19:impl protocol::MutationDiff<Block2dPresence> for Block2dPresence {`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:100:impl MutationDiff<Block2dSnapshot> for Block2dDiff {`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:19:impl protocol::MutationDiff<Block5dPresence> for Block5dPresence {`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:74:impl MutationDiff<Block5dSnapshot> for Block5dDiff {`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:19:impl protocol::MutationDiff<Block3dPresence> for Block3dPresence {`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:88:impl MutationDiff<Block3dSnapshot> for Block3dDiff {`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/👥️presence/🦀️component.rs:39:impl protocol::MutationDiff<SpacePresence> for SpacePresence {`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:16:impl protocol::MutationDiff<HomePresence> for HomePresence {`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:37:impl MutationDiff<SHomeSnapshot> for SHomeDiff {`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs:31:impl protocol::MutationDiff<SourcingCuratePresence> for SourcingCuratePresence {`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:113:impl MutationDiff<CurateSnapshot> for CurateDiff {`
