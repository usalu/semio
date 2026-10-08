# Record Finite Constructor Authority Matrix Audit 6

Fresh read-only physical constructor audit. No implementation or native tests by this agent. Current Record impls still omit exact birth/support; proposed formulas below match actual constructors or explicitly identify required canonical refactors.

Let B(x) be defining child retirement_birth_bytes; L(T) leaf_birth_bytes T; D(T) deferred_birth_bytes T; S([b]) sequence_birth_bytes with exact field scaffold sizes. Eager Sequence birth is S(child B values); deferred Sequence birth is S(D field types). Both include Sequence Box and vector slots. Deferred child later publishes B(x) before constructing its real cursor. The macro artifact_retirement_sequence currently is eager; artifact_retire_struct is deferred and unconditionally marks support true, so its use requires a complete finite family audit rather than an assumed generic proof.

| FieldValue variant | Actual root constructor/birth |
|---|---|
| Bool,Int,UInt,Float,Enum,Absent | Existing implementation discards inline scalar and creates leaf unit; exact L(unit). Do not invent a scalar copied-byte oracle inconsistent with this constructor. |
| Text | String Bytes cursor, B(text). |
| Bytes64 | Vec u8 Collection, B(bytes). |
| Tuple,List | Vec FieldValue Collection, B(items). |
| Record | RecordValue delegation, B(record). |
| Block | Box FieldValue BoxedOwner, B(box). |
| Statements | Vec(String,RecordValue), B(items); tuple deferred child authorities. |
| Map | Vec(String,FieldValue), B(items). |
| Value | Now-certified General DslValue ValueRetirement, B(value). |
| Wire | WireValue sequence; exact field scaffold formula below. |
| Expr | ExprValue variant formula below. |

All seventeen alternatives are covered. A finite audited FieldValue support implementation must not recursively ask Box FieldValue or Vec FieldValue support and loop back at runtime. Exact variant births still delegate each live child; unsupported variants must remain explicit failures until their whole family is audited.

Shape twenty-seven variants: Enum delegates Vec(String,u32); Tuple/List/Block/Map delegate Box Shape; Record/Table delegate RecordSpecProducer leaf; Statements delegates Vec(String,RecordSpecProducer). Remaining Bool/Int/UInt/Float/Text/Bytes64/Value/Wire/Quantity/Angle/Ref/Coord/Dir/Dim/Range/Count/Expr/Embed/EmbedFrom currently construct leaf unit. Quantity/Angle static UnitSpec and Ref/Embed static strings do not own their referenced memory. Match these twenty-seven alternatives exhaustively in birth code rather than wildcard missing-new-variant authority. Recursive family support must be finite.

Expr five variants: Num L(unit); Var B(String); Neg B(Box Expr); Binary current eager S([B(left),B(right)]) ignoring inline op; Call current eager S([B(name),B(items)]). Preferred canonical deferred constructors for Binary/Call use S([D(Box Expr),D(Box Expr)]) and S([D(String),D(Vec Expr)]) with exact child demand on subsequent transitions. Constructor and formula must change together.

WireNode current eager S([B(id),B(kind),B(port)]) (String,Option String,Option String). WireEdgeLabel current eager S([B(id),B(kind)]) (two Option String). WireValue current eager S([B(from),B(edge),B(edge_label),B(properties)]) (WireNode,Option(bool,WireNode),WireEdgeLabel,DslValue). Switching to deferred struct construction permits exact S(D types) at root and preserves later child admission; no child payload clone. All144 optional/direction combinations must share unchanged literal values and preserve exact empty capacities.

FieldSpec current eager S([B(key),B(shape)]); RecordSpec eager S([B(keyword),B(fields)]). Static/non-owning fields do not need retired heap payload, but use a deliberate exact scaffold list. RecordSpecProducer macro leaf already has exact L(RecordSpecProducer) and finite support.

RecordFields physically contains Vec(u16,FieldValue) in defining record/fields Rust. Current retire delegates self.into_iter, which loses truthful iterator backing authority. Canonical fix should own the original entries Vec directly from that defining module; root birth delegates Vec, support depends on the audited FieldValue family. Do not materialize a new Vec or sorted mirror. RecordValue delegates exact RecordFields. Test reserved-empty RecordFields via from_empty_slots and retained actual capacity.

Writer next cut must replace its erased retiring field with a typed child/spec retirement sum whose variant owns ControlledRetirement RetainedRecordWriter or RecordSpec directly. Exact sum root birth follows the actual typed constructor; child native full grants preserve independent copy/capacity/release/depth. Writer retirement needs all fields source/spec/order/text/intrinsic frames/path/output/compound/wire/nested/child_spec and live typed retirement owner; adopt exact deferred scaffold or exact tuple constructor without Vec push growth. Projection RetainedFieldProjection current eager four-field scaffold also lacks authority; frames/path/complete/fault_value all need exact typed families before support.

Terminal witnesses: String/Vec require original backing capacity released; Box requires original Box allocation release then child drain; deferred/sequence require real scaffold release; direct ControlledRetirement requires all original paged frontier backing empty and terminal Box release; no len0 substitute. Current certified DslValue/Box/Vec owners provide these under full granted port, while old iterator/erased families remain outside certified scope.

Neutral test schema should fix exact constructor rosters17/27/5/144 and vector budgets, reserved capacities, typed wrapper prefixes, cancellation/depth admission and exact undergrant nonmovement. Trace actual allocation observer verifies constructor/terminal formulas; original Serde/SQL value/text oracles remain unchanged. This is source-only preparation; Record success remains unproved.
