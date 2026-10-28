# IFC2x3 TC1 Catalog Source and Independent Audit

Read-only audit; no production changes. Retained exact official source official-IFC2X3-TC1.exp is257080bytes; SHA256 bf89eda341bccec041df37dffb79f0b8d4e3d5411d07541560d05bf69d53fc3a. SCHEMA is IFC2X3. Header identifies IFC2x Edition3 Technical Corrigendum1, EXPRESS longform, compiled for EXPRESS version1 technical corrigendum2, issue July10,2007. This is version-specific schema metadata for IFC2x3 surfaces, not IFC4-derived metadata.

## Provenance and License Notices

Official pinned source: https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC2x3/TC1/EXPRESS/IFC2X3_TC1.exp . Official publication links are embedded in independent metadata under https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ . Pinned repository README: https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/README.md .

The original file header says copyright buildingSMART International Ltd.1996–2020 and retains an all-rights-reserved reproduction notice. The pinned repository README separately states final IFC is CC BY-ND4.0. Record this distinction accurately; preserve the unchanged original header and add adjacent attribution/provenance referring to the repository final-release license. Do not claim the original EXPRESS header itself contains the IFC4 software-use permission language. Derived catalog should retain exact normative expressions and source identity; do not alter the normative input.

## Independently Counted Declarations

653entities;327types comprising164enumerations,46selects,117defined types;38functions;2global rules. Counts were read from unchanged source after lexical comments removal. Declaration namespace IFC2X3 is deliberately distinct from IFC4 and IFC4X3_ADD2. Same extractor/query contracts can serve each version with its own source and manifest; capability flags must not imply a complete EXPRESS runtime validator merely because declarations are complete.

## Actual IfcOpenShell Metadata Comparison

Existing installed ticket oracle package IfcOpenShell0.9.0.post1 contains independent util/schema/ifc2x3_entities.json and ifc2x3_types.json. Actual Node filesystem evaluation compared all653entity names and all327type names against official declaration names: exact set equality. All1402documented attribute names appear in corresponding normative entity declarations, including derived/inverse names. All88complete documented predefined-type domains match the corresponding normative PredefinedType enum, resolving inherited PredefinedType when needed. Zero failures across these comparisons.

Independent metadata SHA256:

- ifc2x3_entities.json:48d4fda95f576e9d45bb621906a024ecaa8fbf47a74fad6efa52a58bd1f6eb4e
- ifc2x3_types.json:9a52b35a4e6c4def1ac010efa33d0ac8d2d59694bc91eaec6f8bdf524ac1c02c

This mirrors the actually executed IFC4 metadata method documented in ifc4-catalog.md. It does not execute compiled IfcOpenShell schema introspection. Metadata descriptions omit many attributes and are not an oracle for serialization order, optionality, aggregate bounds, derived slot flags, abstraction or all enum/select domains. The separate existing ifc4-catalog-oracle.py demonstrates the fuller public wrapper comparison: schema_by_name('IFC2X3'), declaration_by_name, all_attributes, derived, supertype, is_abstract, enum/select lists and recursive aggregation/simple/named domains. Such a source-adjusted IFC2x3 wrapper harness would be stronger, but is not represented as executed here.

## Integration Recommendations

Use shared IFC/📚️catalog extraction and query implementation, retaining IFC2x3 own serialized catalog/native data and source manifest in base schema. Rust shared declaration nominal type must be consistent across details wrapper and catalog public API. Version metadata should remain supplied by the standard profile, never selected from entity-name guesses or an IFC4 fallback. Include neutral regressions for IFCWALL lacking IFC4 occurrence PredefinedType, IFCWALLTYPE's actual IFC2x3 enum, IFCCARTESIANPOINTLIST2D absent from IFC2x3, and inherited serialization slots. The catalog's schema identity can gate geometry's indexed-curve capability without adding cross-version model conversion.

No native compilation, Python wrapper execution or full UI execution claimed.

## Retained Independent Test Facts

ifc2x3-ifcopenshell-reference.json contains the exact independently read entity names, documented attribute-name sets, complete documented predefinedTypes domains and all type names, using the existing IFC4 reference fixture shape. implementation/version/schema/sourceFiles identify IfcOpenShell0.9.0.post1 and both exact metadata hashes above. Descriptive prose is omitted. This is an input fixture retained outside generated output; main owns production registration.

## Fresh Final Source Review

Verified current production base catalog source IFC2X3_TC1.exp remains exactly257080bytes/SHA256bf89eda341bccec041df37dffb79f0b8d4e3d5411d07541560d05bf69d53fc3a. source.json pins namespaceIFC2X3/version2.3.0.1 and official pinned mirror. NOTICE.txt accurately retains original header vs repository final-release license distinction and independent IfcOpenShell notice. Native facade's six-level common-source relative path resolves. Owned base Rust model explicitly mounts schema::catalog alongside native geometry and snapshot. Catalog JSON independently retains653entities327types38functions2rules.

Native query lifetimes are sound by source inspection: singleton OnceLock returns static catalog; borrowed declaration/attribute/enum/select results are static; resolve_type ties caller expression and catalog to one lifetime and safely shortens static catalog to expression borrow. No transient owned-name borrow escapes; case normalization is used only for lookups. Each separately source-mounted Rust catalog still defines its own nominal declaration types. That is safe for isolated IFC2x3 public facade; a future shared details consumer must deliberately share one nominal module or use profile-specific source mounting, rather than passing IFC2x3 catalog into an IFC4 nominal catalog parameter.

Fresh mapping review confirms TS/Rust now validate original exact3D aggregate dimensions for LocalOrigin and every present mapping axis; 3D placement location and directions have the same gate. Optional references distinguish unset from malformed present values, and optional scales default only missing/unset rather than Derived. Native Copy Option<u64> axes remain usable after into_iter by array Copy. Numeric scale parsing rejects nonfinite inputs before positivity. Both implementations retain exact official projected-axis defaults and reflection sense rather than inserting an unsupported handedness fallback. No new concrete mapping defect found in these changed blocks; this is static source parity, not native typechecking.
