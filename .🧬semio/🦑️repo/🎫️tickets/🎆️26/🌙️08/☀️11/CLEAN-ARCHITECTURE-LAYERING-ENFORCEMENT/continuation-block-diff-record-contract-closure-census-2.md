# Block Diff Record Contract Closure Census

A root-only DslRecord derive does not establish the complete contract. Block2dDiff references thirteen nested diff records (author list and four delta/patch-entry/patch triples), plus its owned Artifact and existing shared/presentation fields. The file declares fifteen records including unused StringList. 3D declares nineteen and 5D eighteen, including their own extra list/delta records. All logical fields must remain intact; derive count alone is not an admission law.

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

Source SHA256: fee1e950614d69fe08333a8e50c1c79238b25cebaf5d55d193d0d188a7920e2a.

- Block2dDiff: #[state(artifact)]     pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::Block2dArtifact>>,     #[state(artifact)]     pub schema: Option<String>,     #[state(artifact)]     pub node_kind: Option<BlockKindIdentity>,     #[state(artifact)]     pub presentation: Option<Block2dPresentation>,     #[state(artifact)]     pub handle_kinds: Option<Block2dHandleKindsDelta>,     #[state(artifact)]     pub handles: Option<Block2dHandlesDelta>,     #[state(artifact)]     pub compatibility: Option<Block2dCompatibilityDelta>,     #[state(artifact)]     pub attributes: Option<Block2dAttributesDelta>,     #[state(artifact)]     pub authors: Option<Block2dAuthorList>,     #[state(artifact)]     pub camera2d: Option<BlockCamera2d>,     #[state(artifact)]     pub meta: Option<BlockMeta>,
- Block2dStringList: pub values: Vec<String>,
- Block2dAuthorList: pub values: Vec<BlockAuthor>,
- Block2dHandleKindsDelta: pub added: Vec<Block2dHandleKind>,     pub removed: Vec<String>,     pub patched: Vec<Block2dHandleKindsPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block2dHandleKindsPatchEntry: pub id: String,     pub patch: Block2dHandleKindsPatch,
- Block2dHandleKindsPatch: pub replacement: Option<Block2dHandleKind>,
- Block2dHandlesDelta: pub added: Vec<Block2dHandleTemplate>,     pub removed: Vec<String>,     pub patched: Vec<Block2dHandlesPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block2dHandlesPatchEntry: pub id: String,     pub patch: Block2dHandlesPatch,
- Block2dHandlesPatch: pub replacement: Option<Block2dHandleTemplate>,
- Block2dCompatibilityDelta: pub added: Vec<BlockCompatibilityRule>,     pub removed: Vec<String>,     pub patched: Vec<Block2dCompatibilityPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block2dCompatibilityPatchEntry: pub id: String,     pub patch: Block2dCompatibilityPatch,
- Block2dCompatibilityPatch: pub replacement: Option<BlockCompatibilityRule>,
- Block2dAttributesDelta: pub added: Vec<BlockAttribute>,     pub removed: Vec<String>,     pub patched: Vec<Block2dAttributesPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block2dAttributesPatchEntry: pub id: String,     pub patch: Block2dAttributesPatch,
- Block2dAttributesPatch: pub replacement: Option<BlockAttribute>,

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

Source SHA256: 8829ad05b50a2a65c5b2b524d01e00c6933abe044a14f56a761865bd1de5da26.

- Block3dDiff: #[state(artifact)]     pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::Block3dArtifact>>,     #[state(artifact)]     pub schema: Option<String>,     #[state(artifact)]     pub object_kind: Option<BlockKindIdentity>,     #[state(artifact)]     pub representations: Option<Block3dRepresentationsDelta>,     #[state(artifact)]     pub vortex_kinds: Option<Block3dVortexKindsDelta>,     #[state(artifact)]     pub vortices: Option<Block3dVorticesDelta>,     #[state(artifact)]     pub compatibility: Option<Block3dCompatibilityDelta>,     #[state(artifact)]     pub attributes: Option<Block3dAttributesDelta>,     #[state(artifact)]     pub authors: Option<Block3dAuthorList>,     #[state(artifact)]     pub camera3d: Option<BlockCamera3d>,     #[state(artifact)]     pub meta: Option<BlockMeta>,
- Block3dStringList: pub values: Vec<String>,
- Block3dAuthorList: pub values: Vec<BlockAuthor>,
- Block3dWindowsList: pub values: Vec<Block3dWindowView>,
- Block3dRepresentationsDelta: pub added: Vec<BlockRepresentation>,     pub removed: Vec<String>,     pub patched: Vec<Block3dRepresentationsPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block3dRepresentationsPatchEntry: pub id: String,     pub patch: Block3dRepresentationsPatch,
- Block3dRepresentationsPatch: pub replacement: Option<BlockRepresentation>,
- Block3dVortexKindsDelta: pub added: Vec<Block3dVortexKind>,     pub removed: Vec<String>,     pub patched: Vec<Block3dVortexKindsPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block3dVortexKindsPatchEntry: pub id: String,     pub patch: Block3dVortexKindsPatch,
- Block3dVortexKindsPatch: pub replacement: Option<Block3dVortexKind>,
- Block3dVorticesDelta: pub added: Vec<Block3dVortexTemplate>,     pub removed: Vec<String>,     pub patched: Vec<Block3dVorticesPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block3dVorticesPatchEntry: pub id: String,     pub patch: Block3dVorticesPatch,
- Block3dVorticesPatch: pub replacement: Option<Block3dVortexTemplate>,
- Block3dCompatibilityDelta: pub added: Vec<BlockCompatibilityRule>,     pub removed: Vec<String>,     pub patched: Vec<Block3dCompatibilityPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block3dCompatibilityPatchEntry: pub id: String,     pub patch: Block3dCompatibilityPatch,
- Block3dCompatibilityPatch: pub replacement: Option<BlockCompatibilityRule>,
- Block3dAttributesDelta: pub added: Vec<BlockAttribute>,     pub removed: Vec<String>,     pub patched: Vec<Block3dAttributesPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block3dAttributesPatchEntry: pub id: String,     pub patch: Block3dAttributesPatch,
- Block3dAttributesPatch: pub replacement: Option<BlockAttribute>,

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

Source SHA256: 19f9aa7d8e9b3bbc9d47cd505d806ab463c649d272fcc9074ad8055c644d10d5.

- Block5dDiff: #[state(artifact)]     pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::Block5dArtifact>>,     #[state(artifact)]     pub schema: Option<String>,     #[state(artifact)]     pub part_kind: Option<BlockKindIdentity>,     #[state(artifact)]     pub part_2d: Option<Block5dPart2d>,     #[state(artifact)]     pub part_3d: Option<Block5dPart3d>,     #[state(artifact)]     pub representations: Option<Block5dRepresentationsDelta>,     #[state(artifact)]     pub grip_kinds: Option<Block5dGripKindsDelta>,     #[state(artifact)]     pub grips: Option<Block5dGripsDelta>,     #[state(artifact)]     pub compatibility: Option<Block5dCompatibilityDelta>,     #[state(artifact)]     pub attributes: Option<Block5dAttributesDelta>,     #[state(artifact)]     pub authors: Option<Block5dAuthorList>,     #[state(artifact)]     pub camera2d: Option<BlockCamera2d>,     #[state(artifact)]     pub camera3d: Option<BlockCamera3d>,     #[state(artifact)]     pub meta: Option<BlockMeta>,
- Block5dStringList: pub values: Vec<String>,
- Block5dAuthorList: pub values: Vec<BlockAuthor>,
- Block5dRepresentationsDelta: pub added: Vec<BlockRepresentation>,     pub removed: Vec<String>,     pub patched: Vec<Block5dRepresentationsPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block5dRepresentationsPatchEntry: pub id: String,     pub patch: Block5dRepresentationsPatch,
- Block5dRepresentationsPatch: pub replacement: Option<BlockRepresentation>,
- Block5dGripKindsDelta: pub added: Vec<Block5dGripKind>,     pub removed: Vec<String>,     pub patched: Vec<Block5dGripKindsPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block5dGripKindsPatchEntry: pub id: String,     pub patch: Block5dGripKindsPatch,
- Block5dGripKindsPatch: pub replacement: Option<Block5dGripKind>,
- Block5dGripsDelta: pub added: Vec<Block5dGripTemplate>,     pub removed: Vec<String>,     pub patched: Vec<Block5dGripsPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block5dGripsPatchEntry: pub id: String,     pub patch: Block5dGripsPatch,
- Block5dGripsPatch: pub replacement: Option<Block5dGripTemplate>,
- Block5dCompatibilityDelta: pub added: Vec<BlockCompatibilityRule>,     pub removed: Vec<String>,     pub patched: Vec<Block5dCompatibilityPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block5dCompatibilityPatchEntry: pub id: String,     pub patch: Block5dCompatibilityPatch,
- Block5dCompatibilityPatch: pub replacement: Option<BlockCompatibilityRule>,
- Block5dAttributesDelta: pub added: Vec<BlockAttribute>,     pub removed: Vec<String>,     pub patched: Vec<Block5dAttributesPatchEntry>,     pub reordered: Option<Vec<String>>,
- Block5dAttributesPatchEntry: pub id: String,     pub patch: Block5dAttributesPatch,
- Block5dAttributesPatch: pub replacement: Option<BlockAttribute>,

## Canonical construction route

General DSL owns DslRecord, DslField, Shape and controlled typed projection/construction. Owners: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs (derive entry8, field classification257, key329, emitted methods834–857) and /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs (DslField9 and Box41). Box<T> preserves the inner DslField and charges allocation during controlled reconstruction. Option and Vec classification delegates to inner DslField; root Artifact therefore also needs a complete owned record contract. ToValue/FromValue and ArtifactSchema do not supply that contract.

Block shared types are owned by /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🧬️schema/🧱️shared/🦀️.rs and already declare DslRecord. Dimensional presentation/template records are owned in each dimensional root. Complete nested closure must be validated without copying them into foreign payloads. Diff and Artifact schema owners are each dimension’s 🧬️schema/🔺️diff/🦀️.rs and 🧬️schema/🦀️.rs; ArtifactSchema retains their five-language schema leaves.

The derive defaults DSL keys to kebab-case independently of Value camelCase names. Preserve explicit logical/wire key ownership with schema-first fixtures and explicit declared keys wherever required, rather than assuming Value rename attributes drive DSL. Record projection supports explicit owner declarations if direct deriving cannot express a facet, but must preserve full fields and refusal semantics without generic Value/JSON fallback.

## Physical facet behavior and controls

Rust text/binary facet bodies currently invoke OS Kernel diff_text!/diff_binary!, which require ordinary __dsl_spec/__dsl_to_record/__dsl_from_record. Native3’s actual fresh compiler missing-method observations are documented separately. Adding declarations cannot be counted as successful Rust runtime behavior until a fresh compiler and actual parse/print/pack/unpack execute.

Handcrafted text grammar owners are each dimension’s 🚪️io/📝️text/🔺️diff/📖️.grammar.semio; current 2D grammar admits schema header plus opaque OCTET payload. Binary normative owner 🚪️io/💾️binary/🔺️diff/📡️.protocol.semio declares magic, fixed headers/footer, payload and CRC. These authored descriptions require concrete codec agreement, not automatic inference from record derives.

Actual read-only Bun imports of all three TS text carrier parsers returned the identical opaque string and rejected a number. TS binary carrier files only alias Uint8Array. Thus observed TS carrier acceptance is not semantic Diff parse/print or binary validation. Runtime observations and source hashes are retained in generated/block-diff-record-census-2.json.

Required closure controls: full Artifact replacement and every sparse top-level field; absent versus explicit empty author/reorder lists; every added/removed/patched/reordered nested delta; patch replacement omission/presence; all nested shared/presentation fields and Unicode; unknown/duplicate keys, malformed scalar/list/record, nonfinite/out-of-range numbers; schema metadata and field order conservation; text and binary roundtrips and semantic apply/inverse consistency; language-neutral fixture comparison with independent JSON schema/Serde where applicable; cumulative depth/allocation/output/work budgets and cancellation with partial-owner retirement. Controlled General methods exist, but current OS diff macro paths call ordinary methods, so controlled codec integration remains a separate required route review.

This is a read-only census and canonical interface assessment. No source changes, complete closure compiler pass, Rust codec runtime pass, deletion admission or narrowed behavior is claimed.
