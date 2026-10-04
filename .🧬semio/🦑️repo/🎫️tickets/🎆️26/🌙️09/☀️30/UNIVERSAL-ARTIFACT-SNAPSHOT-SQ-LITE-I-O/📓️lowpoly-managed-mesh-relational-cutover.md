# Lowpoly Managed Mesh Relational Cutover

## Current authority and distinction

The actual kernel `HalfedgeMesh` owns vertices (position, optional normal, optional halfedge), halfedges (vertex, optional twin, next, optional face, UV), faces (halfedge, smooth, flipped), seam identities, named attributes (domain, semantic, interpolation, full intrinsic samples, optional index stream), full intrinsic materials, and named textures (MIME plus octets). Rendering the mesh into a tessellated SemioMesh would lose this topology and authored channels.

Lowpoly editor `reload_meshes` currently parses persisted `mesh_content` with the first-party Pack JSON parser; `sync_meshes_to_snapshot` writes the actual managed kernel back through `HalfedgeMesh::to_json`. Consequently SQL `lowpoly_object.mesh_content TEXT` currently hides interpreted topology. The separate legal neutral corpus also deliberately owns malformed source text with NUL and arbitrary independent SQL edits. Those literal source states must remain first-class text rather than being parsed unconditionally.

The intended owner separates explicit source text from optional complete typed `meshState`. Managed editor reload/sync/import and mutation/diff authority will consume and publish the typed state directly; source text remains an independently authored document field and cannot act as a fallback typed owner. No SQLite interpretation requires JSON parsing. The existing child handle remains distinct from actual topology and source text. Optional typed state must distinguish absence from an empty complete mesh.

## Test-first stage

Three adjacent files are handcrafted under Lowpoly Snapshot: strict language-neutral schema, full topology/rich-value fixture, and actual registered Source tests. The existing Source test root imports the new facet; no command or runner is added. The first test uses AJV closed validation, DataView and Buffer binary32 word agreement, and independent Bun SQLite topology joins. The second calls the real Lowpoly owner projection then independently queries halfedge relations, every vertex raw word, face flags, seams, attribute domains/options, exact full UInt64 decimal and Int64, ordered duplicate intrinsic members, texture MIME/octet entities, full roundtrip, and direct SQL edits. It preserves explicit source text while editing semantic topology/octets. Current producer has no semantic mesh tables; authentic Source demand replay is in progress.

Paths:

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🕸️mesh/🔣️.json`
- same directory `🧬️schema/🔣️.json`
- `.../📸️snapshot/🧪️tests/🪶️sqlite/🕸️mesh/🟦️.ts`
- existing `.../📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts` import only

Command: `@semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-source`, quick profile, uncached, same task-owned Nx pool. Full output remains ticket generated. No Native command was run and no production mesh pair is mounted yet.

## Complete paired implementation obligations

1. Declare a canonical first-party halfedge state owner and five public schema facets, including all original topology and channels. Typed metadata must use the actual nine-kind IntrinsicValue with raw binary64, full integer domains, octets, arrays and ordered duplicate object members. Named kernel maps are unique; seam ownership is unique. Optional indices remain absent versus empty.
2. Add the actual Lowpoly typed state field to artifact/snapshot/diff/patch/mutation/native grammar and public facets, with handcrafted fixtures/constructors. Original malformed source literals remain exact and independent. Actual source import is an explicit boundary, not an implicit SQL adapter.
3. Author mesh/vertex/halfedge/face/seam/attribute/index/sample/material/texture/octet/value/scalar/array/member entities. Relationships use surrogate owner identities plus ordinal or literal named keys. Logical topology indexes are retained even in incomplete editable states; reconstruction validates ownership/cardinality/width/ordinals without normalizing their literal values. Float32 positions/normals/UVs and Float64 intrinsic samples keep query value plus exact words and class; UInt64 uses exact decimal TEXT with range checks.
4. Use the same caller operation and current shared checked RowIndex for Source collection/relationship lookup; capture full typed input before callbacks. Rust projection/reconstruction must pay real Vec/string/octet/frontier requests via the canonical caller and guards. No guessed BTree backing or copied intermediary bridges.
5. Pair editor reload/sync/primitive/import/export, owned document helper, cache, child hash, inverse and paint sessions against the direct typed owner. Original child identity is not reconstructed from lossy tessellation. Declare any separate explicit source publication hashing policy and test it independently.
6. Add a genuine owning Native demand preserving complete fields, both encodings, independent SQL edits, full words and guarded cancellation. Root owns its runtime selection. Source success will not imply Native allocation proof; numerical System laws remain distinct gates before claimed complete backing closure.

Authentic registered Source demand RED:8selected/7passed/1failed,49expectations,4.03s assertions/Nx1m34. The real output independent Bun SQLite query fails `no such table: lowpoly_mesh_halfedge` at new Source facet26. Prior6laws and strict corpus differential law pass. Receipt `🗑️generated/physical-lowpoly-managed-mesh-source-before.log`. The whole typed owner/cell writer/editor pair is authorized by this actual semantic omission, but production remains unmounted while the complete declared model and consumer set is prepared.

Expanded registered Source before production: 9 selected, 8 passed, 1 failed, 57 expectations, 4.00 s Bun, Nx 1m7. Same missing `lowpoly_mesh_halfedge` remains; both expanded independent closed corpus/unique namespace laws pass. Log `🗑️generated/physical-lowpoly-managed-mesh-expanded-source-before.log`.

## Current Source Semantic Receipt

Registered `@semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-source`, quick, uncached, completed with 9/9 passed and 80 expectations (Nx 51.3 seconds). Receipt: `🗑️generated/physical-lowpoly-managed-mesh-source-full-safe-integer-oracle.log`. All six original tests remain. The three added laws validate closed AJV ranges and variants, independent SQLite named uniqueness versus duplicate intrinsic member occurrence, and actual 27-table provider full managed-owner roundtrip/edit/presence semantics. The original arbitrary invalid/NUL `meshContent` remains unchanged and independently editable. The signed-integer oracle uses actual `Database.deserialize(...,{safeIntegers:true})`, preserving Int64 minimum without JS Number narrowing.

Current mounted semantic production roster: schema `🕸️mesh/🟦️.ts`; schema root `🟦️.ts`; snapshot SQL root `🟦️.ts`; adjacent SQL `🕸️mesh/{🗄️.sql,🟦️.ts}`. Existing source and package fixtures now explicitly construct `meshState:null`. The SQL uses authored primary keys/references/checks; it does not require automatic UNIQUE indexes that the first-party file grammar disallows. Complete relation ordinal consumption and typed namespace uniqueness reject duplicated owners.

Rust remains the previous six-table model at this receipt. The typed Rust model/enum factories/kernel owned-topology seams/public facets are still being assembled. No current Native, full allocator, JavaScript heap accounting, or managed editor runtime claim follows from this Source result. Low documented pre-callback captures/plain Maps, unbounded managed census and existing parent paint temporary ownership; these remain explicit source-control/backing obligations, not guessed solved bytes.

## Public lossless JSON gate and direct Rust test-only stage

The registered Source public-contract baseline selected ten laws: nine passed and the new public mesh JSON law failed because the actual LowpolyObject schema had no meshState. There were 81 expectations; uncached Nx took 1m12. This is an assertion RED, not a parser failure. The first paired replay reached AJV and found its real ArtifactRef schema registration dependency; the next paired replay includes the actual first-party IO schema.

The published mesh facet now references the canonical full-nine intrinsic JSON facet, while the old primitive DslValue JSON projection remains unchanged. Managed positions/UVs use exact eight-digit binary32 words; intrinsic floats use sixteen-digit binary64 words, integer magnitudes use bounded decimal text, and octets and ordered duplicate object members stay explicit. The actual Lowpoly JSON boundary delegates this lossless facet, with no JSON String inside the managed SQL owner.

A sixth Native test-only law is mounted adjacent to the existing five: sqlite_snapshot_lowpoly_managed_mesh_complete_owner_and_independent_sql_edits. It constructs the real current Snapshot through FromValue, demands preservation of all seven mesh roots, then exercises both actual erased native codecs and an independent Bun SQLite edit. Current Rust producer and six-table provider remain unchanged, so this is the missing actual owner gate. Root owns its Native execution. Parser success is not a runtime receipt.

### Authentic public contract paired receipt

Registered Source replay `physical-lowpoly-managed-mesh-public-contract-full-schema-source.log` completed exit zero: 10 selected / 10 passed / 85 expectations, Bun 1.97s and uncached Nx 19.6s. The complete actual IO/child/primitive-value/full-intrinsic/managed-mesh schema graph is registered in AJV; no stub schema was added. This proves the declared lossless public JSON decoder alongside all prior SQL and literal source laws. It does not prove Native or full JS heap accounting.

### Held Rust ownership assembly

Rust production remains unchanged pending Root's authentic six-law baseline. Inputs now contain the complete 21-table `RowWriter` managed-mesh visitor, seven-root record owner, canonical mesh-enum controlled DslField facet, all raw-word equality, a lossless public JSON codec and a shared full-nine tagged intrinsic JSON codec. The SQL visitor pays its borrowed frontier via actual caller reserve/growth. Its UInt64 formatting is a fixed twenty-byte stack decimal primitive; no heap decimal String or guessed backing debit is used. Public ordinary JSON parsing is a distinct declared boundary, not a reconstruction bridge. The held JSON codec retires partial intrinsic output through the existing canonical constant-space authority; no new shared retirement implementation is introduced. Full paid SQL reconstruction and all editor/mutation/asset joins are still in progress. Parser checks for held files are syntax evidence only.

## Root Actual Six-Law Native Before

The real registered current native target ran6 owning laws: original5 passed and only additive managed full-owner/independent SQL edit law failed. Actual nextest2.071s, Nx6m22s, exit1. The complete binary hash and failure excerpt are retained in root-lowpoly-managed-mesh-six-native-before-receipt.json. Rust model/provider remained old during this authentic before run; the held full cutover is now authorized to mount against exact current guards and then replay the unchanged6 owning laws. This does not establish current allocated reconstruction/capture correctness before actual after verification.

## Current Authorized Rust Mount and Closed Patch Pair

Root obtained the genuine six-law BEFORE receipt in `🗑️generated/root-lowpoly-managed-mesh-six-native-before-receipt.json`: the original five laws passed; the additive complete managed owner/independent SQL edits law failed because the actual owning constructor returned Null for meshState. The compiled binary SHA256 was `7d13618214995639ccded5cf8a357b48081079b47ef58c546dcfe05902410fa0`; assertions took 2.071 s, uncached Nx 6m22s, exit 1. This authorized the current Rust mount. No AFTER runtime receipt is claimed here.

The current canonical model owns vertices, halfedges, faces, seams, complete named attribute channels, full nine-kind material/sample values, and named texture MIME/octet owners. Literal meshContent remains independent, including the original invalid NUL text. LowpolyObject's native record adds mesh_state after its existing fields. Public JSON uses raw binary32 words and the shared full tagged intrinsic JSON facet; the managed native factory remains the real DslRecord field factory. GraphQL and Proto publish the same seven roots, optional topology/indices, full intrinsic values and octets. The Proto facet uses fixed32 geometry words and canonical DslValue fixed64 intrinsic words.

The current Rust SQLite provider owns 27 tables (six parent plus 21 managed mesh). One RowWriter supplies owned output and both borrowed native-phase semantic censuses. Reconstruction validates the authored complete schema with the supplied caller, then runs a real allocation_stage and settles NativeDecodeControl.owned_bytes on success/refusal/cancellation. Typed paid identities, grouping references and consumed flags are used in reconstruction; signed surrogate IDs and dense relationship ordinals are preserved. Partial recursive intrinsic frame/build guards retire their retained values through the canonical authority. These are current source facts, not numerical allocator proof; the selected six-law gate is semantic/control evidence, not a complete System observer.

Editor geometry now reads only persisted mesh_state. The session workspace is a typed LowpolyMeshState cache. Kernel edits publish complete typed state plus a content identity over its canonical tagged Value bytes, without rewriting the independently retained source text. Explicit external authored default geometry JSON is admitted once at its declared input boundary and transferred into typed state. Selection mutations/inverses preserve typed prior state; create/delete mesh events contain both the explicit literal source channel and the typed managed channel. Missing managed state is an explicit editor refusal, not a source parsing fallback.

The distinct closed patch-presence Source demand executed 11 laws: 10 passed/1 failed, 87 expectations, uncached Nx 29.1s (`physical-lowpoly-managed-patch-closed-before-source.log`). Its closed JSON schema/TS parser pairing executed 11/11, 95 expectations, Bun 376 ms, uncached Nx16.8s (`physical-lowpoly-managed-patch-closed-paired-source.log`). Untouched null, touched-cleared `{state:null}`, and touched-full seven-field state are distinct; unknown slot/state keys refuse through AJV and the actual Source parser. This increases only the Source roster 10→11; the Native six laws are unchanged.

The binary64 SQL declaration independently failed when PRAGMA reported BLOB, then passed after the declaration was paired to the actual signed INTEGER word companion used by both implementations (`physical-lowpoly-managed-mesh-binary64-schema-before-source.log`, `physical-lowpoly-managed-mesh-binary64-schema-paired-source.log`, latter 10/10 and 86 expectations). No word codec or comparison was weakened.

The existing TypeScript :build completed successfully (30 outputs; Nx5.2s). A current package :check completed after the managed model (suites2; Nx9.9s). The later full public-facet :check exposed one extra parser argument (TS2554); that exact join was removed and the current paired :check completed successfully, suites2, uncached Nx14.6s (`physical-lowpoly-current-managed-public-final-paired-check.log`). Rust parser-only sweep covers 244 current Rust files and found zero parse failures (`physical-lowpoly-current-coherent-parser.json`); this is not Rust compilation.

Named current mounted production joins:

- artifact `🦀️.rs`; subset schema `🦀️.rs`; managed mesh `🦀️.rs`, `🔣️json/🦀️.rs`, `🔗️.graphql`, `🛰️.proto`; public root/diff/mutation schemas and typed facets.
- snapshot `🪶️sqlite/🦀️.rs`; managed `🛫️projection/🦀️.rs`, `🔍️rows/🦀️.rs`, `🛬️reconstruction/🦀️.rs`, `📏️preflight/🦀️.rs`, SQL declaration; root/native opt-in remains mounted.
- editor `⚙️engine/🦀️.rs`, `🖌️session/🦀️.rs`; create/delete mesh and selection-motion producers/inverses; direct default/document/test consumers; authored full object and event/diff assets.
- shared Value `🧬️schema/🌳️intrinsic/🔣️json/🦀️.rs` plus its root include; canonical mesh-engine native enum binding plus actual direct DslRecord dependency; canonical kernel owned-part ingress/egress/public topology fields.

Remaining explicit qualifications: Source ordinary Map/capture frontiers and temporary paint Uint8Array are not full JS allocator evidence. Ordinary editor kernel conversion builds actual maps/sets outside the paid native/SQLite construction path; no physical bridge credit is claimed. Full runtime release/overflow safety for every arbitrary editor-owned mesh depth is not established by the six-law gate. Native production is coherently authored but awaits Root's unchanged registered six-law AFTER.

## Current Native AFTER Compiler Prerequisites

Root's unchanged six-law AFTER reached zero assertions: exactly five compiler diagnostics in four actual files. The panel density fixture now constructs/caches complete typed mesh state from its real ico-sphere owner; its original object/window/extent bodies remain. Geometry import now transfers the already built full HalfedgeMesh into LowpolyMeshState and derives the managed child identity, retaining the independently generated source literal. The artifact's temporary identity Value uses the explicitly qualified canonical FromValue retirement authority. The intrinsic text reconstruction takes the row into a local before calling the same caller's copy_text, resolving the nested mutable borrow without changing admission. All six native laws remain unchanged.

Parser-only rustfmt of the actual artifact root and joined modules succeeded (exit0), receipt physical-lowpoly-five-compiler-prerequisites-parser.log. No typecheck or assertion claim. Root may retry the unchanged six against this exact paired state.
