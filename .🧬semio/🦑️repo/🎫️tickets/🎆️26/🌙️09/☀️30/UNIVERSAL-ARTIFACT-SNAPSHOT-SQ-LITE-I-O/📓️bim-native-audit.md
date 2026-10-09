# BIM Native Provider and Census Audit

October 9 read-only actual defining source. No Native compiler/target executed; no copies of checked-in Source, production edits or broad tests. Machine input `📥️inputs/bim-native-findings.json`; both paths validated <=256 codepoints/UTF16 units.

## Scalar, Shape and Budget Evidence

Provider uses a64-cell stack buffer; actual maximum DDL width55 fits. Authoritative44-table widths match reviewed scalar/entity slot layouts: axis19, top13, profile23, opening29, roof32, stair55. Entity rows have persisted id/doc/ordinal/key =24+key UTF8 bytes; ordered children id/parent/ordinal24; shared axis/top/profile only id+one actualowner16 plus kind and activepayload; null owner slots0. Hole parent24, project16+fourtext, document8+schema, phase24+text, propertyelement/set/value24+literalkey/name+activepayload. Borrowed finitef64 cost16+classlen, nonfinite8+classlen agrees queryREAL-orNULL +bitsINTEGER +classTEXT. Optional None0. No concrete cell-budget mismatch established; independent rich fixture expectation remains192rows, not an executed Native census receipt.

`PropertyValue` census at cells60 correctly reads F::Value(DslValue::Object): exactly one PascalCase tag, inner Object containing only value and exact declared scalar. Parametric enums separately read Block(Statements exactly1) with kebab keywords; scalar enum ordinal maps to domain label lengths. Property Integer checks i32; Boolean is typedBool; all5 floating property measure kinds retain distinct kind text. No generic EAV, encoded JSON/blob or derived geometry found.

## Before Typed Birth and Owned Children

Native decode at provider221 calls record decoder, semantic_cells::admit_record, then __dsl_from_record_controlled. Thus borrowed record semantic row/value census precedes typed Snapshot reconstruction. Native encode220 invokes borrowed RowWriter admission before generated record allocation. preflight217 also admits typed rows; to_sqlite_database218 uses controlled RowWriter projection. Actual public route integration determines which preflight callbacks run; unhooked provider source is not a public runtime qualification.

Empty holes retain hole parent rows; empty property element/set boundaries retain parent rows; Custom and Explicit parent kind survives empty child outline. Optional nested slope/baluster/cut_height/overrides preserve absence, with exact optional aggregate-width reads. Native f64 append retains to_bits signed64, finite query orNULL, exact class; restore uses from_bits and compares query/class, retaining signedzero through bits and NaN payload by bit construction. Full NaN-payload publicIO/runtime law remains required.

## Concrete Control Frontier

cells::map22 validates value.windows(2).any before the stepped per-entry loop at74. This full UTF8 key ordering scan can process arbitrarily many or long common-prefix keys without a native checkpoint. Move adjacent pair validation into the native.step loop, with bounded comparator checkpoints for long keys if necessary. It is source-level cancellation/performance evidence, not an executed allocation receipt.

## Default Scope Caveat

Native text parser fills missing keyed fields with F::Absent (dsl schema controlled decoding150). Census map/items require actual Map/List. Existing generated Map/List typed DslField decoding likewise requires those shapes; therefore no previously supported native omission behavior is established. Native typed default Snapshot already carries empty containers and generated record encoding emits them. Neutral Source optional24maps/defaultarrays should normalize empty before encoding; add original minimum-input law rather than silently assuming JSON omission and Native text omission share identical parser behavior. Do not treat unsupported original omitted native Map as a new census-only defect without a lawful original parser witness.

Original hook mounting, shared compiler coherence, public Text/Binary IO, third-party relations/edits, exact one-short rows/bytes and cancellation remain execution owner's scope. No Native green claim from this audit.
