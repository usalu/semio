# Audit — ISO 16757 (`📇️iso16757`)

**Executive summary:** ISO 16757 is the most structurally ambitious norm artifact (rich `Iso16757Snapshot` with catalogue, dictionary, geometry, selection, and Part 5 exchange state) and **does** wire `NormFamily::evaluate` → `CheckReport` end-to-end (`✏️editor/🦀️.rs:192–202` → `💡️inferences/🦀️.rs:85–186`). Compliance is nevertheless **demo-fixture theatre**: `evaluate()` hardcodes IDs and limits from `reference_fixture()` (`geom.valve.50`, `index.cv50`, `class.valve`, volume `0.003 m³`, part-number `550`, clearance `0.05 m`), ignores most snapshot fields (`exchange_process`, accessories, compositions, most geometry objects), never emits remediation, and passes on the default document even when large parts of a real catalogue would be non-conformant. Helper libraries in `🧬️schema/🦀️.rs` are solid unit-tested building blocks; the report layer is a thin, non-normative smoke test. **Not feature-complete** for coordination objective §1–3.

Associated goal: `🎯norm`.

---

## 1. Inventory

### 1.1 Snapshot / document fields (`Iso16757Snapshot`)

Source: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:15–31`, entity types in `📇️iso16757/🦀️.rs`.

| Field | Type (root) | Units / notes |
|-------|-------------|---------------|
| `catalogue` | `part_1::Catalogue` | Manufacturer product catalogue: groups, classes, series, products, variants, property definitions/values, indexes, accessories, compositions, descriptive media |
| `dictionary` | `part_4::Dictionary` | Data dictionary: subjects, relationships, properties, controlled lists, meta-subjects |
| `geometry` | `part_2::GeometryCatalogue` | CSG geometry objects, primitive registry; lengths in **m**, volumes **m³** |
| `selection` | `part_1::SelectionRequest` | Class/series filter + property constraints |
| `part_number_rule` | `part_5::PartNumberRule` | Literal / table / script |
| `part_number_inputs` | `BTreeMap<String, CatalogueValue>` | Script/table inputs |
| `script_limits` | `part_5::ScriptLimits` | `max_steps`, `max_recursion`, `timeout_ms` |
| `exchange_process` | `part_5::ExchangeProcess` | Workflow stage enum |

**Nested highlights** (`📇️iso16757/🦀️.rs`):

- `CatalogueValue`: boolean, integer, decimal, text, identifier, enumeration, controlled, quantity (with `CatalogueUnit` + `DimensionSignature`), range, null states, reference, list.
- `PropertyKind`: Static, Dynamic, Selection, External (`🦀️.rs:280–285`).
- `GeometryNode`: Primitive / Transform / Boolean / Reference; `SpaceKind`: Overall, Operation, Access, PlacementTransportation, Installation (`🦀️.rs:479–488`).
- `EditionProfile`: Part1_2015, Part2_2016, Part4_2025, Part5_2025, FullPublished (`🦀️.rs:405–415`).

`Default for Iso16757Snapshot` = `reference_fixture()` — a single demo HVAC control valve (`🦀️.rs:893–1009`).

### 1.2 Composed children / facets

- **Artifact schema:** `🧬️schema/🦀️.rs` (`Iso16757Artifact` mirrors snapshot).
- **Mutations:** 21 kinds (`🧬️mutations/🦀️.rs:82–104`) — document scalars, selection, part-number, catalogue naming, product groups/products CRUD, property definitions, dictionary subjects. **Deferred:** product_classes, product_series, product_indexes, accessories/compositions, dictionary relationships/properties/controlled_lists, **entire geometry catalogue** (`🧬️mutations/🦀️.rs:12–15`).
- **Inference schema:** `Iso16757Outline` only (`💡️inferences/🦀️.rs:20–22`).
- **IO:** JSON/DSL/pack codecs (`📸️snapshot/🦀️.rs`, `🚪️io/`).
- **Oracle:** mutation-only Python second implementation (`🔮️oracles/🔣️.json`); **no evaluate/compliance oracle**.

### 1.3 `evaluate()` call graph

Entry: `💡️inferences/🦀️.rs:85` `evaluate(document) -> CheckReport`.

```
evaluate
├── part_1::validate_catalogue_structure(catalogue)          → clause 1 §3.1
├── part_1::select_products(catalogue, selection)            → clause 1 §4.2
├── part_1::resolve_bim_embedding(catalogue, "index.cv50", hardcoded params) → 1 §10  [FIXTURE IDs]
├── geometry.objects.get("geom.valve.50")                    [FIXTURE ID]
│   ├── part_2::evaluate_bounding_box(shape, geometry)
│   ├── part_2::project_step_entity(geom, bbox)            → 2 §7.4 (string contains IFCBOUNDINGBOX)
│   ├── part_2::validate_geometry_graph(geom, geometry)    → 2 §6.1
│   └── hardcoded BoundingBox::from_size(0.15,0.20,0.10) vs install space → 2 §5.3.5
├── part_4::validate_dictionary(dictionary)                  → 4 §4.3
├── part_4::filter_controlled_values(first_list, "subject.valve", dictionary) → 4 §6.3.2 [FIXTURE]
├── part_4::to_iso12006_mappings(dictionary)                 → 4 §5.1
├── part_5::build_ifc_catalogue + validate_exchange + export_ifc_step → 5 §6.1
├── part_5::calculate_part_number(rule, inputs, runtime)    → 5 §6.10 (expected 550.0 hardcoded)
└── runtime.execute("1/(0)", …)  NOT document script        → 5 §8
```

**`NormFamily` binding:** `✏️editor/🦀️.rs:192–202` `Iso16757Family::evaluate` delegates to above. **Exists and is tested** (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:181–185`).

**Helpers implemented but NOT reached by `evaluate()`:**

| Helper | Location | Used only in |
|--------|----------|--------------|
| `part_2::substitute_parameters` | `🧬️schema/🦀️.rs:351` | unit tests (`⚖️compliance/🦀️.rs:185`) |
| `part_4::resolve_property` | `🧬️schema/🦀️.rs:491` | unit tests (`⚖️compliance/🦀️.rs:366`) |
| `part_1::evaluate_constraint` | `🧬️schema/🦀️.rs:230` | via `select_products` only |
| `document.exchange_process` | snapshot field | mutations/diff only — **never read in evaluate** |

### 1.4 Editor / viewer

| Surface | Path | Capability |
|---------|------|------------|
| Inputs | `✏️editor/…/📥️inputs/🦀️.rs:19–21` | **Whole-document pretty JSON** via `render_document_json` — no field-level forms |
| Results | `✏️editor/…/📊️results/🦀️.rs:22–24` | Virtualized check list via shared `render_report` |
| Inspection | `✏️editor/📌️panels/🔍️inspection/🦀️.rs:19–20` | Single check: clause, status, utilization, **English message only** |
| Artifact summary | `✏️editor/📌️panels/🗿️artifact/🦀️.rs:18–19` | Count + worst utilization |
| Catalogue panel | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:3–5,20–22` | **Placeholder** headline |
| Viewer report | `👁️viewer/…/📊️report/🦀️.rs:30–32` | Table: Clause, Status, Utilization, Message (`🖥️app-surface/🦀️.rs:159–166`) |
| Commands | `setSnapshot`, `evaluate`, `setSelectedCheckIndex`, `setActiveExample` | Localized labels en+de in manifest (`✏️editor/🦀️.rs:249–252`) |

### 1.5 Examples / assets / tests

- **Example:** one `demo` — DSL asset `🖼️assets/🎬️demo/🗣️.dsl.semio` (= `reference_fixture` output) (`📚️examples/🎬️demo/🦀️.rs`).
- **No** dedicated compliant vs non-compliant catalogue examples.
- **Tests:**
  - `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — **strong** helper tests with numeric assertions (bbox `0.003`, part-no `"550"`, constraint ops).
  - `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:5–16` — **weak**: `!is_empty()`, clause prefixes, `550.0` only.
  - `🧪️tests/📇️mutate-iso16757-1/` — mutation fixtures (21 vectors), not compliance.
  - Editor unit tests: wire format, `NormFamily` id, report JSON export.

---

## 2. Stub / fake detection

| Issue | Evidence | Severity |
|-------|----------|----------|
| Hardcoded fixture IDs in evaluate | `"geom.valve.50"` `🦀️.rs:111`; `"index.cv50"` `💡️inferences/🦀️.rs:102`; `"subject.valve"` `💡️inferences/🦀️.rs:147`; `"class.valve"` expected match `💡️inferences/🦀️.rs:96` | **Critical** — checks skip arbitrary user catalogues |
| Hardcoded numeric limits | volume limit `0.003` m³ `💡️inferences/🦀️.rs:116`; part-no expected `550.0` `💡️inferences/🦀️.rs:168`; clearance `0.05` m `💡️inferences/🦀️.rs:136–140`; product bbox `0.15×0.20×0.10` `💡️inferences/🦀️.rs:135` | **Critical** — not ISO limits, demo geometry |
| Synthetic script check ignores document | `runtime.execute("1/(0)", …)` `💡️inferences/🦀️.rs:177` vs actual rule `dn * 10 + 50` in fixture | **High** — can pass while user script is broken |
| `check_count` encodes pass as `actual==1` else `2` | `💡️inferences/🦀️.rs:81–82,90,146,158` | **Medium** — odd utilization semantics |
| IFC pass = substring search | `step.contains("IFCPRODUCT")` `💡️inferences/🦀️.rs:160` | **High** — trivially satisfiable |
| Controlled-value pass hardcodes `50.0` | `💡️inferences/🦀️.rs:148–149` | **High** — not tied to selection/catalogue |
| `AnnexChoice::En` only | `💡️inferences/🦀️.rs:87` | **N/A for ISO** — no DE-NA divergence (see §3 note) |
| `exchange_process` never evaluated | field in snapshot; zero refs in `evaluate` | **High** — Part 5 workflow unconstrained |
| Geometry evaluated for one object only | single `get("geom.valve.50")` | **Critical** |
| `substitute_parameters` / `resolve_property` dead in report path | §1.3 | **Medium** |
| Catalogue panel placeholder | `📚️catalogue/🦀️.rs:3–5` | **Low** (UX) |
| 27× `📌️.empty.md` taxonomy placeholders | editor/viewer tree | **Info** — structural, not logic stubs |
| Default snapshot always passes | compliance-report test assumes pass on `default()` | **High** — no failing fixture in evaluate tests |
| `calculate_part_number` Script branch ignores `document.script_limits` | uses `ScriptLimits::default()` inside `🧬️schema/🦀️.rs:644` | **Medium** |
| `project_step_entity` is minimal bbox stub | `🧬️schema/🦀️.rs:438–439` — not real Part 2 geometry export | **High** |
| `build_ifc_catalogue` / `export_ifc_step` simplified | flat nodes, no geometry/properties in IFC `🧬️schema/🦀️.rs:650–693` | **High** |
| Clause IDs vague vs norm | e.g. `1 §10` for BIM embedding — Part 1 has no obvious "§10" conformance rule in exchange sense | **Medium** |
| Checks that can never fail on default doc | mapping pass if `!mappings.is_empty()` `💡️inferences/🦀️.rs:152`; STEP projection if bbox succeeds | **High** |

No `todo!()` in evaluate path. No DE annex copy-paste issue (ISO 16757 is international; DIN EN ISO adoption does not alter data rules).

---

## 3. Complete subject definition (engineering)

**Assessment subject:** a **manufacturer's electronic product catalogue** for building services (TGA/HVAC-style), plus its **dictionary**, **geometry catalogue**, and **exchange artefacts**, evaluated for conformance to ISO 16757 Parts 1, 2, 4, 5.

**Not applicable:** structural/thermal design rules, national annex numeric factors (unlike Eurocodes). Germany uses **DIN EN ISO 16757** — same information requirements; plugin should treat **one profile** unless explicitly modelling CEN/ISO edition flags via `EditionProfile`.

### 3.1 Domain graph

```
Manufacturer
 └── Catalogue (id, metadata, edition_profile, lifecycle)
      ├── DictionaryRef → Dictionary (Part 4)
      ├── ProductGroup[] → ProductClass[] (tree) → ProductSeries[] → Product[] → ProductVariant[]
      ├── PropertyDefinition[] + PropertyValue[] (per product/variant/series)
      ├── ProductIndex[] (selection / search)
      ├── AccessoryRelationship[] (per host product)
      ├── CompositionRelationship[] (hasPart)
      └── DescriptiveObject[] (media)

Dictionary (Part 4)
 ├── Subject[] (typed: productGroup, productClass, …)
 ├── Relationship[] (isSubtypeOf, hasPart, hasBlock, …)
 ├── DictionaryProperty[] + ValueConstraint[]
 └── ControlledValueList[] (context-filtered)

GeometryCatalogue (Part 2)
 └── GeometryObject[] (shape CSG, symbolic, spaces, surfaces, ports, parameter_bindings)

Exchange session (Part 5)
 ├── ExchangeProcess stage
 ├── PartNumberRule + inputs
 ├── ScriptLimits
 └── derived: IfcCatalogue / STEP export
```

### 3.2 Mandatory conformance themes (what a complete checker must cover)

**Part 1 — Concepts / catalogue model**

- Unique stable identifiers across catalogue entities.
- Referential integrity: `series.class_id`, `product.series_id`, `variant` parameters ⊆ `ParameterDomain`, required class properties present with correct `PropertyKind` and units.
- Multilingual `Names` (≥1 `preferred` locale; dictionary linkage via `dictionary_subject_id` / `dictionary_property_id`).
- Selection: constraints resolvable, non-ambiguous when required, explanations on failure.
- BIM embedding: resolved variant, frozen parameters ∈ domains, geometry resolution chain (variant → series).
- Composition DAG acyclic; accessory cardinality.

**Part 2 — Geometry**

- Valid CSG tree (no dangling `Reference`, no cycles) — helper exists.
- Primitive parameters complete; unit consistency (SI m).
- `SpaceEnvelope` kinds and bounds enclose operational geometry + required clearances (installation, access, transport) per product class rules.
- `PortDefinition` positions/directions; medium/type vocabulary.
- Parameter bindings resolve to catalogue property IDs.
- Exportable symbolic vs detailed shape per LOD policy.

**Part 4 — Dictionary**

- Acyclic `isSubtypeOf`; dangling relationship endpoints — helper exists.
- Property applicability vs subject closure; controlled lists filtered by context.
- Value constraints (min/max, enumerations) enforced on catalogue property values.
- ISO 12006-3 mapping completeness for published subjects.

**Part 5 — Exchange**

- Exchange-process-appropriate mandatory content (create/provide/determine/integrate/exchange).
- Part-number determinism from declared rule + inputs; script sandbox limits from `script_limits`.
- IFC/STEP syntactic validity, entity coverage matching catalogue counts, geometry linkage.
- External media checksums / languages where required.

### 3.3 Valid ranges (representative)

| Quantity | Typical unit | Validity |
|----------|--------------|----------|
| Geometry lengths | m (SI) | > 0 for extents |
| Nominal diameter (HVAC) | mm in catalogue, m in geometry | dictionary min/max (e.g. 15–300 mm in fixture) |
| Clearances | m | class-specific minimums from Part 2 space rules |
| Script steps/time | counts / ms | user `script_limits` |

---

## 4. Check catalogue

### 4.1 Currently implemented (as coded)

| Part | Clause (as coded) | Verified | Subject inputs used | Limit source | Failure meaning |
|------|-------------------|----------|---------------------|--------------|-----------------|
| 1 | §3.1 | Non-empty products; series refs; no composition cycle; non-empty property ids | `catalogue` | implicit ≥1 product | Structure issue strings |
| 1 | §4.2 | Selection match count vs expected | `catalogue`, `selection` | **hardcoded** 1 if `class.valve` | Ambiguity / count |
| 1 | §10 | BIM embedding has geometry | **fixed** `index.cv50`, dn=50 | bool | Missing geometry |
| 2 | §6.1 | Geometry graph issues | **only** `geom.valve.50` | — | Graph errors |
| 2 | §7.1 | Bbox volume utilization | primitive params | **0.003 m³ fixture** | Volume ≠ demo |
| 2 | §7.4 | STEP string has IFCBOUNDINGBOX | projected stub | substring | No projection |
| 2 | §5.3.5 | Installation clearance | **hardcoded** product bbox | 0.05 m clearance | Overlap |
| 4 | §4.3 | Dictionary structure | `dictionary` | issue count | Cycles/dangling rels |
| 4 | §6.3.2 | Controlled value "50" allowed | **first list**, `subject.valve` | contains "50" | Filter fail |
| 4 | §5.1 | ISO 12006 mapping exists | subjects | ≥1 mapping | Empty dict |
| 5 | §6.1 | IFC product count / STEP | `catalogue` | count match | Exchange issues |
| 5 | §6.10 | Part number numeric | `part_number_rule`, `inputs` | **550.0** | Script/table fail |
| 5 | §8 | Division by zero in **synthetic** script | not document script | reject `1/(0)` | Script guard |

### 4.2 Required but missing (normative gaps)

| Part | Topic | Should verify |
|------|-------|---------------|
| 1 | Multilingual names | Each `Names.preferred` has allowed locale set |
| 1 | Required properties | For each `ProductClass.required_property_ids`, every product/variant in class |
| 1 | Unit/dimension | `CatalogueValue::Quantity` matches `PropertyDefinition.unit.dimension` |
| 1 | Parameter domains | Variant parameters ∈ allowed_values |
| 1 | Accessories / compositions | Cardinality, compatibility conditions |
| 2 | All geometry objects | Not just demo id |
| 2 | `substitute_parameters` + bindings | Parameter-driven geometry matches catalogue |
| 4 | Property value vs `ValueConstraint` | min/max on catalogue values |
| 4 | `resolve_property` cross-check | Catalogue `dictionary_property_id` exists |
| 5 | `exchange_process` | Stage-specific mandatory fields |
| 5 | Real IFC schema | Not flat `IfcProduct` list stub |
| 5 | Script limits from document | `script_limits` on user rule execution |

---

## 5. Remediation strategy (target behaviour)

Core `CheckResult` has no remediation field (`⚖️compliance/🦀️.rs:138–145`) — Wave B must add localized `remediation: LocalizedText` + `subject_ref: SubjectRef`.

| Check (target) | Remediation pattern | Fields to change | Compute target |
|----------------|--------------------|------------------|----------------|
| Missing required property `prop.dn` on variant `variant.50` | "Add property `prop.dn` = 50 mm to variant `variant.50` (class `class.valve` requires it)" | `products[].variants[].property_values` | Copy from class `required_property_ids` |
| Selection ambiguity | "Remove duplicate index `index.cv50.dup` or tighten constraint on `prop.dn` to a single variant" | `product_indexes`, `selection.constraints` | Reduce matches to 1 |
| Constraint fail `prop.dn` | "Change `selection.constraints[0].value` from 999 to 50, or add variant with dn=999" | `selection` or catalogue | Value ∈ variant property |
| Composition cycle | "Break cycle: remove composition `product.cv` → `product.cv`" | `catalogue.compositions` | DAG |
| Geometry unresolved ref | "Add geometry object `geom.x` or fix `Reference.geometry_id` on `geom.valve.50`" | `geometry.objects` | ID exists |
| Installation clearance | "Increase `spaces[Installation].bounds` min extent to ≥ product_bbox + 0.05 m on each axis" | `GeometryObject.spaces` | `install.min[i] ≤ product.min[i]-0.05` etc. |
| Dictionary dangling rel | "Create subject `target_id` or delete relationship `rel.id`" | `dictionary.subjects/relationships` | endpoint exists |
| Controlled value violation | "Set `prop.dn` to one of [50, 80, 100] mm for subject `subject.valve`" | variant property | ∈ filtered list |
| Part number mismatch | "Adjust input `dn` to satisfy script `dn*10+50` = desired article no" | `part_number_inputs` | invert script |
| IFC count mismatch | "Export missing products: add IFC nodes for …" | export pipeline | `ifc.products.len() == catalogue.products.len()` |
| Script timeout | "Raise `script_limits.timeout_ms` from 50 to ≥ estimated, or simplify `part_number_rule`" | `script_limits` | measured runtime |

---

## 6. Report & UX gaps

| Requirement | Status |
|-------------|--------|
| Per-clause pass/fail | **Partial** — table shows status + message |
| What complies / doesn't | **Weak** — messages are short English strings, no grouping by catalogue entity |
| How to comply | **Missing** — no remediation |
| Localized en + de | **Partial** — chrome localized; **check messages English only** |
| Edit all subject fields | **No** — JSON blob only; mutations cover ~30% of schema |
| Readable report | **Adequate** for list; inspection panel English-only |
| `exchange_process` visible in workflow | Stored but not assessed or guided |

---

## 7. Target design

### 7.1 Snapshot (sketch — extend, don't shrink)

```rust
pub struct Iso16757Snapshot {
    pub catalogue: Catalogue,
    pub dictionary: Dictionary,
    pub geometry: GeometryCatalogue,
    pub selection: SelectionRequest,
    pub part_number_rule: PartNumberRule,
    pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    pub script_limits: ScriptLimits,
    pub exchange_process: ExchangeProcess,
    // NEW: assessment config
    pub assessment_profile: AssessmentProfile, // edition + optional BIM index override
}
```

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &Iso16757Snapshot) -> CheckReport {
    let mut r = CheckReport::default();
    check_catalogue_integrity(&doc.catalogue, &mut r);
    check_dictionary(&doc.dictionary, &doc.catalogue, &mut r);
    for obj in doc.geometry.objects.values() {
        check_geometry_object(obj, &doc.geometry, &doc.catalogue, &mut r);
    }
    check_selection(&doc.catalogue, &doc.selection, &mut r);
    check_bim_embedding(&doc.catalogue, doc.assessment_profile.bim_index_id(), &mut r);
    check_exchange_process(doc.exchange_process, &doc.catalogue, &doc.geometry, &mut r);
    check_part_number(&doc.part_number_rule, &doc.part_number_inputs, doc.script_limits, &mut r);
    r
}
```

Each `check_*` emits `CheckResult` + remediation with `SubjectRef { path: "catalogue.products[0].variants[0]", field: "prop.dn" }`.

### 7.3 Example subjects

1. **Compliant:** extend `reference_fixture()` — already near-compliant; add explicit `exchange_process` checks + second product to prove generality.
2. **Non-compliant (`catalogue-broken`):** duplicate indexes (ambiguity), missing `prop.dn` on variant, composition cycle, dangling geometry ref, `part_number_inputs dn=40` (expect 450 not 550), `script_limits` too low for real script.

### 7.4 Numeric worked tests (evaluate level)

```rust
// derive: bbox 0.15×0.20×0.10 = 0.003 m³
let vol = evaluate(&fixture()).find("2","7.1").computed; assert_eq!(vol, 0.003);
// part number: 50*10+50 = 550
let pn = evaluate(&fixture()).find("5","6.10"); assert_eq!(pn.computed, 550.0);
// failing: dn=40 → 450, utilization > 1 vs limit 550
let mut bad = fixture(); bad.part_number_inputs["dn"] = dec(40.0);
assert!(evaluate(&bad).find("5","6.10").is_fail());
```

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Evaluate + checks | `🧬️schema/💡️inferences/🦀️.rs`, split into `checks/part_{1,2,4,5}.rs` under inferences |
| Helpers | `🧬️schema/🦀️.rs` — wire `substitute_parameters`, property constraint validation |
| Core remediation | `⚖️compliance/🦀️.rs`, `🖥️app-surface/🦀️.rs` |
| Mutations | geometry, classes, series, indexes, relationships |
| Editor | structured forms replacing JSON dump; catalogue panel |
| Examples | `📚️examples/` compliant + non-compliant |
| Tests | `💡️inferences/🧪️tests/🔬️compliance-report/`, new oracle vectors for evaluate |
| Oracles | `🔮️oracles/🔣️.json` — compliance capability |

---

## 8. Risks / open questions

1. **Scope of Part 2 fidelity:** Full CSG→STEP/IFC vs bbox stub — need product decision on geometry oracle (Open CASCADE class tool vs in-repo).
2. **EditionProfile enforcement:** Should checks branch on `Part4_2025` vs `Part1_2015` rules?
3. **Third-party reference:** Oracle manifest admits no external ISO 16757 validator — is a reference implementation (e.g. buildingSMART sample files) required for Wave D?
4. **Selection semantics:** Is ambiguity always failure, or warning per exchange stage?
5. **VDI 3805 overlap:** Coordinate property dictionaries to avoid duplicate catalogues in `🎯norm`.
6. **JSON editing UX:** Acceptable interim if mutations reach 100% schema coverage?
7. **Performance:** Whole-catalogue evaluate may be large — need incremental/check caching (inference schema currently only `outline`).

---

## Appendix — `NormFamily::evaluate` confirmation

| Item | Present | Location |
|------|---------|----------|
| `trait NormFamily` | Yes | `⚖️compliance/🦀️.rs:533–538` |
| `Iso16757Family` impl | Yes | `✏️editor/🦀️.rs:192–202` |
| `CheckReport` output | Yes | `💡️inferences/🦀️.rs:85` |
| Viewer/editor consume | Yes | `👁️viewer/…/report/🦀️.rs:31`, `NormHost::report()` |

**Verdict:** plumbing exists; **semantic compliance assessment does not.**
