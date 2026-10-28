# IFC4 Conformance Fixture and I/O Boundary

## Scope

The IFC4 slice adds unchanged official reference samples and enforces exchange graph identity at import/export. These checks establish the declared IFC4 schema, unique positive ids, resolved nested references, and preserved STEP graph semantics. They do not establish full IFC4 EXPRESS entity arity, SELECT/type constraints, WHERE/UNIQUE rules, geometric validity, or MVD compliance.

## Source and Redistribution

The official [buildingSMART IFC examples page](https://technical.buildingsmart.org/standards/ifc/ifc-examples/) links Sample-Test-Files, now redirected to [Certification-datasets](https://github.com/buildingSMART/Certification-datasets). Both samples are from IFC 4.0.2.1 (IFC4 ADD2 TC1), ISO Spec ReferenceView_V1.2, pinned at revision 80d976a9b193a26a8e928c3e79bff67af1de68a8. Copyright buildingSMART International Ltd.; CC-BY-4.0, original LICENSE.txt retained with fixtures. The neutral JSON contract includes original paths, pinned raw URLs, SHA-256, exact byte sizes, schema, and entity counts.

## Changes

- Added official tessellation-with-individual-colors.ifc (32 records, 2593 bytes) and column-straight-rectangle-tessellation.ifc (26 records, 4594 bytes), without edits.
- Added twenty-two language-neutral boundary vectors, covering forward references, foreign/mixed/duplicate/malformed schema declarations, duplicate/zero ids, direct/nested dangling references, mandatory header presence/uniqueness/type shape, terminators, trailing garbage, and markers inside quoted text/comments.
- Applied IFC4 token-aware exchange framing validation on import, mandatory header type checks on both boundaries, and iterative nested reference traversal. Text and binary import/export now use that boundary; binary serialize returns Result so refusal can propagate.
- Added unit tests applying neutral vectors to direct, text and binary boundaries, plus official unchanged sample import/export equality.
- Extended the existing differential-ifc-4 feature with two official sample rows. The Rust subject materializes the IFC snapshot and regenerates it through the IFC4 boundary; ruststep independently reads the full input/output graph. The existing IfcOpenShell oracle materializes the same official input and independently writes its own result; all header/entity/argument semantics are compared. Existing differential mutation scenarios remain unchanged.

## Validation

The executable repository route attempted was:

```powershell
& 'C:\Users\Ueli\.bun\bin\bun.exe' run repo run '@semio-tech/stdio-ifc-rs:test' --output=stream
```

It exited 1 before Rust compilation or tests: native-build cache allocation refused per-directory Windows case sensitivity with access denied. No build files were changed. The native semio-repo test source-path command returned success without output; this is a no-op and is not evidence of test execution. IfcOpenShell/ruststep assertions are wired but remain unexecuted in this environment.

A Node file/network audit verified byte equality against both pinned upstream blobs, SHA-256, exact byte lengths, record count, uniqueness, reference closure, and path lengths (maximum 167 characters). This is a fixture provenance/static audit, not an implementation test.

```json
[
  {
    "file": "tessellation-with-individual-colors.ifc",
    "byteIdentical": true,
    "sha256": "3ab1290207d8538da9d8ebc14826b769eeb79fce7d75aaa85c35ed7a5a8dbd92",
    "bytes": 2593,
    "recordCount": 32,
    "uniqueIds": 32,
    "missingRefs": [],
    "pathLength": 164
  },
  {
    "file": "column-straight-rectangle-tessellation.ifc",
    "byteIdentical": true,
    "sha256": "58bb9b2cae96edf2c368de95f526650f3f282e4e7fc37c0065c501bdbeed00a1",
    "bytes": 4594,
    "recordCount": 26,
    "uniqueIds": 26,
    "missingRefs": [],
    "pathLength": 167
  }
]
```

## Shared Parser and IFC4 Framing

The shared generic Part21 parser remains unchanged. IFC4 now adds a local token iterator that consumes comments and quoted strings separately from structural markers. It requires all three mandatory FILE_* headers exactly once in their prescribed order, HEADER/DATA section boundaries, the complete END-ISO-10303-21 terminator, and EOF (trailing comments/whitespace are allowed). It checks mandatory header attribute types/nonempty lists before accepting the materialized graph. Unsupported extra HEADER records are refused explicitly because the model cannot retain them; accepting and silently dropping them during export would lose source information. This closes the generic parser's lax framing behavior for the IFC4 I/O boundaries without changing other STEP-family formats. A neutral case proves strings and comments containing FILE_NAME, ENDSEC and END-ISO text are harmless.

The schema owner corrected default IFC4 headers to declare IFC4. The boundary tests seed wrappers with a populated demo graph and require identical refusal policy across document/text/binary import and export. Framing-only malformed text can still be represented as a valid materialized graph by the generic parser; the neutral contract explicitly distinguishes import refusal from valid canonical export in those cases.

## Geometry Review

Read-only audit of the shared IFC4 geometry projector identified these exact concerns:

1. The mapping operator normalizes raw Axis1/Axis2/Axis3 and rejects nonorthogonal axes. IFC4 ADD2 TC1 derives operator U through IfcBaseAxis, which projects Axis1 perpendicular to Axis3 and derives Axis2 through IfcSecondProjAxis. Valid supplied nonorthogonal axes therefore require projection rather than refusal. Default global axes also fail for a supplied inclined Axis3. Source: [official IFC4 EXPRESS](https://raw.githubusercontent.com/buildingSMART/IFC4.3.x-development/master/reference_schemas/IFC4_ADD2_TC1.exp), IfcCartesianTransformationOperator3D, IfcBaseAxis, IfcFirstProjAxis.
2. Products linked through IfcRelVoidsElement are projected independently. A supported extruded host plus opening representation will display the uncut host and an opening as a positive solid, with no issue about unsupported subtraction. The geometric void relationship must be implemented or explicitly diagnosed and omitted as incomplete geometry.
3. A mapping operator can be mirrored (Axis2 sign), yielding negative determinant. Geometry vertices transform but triangles keep original winding; regenerated mesh normals then point inward. A reflected solid needs winding correction.
4. Shape product detection silently skips a missing/dangling representation reference or a reference to an unsupported product representation type; unsupported shape representations inside a product are also silently continued. If the product is intended renderable, this loses geometry without diagnostics.

These are review findings, not runtime measurements. The parent agent was notified; this slice did not edit geometry.

## Mapping Origin Verification

The shared projector initially inverted MappingOrigin. Independent IfcOpenShell v0.8.0 [C++ geometry implementation](https://github.com/IfcOpenShell/IfcOpenShell/blob/v0.8.0/src/ifcgeom/mapping/IfcMappedItem.cpp) and [Python placement utility](https://github.com/IfcOpenShell/IfcOpenShell/blob/v0.8.0/src/ifcopenshell-python/ifcopenshell/util/placement.py) both compose target transformation times the direct MappingOrigin matrix. [buildingSMART MappingOrigin documentation](https://raw.githubusercontent.com/buildingSMART/IFC4.3.x-development/master/docs/schemas/resource/IfcGeometryResource/Entities/IfcRepresentationMap.md) defines the source representation within that coordinate system. The geometry owner received this finding and is correcting the projector and neutral mapped fixture expectation.

The exact official IFC4 IfcSecondProjAxis rule uses supplied Axis2 (or global Y), subtracts its projections onto derived Z and X, then normalizes the remainder. This preserves reflected orientation and admits raw axis vectors that need projection.

## Owned Files

All paths below are relative to the IFC4 any subset unless stated otherwise.

- 🚪️io/🦀️.rs
- 🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs
- 🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs
- 🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs
- 🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs
- 🚪️io/🧪️tests/🔬️unit/🦀️.rs
- 🧪️tests/🔺️differential-ifc-4/🥒️.feature
- 🧪️tests/🔺️differential-ifc-4/🦀️.rs
- 🧪️tests/🔺️differential-ifc-4/🐍️.py
- 🧪️tests/🧫️fixtures/🏛️official/🔣️.json
- 🧪️tests/🧫️fixtures/🏛️official/LICENSE.txt
- 🧪️tests/🧫️fixtures/🏛️official/tessellation-with-individual-colors.ifc
- 🧪️tests/🧫️fixtures/🏛️official/column-straight-rectangle-tessellation.ifc
- Shared ticket/ifc4-conformance.md

No mutation/diff/build-system/AGENTS files were edited in this slice. The parent later authorized the optional-complex TypeScript schema interface correction described below. Root/v4 Rust call-site audit found only serialize_bytes and the new tests consuming the binary serialize function; both propagate/inspect Result.

## Executed TypeScript Implementation

Added shared, format-neutral Part21 TypeScript parsing/writing under the stdio contract domain and IFC4 snapshot boundary adapters in the existing TypeScript IO root/four leaves. Values retain exact u64/int64 bigints, framework-owned Binary64 identities (including negative zero), nested aggregates/typed values, complex records, escaped Unicode, and canonical binary literals. Generic parsing/writing has bounded input/work/depth and cancellation/progress callbacks. IFC4 parsing additionally validates mandatory header shape/single IFC4 schema and nested reference closure. Unsupported or unrepresentable real values are refused explicitly. The shared parser has no IFC imports or external runtime dependencies.

The checked-in source suite is IFC4 any/🧪️tests/🔬️step-io/🟦️.ts; the parent registered it with the existing repository TypeScript package router. Its assertions were also executed directly through built-in Node stripTypeScriptTypes(mode transform), preserving production imports through generated MJS. The input reproducer is ticket/ifc4-ts-check.mjs. Installed TypeScript 5.9.3 typechecked the shared parser, IFC4 boundary, four leaves and suite with zero diagnostics. Direct exported assertions all passed:

- checkOfficial: SHA-256/byte count/entity count of both unchanged official blobs and full snapshot roundtrip equality.
- checkBoundaries: all 22 language-neutral framing/schema/id/reference cases, with explicit canonical-export expectations where a malformed text frame describes a valid graph.
- checkScalars: exact u64/int64 values, complex instances, negative zero, exponent reals, doubled apostrophes, UTF16 surrogate pairs, UTF32 scalars, ISO8859 shifted characters, typed nested references, and canonical binary roundtrips/refusals.
- checkLimits: work, input length, depth, cancellation, invalid UTF8 and nonfinite export refusal.

The neutral manifest additionally contains canonical binary valid/invalid lists, matching the shared Rust is_part21_binary helper (empty 0 allowed; unused leading bits must be zero). Native IFC4 boundary validation now calls the same Rust helper on export, and a native neutral test consumes those same lists.

These are actual TypeScript source results. Rust compilation and independent IfcOpenShell/ruststep execution remain blocked at the previously recorded repository native cache allocation. No installed JavaScript IFC/STEP parser was found. Existing independent Rust/Python differential scenarios remain available for the official samples; their runtime status must not be reported as passed.

Additional owned files:

- stdio/📇️registry/🧬️contract/📐️part21/🟦️.ts
- IFC4 any/🚪️io/🟦️.ts
- IFC4 any/four text and binary IO leaves/🟦️.ts
- IFC4 any/🧪️tests/🔬️step-io/🟦️.ts
- Shared ticket/ifc4-ts-check.mjs

## Final Read-only Integration Audit

The TypeScript sparse diff/mutation/public-facade audit found no clear new regression: arguments use original positions before removal, additions use final insertion positions, inverses transport modified positions, entity removals retain original indices, and structuredClone preserves BigInt/Binary64/Binary identities. The public facade exports IFC4 parse/write/control and geometry surfaces through its own types. This was a static audit, not exhaustive independent proof of sparse composition semantics.

Framing parity is consistent across native/TypeScript boundaries: ordered mandatory headers, strict terminator/EOF, extra-header refusal, typed header fields, a single exact IFC4 schema, positive unique ids, nested reference closure, and canonical binary spelling. Shared native supplementary Unicode was found broken: the writer emits Unicode scalar values directly into X2 groups and the lexer rejects UTF16 surrogate pairs. The parent now owns fixing the generic native writer/lexer and adding neutral tests; this slice did not overwrite those peer-owned paths. TypeScript already supports X2 surrogate pairs and X4 scalars, with actual executed assertions.

Geometry findings above were initial audit observations. The geometry owner subsequently reported corrections for mapped source origin composition, projected transformation axes, reflected triangle winding, explicit opening-subtraction refusal, unsupported-representation diagnostics and referenced-only point-list vertices. Current-source static review found no obvious new borrow/type/index defect. Full IFC geometry support is not claimed: face holes/voids, unsupported curve/profile/boolean representations and relation-driven opening subtraction still require explicit issues rather than approximate complete geometry.

The source parser/adapter shares one owned Part21Control across parsing, validation, snapshot conversion and writing. Progress is monotonic over the complete operation; one work/depth budget and cancellation callback govern every phase. Completion reports the exact final work count.

## Canonical Optional Fields and Operation Control Follow-up

The final API audit found the canonical native/schema model can omit empty complex records. The TypeScript IfcEntity interface now makes complex optional, and STEP export normalizes omitted records to an empty collection. The execution owner is applying the same normalization in SQLite consumers. A permanent regression imports the unchanged official sample, removes every empty complex field, exports it and requires the same full snapshot on readback.

The phase-local counters were replaced with a shared format-neutral Part21Control. decodeIfc4 uses the same control for generic parsing, IFC header/reference validation and snapshot construction; encodeIfc4 uses one control for snapshot conversion, validation and text writing. Progress never decreases, callbacks include final exact work, and maxWork applies to the entire operation. Tests deliberately set a decode budget just above pure-parser cost and an encode budget one below observed total cost; both must refuse, which detects the previous per-phase budget loophole. Invalid NaN/infinite/negative work limits are refused.

Installed TypeScript compiler and direct source assertions were rerun after this change: all five groups passed, including checkOptionalComplexAndProgress. The source reproducer now executes that fifth group.

Additional schema interface edit authorized by the parent: IFC4 any/🧬️schema/📸️snapshot/🟦️.ts (complex becomes optional).
