# Nested Norm and glTF Branch Audit

Read-only source work; no builds or runtime passes. This extends top-level coverage to specific original unions and empty/nullable children. No new proven omitted legal branch was found in the structures inspected below.

## Norm Roster and EN1990

All15 selected Norm snapshot roots were compared with aggregate mounted SQLite/helper sources: EN1995(3 fields),EN1999(9),DIN4108(10),EN1998(10),EN1991(70),EN1990(20),EN1996(5),EN1994(9),EN1992(10),DIN16798(10),DIN18599(19),EN1993(17),ISO16757(8),VDI3805(8),EN1997(13). All231 top-level names occur in actual handwritten P/R helpers. This is an unresolved-field screening count, not231 independent behavioral proofs. Macro provider injection was separately verified; no missing-provider conclusion comes from literal declaration grep.

EN1990 original PermanentAction(id,kind,gk), VariableAction(id,category,qk), AccidentalAction(id,ad), SeismicAction(id,a_ek,importance_class) are explicit action entities. Its handwritten semantic root uses binary64 word/class columns and dense ordered entities. ImportanceClass I/II/III/IV has all4 write/read strings; AnnexChoice En/De has both. The complete20-field constructor restores actions,members,bridge_sls,effects independently of document scalars; no derivation replaces original action values with computed design values. Struct and implementation paths respectively `📕️norm/🗿️artifacts/⚖️en1990/🦀️.rs`, standard1/any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs. Every optional Member/BridgeSls nested field has not yet been exhaustively compared.

## ISO16757 Recursive Value Union

Original CatalogueValue has12 legal variants: Boolean,Integer,Decimal,Text,Identifier,Enumeration,Controlled(value/list_id),Quantity(value/unit),Range(min/max/optionalunit),Null(state),Reference(target_id),List(items). Actual catalogue_value and read_value match all12, with explicit active-column validation and tagged rows. NullState Unavailable/Unknown/NotApplicable is retained distinctly. Range optional unit writes nullable link and read_optional_unit restores it. A List always gets its own value row even when items empty; ordered element rows carry each child ordinal; iterative Visit/Finish reconstruction preserves nested empty list versus Null. Strings are direct semantic values, not JSON serialized alternatives.

Exact original type is `📕️norm/🗿️artifacts/📇️iso16757/🦀️.rs:106`; actual P/R is its standard1/any SQLite snapshot root (catalogue_value:20,read_value:87). Remaining runtime/custody issue is separate: reconstruction uses local stack, children.clone, Vec/BTreeMap until returned; original workspace allocation/retirement behavior must be qualified under genuine factory. No missing semantic union branch is claimed from those allocation paths.

## VDI3805 Configuration Union and Map

Original SheetAttributes has5 tags ValveHeating,Radiator,PumpHeating,HeatGenerator,Generic. Actual reconstruction covers5 and validates exactly one corresponding mandatory attribute entity for each configuration, rejects mismatched tag ownership and orphan/duplicate entity census. GenericAttribute retains key/value/optionalunit; ordered empty generic entries has its parent row and empty child list, distinct from another sheet tag. CharacteristicCurve keyed map preserves its literal key and own id; x/y unit rows retain symbol/quantitykind/delta/sifactor and require both unique axes0/1; ordered points preserve exact x/y scalar values. Duplicate curve map keys and duplicate axes reject rather than overwrite. Original type at `📕️norm/🗿️artifacts/🏭️vdi3805/🦀️.rs:1082`, actual root semantic helpers rows71–87. Other nested catalogue/function/product-record branches remain unreviewed.

## glTF Optional and Tagged Branches

Original GltfCameraProjection has Perspective/Orthographic; 🎥️camera P/R matches both. Perspective optional aspect_ratio/zfar remain optional binary64 cells, yfov/znear required; Orthographic xmag/ymag/zfar/znear required. Both projection object and parent camera retain their separate name/extensions/extras where declared. Raw JSON None versus present structured Null retains distinct nullable link versus actual JSON-value row.

📦️buffer::bound/read_bound specifically create marker row for Some(Vec), even Some(empty), and only absent marker means None; therefore accessor min/max optional empty list is preserved. Sparse marker independently distinguishes None/Some; original GltfSparseIndices contains buffer_view/byte_offset/component_type, SparseValues contains buffer_view/byte_offset, SparseAccessor contains count/indices/values; handwritten constructors cover these actual fields (do not invent missing extensions not declared by those types). Uint source indices/count/offset are high/low words rather than narrowing to SQLite signed integer. Original source is glTF standard2.0/any schema snapshot; exact helpers 🎥️camera/🦀️.rs and 📦️buffer/🦀️.rs.

## Remaining Unreviewed and Meaningful Laws

Still unreviewed nested scope: EN1991's70 original fields including every optional/load-member variant; all other Norm nested records/tag switches; ISO units/cardinality/catalogue part relationships; VDI function/record/lifecycle extension maps; glTF material/mesh/skin/animation sampler variants and all nullable lists; SVG XML lexical flags and PDF COS/navigation/resource branches. Broader statement of complete231 fields would be unsupported by this screening.

Schema-first independent fixtures should contain every inspected variant, nested empty List/None/Some(empty), all3 Null states, unit absent/present, map key distinct from id, UInt edge high/low and scalar word/class cases. Compare exact independent relational rows before real typed supplied-owner file roundtrip/canceled cleanup. No reduced decoder, narrowed legal type or opaque JSON field fixes are justified by this audit.
