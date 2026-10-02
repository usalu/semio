# Foundational Schema OS Cut Design

Read-only bounded source audit; no jobs. Prioritize schema over compiler/graph/surface: its offending dependencies are largely existing neutral source definitions reached through OS mounts, while graph adds neural value conversion and surface embeds actual OS canvas interaction.

## Actual schema authority

Schema package Cargo34 OS dependency compiles component at package Rust7. Component8 obtains StateClass through OS; actual sole definition is neutral replication/🧾️wire/🦀️.rs294. Component123 obtains ArtifactCompositionFields/ChildFieldRefs/ChildRefFields/ChildRefVisitor/ChildSlotSpec/LinkSlotSpec through OS os_schema_composition; actual source is schema/🧩️composition/🦀️.rs, mounted by OS package265–266. Component249 obtains three descriptor families, facet leaves and register/catalog functions through OS; actual definitions begin neutral replication wire331/341/360. Thus several general→OS dependencies are facade paths to already-general source, not OS-specific behavior.

Schema tests component-unit158 use ArtifactChild/ChildRestoreProjection/Error genuinely defined OS store3242/3431/3411; ArtifactRef/Dialect at test170 are actually neutral IO schema174/71 reached through OS. Those child lifecycle integration assertions must move to higher OS test composition, preserving their original bodies, while schema pure slot/visitor contract tests remain lower. Do not remove assertions or implement fake ArtifactChild.

## Coherent canonical cut

1. Create a single physical low composition package for existing schema/🧩composition source (explicit taxonomy/package contract first), directly consumed by both schema and OS store/kernel. Do not mount the same definitions twice: that would create distinct Rust trait/type identities. Rebind schema123 and OS266 to the one lower crate; preserve all actual derive-generated callers and trait implementations.
2. Rebind StateClass and schema descriptor/catalog APIs to actual neutral replication wire owner, preferably a narrow schema-catalog leaf package if replication package introduces a reverse schema dependency. Audit real dependency cycle before choosing package: source ownership alone does not prove package independence. Rename Kernel descriptor names only as one complete canonical API change if owned schema declares neutral names, no OS forwarders/legacy aliases.
3. Separate generic schema registration from actual process-global OS assembly. Component conversions249–275 already duplicate KernelFacetLeaves into FacetLeaves; unify one schema-owned contract, retaining conflict validation and registration callback semantics. Global Mutex/OnceLock catalogs are process composition concerns; explicit registry instance/caller ownership is cleaner than hidden OS catalog dependency, but requires schema-first existing caller behavior proof.
4. Move only ArtifactChild lifecycle test composition higher. Rebind neutral ArtifactRef/Dialect directly to IO schema. Remove schema OS Cargo edge after all mounted production/test contexts bind lower owners.

Original schema module-compile manifest target remains. Component unit mount871–872 and schema fixtures/schemas must remain in full collection. Capture exact original laws/hash before implementation; this read-only audit does not invent law counts. Independent portable validators and current schema catalog conflict tests should prove descriptor equivalence/conflict behavior; native lower products-absent compile must prove trait identity and full schema corpus, higher OS native tests must prove real child restore behavior.

## Other concrete paths, separate scope

Compiler package Rust5 forwards dsl_core::os_dsl; actual syntax14 consumes escape_text/lex/unescape_text/Limits/TextError/TextSpan/TokenKind. Compiler main63 exposes TextError; syntax native tests178–182 parse/print/reparse actual grammar. A complete neutral text/lexer/grammar package is coherent but larger than schema cut; grammar/derive metadata consumer authority must remain exact.

Graph package Rust11 aliases OS as dsl_core; engine56 onward uses neutral ToValue/FromValue/DslValue/ValueError already owned value. Graph DSL additionally uses OS text/JSON/wire parser, so direct value rebinding alone does not remove OS edge. Graph manifest3–4 consumes neural_engine Value/ValueType and658–661 converts PropertyValue to Atom; evaluate general neural value vocabulary separately, retaining real conversion semantics.

Surface package Rust5–6 aliases OS as dsl/store; paint/tiled-map publicly forward infinite_canvas, node-graph21–26 uses OS canvas directed DAG and DomainHover/DomainSelection/SelectionMethod/Viewport2d. These are interaction/composition dependencies needing a broader neutral canvas/domain interface cut; not the first foundational deletion proof.

Bounded next step: enumerate full composition trait callers and descriptor registry import origins using captured source graph, define closed schema/fixture contract, establish native RED then move one sole source authority. Package names must never substitute for actual role/path/mount classification.
