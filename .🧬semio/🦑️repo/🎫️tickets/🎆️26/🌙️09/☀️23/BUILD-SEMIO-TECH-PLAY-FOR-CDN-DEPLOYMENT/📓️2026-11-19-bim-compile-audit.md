# BIM Retained Clone Compile Audit

The snapshot-map migration left two classes of incompatible Rust types. Test fixtures still supplied standard maps to retained property and classification fields. Nine inferred record types and the `PlaceElements` mutation still derived `RetainedClone` over standard map fields.

The retained-clone derive creates a `RetainedFieldCursor<FieldType>` for each field. The current value framework implements the required clone trait for its native map types. The standard collection implementation is absent. The affected BIM types now use `RetainedOrderedMap`: `OptionScope`, `ComponentValue`, `QuantityTotals`, `StoreyAnnotations`, `StructuralAnalysis`, `EffectiveProperties`, `FamilyValue`, `Diagnostic`, `DiagnosticIndex` and `PlaceElements`.

Placement command and inverse construction now collect into the retained map. Inference builders use cold insertion or collect from their local accumulation maps. Conversions at the family metrics, diagnostics export and analytical member projection boundaries preserve their existing output types. The existing classification and property fixtures construct the retained inner maps from arrays. IFC4x3 native spatial tests use `cold_insert` for site, building and storey fixtures.

Rust source parsing passed for all 29 edited Rust files using the pinned compiler's parse-only mode. A static census found no standard collections or aliases to them in BIM's remaining direct `RetainedClone` derive fields. The scoped Git diff check passed. Rust type checking and runtime tests remain unverified because the active release lane has exclusive use of the build path.

## Independent API Verification

An isolated executable uses the existing host Rust artifacts for the real retained-map implementation and `serde_json`. The probe passed borrowed entry iteration, borrowed string lookup and indexing, collection from an owned `BTreeMap` and into the public `BTreeMap` projection, nested map construction and iterator flattening, in-place quantity updates, an option grouping borrow across disjoint fields, and moving a placement map out of a record while keeping its join set. It also verified matching JSON and framework value output against the equivalent standard map and round-tripped both formats. The probe input is `bim-api-probe.rs`; its compiled artifacts were removed.

The source review checked the corresponding model builders and the diagnostics export, family metrics and analytical member projection. Those outputs still collect owned keys and values into their declared types. The retained map's source emits a sorted object with both serde and the framework value codec. No further mismatch was found in the reviewed expressions. The isolated probe exercises the map and the builder patterns with representative types. A complete BIM type check or test run still requires the normal build lane.
