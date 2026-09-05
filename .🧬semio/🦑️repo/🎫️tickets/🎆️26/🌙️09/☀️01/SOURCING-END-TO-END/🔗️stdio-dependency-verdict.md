# Verdict

**YES.** The sourcing Pool can display the ten demo stock kinds with stdio broken. `stock_of()` reads only `stock_extra`, which is self-contained and populated from sourcing's built-in modules (beams, windows, slabs). The `catalog` child field is never read by pool rendering and never by any render/export/inference call site.

---

## Evidence

### 1. CurationSnapshot Structure (file:line evidence)

**File:** `./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

```rust
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "curation.curation", layout = "lines")]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationSnapshot {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.kit")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    #[value(default)]
    pub stock_extra: Vec<ObjectKindExtra>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub curated: Vec<CuratedItem>,
}
```

- **catalog field** (line 9): `ArtifactChild<SemioKitSnapshot>` with `#[child(kind = "s.stdio.semio.kit")]` — requires stdio child artifact to resolve.
- **stock_extra field** (line 11): `Vec<ObjectKindExtra>` with `#[value(default)]` — self-contained, no external dependency.

Docstring explicitly states: "`catalog`/`stock_extra` together replace the former inline `stock: Vec<ObjectKind>` field ... see `crate::artifacts::curation::stock_of` for the reassembly accessor every reader funnels through."

### 2. The `stock_of()` Function — Decisive Evidence

**File:** `./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs`, line 219

```rust
/// 👁️ The one accessor every render/export/inference call site funnels through to read the full
/// reassembled stock catalogue from snapshot-owned overflow records.
pub fn stock_of(document: &CurationSnapshot) -> Vec<ObjectKind> {
    let _ = &document.catalog;
    document.stock_extra.iter().map(|row| ObjectKind { id: row.id.clone(), name: row.name.clone(), module_id: row.module_id.clone(), typology_path: row.typology_path.clone(), availability: row.availability, geometry: row.geometry.clone() }).collect()
}
```

**Verdict:** `stock_of()` reads **only `stock_extra`**. The line `let _ = &document.catalog;` is a no-op borrow (produces no value). The entire ObjectKind reassembly reads fields directly from `stock_extra`, not from `catalog`.

Compare with `stock_from_catalog_and_extra()` (line 179), which does join:
```rust
pub fn stock_from_catalog_and_extra(catalog: &SemioKitSnapshot, extra: &[ObjectKindExtra]) -> Vec<ObjectKind> {
    let extra_by_id: std::collections::HashMap<&str, &ObjectKindExtra> = extra.iter().map(|e| (e.id.as_str(), e)).collect();
    catalog.types.iter().filter_map(|kit_type| extra_by_id.get(kit_type.id.as_str()).map(|extra| object_kind_from_parts(kit_type, extra))).collect()
}
```

But **`stock_from_catalog_and_extra()` is never called** by pool rendering or by any UI code path.

### 3. Pool Viewer Calls stock_of()

**File:** `./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏊️pool/🦀️.rs`, line 26

```rust
pub fn view_model(document: &CurationSnapshot) -> TableView {
    let stock = stock_of(document);
    let rows = stock.iter().map(|kind| vec![kind.id.clone(), kind.name.clone(), kind.module_id.clone(), kind.typology_path.join(" / "), kind.availability.to_string()]).collect();
    TableView { columns: vec!["Id".into(), "Name".into(), "Module".into(), "Typology".into(), "Availability".into()], rows }
}
```

The pool viewer obtains stock via `stock_of(document)` and displays it. No reference to `catalog`.

### 4. Demo Stock Origins — Hardcoded in Sourcing

**File:** `./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`, lines 739, 682

```rust
pub fn demo_stock() -> Vec<ObjectKind> {
    sourcing_modules("[]").iter().flat_map(|module| module.demo_kinds()).collect()
}

pub fn sourcing_modules(contributions_json: &str) -> Vec<SourcingModules> {
    let mut modules: Vec<SourcingModules> = vec![beams::BeamsModule.into(), windows::WindowsModule.into(), slabs::SlabsModule.into()];
    modules.extend(contributed_sourcing_modules(contributions_json).into_iter().map(SourcingModules::from));
    modules
}
```

The `demo_stock()` initializes modules with **hardcoded built-in modules** (beams, windows, slabs) and no contributions from stdio or any external source.

### 5. Demo Kinds — All From Built-In Modules

**Beams** (lines 372–428): 4 kinds
- beam-glulam-gl24h (Glulam GL24h 200×400)
- beam-kvh-c24 (KVH C24 100×200)
- beam-steel-ipe200 (Steel IPE 200)
- beam-steel-hea160 (Steel HEA 160)

**Windows** (lines 433–475): 3 kinds
- window-casement-100x120 (Casement Window 100×120)
- window-fixed-150x150 (Fixed Window 150×150)
- window-tilt-turn-120x140 (Tilt & Turn Window 120×140)

**Slabs** (lines 479–537): 3 kinds
- slab-concrete-240 (Concrete Slab 240mm)
- slab-clt-160 (CLT Slab 160mm)
- slab-hollow-core-265 (Hollow Core Slab 265mm)

**Total: 10 kinds.** All defined inline in the sourcing plugin's schema. Verified in DSL fixture:

**File:** `./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`

```
stock-extra=[ id=beam-glulam-gl24h name="Glulam GL24h 200×400" module-id=beams availability=24 ...
  id=beam-kvh-c24 name="KVH C24 100×200" module-id=beams availability=60 ...
  id=beam-steel-ipe200 name="Steel IPE 200" module-id=beams availability=12 ...
  id=beam-steel-hea160 name="Steel HEA 160" module-id=beams availability=8 ...
  id=window-casement-100x120 name="Casement Window 100×120" module-id=windows availability=18 ...
  id=window-fixed-150x150 name="Fixed Window 150×150" module-id=windows availability=10 ...
  id=window-tilt-turn-120x140 name="Tilt & Turn Window 120×140" module-id=windows availability=14 ...
  id=slab-concrete-240 name="Concrete Slab 240mm" module-id=slabs availability=30 ...
  id=slab-clt-160 name="CLT Slab 160mm" module-id=slabs availability=20 ...
  id=slab-hollow-core-265 name="Hollow Core Slab 265mm" module-id=slabs availability=16 ... ]
```

### 6. Catalog Child Not Actually Used by Render Paths

**File:** `./✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`, lines 313–314

```rust
pub fn curation_decision_for_delta(document: &CurationSnapshot, object_id: &str, delta: i64) -> CurationDecision {
    // 🔎️ `availability` lives on `stock_extra` (the sourcing-owned overflow half) — no need to
    // resolve the composed `catalog` child just to clamp a count, so this reads `stock_extra`
    // directly rather than going through `stock_of`'s full reassembly.
```

Comment confirms the design: catalog child resolution is **explicitly avoided** for performance and to support degradation when the child is unavailable.

### 7. Stdio Plugin Descriptor Status

**File/Directory:** `./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🗄️stdio/`

No `🔣️descriptor.json` or `🛂️descriptor.semio` file present. Only binary artifacts from Aug 18 (pre-link failure). Contrasts with sourcing's present descriptors:
- `./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/🔣️.json` (315 KB, Sept 4)
- `./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🪵️sourcing/🛂️descriptor.semio` (72 KB, Sept 5)

### Contradiction Summary

| Claim | Source | Verdict |
|-------|--------|---------|
| "Sourcing's own pool does not depend on it — `stock_of` reads `stock_extra` only" | 📓️status.md | **CORRECT** |
| "`CurationSnapshot::catalog` is composed from `s.stdio.semio.kit` — the pool's stock is joined out of that child plus `stock_extra`" | 🧪️runtime-verification.md | **Structurally correct but operationally misleading** — catalog IS a composed child, BUT pool rendering never joins; it reads stock_extra only via `stock_of()`. |

The runtime-verification.md describes the **schema structure** accurately but misstates the **runtime dependency**. The design supports catalog as a composition boundary, but stock_of() and all render paths degrade to stock_extra-only when catalog is unavailable.
