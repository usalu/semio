# JSON-first Projection and Paged Pretty Program

Read-only preparation during dependency sync9214. No production, test or manifest source is changed. Implementation still requires actual sync completion and parent go.

## One Normative Schema Metadata Owner

Current value derive canonical owner is:
- value/derive/canonical/rs: FieldRole, named_roles, presence, roles.
- named_roles consumes the actual named_fields wire_name, skip, skip_serializing_if, flatten, custom serializer role and borrowed field references.
- Current presence already supports PagedList::is_empty and PagedUtf8::is_empty explicitly. Earlier absence observations must not override this current source.
- Puzzle2dSnapshot already derives CanonicalJsonTree with camelCase and target_regions empty omission. Reuse its generated tree; do not add a second hand-maintained root traversal or use DSL's kebab/block/table projection.

The generated canonical tree is the shared direct-native projection for leaf and owned writer. Extend this same metadata plan only where its actual custom-role coverage proves insufficient; retain explicit errors for unknown custom serializers.

## Concrete Native/Opaque Scalar API

Preferred minimal role addition beside ArtifactCanonicalJsonTree:

```rust
pub enum CanonicalJsonScalarRole { Semantic, NativeBinary64 }
pub trait ArtifactCanonicalJsonTree: Sync + 'static {
    // existing node/child/key methods
    fn canonical_tree_scalar_role(&self) -> CanonicalJsonScalarRole {
        CanonicalJsonScalarRole::Semantic
    }
}
```

Primitive f64 explicitly returns NativeBinary64. Intrinsic Number::Float and DslValue retain Semantic, including their nested values. The scalar node remains F64 and every existing general JSON consumer retains its current decimal semantics. The role is metadata queried by the declared wire adapter, not a new global node variant.

Actual wrappers in tree/rs that delegate their own node must forward the role: RetainedPublicationRoot, SealedShared, Option (None remains Semantic) and Box. Current source has no Arc tree implementation, so do not invent one. Derived transparent/newtype wrappers in value derive canonical must forward the inner role; named records, arrays, maps and tuple containers retain their child roles through existing canonical_tree_child references. Manually delegated scalar wrappers discovered by the consumer census need the same explicit forwarding only where they truly wrap a native scalar.

This metadata avoids broad enum-match churn across unrelated products. No global Float-to-word rewrite occurs. The declared Puzzle wire view selects NativeBinary64 only; opaque Number/DslValue numbers and user objects named bits stay semantic JSON. Existing explicit f32 hexadecimal roles remain unchanged. A custom schema serializer still requires its declared explicit canonical role.

## By-value Ordinal Wire View

Add a first-party adapter near pack/json's existing JsonWriteSource. API shape:

```rust
pub enum NativeFloatWire { Decimal, Binary64WordObject }
pub struct CanonicalJsonSource<'a> {
    root: &'a dyn ArtifactCanonicalJsonTree,
    native_float: NativeFloatWire,
}
```

The controller owns the actual SnapshotRead; it creates this borrowed view only for each step. JsonWriteSource.node_at_path resolves actual canonical child ordinals. Its small by-value resolution handle distinguishes Native(&dyn Tree), virtual WordObject(u64), and virtual WordString(u64). No heap DTO or persistent reference to a temporary wrapper is needed.

The virtual object is Object(1), key bits, and one scalar child. Extend JsonWriteNode with an explicit HexWord64(u64) scalar whose canonical string is exactly sixteen lowercase hexadecimal characters. The current writer's F32HexWord is the reference for syntax/bit preservation, not an f32 conversion path.

CanonicalJsonSource uses the same generated tree field counts, keys, presence and flatten rules. Native float wire policy is selected explicitly by the Puzzle declared JSON route. The semantic native snapshot JSON boundary continues to select Decimal.

## Pretty Formatting and Paged Sink

Leaf currently produces pretty JSON. Parsing equality alone is insufficient. Define a first-party layout value (Compact or the existing canonical two-space Pretty layout), and use exactly the same layout/emission core for the synchronous leaf and owned program. Neutral fixtures author exact expected bytes, including empty object/array, nested object-in-array, strings/escapes and final newline policy. Do not silently switch Puzzle files to compact output.

Current JsonBorrowedWriteCursor combines traversal and a whole measured String allocation. Reuse its structural state machine and bounded64-source-byte text transitions, but split output storage from emission:
- traversal state owns frames/path/phase and exact measure/write position;
- a bounded sink window receives emitted bytes after its original capacity and copy demand is admitted;
- a counting sink measures the same layout without materializing output;
- the owned sink stores real pages in OwnedSerializerOutput.pages, leaving contiguous empty;
- a synchronous string sink uses the identical program for existing leaf calls.

Use genuine output pages no larger than16384 bytes (the existing output-page bound). A new page's Vec backing and PagedList slot/backing are separately quoted and paid before allocation/initialization. Never request the entire1.2MiB file as one guest contiguous allocation or hide initialization work. Actual per-step work/receipt includes text escaping, indentation, cursor state updates and all actual buffer copies.

The controller captures the measured total at the completed measure frontier before entering write phase. Progress is phase encode, completed actual bytes written, total that captured exact length. Cancellation qualifies only with0<completed<total. Measurement progress is useful but cannot satisfy the interior conversion predicate.

## Factory and Controller Files

Expected natural responsibilities:
- value/derive/canonical/rs: sole field metadata generation; avoid changes unless needed after exact role tests.
- pack/json/encode/tree/rs and value derive canonical: explicit scalar provenance metadata and transparent-wrapper forwarding; existing scalar node vocabulary/decimal consumers stay unchanged.
- pack/json/controlled-and-borrowed-bridge: by-value canonical tree source, HexWord64 and shared bounded pretty emission core.
- pack/json/encode's natural sink/page responsibility: genuinely admitted output pages and same emitter counting/string sinks.
- framework OS io/serialization/owned: common captured-text job lifecycle, metadata/outcome/source/output handoffs and terminal frame release.
- Puzzle2d/3d/5d IO export owned: exact typed factory instantiations over real captured SnapshotRead and declared native-float wire policy.
- existing three IO/routes JsonFormat implementations: OWNED_FACTORY and synchronous leaf share normative projection/emitter.
- existing2d text-inverse manifest: explicit normal integration target owned-text, with its own HeapWitness.

No new registry, ComposerEntry, DslValue mirror, Draw read conversion or compatibility backend is introduced. All actual consumer matches and production/test Source closures must be measured before editing.

## Bounded Consumer Inventory

The directly observed canonical source consumers are:
- value derive's generated node/child/key methods, struct/enum wrappers and flattened field traversal;
- pack/json canonical scalar byte formatter;
- pack/json tree cursor scalar inspect/emission;
- pack/json whole-wire tree cursor;
- generic canonical JSON public encode/measure adapters and their neutral tests;
- existing explicit native f32 word wrappers, arrays/lists/options;
- DslValue and Number manual tree implementations.

Before implementation run a precise repository-wide symbol census for ArtifactCanonicalJsonNode and JsonWriteNode consumers, excluding outputs/symlinks, and persist exact files. The current list is a bounded directly inspected inventory, not a claim that exhaustive enum matches outside pack/json do not exist.

## Minimal Closed Neutral Cases

The proposed case schema uses exact format, nativeFloatPolicy, source document/role description, expected UTF8 bytes, operation Source, turn grant, cancellation phase/byte predicate and terminal witnesses. Unknown fields are rejected. Cases:
1. Every authored IEEE word as a typed native scalar: negative zero, finite extremes/subnormal/adjacent values, infinities and NaN payloads; sixteen lowercase digits preserved exactly.
2. The identical numeric value inside opaque intrinsic Number/DslValue stays numeric. An unrelated opaque object with key bits is unchanged.
3. Multiple array rows and catalog rows carry native word roles;43 existing fixture pointers are specimen occurrences, not an exhaustive role list or iteration limit.
4. Optional native scalar absent with skip, explicit null without skip, Some(word), default retained/omitted according to actual metadata, empty PagedList omission, flattened present/absent fields and renamed camelCase names.
5. Exact pretty byte fixtures for nested objects/arrays, empty collections, multibyte/escaped paged text, quotes/control characters and page boundaries.
6. Zero and underfunded grants leave complete cursor/source/page identity unchanged; one-unit funded steps make actual bounded progress; metadata/copy/capacity/release receipts match physical observations.
7. Cancel during measure, after positive incomplete encode, at page handoff and during close. Require actual canceled Fault, forbid completion/output publication, and paid source/outcome/partial-page/frame terminal closure.
8. Stale generation/revision, wrong type/route/children and refusal return the same genuine captured request and actual incurred progress.

Retain the complete relational/word specimens and all seven exact native route pairs. Independent serde_json must parse the whole output and compare independently authored expected word strings, presence and every semantic field. It also supplies pretty-byte differential expectations for finite material/opaque numbers; explicit bit-object expected bytes are assembled independently from integer bits. Actual1024 material baseline and strict interior cancellation are native integration proof, not yet executed.

## Exact Read-only Symbol Census

The framework/Hub/extensions Rust sweep found 127 files referring directly to ArtifactCanonicalJsonNode or JsonWriteNode. This includes imports and tests, not just exhaustive match consumers. Paths/line numbers follow; no build outputs or symlink trees were scanned.

- 🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/⚠️diagnostic/🌲️source/🦀️.rs:3
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:31
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🧯️retirement-laws/🦀️.rs:72
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🥣️companion/🧭️tree/🦀️.rs:3
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs:67,71,76,77,78,114,115,116,117,118,178,179,180,181,182,183,184,189,194,199,206
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🫳️borrowed/🦀️.rs:2,43,45,46,47,48,49,54
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs:2
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/📨️wire/🧪️tests/🦀️.rs:3
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🔣️scalar/🦀️.rs:7,42,49,53,54,55,56,57,58,59,60,61,62,63
- 🧰️framework/🔨️modules/🛂️manifest/🧩️contribution/🦀️.rs:4
- 🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🦀️.rs:6
- 🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🆔️ids/🦀️.rs:3
- 🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🌐️foreign/🦀️.rs:3
- 🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🧾️policy/🦀️.rs:3
- 🧰️framework/🔨️modules/📡️replication/📡️wire/🧪️tests/🔬️interaction-wire/🦀️.rs:5
- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-native/🦀️.rs:136
- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧭️canonical/🦀️.rs:3
- 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/author.rs:3,6
- 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs:279
- 🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:836
- 🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🦀️.rs:88,98,144,151,177,185,198
- 🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🌳️canonical/🦀️.rs:5
- 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌲️canonical/🦀️.rs:3
- 🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🦀️.rs:92,97
- 🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/🧪️tests/🦀️.rs:25,29
- 🧰️framework/🔨️modules/🏗️mesh-engine/🌲️canonical/🦀️.rs:3
- 🧰️framework/🔨️modules/🏗️mesh-engine/🎯️component-references/🦀️.rs:72
- 🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🪆️binding/🦀️.rs:41
- 🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🦀️.rs:80
- 🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🦀️.rs:389,391,394,395,399,400,417,429
- 🧰️framework/🔨️modules/⚠️diagnostic/📤️wire/👣️cursor/🦀️.rs:5
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🦀️.rs:25
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/⚖️compare/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/📨️request/🦀️.rs:3,15
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/📡️invocation/🌳️dependency-work/🪞️view/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🖼️display/🦀️.rs:3,37,40,43,44,47,48
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-canonical/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/📨️intent/🦀️.rs:35,36
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/🔏️canonical/🦀️.rs:6
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🎞️actions-b/📤️drawing-export-and-snapshots/🔍️trace/🦀️.rs:6
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/📥️request/🎟️context/🦀️.rs:3,11,12,14,15
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🧪️tests/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🧪️tests/🦀️.rs:97
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/💡️inference/🚪️gateway/🦀️.rs:18,34,38,40,42,48,51,52,53,55,56,57
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🌳️field-catalog-parent-app-and-branch-snapshot/🦀️.rs:285,287,288,301
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs:242
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️.rs:6
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🦀️.rs:5,44
- 🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🪶️transfer/🦀️.rs:22
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🌲️canonical/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🌲️canonical/🧪️tests/🦀️.rs:4
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🌲️canonical/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🌲️canonical/🧪️tests/🦀️.rs:4
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:419
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📤️source/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧪️tests/🔬️unit/🦀️.rs:2,82
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧪️tests/🦀️.rs:4,104,105
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/📝️text/🧬️mutations/🎚️fields/🦀️.rs:6
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🔏️digest/🦀️.rs:70
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🏁️publication/📚️metadata/🧪️tests/🦀️.rs:12
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🧵️operations/🦀️.rs:35,36,110,183,192,194,196,200,201
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🧭️prelude/🦀️.rs:5
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🧮️snapshot/🦀️.rs:387
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:214
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs:6
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs:117,401
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs:351,624
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️replay-retirement/🦀️.rs:51,255
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:14,65
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📥️lifted/📝️text-snapshot/🦀️.rs:4
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📥️lifted/🧬️mutations/🦀️.rs:2
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:30,58,89,120,173,243,251,252,253,254,255,257,275,380,381,403,404,405,422,423
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🧪️tests/📖️reader/🦀️.rs:8,210,212,213,218,235,238
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🔤️text/🧪️tests/🦀️.rs:8,10,11
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧵️borrowed/🦀️.rs:8,168,169
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs:6,66,71,76,81,144,151,170,171
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs:3,48,52,54,57,60,63,66,78,86,99,104,110,111,465,469,470
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🌱️value/🦀️.rs:3,28,29
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs:20,243
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🔏️canonical/🦀️.rs:2,15
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🦀️.rs:12
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🧾️dag-output-sources/🦀️.rs:65,67,70,71,72,73,74,75
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/📚️dictionary/🦀️.rs:145,146,150,153,373,374,380,390
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧪️tests/🌲️canonical/🦀️.rs:3
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/📤️output/🦀️.rs:23
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️media-slots/📤️slot/🦀️.rs:5,57
- 🧰️framework/🛍️products/📓️print/🚪️io/📝️text/🧬️mutations/🦀️.rs:9
- ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗺️owned-map/🦀️.rs:496,497
- ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🖼️asset-content-and-patch/🦀️.rs:99,101,102,103
- ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/📬️resolve/📡️resume.rs:5,9,11
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📐️tessellate/🦀️.rs:5
- ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:32
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/📦️codec/🫳️borrowed/🦀️.rs:3,37,38,42,51,52,54,55,56,57,58,59,60,61,62
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🦀️.rs:46
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🧾️value-and-canonical-codecs/🦀️.rs:148,149,202,203,224,225,237,238
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/wire.rs:3
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/native-gltf-write.rs:4
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/📨️wire/pointer.rs:3
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/📨️wire/wire.rs:3
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:443
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔏️canonical/🦀️.rs:3
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🦀️.rs:84
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🦀️.rs:127,128
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/💾️binary/🧬️mutations/🫳️borrowed/🦀️.rs:5
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:192,193
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔏️canonical/🦀️.rs:3
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs:236,258
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧵️canonical/🦀️.rs:3
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🌱️value/🧵️canonical/🦀️.rs:3
- ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:15
- ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:17
- ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:15
- ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:153,155
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/📄️document/⚖️equality/🦀️.rs:2
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📚️document-model/🦀️.rs:27,29,30
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs:80,82,83
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🫧️transient/🦀️.rs:152,153
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:597,598,599,602,609
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/⚖️equality/🦀️.rs:2
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📋️clipboard/🛫️encode/🦀️.rs:3,52,53,54,65
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🕸️mesh/🌳️canonical/🦀️.rs:5
- ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🏷️ids-and-fixed-table/🦀️.rs:68,69
- ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🦀️.rs:67,68
- ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/📦️pack/🦀️.rs:4
- ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🦀️.rs:65
- ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️native-extent/🦀️.rs:3

## Native Key and Forwarding Follow-through

A concrete additional borrowed-source API requirement follows from reuse of canonical metadata: ArtifactCanonicalJsonTree.canonical_tree_key returns ArtifactCanonicalJsonText (borrowed or native paged text), while current JsonWriteSource.object_key_at_path returns only &str. The adapter must not flatten PagedUtf8 map keys. Change that key result to the existing first-party ArtifactCanonicalJsonText and teach bounded key emission to preserve its chunk and character cursor. Reuse the existing text type, not a second string-owner enum. For a native key, object member ordinal and text chunk ordinal must occupy separate bounded cursor fields; existing frame.index already serves member selection.

Actual currently inspected forwarding owners are RetainedPublicationRoot, SealedShared, Option and Box in tree/rs, plus generated transparent/newtype structs in value derive canonical. None maps to Semantic; Some forwards actual scalar role. No Arc implementation was found in that current tree module; if the full forwarder census finds one elsewhere it must forward, but do not add an unused Arc implementation merely by analogy.

Additional closed cases: native paged object keys with escape/multibyte data spanning page boundaries; native f64 under every actual transparent/optional/root wrapper; intrinsic Number and nested DslValue under the identical wrappers; None/null/skip; and a custom serializer that explicitly delegates to its declared canonical role. All expected byte strings include the original two-space pretty layout.

Scalar provenance does not alter ordinary Core or any general interpreter semantics. The separate paid actor operation work must reuse existing ordinary CoreInstance semantics; this JSON-first plan does not implement or close that prerequisite.

## Exact Forwarder Classification

The same framework/Hub/extensions scan inspected canonical_tree_node definitions with delegation in their function body (up to20physical lines). Eighteen definitions were found; generated quote bodies are separately covered by derive canonical.

Transparent generic production forwarders requiring role forwarding:
- pack/json/encode/tree lines20 RetainedPublicationRoot,25 SealedShared,175 Option,180 Box.
- replication/io/text/canonical/ids line27 transparent identifier tuple wrapper; forwarding is semantically harmless for its current string payload and preserves the wrapper contract.
- value derive canonical transparent struct branch133, applied by the generated impl198.

Intentional opaque boundary, keep Semantic:
- pack/json/encode/tree258 DslValue delegates intrinsic Number only.
- graph/manifest/canonical13 is a semantic dynamic value (numeric arm explicitly F64), not a native domain scalar view.
- neural engine model/dictionary390 is its dynamic Value wrapper (Atom/Dictionary), so inspect the actual Atom role contract before treating any delegated scalar as native. Do not infer typed-native provenance merely from implementation delegation.

Object/collection-only production delegation retains Semantic at its own node; existing child refs preserve each field role:
- replication/io/text/canonical46 augments an object with one tag.
- renderer browser-worker/output23 returns empty/media-slot arrays.
- Forms change-block-field153 augments a JSON object.
- Draw schema597 flattening macro augments a JSON object.

Test-only direct node wrappers found:
- OS io/tests/transfer22.
- store ledger/publication/metadata tests12.
- store supersede-law401, deferred-reprojection624, replay-retirement51.
- store canonical-edit/borrowed tests71 (text/array/object dynamic owner).

For test wrappers genuinely transparent to arbitrary native payload, forward the role alongside node/child/key and add a wrapped-f64 regression. For wrappers deliberately testing semantic dynamic trees, keep explicit Semantic; do not broaden the production word policy. No Arc canonical-tree implementation was found by this current scan.

### Manual Native Scalar Owners

Puzzle3dScale in3d schema/document-model lines26–36 and Puzzle5dScale in5d schema/document-records lines79–89 manually emit F64 for Uniform and Array(3) for Vec3. Their scalar-role method must explicitly return NativeBinary64 for Uniform and Semantic for Vec3. Vec3 child references already borrow primitive f64 and retain the native role. Add both variants to the neutral/native full-specimen checks; these owners would not be found by a delegation-only search.
