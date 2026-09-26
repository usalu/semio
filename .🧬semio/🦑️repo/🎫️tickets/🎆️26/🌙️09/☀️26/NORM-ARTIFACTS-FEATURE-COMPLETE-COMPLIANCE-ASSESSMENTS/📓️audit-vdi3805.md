# Audit — VDI 3805 (`🏭️vdi3805`)

**Executive summary.** The family ships a real `evaluate()` → `CheckReport` pipeline wired end-to-end through editor, viewer, and `NormFamily::evaluate`, but it assesses **infrastructure health** (sheet-registry reachability, JSON/native round-trips, index filters) rather than **VDI 3805 Part 1 + Blatt conformance**. The snapshot schema can hold a manufacturer dataset (header, products, native records, geometry, curves), yet native I/O parses only a thin slice of Part 1, sheet modules are macro stubs (identity/article-number or N/A), and no mandatory Blatt attributes, record grammars, value ranges, or referential-integrity rules from the norm are enforced. Tests prove dispatch and plumbing, not norm numbers. Feature-complete compliance requires typed record models per `RecordFamilyId`, per-sheet attribute catalogues, real Part 1 structural validation, and localized remediation-bearing reports — none of which exist today.

**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805`  
**Audit date:** 2026-09-26  
**Evaluator verdict:** `evaluate` / `CheckReport` **exists and runs**; norm assessment is **not feature-complete**.

---

## 1. Inventory

### 1.1 Snapshot / document fields (`Vdi3805Snapshot`)

Defined at `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:15–34` (mirrored in artifact schema `…/🧬️schema/🦀️.rs:15–34`).

| Field | Type (Rust) | Role | Units / notes |
|-------|-------------|------|----------------|
| `manufacturer_file` | `ManufacturerFile` | Part 1 file header mirror | `header_version`, `manufacturer`, `building_system_number` (`SYS.SUB.NNN`), `created`, `charset`, `record_count` |
| `catalog` | `ManufacturerCatalog` | Products + nested native records | `file`, `products[]`, `extensions` |
| `edition_profile` | `BTreeMap<String, EditionProfileChoice>` | Per-sheet Legacy/Current override | Keys are sheet numbers as strings (`"8"`, `"10"`, …) |
| `correction_as_of` | `EditionId` | Correction overlay cut-off | `{ year, month }` |
| `strict_mode` | `bool` | Blocks historical proposal sheets | Default `false` in `reference_fixture` |
| `index` | `CatalogIndex` | Derived search index | `entries[]` with `product_id`, `sheet`, `tags`, `dn` (optional, from `configuration.parameters["dn"]`) |
| `geometry` | `BTreeMap<String, ParametricGeometry>` | Parametric 3-D blocks | `bbox` [m], `connections[]` (`diameter_mm`), `parameters` (e.g. `scale`) |
| `curves` | `BTreeMap<String, CharacteristicCurve>` | Characteristic curves | `x_unit`, `y_unit`, `points[]`; linear interpolation |
| `limits` | `SecurityLimits` | Untrusted-input caps | `max_file_bytes`, `max_records`, `max_field_length`, `max_nesting_depth` |

**Nested product (`CatalogueProduct`, `🦀️.rs:863–876`):** `identity` (`manufacturer_code`, `product_group`, `article_number`), `title` (`Vec<LocalizedText>` de+en), `sheet: SheetId`, `records: Vec<NativeRecord>`, `configuration` (`parameters: BTreeMap<String, VdiValue>`, `geometry_ref`, `function_refs`), `accessories`, `components`, `extensions`.

**Value typing (`VdiValue`, `🦀️.rs:155–164`):** Boolean, Integer, Decimal+unit, Text, Enumeration, Range, List, Null. **`VdiUnit`** (`🦀️.rs:131–137`): `symbol`, `kind: VdiQuantityKind`, `delta`, `si_factor`.

**Record families (`RecordFamilyId`, `🦀️.rs:639–837`):** Constants `010`…`970.41` listed; used only for “known family” info diagnostics, not typed parsing.

### 1.2 Composed children / registries

| Component | Location | Content |
|-----------|----------|---------|
| Sheet registry | `🦀️.rs:380–513` `SHEET_ENTRIES` | 100 `SheetEntry` rows: id, de/en title, `SchemaStatus`, `Domain[]`, editions |
| Correction overlays | `🦀️.rs:516–548` `CORRECTION_OVERLAYS` | 32 overlays; `applies_as_of` at `🦀️.rs:366–368` |
| `SchemaCatalog` | `🦀️.rs:550–587` | `operative_sheets`, `reserved_numbers`, `corrections_for_sheet`, domain filter |
| Inference outline | `…/💡️inferences/🧾outline/🦀️.rs` | Static `section_outline` of snapshot top-level fields — not norm clauses |
| Reference fixture | `🦀️.rs:1102–1145` | Sheet 2 heating control valve DN50, `kvs=4.5 m³/h`, geometry + curve |

### 1.3 Mutations (19 kinds)

`…/🧬️schema/🧬️mutations/🦀️.rs:97–117`: `update-manufacturer-file`, `change-correction-as-of`, `change-strict-mode`, `update-limits`, `change-edition-profile`, `remove-edition-profile`, `create-product`, `delete-product`, `rename-product`, `replace-product-configuration`, `create-geometry`, `delete-geometry`, `resize-geometry`, `add-geometry-connection`, `remove-geometry-connection`, `replace-geometry-parameters`, `create-curve`, `delete-curve`, `replace-curve-points`.

Mutations maintain `catalog.index` via `catalog_index_entry_for` (`🦀️.rs:27–29` in mutations root). No mutation adds or validates native `NativeRecord` rows against Part 1 grammars.

### 1.4 `evaluate()` call graph

**Entry points:**
- `…/💡️inferences/🦀️.rs:387` `pub fn evaluate(document: &Vdi3805Snapshot) -> CheckReport`
- `…/✏️editor/🦀️.rs:200–202` `Vdi3805Family::evaluate` → delegates to above
- `…/👁️viewer/…/📊️report/🦀️.rs:31` viewer calls same `evaluate`
- `NormHost::report()` re-evaluates on read (editor evaluate command is no-op `Emit::default()`, `…/🎮️commands/🧮️evaluate/🦀️.rs:24–25`)

**Call graph (in order):**

```
evaluate
├── all_part_checks → part_1::check … part_100::check   [macro at :83–278, invoked :282–384]
│   ├── part_1::check → validate_structure               [:170–176, :337–362]
│   ├── default sheet → product_for_sheet + article_number [:149–158]
│   ├── multi_profile → edition_profile key only         [:130–138]
│   ├── historical → strict_mode gate                      [:106–112]
│   └── reserved → na_check (ignores document)           [:92–95]
├── reserved loop 15..98 → na_check (duplicate of macro) [:396–400]
├── pass_check registry operative count                  [:402–403]
├── corrections_for_sheet(2) only → pass_check each      [:405–408]
├── catalog_to_json / catalog_from_json product count    [:410–422]
├── serialize_native_text / parse_native_text count      [:424–434]
├── catalog index + heating domain count                 [:436–437]
├── filter_by_dn(50) if non-empty → pass                [:439–442]
├── geometry["geom.valve.50"] → bbox vs 0.003 m³          [:444–452]
├── curves["curve.kvs"] → interpolate(50) vs 2.25       [:455–458]
├── validate_structure → diagnostics_to_report           [:460–462]
└── strict_mode → pass_check                             [:464–466]
```

**Helpers (schema, not all reached for norm checks):** `clause`, `pass_check`, `fail_check`, `na_check` (`…/🧬️schema/🦀️.rs:241–263`); `parse_native_text`, `serialize_native_text`, `validate_structure` (`:267–362`); `linear_map` (`:367–372`) — **not called from `evaluate`**; `diagnostics_to_report` (`:376–396`).

### 1.5 Editor / viewer capabilities

| Surface | File | Editable? | Report |
|---------|------|-----------|--------|
| Inputs window | `…/📥️inputs/🦀️.rs:19–20` | Whole snapshot as pretty JSON (`render_document_json`) | — |
| Results window | `…/📊️results/🦀️.rs:22–23` | Read-only | `render_report` → virtualized check list |
| Document panel | `…/📌️panels/🗿️artifact/🦀️.rs:18–19` | Summary only | check count, worst u, all_pass |
| Inspection panel | `…/📌️panels/🔍️inspection/🦀️.rs:19–20` | Selected check index | Single `CheckResult` fields |
| Catalogue panel | `…/📌️panels/📚️catalogue/🦀️.rs:3–5,20–21` | **Placeholder** headline | — |
| Viewer report | `…/👁️viewer/…/📊️report/🦀️.rs:30–32` | Read-only table | Columns: Clause, Status, Utilization, Message (`🖥️app-surface/🦀️.rs:159–166`) |

**Editable subject fields in UI:** only via raw JSON (no per-record / per-attribute forms). Mutations exist in the protocol but are not exposed as structured editors in the play app windows.

**Localization:** Window titles and action descriptions are de+en (`…/✏️editor/🦀️.rs:248–250`). **Check `message` strings are English-only** runtime prose from `evaluate`; report columns are English (`🖥️app-surface/🦀️.rs:160`).

### 1.6 Examples / assets / tests / oracles

| Asset | Path | Notes |
|-------|------|-------|
| Demo example | `…/📚️examples/🎬️demo/` + `🖼️assets/🎬️demo/🗣️.dsl.semio` | Byte-identical to `reference_fixture` DSL |
| Demo session | `…/✏️editor/📚️examples/🎬️demo-session/` | Command script asset |
| Mutation fixtures | `…/🧫️fixtures/🧬️mutations/**` | Per-mutation before/after snapshots |
| Compliance-report tests | `…/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | Sheet **presence**, strict mode, geometry skip — no norm values |
| Compliance helper tests | `…/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | Parser/validate_structure; `linear_map` numeric |
| Crate unit tests | `🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` | Bbox `0.003`, curve `2.25` at 50% — fixture math, not norm limits |
| Mutate integration | `…/🧪️tests/🏭️mutate-vdi3805-1/` | 19 mutation kinds + Python second implementation |
| Oracle manifest | `…/🔮️oracles/🔣️.json` | Mutation vocabulary only; **no compliance oracle** |

**27× `📌️.empty.md` placeholders** under editor/viewer taxonomy (config, presence, transient, actions, utilities).

---

## 2. Stub / fake detection

| # | Kind | Location | Evidence |
|---|------|----------|----------|
| S1 | Ignored input | `…/💡️inferences/🦀️.rs:92–94` | `let _ = document;` in `reserved` macro arm — check cannot depend on dataset |
| S2 | Identity-only sheet check | `…/💡️inferences/🦀️.rs:149–157` | Operative sheets: pass if no product; else pass if `article_number` non-empty — **no Blatt attributes** |
| S3 | Profile stub | `…/💡️inferences/🦀️.rs:130–137` | `multi_profile`: formats `edition_profile` enum name only |
| S4 | Historical gate only | `…/💡️inferences/🦀️.rs:106–112` | Sheets 12, 13, 25: strict_mode fail/pass — not historical content rules |
| S5 | Hardcoded bbox limit | `…/💡️inferences/🦀️.rs:446–451` | `Quantity::new(QuantityKind::Volume, 0.003)` — fixture size `0.15×0.20×0.10`, not VDI geometry rules |
| S6 | Hardcoded curve limit | `…/💡️inferences/🦀️.rs:456–457` | Limit `2.25` = linear interpolate of fixture (`0→0`, `100→4.5` at `x=50`) — **self-referential**, not norm |
| S7 | Wrong quantity kind | `…/💡️inferences/🦀️.rs:449,457` | kvs and bbox tagged `QuantityKind::Volume` for utilization display |
| S8 | Warnings → Pass | `…/🧬️schema/🦀️.rs:380–383` | `Severity::Warning \| Info` mapped to `CheckStatus::Pass` in `diagnostics_to_report` |
| S9 | Duplicate reserved checks | `…/💡️inferences/🦀️.rs:396–400` + macro `reserved` | Same sheets checked twice (macro N/A + loop N/A) |
| S10 | Correction theatre | `…/💡️inferences/🦀️.rs:405–408` | Only sheet **2** corrections; both branches `pass_check` — never fails |
| S11 | IO self-test as compliance | `…/💡️inferences/🦀️.rs:410–434` | JSON/native round-trip product **count** only |
| S12 | Registry vanity check | `…/💡️inferences/🦀️.rs:402–403` | `pass_check` operative sheet count — always passes |
| S13 | DN filter vanity | `…/💡️inferences/🦀️.rs:439–441` | `pass_check` if DN50 index hit — fixture always has DN50 |
| S14 | Thin native parser | `…/🧬️schema/🦀️.rs:267–314` | Only header + `100` spawns products; other lines stored untyped; **no 010/110/200 grammars** |
| S15 | `record_count` not validated | `validate_structure` `:337–362` | Header `record_count` never compared to actual records |
| S16 | `linear_map` dead in evaluate | `…/🧬️schema/🦀️.rs:367–372` | Helper tested but **unreachable** from `evaluate` |
| S17 | Sheet titles placeholder | `🦀️.rs:411–510` | Many sheets titled `"Blatt N"` / `"Sheet N"` — registry not norm-sourced |
| S18 | Catalogue UI placeholder | `…/📌️panels/📚️catalogue/🦀️.rs:3–5` | Explicit “headline placeholder” |
| S19 | Tests avoid norm numbers | `…/🔬️compliance-report/🦀️.rs` | Asserts `parts.contains(&part)`, `status == Pass/Fail/NA` — no attribute values |
| S20 | Misleading comment | `…/💡️inferences/🦀️.rs:77–78` | Claims “99 macro-generated per-sheet **conformance laws**” — laws are stubs (S2–S4) |

**Checks that can never fail (default fixture):** registry operative (`:403`), io round-trips (`:414,428`), catalog index (`:437`), DN filter (`:441`), sheet 2 identity (`:154`), most operative sheets with no product (`:156`), correction passes (`:407`), strict_mode pass when enabled (`:465`).

---

## 3. Complete subject definition (norm engineering target)

The assessment subject is a **manufacturer VDI 3805 dataset**: one or more **Part 1** interchange files plus the typed product data for each claimed **Blatt** (sheet 2–100). German edition (VDI 3805 is a VDI guideline, not EN+NA); corrections identified by edition date.

### 3.1 Part 1 — file and record layer

**File header (record family 010):** Manufacturer identifier, file creation date, character set, record count, building-system number (Anlagenkennzeichen), version/edition markers. **Mandatory:** consistent record count; valid charset; BSN format.

**Core product records:**
- **100** — product master: manufacturer, product group, article number, sheet reference, variant keys.
- **110** — variant/configuration: links to geometry, functions, parameters.
- **120–190** — texts, documents, classifications (per Part 1 tables).
- **200–290** — geometry (parametric dimensions, connection points, clearances) with SI units.
- **300–390** — media / drawings references.
- **400–590** — characteristic curves, operating points (x/y units, monotonicity where required).
- **600+** — accessories, components, relationships (`hasPart`, required quantities).
- **900–970.41** — extensions; unknown families preserved but flagged.

**Cross-record integrity:** Every `110` references existing `100`; geometry IDs in `200` referenced from `110`; curve IDs in `400` referenced from product; accessory/component links resolve; **one primary sheet** per product group with sheet-specific mandatory families present.

### 3.2 Per-sheet (Blatt) product semantics

For each operative sheet, the norm defines **product group codes**, **mandatory attributes** (name, record family, field index, type, unit, allowed values/ranges), and **conditional** attributes. Examples (illustrative — implementation must load per-sheet machine-readable catalogues):

| Sheet | Product domain | Example mandatory attributes (indicative) |
|-------|----------------|---------------------------------------------|
| 2 | Control valves heating | DN, kvs, pressure class, connection type, authority range |
| 3 | Pumps heating | Q-H curve points, hydraulic efficiency, motor power, DN suction/discharge |
| 6 | Heat generators | Nominal heat output, fuel/type, temperature limits |
| 17 | Ducts ventilation | Cross-section, material, leakage class |
| 53 | Heat pumps | COP at reference points, refrigerant, sound power |

**Edition / profile:** Sheets 8, 10, 14, 18, 33, 36, 37, 40, 42, 53, 100 have **Legacy vs Current** attribute sets — `edition_profile` must select the correct mandatory set.

**Value domains:** Enumerations from norm tables; numeric ranges (e.g. DN series); units per VDI 3805 unit rules (absolute vs delta temperature, pressure, flow).

### 3.3 Session / evaluation context

| Input | Purpose |
|-------|---------|
| `correction_as_of` | Apply correction overlays per sheet |
| `strict_mode` | Reject withdrawn/historical proposal sheets |
| `edition_profile` | Pick Legacy/Current mandatory attribute set |
| `limits` | Cap parse size (security, not norm) |

### 3.4 Gap vs current schema

Current `configuration.parameters: BTreeMap<String, VdiValue>` is **untyped** — sufficient for demo DN/kvs only. Missing: per-sheet attribute schemas, typed `NativeRecord` decoders per family, variant graph, text/curve record linkage, file-level 010 record, validation of semicolon field counts per family.

---

## 4. Check catalogue (current vs required)

| Part | Clause / rule (norm) | What is verified today | Required inputs | Limit source | Failure meaning (today) |
|------|----------------------|------------------------|-----------------|--------------|------------------------|
| 1 | File structure | `validate_structure` — manufacturer, article, config id | `catalog` | Plugin heuristics | Missing manufacturer/article |
| 1 | Record count vs header | **Not checked** | `file.record_count`, records | Part 1 | — |
| 1 | Record family grammar 010/100/110/… | Info on unknown family only | `records[]` | Part 1 tables | — |
| 1 | Referential integrity | **Not checked** | geometry/curve refs | Part 1 | — |
| 2–100 (operative) | Sheet mandatory attributes | Article number only if product present | `catalog.products` | Blatt tables | Empty article only |
| 2–100 (reserved) | N/A | `na_check` | — | Registry | Always N/A |
| 12,13,25 | Historical proposal | `strict_mode` flag | `strict_mode` | Registry status | Fail if strict |
| 8,10,… | Multi-profile attributes | Profile enum echoed | `edition_profile` | Blatt edition | Never fails |
| * | Correction overlays | Pass message for sheet 2 only | `correction_as_of` | `CORRECTION_OVERLAYS` | Never fails |
| io | JSON round-trip | Product count | `catalog` | — | Count mismatch |
| io | Native text round-trip | Product count | `catalog`, `limits` | — | Count mismatch |
| — | Geometry bbox | Volume vs **0.003 m³** | `geometry["geom.valve.50"]` | **Hardcoded fixture** | u>1 if resized |
| — | kvs @ 50% | y vs **2.25** | `curves["curve.kvs"]` | **Half of fixture span** | u>1 if points change |
| registry | Operative sheet count | Count string | `SchemaCatalog` | Internal | Never fails |
| catalog | DN50 index | Non-empty filter | `index` | Demo DN | Never fails on fixture |

**DE vs EN:** VDI 3805 is German-origin; `ANNEX = AnnexChoice::De` (`…/🧬️schema/🦀️.rs:239`) is cosmetic for `CheckResult.annex` — there is no EN parallel edition in-plugin.

---

## 5. Remediation strategy (target behaviour)

Each failing check should emit **localized** `message` + structured remediation (Wave B core). Illustrative mappings:

| Check | Remediation target | How to compute target |
|-------|-------------------|------------------------|
| Missing 010 header | `catalog.file` / add `NativeRecord` 010 | List required fields from Part 1 §header table |
| Record count mismatch | `manufacturer_file.record_count` | Set to `count(parsed records)` |
| Missing sheet-2 kvs | `product.configuration.parameters["kvs"]` or record 200 field | Read Blatt 2 mandatory list for product group `HV` |
| kvs below min curve envelope | `curves[id].points` | Norm min curve at operating points; invert linear segment to required y |
| DN not in norm series | `parameters["dn"]` | Snap to nearest allowed DN from Blatt table |
| Missing geometry ref | `configuration.geometry_ref` | Add `200` record + `geometry` entry; cite clause |
| Broken accessory link | `accessories[].accessory_id` | Add product or remove link |
| Wrong edition profile | `edition_profile["8"]` | Set `Current` if using 2023-03 attributes; list delta fields |
| Historical sheet in strict | Remove product with `sheet=12` or set `strict_mode=false` | Name forbidden sheet |
| Native parse field count | `records[].fields` | Expected arity per `RecordFamilyId` schema |

**Report phrasing example (sheet 2, de):** „Blatt 2 §4.2: Pflichtfeld `kvs` fehlt — setzen Sie `kvs ≥ 4.5 m³/h` in Datensatz 200 oder Parameter `configuration.parameters.kvs`.“

---

## 6. Report & UX gaps

| Gap | Detail |
|-----|--------|
| No remediation | `CheckResult` has `message: String` only (`⚖️compliance/🦀️.rs:138–145`) |
| English-only check text | All `evaluate` messages monolingual |
| No subject pointer | Checks don't reference `product_id`, record family, field index |
| JSON-only editing | No forms for records, units, curves, sheet picker |
| Placeholder catalogue | No browsable Blatt/clause index (`📚️catalogue/🦀️.rs:3–5`) |
| Utilization misuse | Pass/fail for non-utilization checks forced to u=1 or 2 (`pass_check`/`fail_check`) |
| Warnings invisible as failures | Empty products = warning but `part_1` may still pass (`:171–175` vs `:342–343`) |
| 100+ row noise | Every sheet emits a row even when N/A — no grouping by product/sheet |
| Evaluate button no-op | `evaluate` command emits zero mutations (`🧮️evaluate/🦀️.rs:24–25`) — relies on implicit re-read |

---

## 7. Target design

### 7.1 Snapshot schema sketch

```rust
pub struct Vdi3805Snapshot {
    pub header: Part1HeaderRecord,           // typed 010
    pub products: Vec<ProductSubject>,       // replaces flat CatalogueProduct
    pub edition_context: EditionContext,
    pub security: SecurityLimits,
}

pub struct ProductSubject {
    pub identity: ProductIdentity,
    pub sheet: SheetId,
    pub variants: Vec<VariantRecord>,        // 110 + children
    pub native_records: Vec<TypedRecord>,    // enum per RecordFamilyId
    pub geometry: Option<GeometrySubject>,
    pub curves: Vec<CurveSubject>,
    pub texts: Vec<TextRecord>,
    pub links: CompositionGraph,
}

pub enum TypedRecord { R100(Product100), R200(Geometry200), R400(Curve400), … }
```

Keep `geometry`/`curves` maps only if indexed by norm IDs; otherwise embed under `ProductSubject`.

### 7.2 `evaluate()` structure

```rust
pub fn evaluate(doc: &Vdi3805Snapshot) -> CheckReport {
    let mut r = CheckReport::default();
    r.extend(check_part1_header(&doc.header));
    r.extend(check_record_graph(&doc.products));
    for product in &doc.products {
        let sheet = sheet_catalog(product.sheet, doc.edition_context);
        r.extend(sheet.check_mandatory_attributes(product));
        r.extend(sheet.check_value_domains(product));
        r.extend(sheet.check_curves_and_geometry(product));
    }
    r.extend(check_corrections(doc));
    if doc.edition_context.strict_mode { r.extend(check_no_historical_sheets(doc)); }
    r
}
```

### 7.3 Example subjects

**Compliant:** Sheet-2 valve matching Blatt 2 tables — DN50, kvs 4.5 m³/h, connections, 200/400 records, all refs closed. Expect all sheet-2 checks `Pass`, Part 1 structure `Pass`.

**Non-compliant:** Same file with (a) missing `kvs`, (b) `record_count=3` but 2 lines, (c) `geometry_ref` dangling, (d) DN 47 non-standard, (e) sheet 12 product with `strict_mode=true`. Expect ≥5 distinct `Fail` with remediation strings.

### 7.4 Worked numeric tests (required)

| Test | Derivation | Expected |
|------|------------|----------|
| Sheet 2 min kvs | Blatt 2 table min for DN50 (e.g. 0.63 m³/h — use real table) | `Fail` if kvs=0.5; remediation cites min |
| Record 200 DN field | Part 1 field index for DN | Parse `200;dn;50` → attribute present |
| Curve monotonicity | kvs(h) non-decreasing | Add decreasing segment → `Fail` |
| Bbox clearance | Blatt 2 connection spacing | Compute from norm formula, not 0.003 |
| Correction overlay | `correction_as_of=2023-01` vs `2023-03` | Attribute optional/required flip |

### 7.5 Files to change (Wave C)

| Area | Paths |
|------|-------|
| Entity model | `🦀️.rs`, `…/📸️snapshot/🦀️.rs`, DSL/JSON/proto facets |
| Sheet catalogues | New `🏅️standards/…/sheets/<n>/` attribute schemas |
| Part 1 parser | `…/🧬️schema/🦀️.rs` `parse_native_text`, `validate_structure` |
| Evaluate | `…/💡️inferences/🦀️.rs` — replace macro `define_vdi_part` |
| Remediation | `⚖️compliance/🦀️.rs` + `🖥️app-surface/🦀️.rs` (Wave B) |
| Editor | `…/📥️inputs/` structured forms; `📚️catalogue/` real sheet browser |
| Examples | `reference_fixture`, `🖼️assets/🎬️demo/`, non-compliant fixture |
| Tests | `…/🔬️compliance-report/`, new `…/⚖️sheet-02/` numeric tests |
| Oracles | Third-party or reference-tool oracle for native `.vdi3805` text (manifest gap) |

---

## 8. Risks / open questions

1. **Sheet index accuracy:** `SHEET_ENTRIES` assigns Blatt 3 → „Heizkörper“, Blatt 5 → pumps (`🦀️.rs:383–385`) — cross-check against current VDI 3805 sheet index before encoding attribute tables.
2. **Placeholder sheet titles:** Sheets 15–98 mostly „Blatt N“ — registry may not match published VDI numbering/status.
3. **Correction coverage:** 32 overlays declared but evaluate only reports sheet **2** — apply rules must be sheet-complete.
4. **Native vs DSL canonical:** Demo is DSL; real manufacturers deliver semicolon text — parser gap blocks real datasets.
5. **ISO 16757 overlap:** `AGENTS.md` cites both VDI 3805 and ISO 16757 for product exchange — clarify boundary to avoid duplicate attribute models.
6. **No external compliance oracle:** `🔮️oracles/🔣️.json` discharges mutations only; no reference validator for VDI 3805 files.
7. **Wave B dependency:** Remediation/localization blocked on shared `CheckResult` enrichment (`📓️coordination.md` baseline).
8. **`multi_profile` sheets:** Legacy vs Current attribute diffs not captured in data — need per-sheet diff documents.
9. **Performance:** Full dataset (10⁵ records) — checks must be incremental; current `evaluate` is O(sheets) not O(records).
10. **Reserved duplicate checks:** Clean up double N/A rows before UX work to avoid report clutter.

---

## Appendix A — `evaluate` / `CheckReport` existence (confirmed)

| Criterion | Status | Evidence |
|-----------|--------|----------|
| `evaluate` function | ✅ | `…/💡️inferences/🦀️.rs:387` |
| Returns `CheckReport` | ✅ | Same; pushes `CheckResult` items |
| Wired to UI | ✅ | `Vdi3805Family::evaluate` `…/✏️editor/🦀️.rs:200–202`; viewer `…/📊️report/🦀️.rs:31` |
| Export `report:out` | ✅ | `export_media` via `app_surface` |
| Tests call `evaluate` | ✅ | `…/🔬️compliance-report/🦀️.rs` |
| Norm-complete assessment | ❌ | Stubs S1–S20 |

---

## Appendix B — Default fixture key numbers (for test authors)

| Quantity | Value | Source |
|----------|-------|--------|
| DN | 50 | `reference_fixture` `🦀️.rs:1106` |
| kvs | 4.5 m³/h | `🦀️.rs:1107` |
| Bbox volume | 0.003 m³ | `0.15×0.20×0.10` `🦀️.rs:1127` |
| kvs at 50% stroke | 2.25 m³/h | Linear interp `🦀️.rs:1141`, test `🧪️tests/🔬️unit/🦀️.rs:39` |
| Operative sheets (registry) | 66 of 100 | `SchemaStatus::is_operative` excludes Reserved/Historical |
