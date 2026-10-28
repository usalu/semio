# Normative IFC4 Declaration Catalog

IFC4 ADD2 TC1 now has its own complete retained declaration catalog under v4/✳️any/🧬️schema/📚️catalog. Its unchanged normative IFC4.exp is adjacent, with the original full buildingSMART International Limited attribution and permission notice, explicit source.json identity, and LICENSE.md attribution.

## Exact Source Identity

Schema IFC4, release 4.0.2.1, official Standards-builds commit a8664407b63d42651e48a78a252c76d09dd6274c. SHA-256 a3e46d39a85c2b683e7167572165d74b4ff6f8ef7e7c1e7f314a4980a63c9edb. Pinned source: https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/EXPRESS/IFC4.exp

The retained catalog contains 776 entities; 397 types (207 enumerations, 60 selects, 130 defined types); 47 functions; and two global rules. It preserves explicit and inherited serialization attribute order, optionality, derived redeclarations, named/nested aggregate domains, inverse declarations, WHERE/UNIQUE source, supertype constraints, functions and global rules. This is catalog completeness; it does not claim semantic validation or an executed native EXPRESS interpreter. IFC4 catalog capability flags are all false.

## Shared Declaration Seam

IFC/📚️catalog owns the language-neutral strict declaration schema, Rust/TypeScript declaration interfaces, borrowed lookup/attribute/subtype/alias/enum/select queries and one reusable pure TypeScript EXPRESS extractor. The extractor checks source namespace, strips comments lexically, retains quoted semicolons and declaration keywords correctly, limits source work/depth and invokes original caller cancellation. The separate first-party Rust renderer emits retained native source without runtime parsing or external dependencies. The async Node generation API checks the manifest source hash before writing caller-owned outputs; generation inputs remain per-standard manifests.

IFC4 facades expose ifc4Catalog, ifc4Declaration, ifc4AttributeDeclarations, ifc4IsSubtype, ifc4Attribute, ifc4ResolveType, ifc4EnumMembers, ifc4SelectMembers and ifc4SelectAccepts. Native snake_case equivalents are mounted in the IFC4 model. The semantic inspector agent was given these paths and signatures.

IFC4x3 declaration types and its two public query functions now delegate this shared implementation through its existing public API. Its validator/population bodies and retained JSON were preserved. Native IFC4x3 data was reproduced from its unchanged existing JSON using the same renderer, retaining function/rule bodies and adding the previously missing supertypeExpression plus explicit version/source metadata. The shared extractor also reads the unchanged IFC4x3 source with 876/436/48/2 declaration counts and equivalent declared domains; its lexical comment handling removes line comments and replaces block comments with whitespace, so four retained expression strings differ only in comments/whitespace from the earlier Python extractor. No IFC4x3 catalog source namespace was repurposed as IFC4.

## Independent Evidence

The existing installed IfcOpenShell 0.9.0.post1 package includes independently maintained IFC4 declaration metadata. Actual source evaluation compared every 776 entity and 397 type name, all 1,454 documented attribute names against retained inherited/derived/inverse declarations, and 252 complete predefined-type enum domains. All agree. The compact independent facts and original metadata file hashes are checked into the permanent test fixture; no external runtime library was introduced.

The separate ifc4-catalog-oracle.py input compares compiled IfcOpenShell schema introspection for every entity's abstraction, parent, serialization order, optional and derived positions; every attribute/defined-type nested domain and bound; and every enum/select membership. It is authored but unexecuted here. Repo-routed native commands remain gated before test execution by the Windows native-build cache per-directory case-sensitivity access denial. The metadata check is not represented as native wrapper execution.

## Validation

- Permanent IFC4 catalog TypeScript suite: all 7 groups passed with zero strict diagnostics. Full source extraction equals the entire retained catalog, the renderer reproduces native data byte-for-byte, and both catalogs pass independent strict AJV declaration-schema checks.
- Neutral JSON vectors cover known/unknown namespace separation, subtype chains, inherited indices, derived redeclarations, enums, selects and aliases; Rust source regressions consume the same fixture and independent declaration facts.
- Actual async generation API reproduced retained IFC4 JSON and Rust byte-for-byte and rejected original caller cancellation.
- Shared extractor and generator APIs plus facade typechecked with zero diagnostics.
- Independent existing Rust grammar parsed shared queries, both native facades and both complete native retained data sources, and IFC4 native test source without syntax errors. This is not native compilation.

Input scripts retained in the shared ticket: ifc4-catalog-check.mjs, ifc4-catalog-generate.mjs, ifc4-catalog-oracle.py. Generated catalog4 outputs are cleaned after checks. The root owns shared ticket closure and package suite/public-root registration.
