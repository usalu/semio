# Impl B2 — Norm App Surface (Wave B)

## Widget / interaction mechanism

Structured Inputs and remedy apply buttons follow the FEM inspection pattern:

1. Build controls with `semio_framework_ui_contract` builders (`input(InputKind::Number|Text)`, `select`, `tree_item` + child control).
2. Bind `Trigger::Change` (scalars) or `Trigger::Activate` (buttons / catalogue rows / remedy apply) via `ActionFactory::new(CONTROLLER_ID).action(verb, args)`.
3. Shell merges the control value under `value` and dispatches `{action, args}` into `norm_command_from_action!` → typed command → `XMutation::from_snapshot` bundle.

Chrome labels use `LocalizedLabel::native(en, de).resolve(Terminology::Native, locale)`. Report domain copy uses `LocalizedCopy::resolve(&protocol_locale(locale))`.

## Verb payloads

| Verb | Args | Effect |
|------|------|--------|
| `setField` | `{path, value}` | Set leaf at path; `value` is host-merged control JSON |
| `insertItem` | `{path, index, value?}` | Insert into array at path (append if index ≥ len); default value `null` |
| `removeItem` | `{path, index}` | Remove array element at path |
| `applyRemedy` | `{checkId, remedyIndex}` | Re-evaluate, write SI `remedy.required.value` into `remedy.target.path` via `setField` |

All four commit `XMutation::from_snapshot(base, target)` as **one** undoable edit (`commit_snapshot_fields`).

## Path syntax (spec v1.2)

CamelCase snapshot paths, dotted fields + list selectors:

| Form | Example |
|------|---------|
| Position | `elements[2].layers[1].thicknessM` |
| Stable id | `members[id=B1].actions[id=ULS-1].nEd` |

- Id = the list element's string `id` field; unique within that list; must not contain `]`, `.`, or `=`.
- `[]` field-meta wildcards match **both** `[index]` and `[id=…]`.
- Inputs editor emits `[id=…]` when the element has a valid `id`, else `[index]` (survives insert/remove reorders).
- `removeItem` accepts either an array path + index **or** an element path ending in `[index]`/`[id=…]`.
- Clear errors: unknown id, duplicate ids, malformed selector.

API: `parse_path`, `get_value_at_path`, `set_value_at_path`, `insert_value_at_path`, `remove_value_at_path`, `list_element_path`.

## SI / unit convention (binding for Wave C)

- Snapshot quantity fields **MUST store SI** (m, N, Pa, W/(m²K), …).
- `Remedy.current` / `Remedy.required` are SI (`Quantity`).
- `applyRemedy` writes `required.value` (SI float) straight into `target.path`.
- Display units are UI-only via `format_quantity` (kN, MPa, mm, W/(m²K), m²K/W, kWh/(m²a), …).
- Do **not** store mm/kN/MPa in snapshot fields.



## Lazy collapsible inputs tree (slot budget)
### Runner summaries (lazy inputs)

```
bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache -- --no-fail-fast
→ Summary [0.152s] 48 tests run: 48 passed, 0 skipped
```

```
bun nx run @semio-tech/norm-iso16757-rs:test --skip-nx-cache -- --no-fail-fast
→ Summary [1.190s] 307 tests run: 306 passed, 1 failed, 0 skipped
  FAIL every_editable_leaf_perturbation_changes_a_check (compliance; concurrent Wave D work — not app-surface)
  inputs::full_default_snapshot_inputs_expose_catalogue_sections_within_slots — ok
```

`cargo check --tests` on 15 family crates: failures observed are pre-existing / concurrent (MutationLeaf, DslValue::Map, missing COOLING_KIND) — none reference `render_document_editor` / lazy TreeWindows API.


**Problem:** Full ISO 16757 (and any deep catalogue) assembled every object/array child eagerly. Projection then exceeded `UI_BUILT_CHILD_RETIRE_SLOTS` (384) even with array windowing ≥64.

**Fix (`🖥️app-surface/🦀️.rs` — `render_value_editor` / `render_object_editor` / `render_array_editor`):**

- Every object and array is a `tree_window_section` keyed by `inputs_section_id(path)` (`norm-inputs-root` / `norm-inputs-{path}`).
- Children materialise only when `TreeWindows::is_open` is true (host `TreeWindowRequest.open`, else author default).
- Defaults: document root **open**; nested collections **closed**. Collapsed headers show localized field-meta label + `(count)`.
- Object maps and arrays share the same host windowing (`offset`/`rows`); ≥64 entries stay inside the viewport grant (`NORM_LIST_VIRTUALIZE_THRESHOLD`).
- Path verbs / `[id=…]` / field-meta / evaluate jobs unchanged.
- `📈️iso16757/.../📥️inputs` calls `render_document_editor` on the **full** snapshot (clearing workaround removed).

**Tests:**

- Contract: `collapsed_deep_document_stays_within_retire_slots`, `expanding_subtree_materialises_windowed_children_within_slots`, updated list-verbs test opens the `items` window.
- Family: `full_default_snapshot_inputs_expose_catalogue_sections_within_slots` (iso16757 inputs).


## Field metadata hook (optional)

```rust
pub struct NormFieldChoice {
    pub value: &'static str,      // wire / select value
    pub label_en: &'static str,   // required localized label
    pub label_de: &'static str,   // required localized label
}
pub struct NormFieldMeta {
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub unit: Option<&'static str>,                 // display hint only
    pub choices: Option<&'static [NormFieldChoice]>, // enum select — families MUST supply en+de labels
}
pub type NormFieldMetaFn = fn(&str) -> Option<NormFieldMeta>;
```

Pass `Some(lookup)` into `render_document_editor(..., meta_fn)`. Lookup matches exact path, `[]` wildcards, then longest prefix (see `lookup_norm_field_meta`). `None` → path segment as label, raw scalars, no choices.

**Choice labels:** enum/select fields MUST list `NormFieldChoice` rows with `label_en` and `label_de`. Raw codes (or humanized codes) as the only select label are not acceptable — families supply both languages in the table.

## Size bounds

- `NORM_RETAINED_RAW_BYTES = 524_288` (512 KiB) — ~500 entities × ~10 fields camelCase JSON
- `NORM_ARTIFACT_STORE_MAXIMUM_BYTES = 2_097_152` (2 MiB) — `from_snapshot` mutation bundles
- `norm_bounded_contract`: `(524_288, 64, 64, 65_536, 15_000)`
- Retained tools: 8 × 15 apps = 120 (`setField`/`insertItem`/`removeItem`/`applyRemedy` added)

## Surfaces updated

- Inputs → structured property editor (`render_document_editor`)
- Results → grouped by `part` + verdict header; fail/warning expand with explanation + apply remedies; virtualized flat list (part headers interleaved)
- Inspection → full check card
- Document → localized summary from `CheckReportSummary`
- Catalogue → example list → `setActiveExample` **plus optional normative reference tables** (see below)
- Viewer table → Part · Clause · Subject · Status · Utilization · Title · Remedy (localized)

## Catalogue reference tables API

Shared schema-first model in `🖥️app-surface` (read-only; row → subject selection is out of scope):

```rust
pub struct CatalogueColumn {
    pub id: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub unit: Option<&'static str>,
}
pub enum CatalogueCell {
    Text(String),
    Number { value: f64, decimals: u8 },
    Empty,
}
pub struct CatalogueRow {
    pub id: String,
    pub cells: Vec<CatalogueCell>,
}
pub struct CatalogueTable {
    pub id: &'static str,
    pub title_en: &'static str,
    pub title_de: &'static str,
    pub clause: ClauseId,
    pub columns: Vec<CatalogueColumn>,
    pub rows: Vec<CatalogueRow>,
}

pub fn render_catalogue(
    examples: &[ExampleSource],
    tables: &[CatalogueTable],
    locale: Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode>;
```

**Family panel call (example):**

```rust
pub fn render(
    examples: Vec<ExampleSource>,
    locale: Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>, // or TreeWindows::unhosted() when the editor has not threaded windows yet
) -> UiAssemblyResult<BuiltNode> {
    crate::app_surface::render_catalogue(
        &examples,
        &reference_tables(), // `&[]` when the family has no typed reference data yet
        locale,
        controller_id,
        windows,
    )
}
```

Rendering: always an **Examples** section (`setActiveExample`), then one collapsible windowed section per table (`norm-catalogue.table-{id}`, default closed) so large tables stay inside `UI_BUILT_CHILD_RETIRE_SLOTS`. Column headers localize en/de and append `[unit]` when present; clause appears in the section title via `ClauseId` Display. Chrome: Examples/Beispiele, Empty table/Leere Tabelle.

## Tests run

`cargo test -p semio-s-artifact-norm-contract --lib` → **33 passed** (grouping, en/de localization, remedies, path set/insert/remove, quantity format, virtualization laws).

Family crates may fail while Wave C migrates `evaluate()` / schemas — plumbing judged by contract crate.

## Remaining gaps

1. Inputs editor does not yet virtualize huge entity lists (full tree materialization); raise if 500+ entities paint stalls.
2. `applyRemedy` for `OneOf` / non-applicable remedies stays inert (`applicable: false`).
3. No per-family `NormFieldMetaFn` tables yet — Wave C should add them where enums/units need labels.
4. Viewer unit tests still calling `report::render(doc)` without locale need a one-arg update if any fail.
5. Concurrent Wave C may still be reshaping family schemas; re-read before further per-family edits.
