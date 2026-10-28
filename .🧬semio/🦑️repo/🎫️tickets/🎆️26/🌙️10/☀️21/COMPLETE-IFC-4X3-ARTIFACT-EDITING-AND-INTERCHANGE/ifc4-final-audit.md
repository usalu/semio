# IFC4 Final Integration Audit

Read-only source audit of the IFC4 diff, mutation, STEP boundary, geometry, facade and suite registration, following the five integration reports. No source, AGENTS, build or Git state was modified. Native execution remains blocked before IFC compilation by the recorded Windows case-sensitivity allocation denial. The previously executed TypeScript suites and independent oracle checks are reported by their owners; this audit does not reclassify them as native success.

## Actionable Findings

### Optional Complex Constituents At The TypeScript Export Boundary

`IFC4 any/🚪️io/🟦️.ts:94` invokes `entity.complex.map` directly. Native `🧬️schema/📸️snapshot/🦀️.rs:129` defaults and omits empty complex lists; the snapshot JSON schema requires only id/name/args. A valid ordinary-entity snapshot from that persisted contract therefore throws in the new TypeScript STEP writer before export. The TypeScript IfcEntity interface declares complex mandatory, so the persisted and public runtime contracts disagree. Normalize omitted empty lists at the boundary and make the public contract consistent; add a neutral native-shaped omitted-complex export regression. The existing SQLite projector also assumes complex is present, so contract reconciliation should include that consumer. Parent notified.

### GraphQL Full-Width Numeric Contract Remains Stale

`🧬️schema/📸️snapshot/🔗️.graphql:17,34` uses GraphQL Int for integer values and entity identities; `🧬️schema/🔺️diff/🔗️.graphql:26,30` does the same for removed/modified identities. GraphQL Int cannot represent the u64 identities and i64 values deliberately retained by Rust/TypeScript. In particular the newly introduced indexed removal facet carries Int! for id. Parsing the GraphQL SDL verifies syntax only and does not establish numeric conformance. Define owned explicit numeric scalars or canonical string representations consistently across these facets, then exercise values above 2^31 and 2^53. Parent notified.

## Wiring And Path Checks

The public facade exports IFC4 apply/inverse/absorb, mutation diff/apply/inverse, STEP read/write/options, geometry entry points and corresponding owned types while retaining IFC4x3 and SQLite exports. The package suite router registers permanent IFC4 mutation, geometry, STEP and diff suites and retains existing IFC2x3/IFC4x3 suites. All 289 IFC4 subtree files are below 256 UTF-16 code units: maximum absolute path length is 189, the new complex before-snapshot fixture. Every explicit Rust #[path] include resolves. Relative TypeScript imports resolve after normal .ts extension resolution; the two extensionless viewer/editor imports target existing .ts modules. No missing include target or lost suite entry was found.

Ordered diff position validation, original-slot removals, exact inverse relocation and same-id reinsertion wiring were reviewed. No additional actionable algebra regression was found beyond the documented source test evidence. Mutation runtimes route through the checked central applier; complex mutation/catalog/codec routing remains present.

## Previously Documented Limits

The conformance report already records silent geometry omission when product shape references are absent or unsupported; projectControlled still continues those references, so that finding remains unresolved. Full EXPRESS/MVD validation, unsupported geometry families, nonidentity context-frame parity, retained projection resumption, richer visible progress, synchronous bounded scene serialization, native compile/test execution and actual IfcOpenShell parity remain explicit limits, not new findings. Supplementary Unicode native Part21 fixes are owned by the parent and were intentionally not duplicated.

## Follow-Up Review Of Final Fixes

The IFC4 STEP boundary now uses `(entity.complex ?? []).map`, resolving the concrete ordinary-native-snapshot export crash above. It constructs one Part21Control per complete import/export and shares that instance across parser, validation, conversion and writer; work counts no longer restart between those phases. The control reports on increasing work and final completion and retains cancellation/depth/work enforcement. This is source verification; execution evidence remains with the owning agent. GraphQL/SQLite numeric reconciliation and geometry shape-reference diagnostics were still being completed by their owners at this review point.

### Native Supplementary Unicode Review

Reviewed shared `✍️writer/🦀️.rs`, `🔤️lexer-and-parse/🦀️.rs`, shared neutral `🔡️unicode.json`, and the three appended native unit tests. The writer selects X4 for a contiguous escaped run containing any non-BMP scalar and emits exact eight-hex-digit Unicode scalars; BMP-only runs remain X2. The lexer infers Vec<u16> from its pushes, decodes the entire X2 run with String::from_utf16, rejects unpaired surrogates and uses char::from_u32 for X4 scalar validity. Terminator consumption and the error propagation expressions appear correct. The native tests use temporary parsed documents only within assertion statements; no escaping temporary borrow was found. No actionable native syntax/type error was identified by inspection. Native compilation remains unverified.

The TypeScript parser performs the equivalent high/low-surrogate pairing and scalar-range checks for all four valid and five invalid entries in the neutral fixture. Native and TypeScript writers may choose different legal directive grouping; decoded string equivalence is the appropriate comparison.

A separate pre-existing malformed-directive parity edge was identified and sent to the parent: the TypeScript S directive currently adds 128 to any UTF-16 operand, while Rust rejects operands outside ASCII. For example `\\S\\Ä` is refused natively but accepted and changed by TypeScript. Add the same operand-domain check and a neutral refusal case when reconciling strict shared string validation. This does not undermine the newly reviewed X2/X4 supplementary-scalar correction.

## Resolved After Audit

- Omitted complex constituents: TypeScript snapshot optionality, STEP writing and SQLite projection normalized; permanent roundtrip regression added.
- Integer transport precision: owned decimal-string scalar codecs and GraphQL/protobuf full-range tests now pass.
- Missing/wrong product representation diagnostics: native and TypeScript projectors updated; six neutral graph cases and executable regression pass.
- Native supplementary Unicode: writer emits X4 for non-BMP runs; X2 decoder uses checked UTF16 decoding. New native tests are unexecuted; matching neutral TypeScript/iconv-lite checks pass.
- Shifted-character parity: shared TS parser now refuses non-ASCII shifted operands; sixth malformed Unicode vector passes.

Current authoritative results and remaining scope: [ifc4-progress.md](ifc4-progress.md).
