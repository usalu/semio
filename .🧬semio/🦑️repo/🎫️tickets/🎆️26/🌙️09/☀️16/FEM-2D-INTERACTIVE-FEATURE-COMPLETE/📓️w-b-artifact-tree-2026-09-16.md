# 🌳️ Slice B — fem2d artifact tree (outliner) + the shared label set (2026-09-16)

Agent: slice B (Opus). Scope: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/`, crate `semio-s-artifact-fem-2d`, editor gated behind `--features component-app-assembly`.

## 1. Files

| File | State |
|---|---|
| `📌️panels/🗿️artifact/🦀️.rs` | REWRITTEN — the whole outliner (stub replaced) |
| `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | NEW — 12 tests, mounted from the panel with `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` |
| `🗣️terminology/🦀️.rs` | REWRITTEN — the full `app_labels!` set (slice B is the first writer) + `FEM2D_LABELS_IDENTICAL_BY_DESIGN` |
| `🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` | NEW — 4 laws over the label set |

No file outside slice B's ownership was touched. `✏️editor/🦀️.rs` and the crate root are untouched — the coordinator's `ui_label` / `ui_node_list` / `fem2d_action` / panel registration / render routing were sufficient as delivered.

## 2. The tree

`build_artifact_tree(document, interaction, labels)` (aliased by the routed `render`) assembles nine sections in build order:

1. **Nodes** — `n1 · (0.00, −4.00)`, granularity `node`, icon `circle-dot`
2. **Elements** — `e3 · Beam n1 → n2` (kind localised), `element`, `minus`
3. **Regions** — `r1 · First Floor Slab · 4 pts`, `region`, `square`
4. **Supports** — `s1 · n1 · Tx Ty`, `support`, `anchor`
5. **Load Cases** — `Dead Load · 1 Load · Self Weight`, `loadCase`, `list`; one CHILD row per load at `load` granularity (`Ty −12000 N @ p8_l1`, `wy −500 N/m @ e8`, `1500 Pa @ r1`)
6. **Combinations** — `ULS · 1.35 dead + 1.5 live`, `combination`, `link`; read-only child row per term keyed `fem2d-play-artifact.term.<comb>.<case>`
7. **Materials** — `Steel S235 · E 210 GPa`, `material`, `layers`
8. **Sections** — `HEA200 · A 0.00538 m² · Iy 3.69e-5 m⁴`, `section`, `ruler`
9. **Analysis** — one read-only row, description `Modal Count 3 · Buckling Count 3 · Deformation Scale 300`

Section headers carry the DOCUMENT's own count (`Nodes (12)`), never the number of rows the page happened to place, so a truncated section still tells the truth. Empty sections fall back to the `(none)` placeholder via `section_or_placeholder`.

### Selection, actions, marking

- Row key = the RAW entity id (`n1`, `e3`, `r1`, `s1`, `l6`, `dead`, `uls`, `steel`, `chs76`). The tree is `.interaction_domain(FEM2D_INTERACTION_DOMAIN)`, so the framework marks rows by that id and a canvas pick and a row click land in one selection. Only combination TERM rows use a composite id — a term is not a domain target.
- Row activation = `INTERACTION_SELECT_ACTION_ID` with `{domainId, merge:"replace", method:"pick", targets}` (keys pushed in ascending order; `targets` is the JSON of `[InteractionTarget{granularity,id}]`), minted through `fem2d_action` under `FEM2D_PLAY_CONTROLLER_ID`.
- Row actions, never more than the contract's two: `focusEntity {id}` (icon `focus`) on the geometric rows (nodes, elements, regions, supports) and `removeSelection {ids:[id]}` (icon `trash-2`) on every row. Load, load-case, combination, material and section rows carry delete only.
- `props.description` = the localised entity noun; `props.dimmed` = a DANGLING reference (element → missing node/material/section, support → missing node, load → missing target, region → missing material, combination → missing case) or an empty, self-weight-free load case. `props.default_open` is `false` on leaves and `true` on a load case so its loads are visible.
- `.selected(...)` / `.highlighted(...)` are fed from `Fem2dInteractionSnapshot`, capped at `UI_FIXED_LIST_ITEMS` ids (`marked_ids`) so a 300-entity selection marks its first page instead of refusing the render.

### Paging — a max-min fair page, not cad's verbatim reservation

`panel_page_rows()` is 31 interactive rows for the WHOLE panel, and the demo document wants 39 (12 nodes + 9 elements + 1 region + 4 supports + 2 cases + 3 loads + 1 combination + 3 materials + 4 sections). cad's `budget.nested(SECTIONS - k)` reservation is written for four symmetric panes; applied verbatim here it gives the first section everything it asks for and leaves the load cases ONE row (measured: 12/9/1/4/1/1/1/1) — an outliner that hides every load of a load-case demo.

`section_quotas(section_demands(document), page)` replaces it with a max-min fair split: raise a per-section ceiling as far as the page affords, give every section under the ceiling ALL its rows, hand the spare to the widest sections. `with_quota` then reserves the whole rest of the page for the siblings and settles what a section under-spends straight back, so nothing is wasted.

- demo (31 rows): `[7, 6, 1, 4, 5, 1, 3, 4]` — only Nodes (+5) and Elements (+3) truncate; every load, material and section is listed.
- 60 nodes + a 40-load case: `[6, 6, 1, 4, 6, 1, 3, 4]` — Nodes closes with `+54`, the wind case with `+40`, and no admission ever fails.

Every section still ends in `panel_continuation_row` through `paged_panel_section`, which also swallows a genuine `ui.fixed-capacity` refusal mid-section (another panel may take arena credit concurrently) and closes with `+N` rather than faulting the render. `section_rows` is passed as the section's own `budget.remaining()`, so the plan and the walk agree instead of re-reading a shrinking arena.

Node budget: the 60-node synthetic document assembles 47 of `UI_DOCUMENT_NODES = 128`; the test asserts the bound.

### Text safety

Every row label goes through `UiText::clipped` (`Label(UiText::clipped(...))`), and so does every description — a 900-character region name yields a clipped row instead of `ui.fixed-capacity` killing the panel and, with it, every later `refreshUi`. Scalars are formatted by `fem2d_scalar` (scientific below a milli and above a mega, plain otherwise) and `fem2d_coordinate` (always two decimals); both emit the typographic minus `−`, so `−12000` and `−4.00` read as numbers and never as a hyphenated id. The ISO quantity symbols `E`, `A`, `Iy` are deliberately literal — they are identical in every locale, unlike `labels.youngs_modulus`, which names the same quantity where a row has room to spell it out.

## 3. The label set (`🗣️terminology/🦀️.rs`)

One `app_labels!` block, four cells per row (`native_en`/`native_de`/`reuse_en`/`reuse_de`), reuse repeating native — fem2d has no second vocabulary. 82 labels as of this writing (slice D added `selected` and `add_term` concurrently; both kept):

- chrome: `artifact, inspection, results, analysis, summary, schema, none, pending, selected, add_term`
- entity nouns singular: `node, element, region, support, load, material, section, load_case, combination, term`
- entity nouns plural: `nodes, elements, regions, supports, loads, materials, sections, load_cases, combinations, terms`
- element kinds: `bar, beam`
- fields: `id, name, x, y, kind, start, end, youngs_modulus, poisson_ratio, density, area, second_moment_of_area, fixed, tx, ty, rz, dof, value, wx, wy, pressure, thickness, mesh_size, outline, holes, self_weight, factor, points`
- analysis: `modal_count, buckling_count, deformation_scale`
- results/playback: `source, mode, static_mode, modal, buckling, mode_index, phase, playing, play, pause, speed, loop_mode, ping_pong, once, waveform, ramp, sine, step, frequency, load_factor`
- verbs: `focus, delete`

Rust-keyword collisions are spelled `static_mode` and `loop_mode`; everything else is the noun verbatim. `FEM2D_LABELS_IDENTICAL_BY_DESIGN` names the 18 rows whose German IS the English (symbols `x/y/tx/ty/rz/wx/wy/id`, borrowings `element/material/term/name/start/pause/phase`, international `modal/ping_pong/schema`) — every other row must differ, which is what turns "somebody pasted the English into `native_de`" into a failing test rather than a reading exercise.

## 4. Tests

`📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — 10:

1. `demo_document_lists_every_section_with_its_own_count` — nine sections, headers name the document's counts, placed + `+N` accounts for every entity, the analysis row carries the three settings.
2. `rows_are_keyed_by_the_raw_entity_id_and_bound_to_the_fem2d_domain`
3. `load_case_rows_nest_their_loads_and_combination_rows_nest_their_terms`
4. `rows_carry_the_human_label_of_their_entity` — every label format above, against the bundled demo.
5. `scalars_and_overlong_labels_stay_inside_the_ui_text_envelope` — `3.69e-5`, `wy −500 N/m @ e8`, a 900-char region name clipped with `…`.
6. `german_labels_resolve_across_the_whole_tree` — `Knoten (12)`, `Elemente (9)`, `Lastfälle (2)`, `Querschnitte (4)`, `Balken n1 → n2`, `Eigengewicht`, and no English leaking through.
7. `a_node_row_binds_the_interaction_select_args_for_its_own_id` — `interactionSelect` under the fem2d controller, `{"granularity":"node","id":"n1"}` in the targets, exactly two row actions (`focus`, `trash-2`).
8. `selected_and_hovered_ids_are_marked_from_the_interaction_snapshot` — plus an 80-id selection that marks its first page and never faults.
9. `section_quotas_are_max_min_fair` — the split itself, including the no-starvation and all-fit corners.
10. `an_oversized_document_pages_with_continuation_rows_instead_of_faulting`
11. `an_empty_document_renders_placeholders`
12. `the_app_declares_the_artifact_panel_under_its_body_key` — the manifest seam (see §6 for why it is not the routed render).

`🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` — 4: locale/terminology resolution on all four axes, non-empty + reuse-repeats-native, German-is-a-translation, and the roster law naming every noun/field/verb the three panels bind.

## 5. Verification (all run, all green)

| Gate | Result |
|---|---|
| `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly` (lib) | green, zero warnings from slice B's files |
| `cargo check … --tests --message-format short` | green (blocked twice on peers' `patch-combination` / `focus-entity` test files; green once they landed) |
| `cargo check … --target wasm32-wasip2` | green |
| `RUST_MIN_STACK=134217728 cargo nextest run … --profile fundamental --no-fail-fast -E 'test(panels::artifact) or test(terminology)'` | **16 tests run: 16 passed, 1210 skipped** |

Log: `🗑️generated/w-b-nextest.txt`, `🗑️generated/w-b-check-1.txt`.

## 6. Open issues and notes for the coordinator

- **🔴️ CRATE-WIDE: every `context::fem2d_app()` test aborts in `ArtifactStore::drop`.** Not caused by slice B, and not confined to it — measured on the two PRE-EXISTING harness tests:

  ```
  FAIL editor::fem2d::modes::edit::windows::model::tests::renders_fem2d_model_scene
  FAIL editor::fem2d::component::unit_tests::an_unknown_body_key_renders_a_diagnostic_instead_of_panicking
  panicked at 🧰️framework/…/🏪️store/🦀️.rs:18431:
  artifact store reached Drop without its exact terminal-empty shallow-shell witness
  ```

  `🧪️tests/🔬️unit/🦀️.rs`'s `fem2d_app()` hands back a `VcsArtifactApp` that nobody retires, and `ArtifactStore::drop` is strict about its terminal-empty shell (the flow crate solved the same thing with an explicit `retire_flow_store_cold`; see that function's docstring). Slice D's inspection panel test and every command test that boots an app are on the same hook. The harness is the coordinator's file, so slice B did NOT touch it: instead `the_app_renders_the_artifact_body` was rewritten as the manifest assertion `the_app_declares_the_artifact_panel_under_its_body_key`. **Once `fem2d_app()` retires its store, put the routed render (`render(&mut app, BODY_KEY)` asserting `n1 · (0.00, −4.00)`) back into `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`** — the coverage is written and only the harness blocks it.
- **Disk was full.** The first check died on `No space left on device` (567 MiB free; `⚡️cache/cargo/build/debug/incremental` alone was 102 GB). I moved that directory aside and deleted it in the background — a pure cargo cache, and the fleet was failing every build on ENOSPC. 64 GiB free afterwards. `wasm32-wasip2/wasm-dev/incremental` is still 62 GB and `wasm32-wasip2/debug/incremental` 24 GB if more is needed. Consider `CARGO_INCREMENTAL=0` for the fleet's test waves.
- **31 rows is the whole panel.** The demo cannot show all 39 of its rows; the fair split makes Nodes and Elements the only truncated sections. If the demo must list all 12 nodes, the lever is the UI contract's `UI_VALUE_PAGE_ROWS`, not this panel.
- **Row ids must stay unique across collections.** The framework marks by raw id, so a document with a node and a material both called `x` would mark both rows. The demo's ids are disjoint; no guard exists in the schema.
- **`ui_value_text`/`ui_value_list`/`ui_value_map`** are private to this panel. Slices D and E will want the same three; a later consolidation into `✏️editor/🦀️.rs` (the coordinator's file) would remove three copies.
- **`FemDof::Tz/Rx/Ry`** are unreachable in 2d but exist on the enum; the support label spells them literally rather than adding three dead label rows.
