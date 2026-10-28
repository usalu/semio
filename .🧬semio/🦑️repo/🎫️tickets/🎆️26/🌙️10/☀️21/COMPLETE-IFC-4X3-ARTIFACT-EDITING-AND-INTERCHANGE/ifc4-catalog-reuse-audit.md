# IFC4 Official Catalog and EXPRESS Reuse Audit

Read-only 2026-10-21; no production changes. Unchanged official input retained as [official-IFC4-ADD2-TC1.exp](official-IFC4-ADD2-TC1.exp). Do not copy this checked source into another ticket path or modify original EXPRESS bytes.

## Exact Official Provenance

- Official repository buildingSMART/Standards-builds, pinned commit a8664407b63d42651e48a78a252c76d09dd6274c.
- [Pinned raw schema](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/EXPRESS/IFC4.exp).
- Path IFC/RELEASE/IFC4/ADD2_TC1/EXPRESS/IFC4.exp (the guessed IFC4_ADD2_TC1.exp name is 404).
- [GitHub metadata](https://api.github.com/repos/buildingSMART/Standards-builds/contents/IFC/RELEASE/IFC4/ADD2_TC1/EXPRESS/IFC4.exp?ref=a8664407b63d42651e48a78a252c76d09dd6274c).
- HTTP200; 359,976 unchanged bytes; saved/read-back byte equality verified.
- SHA256 a3e46d39a85c2b683e7167572165d74b4ff6f8ef7e7c1e7f314a4980a63c9edb; Git blob SHA1 c455f1cc48997c4d672228d58ec41d12504cb685, matching official contents metadata.
- SCHEMA IFC4; release version 4.0.2.1 (IFC4 Addendum2 Technical Corrigendum1), original header issue date Sunday October29,2017.
- Counts after excluding comment text: 776 ENTITY,397 TYPE (207 enum,60 select,130 defined),47 FUNCTION,2 global RULE,1 SCHEMA. Type breakdown should be independently compared against generated extraction; regex counts are audit anchors, not a schema parser.
- Original header permits software-development use/translation with full buildingSMART International Limited attribution; changing technical specification requires prior written consent. Pinned repository README licenses final IFC under CC BY-ND4.0. Keep raw source and attribution unchanged, generate own catalog as software representation, and preserve explicit provenance.

## Actual Current Implementation (Not Reports)

All paths relative C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc unless noted.

Let X=🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema.

- X/📚️catalog/🟦️.ts owns IfcCatalog, IfcEntityDeclaration, IfcAttributeDeclaration and IfcTypeExpression. allAttributes already contains inherited Part21 serialization order plus derived redeclarations; direct attributes and derived/inverse/WHERE/UNIQUE source are retained. Query functions ifcAttributeDeclarations and ifcIsSubtype close over global ifc4x3Catalog. Structural validator is also in this same file and closes over that catalog and4x3-specific snapshot imports/error strings.
- X/📚️catalog/🦀️.rs duplicates catalog types and structural validation using the shared IFC4 model, includes retained data.rs, and builds OnceLock catalog. It is currently version-bound by ifc4x3_catalog calls. Native catalog lacks TS supertypeExpression field: preserve this if extracting common schema so entity subtype constraints are not lost.
- X/⚖️validation/parser.ts and ast.ts are nearly schema-neutral owned EXPRESS syntax. But ExpressProgram.schema is currently literal IFC4X3_ADD2; ast.schema.json should also be updated schema-first. No external runtime parser is required.
- X/⚖️validation/compiler.ts imports global ifc4x3Catalog and builds hardcoded IFC4X3_ADD2 ExpressProgram; otherwise loops retained catalog entity/type/function/global rule sources with original caller controls. This is the natural shared compiler seam.
- X/⚖️validation/evaluator.ts imports global catalog for enum symbols, identifier binding, enum attribute access, entity/type constructors and TYPEOF. TYPEOF explicitly prefixes IFC4X3_ADD2. Merely switching compiler input will produce wrong IFC4 WHERE results unless all these use injected catalog.schema.
- X/⚖️validation/population.ts imports global4x3 catalog/subtype plus4x3 snapshot/diff capture helpers. It resolves attributes, inverse/USEDIN, derived values, defined/select domains, bounds, unique and global rules. Complex instances are normalized against the complete inherited chain. Reuse this engine through a graph-admission/value reader interface; do not import4x3 snapshot into an IFC4 public API or bypass capture/admission/caller cancellation.
- X/⚖️validation/🦀️.rs includes canonical precompiled AST program.rs but explicitly IFC4X3_NATIVE_EXPRESS_EXECUTION=false. There is no complete native semantic interpreter to reuse yet. A full catalog/native AST is not native semantic validation success.
- X/📚️catalog/🔣️.json still has validation flags false for rules/functions/global/complex/binary even though newer TS interpreter has work in those areas. These metadata flags are stale relative to executable capabilities; use separate actual capability manifest per engine, not presence of retained declarations.
- X sibling ✏️editor/details.rs already supplies rich declaration-aware SnapshotDetailsProvider over lossless graph: inherited attribute presentation, derived readonly, optional templates, union/type/enum controls, reference target filtering, cardinality/uniqueness-aware reference insertion and complex constituent templates. It hardcodes ifc4x3_catalog/ifc_attribute_declarations/ifc_is_subtype and IFC4 snapshot identity. This is immediately reusable via injected catalog, not a second IFC4 control implementation.
- Existing ticket schema-build.py is the4x3 catalog extractor, hardcoded input/output/schema metadata. Its declaration parsing/type-expression/flattening logic is reusable, but it is a ticket script rather than permanent shared infrastructure. Do not copy it into a parallel IFC4 generator with another hardcoded version. Refactor into one permanent extraction module + per-standard declarative source manifests, and retain the original script as ticket input evidence.

## Short Clean Next Implementation

1. Add first-party shared IFC declaration schema under ifc/📚️catalog (or equivalent concise common domain path) defining catalog data/provenance and borrowed read interface. Parameterize query operations as attributes(catalog,name), isSubtype(catalog,actual,expected), resolveType(catalog,expr), keeping standard-specific wrappers thin. Catalog identity must include exact schema/version/source hash.
2. Lift one extractor from schema-build.py to permanent repo infrastructure with explicit input schema manifest and target paths. Generate IFC4 raw-source-adjacent catalog JSON and Rust retained data from unchanged official source, count776/397/47/2 and inherited slot order. Keep4x3 output regenerated by the same logic; no ad hoc textual renaming, source schema rewriting, duplicated generator, compatibility adapters or runtime external dependencies.
3. Parameterize the existing4x3 details provider as IfcDetailsProvider(snapshot,catalog) and instantiate per-version wrappers. Inspector semantic descriptors can derive actual attribute index/type/optional/derived from catalog allAttributes; retain explicit UI edit policy separate from declarations. An attribute being present in catalog does not automatically authorize arbitrary semantic edits.
4. Extract common EXPRESS owned AST/parser/compiler/evaluator to ifc/⚖️express with explicit Catalog argument. Suggested contracts: compileExpress(catalog,control)->ExpressCompilation; ExpressEvaluator(program,catalog,control,resolveAttribute,usedIn); validateExpress(graphReader,catalog,admission,control,program?). Program.schema is explicit schema id rather than4x3 literal. Every TYPEOF namespace comes from selected schema. Keep per-version capture/admission implementation behind own first-party graph reader interface, analogous to the just-extracted geometry interface but including full complex constituents and typed wrappers.
5. Add IFC4 wrapper exports and schema-specific neutral tests; preserve4x3 wrappers with exact same semantics. Do not claim full native validator: native stage currently only declaration catalog/AST, runtime flag remainsfalse until actual interpreter is implemented and executed.

## Required Checks

Before editor replacement, compare extraction against independent existing IfcOpenShell schema introspection for exact entity/type counts, names, inherited serialization slot order, optional/derived attributes, enum/select membership, parameterized profiles, rederived context slots and complex-chain representation. Validate pinned official IFC4 fixtures, intentionally invalid count/type/reference/profile dimensions, inverse cardinality and IFC4 TYPEOF namespace; run same shared engine cases under both catalogs to prevent namespace leakage. Any unsupported rule must remain incomplete, never a valid result. Reuse existing rule fixtures by actual enumeration rather than trusting stale report counts. Catalog completeness and compiler completeness are separate from valid model and executed native support.

The report recommends boundaries only; no extraction or validator tests were run during this read-only audit.
