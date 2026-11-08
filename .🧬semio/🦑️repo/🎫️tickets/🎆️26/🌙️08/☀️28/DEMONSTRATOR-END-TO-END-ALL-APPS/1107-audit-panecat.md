# 1107 panecat: read-only audit of the pane smoke law callers (lane panede)

Scope: the `assert_pane_smoke_by_example` / `PaneSmokeLanguage` change in `1107-result-panede.md`. Read-only: no edits, no cargo, no modifying git commands.

Paths are relative to `/Users/ueli/Documents/semio/`. Short names used below:

- `LAW` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws`
- `PLUGIN` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`
- `GEN` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`
- `PROC` = `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `CAD` = `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `GIS` = `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `SRC` = `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `FEM` = `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor`
- `ENE` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `PUZ` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`

## Law (backs every caller)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| law | `PaneSmokeLanguage` fields, catalog check after load and after redo, view on every dispatch | OK | `LAW/🪧️pane-smoke-law/🦀️.rs:9-13` (struct), `:21-24` (catalog `contains`), `:37` (`view_state` carried), `:47` (after load), `:56` (after redo) |
| law | undo/redo verbs carry the caller's `ActionMeta` | OK | `LAW/👯️paired-apps-history-and-convergence-laws/🦀️.rs:64-72` (`settle_history_verb_as`), `:60` (`settle_history_verb` delegates with `meta("local")`) |

## Generator (procedural, `GEN/🧪️tests/🪧️pane-smoke/🦀️.rs`, plus composition copy)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| generator | catalog `labels.widgets` rendered en/de after load and redo | OK | Label `widgets` = "Widgets" / "Elemente" at `GEN/🏅️…/✏️editor/🗣️terminology/🦀️.rs:281`. Rendered as the catalogue section title at `GEN/🏅️…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:101` (`window_section` with `Some(ui_label(labels.widgets))`), dispatched from `…/🖼️play-app-and-retained-command-contracts/🦀️.rs:83`. Title is set unconditionally by `tree_section(label)` (`PLUGIN/🪟️window-kits/🌳️tree/🦀️.rs:699-710`), so it survives undo/redo. |
| generator | imports: `generation3d_labels`, `GENERATION_3D_PLAY_BODY_CATALOGUE`, deps | OK | `generation3d_labels` pub fn at `GEN/🏅️…/✏️editor/🗣️terminology/🦀️.rs:706`. `catalogue` is `pub mod` at `GEN/🏅️…/✏️editor/📌️panels/🦀️.rs:4`; const pub at `…/🛍️catalogue/🦀️.rs:31`. Package `GEN/📦️packages/🦀️rust/Cargo.toml`: `semio-s-artifact-procedural-3d-editor-panels` regular dep line 81, `…-terminology` dev dep line 130, `semio-plugin` with `artifact-app-testing` dev dep line 108, `[[test]] pane-smoke` line 137-139. |
| generator (composition copy) | `set_active_example::` and `add_generation::` used unqualified in the `include!`d copy | UNVERIFIED | `GEN` composition copy at `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️…/✏️editor/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs:11-14` has no `use` lines. Names come from `…/📥️editor-imports/🦀️.rs:9,32`, which is included at `…/🧮️editor-unit-laws/🦀️.rs:8`. The pane-smoke include is at `…/📚️editor-laws/🦀️.rs:7`, a different file. Whether the imports reach that include site was not traced. |

## Process3d (bearbeiten, `PROC/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs`)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| bearbeiten | catalog `stock`, `steps` en/de after load and redo | OK | Labels: `stock` "Stock" / "Rohteil" (`PROC/🗣️terminology/🦀️.rs:8`), `steps` "Steps" / "Schritte" (`:9`). Rendered as section titles at `PROC/📌️panels/🗿️artifact/🦀️.rs:100-101`, dispatched at `PROC/🪟️editor-impl/🦀️.rs:18`. Titles are unconditional. |
| bearbeiten | imports: `document_panel::PROCESS_3D_PLAY_BODY_ARTIFACT`, `artifact_app_laws`, `process3d_labels`, dep | OK | `PROC/🧪️tests/🔬️unit/🦀️.rs:14` aliases `panels::document as document_panel`. Const pub at `PROC/📌️panels/🗿️artifact/🦀️.rs:22`. `artifact_app_laws` imported at `…/🔬️unit/🦀️.rs:86`. `process3d_labels` imported at `…/🔬️unit/🦀️.rs:16`. `artifact-app-testing` feature present in the process3d package Cargo.toml. |

## Cad (koordinator, `CAD/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs`)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| koordinator | catalog `pane_shape`, `nodes` en/de after load and redo | OK | `pane_shape` "Shape" / "Form" at `CAD/🗣️terminology/🦀️.rs:17`, rendered as the `document_pane_section` title (`CAD/📌️panels/🗿️artifact/🦀️.rs:136-138`, used at `:161`). `nodes` "Nodes" / "Knoten" at `…/🗣️terminology/🦀️.rs:22`, rendered at `…/🗿️artifact/🦀️.rs:169`. Both unconditional. |
| koordinator | imports: `cad_labels`, `ViewModel`, `document::CAD_PLAY_BODY_ARTIFACT`, feature | OK | `cad_labels` imported at `CAD/🧪️tests/🔬️unit/🦀️.rs:64`. `ViewModel` imported at `:116`. `CAD_PLAY_BODY_ARTIFACT` pub at `CAD/📌️panels/🗿️artifact/🦀️.rs:31`. `semio-plugin` with `artifact-app-testing` at `CAD/📦️packages/🦀️rust/Cargo.toml:75`. The `semio_s_artifact_cad_cad_examples::ID` path was not traced. |

## Gis (verfolgen, `GIS/🧪️tests/🪧️pane-smoke/🦀️.rs`)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| verfolgen | catalog `layer_water`, `layer_roads` en/de after load and redo | OK | Labels: "Water" / "Wasser" at `GIS/🗣️terminology/🦀️.rs:11`, "Roads" / "Straßen" at `:13`. Rendered by `gis2d_layer_label` (`:52`) in the artifact panel for every `GIS_MAP_LAYER_IDS` entry (`GIS/📋️constants-and-io/🦀️.rs:96` Water, `:98` Roads; section title `GIS/📌️panels/🗿️artifact/🦀️.rs:50-51`). 11 rows, within `TREE_WINDOW_DEFAULT_ROWS` = 48 (`PLUGIN/🪟️window-kits/🌳️tree/🦀️.rs:293`). |
| verfolgen | import `editor::gis2d::terminology::gis2d_labels` | OK | Gis2d mirror re-exports `semio_s_artifact_gis_gismap_snapshot::editor::gis2d::terminology` (`GIS/📦️packages/🦀️rust/🪞️mirror/🦀️.rs`, `pub mod gis2d`). That module is `pub mod terminology` at `GIS/🧬️schema/📸️snapshot/📦️packages/🦀️rust/🦀️.rs:98`. `pub fn gis2d_labels` at `GIS/🗣️terminology/🦀️.rs:46`. Gis2d package depends on the snapshot crate. |
| verfolgen | import `panels::artifact::GIS2D_PLAY_BODY_ARTIFACT` | OK | `pub mod artifact` inside `pub mod panels` in the gis2d mirror. Const pub at `GIS/📌️panels/🗿️artifact/🦀️.rs:25`. Gis2d Cargo dev dep `semio-plugin` with `artifact-app-testing` at `GIS/📦️packages/🦀️rust/Cargo.toml:75`. |

## Sourcing (aussuchen, `SRC/🧪️tests/🪧️pane-smoke/🦀️.rs`)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| aussuchen | catalog `restock`, `col_name` en/de after load and redo | OK | `restock` "Restock From Catalogue" / "Bestand aus Katalog" at `SRC/🗣️terminology/🦀️.rs:29`, rendered as an unconditional button in `filter_bar` (`SRC/🎭️…/🏊️pool/🦀️.rs:132`). `col_name` "Name" / "Name" at `…/🗣️terminology/🦀️.rs:21`, rendered as a table column label in the scene JSON (`…/🏊️pool/🦀️.rs:139`, `…/🧱️editor-vocabulary/🦀️.rs:81`). Note: `col_name` is identical in en and de, so the de check cannot discriminate. |
| aussuchen | imports: `sourcing_curation_labels`, `pool`, `SOURCING_CURATION_BODY_POOL`, `DEMO_STOCK_EXAMPLE_ID`, `stock_of` | OK | `sourcing_curation_labels` pub at `SRC/🗣️terminology/🦀️.rs:37`, re-exported through the domain crate and the curation root. `pool` re-exported at `SRC/🦀️.rs:229`. `SOURCING_CURATION_BODY_POOL` pub at `…/🏊️pool/🦀️.rs:27`. `DEMO_STOCK_EXAMPLE_ID` pub at `…/🧱️editor-vocabulary/🦀️.rs:44`. `stock_of` pub at `…/👥️presence/🦀️.rs:421`. Deps: `semio-plugin` dev with `artifact-app-testing` (`SRC/📦️packages/🦀️rust/Cargo.toml:61`), `semio-framework-trace` dev (`:57`), `semio-framework-async-macros` dev (`:69`), `semio-ui-locale` (`:35`). |
| aussuchen | path `semio_s_artifact_sourcing_curation::schema::curated_count` | UNVERIFIED | `curated_count` is pub at `SRC/🧬️schema/🧾️domain/🦀️.rs:337`. Whether the crate-root `schema` module re-exports it was not traced. |

## Fem3d (statik, `FEM/🧪️tests/🪧️pane-smoke/🦀️.rs`)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| statik | catalog `nodes`, `elements` en/de after load and redo | OK | Labels: "Nodes" / "Knoten" and "Elements" / "Elemente" at `FEM/🗣️terminology/🦀️.rs:35-36`. Rendered as `section_label` "Nodes (N)" (`FEM/📌️panels/🗿️artifact/🦀️.rs:274`, format at `:234`). The count suffix does not break `contains`. The section is unconditional (`window_section_or_placeholder`). |
| statik | import `semio_s_artifact_fem_3d_editor::editor::fem3d::terminology::fem3d_labels` | WRONG | The editor crate root does not expose `terminology`. Its `use …_terminology::…fem3d_labels` at `FEM/🦀️.rs:61` is a private `use`, and the `pub use component::*` re-exports (`FEM/🦀️.rs:175-199`) do not cover it. Correct: `semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::fem3d_labels` (`fem3d_labels` pub at `FEM/🗣️terminology/🦀️.rs:154`; dep `semio-s-artifact-fem-3d-editor-terminology` at `FEM/📦️packages/🦀️rust/Cargo.toml:63`). |
| statik | import `semio_s_artifact_fem_3d_editor::editor::fem3d::panels::artifact::BODY_KEY` | WRONG | Same reason: `panels` is not exposed by the editor crate. Correct: `semio_s_artifact_fem_3d_editor_panels::editor::fem3d::panels::artifact::BODY_KEY` (path used by the editor root at `FEM/🦀️.rs:58`; `BODY_KEY` pub at `FEM/📌️panels/🗿️artifact/🦀️.rs:64`; dep at `FEM/📦️packages/🦀️rust/Cargo.toml:60`). |
| statik | `view.for_panel()` keeps locale; feature | OK | `for_panel` at `PLUGIN/…/🛂️manifest/🖼️view-model/🦀️.rs:204` keeps locale. Editor dev dep has `artifact-app-testing` at `FEM/📦️packages/🦀️rust/Cargo.toml:84`. |

## Energie (`ENE/🧪️tests/🔬️mounted-bestest/🦀️.rs:69-76`)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| energie | catalog "Site" / "Standort" en/de after load and redo | OK | `site_label` at `ENE/📌️panels/🗿️artifact/🦀️.rs:81-83` (`say(locale,"Site","Standort")`). The same string is the unconditional section title at `…/🗿️artifact/🦀️.rs:437`. The site row is built from `std::slice::from_ref(&model.site)` (`:247`). |
| energie | import `semio_s_artifact_energy_model_editor_panels::artifact::site_label` | OK | `pub mod artifact;` at `ENE/📌️panels/🦀️.rs:21-22`. `pub fn site_label(locale: Locale) -> &'static str` at `…/🗿️artifact/🦀️.rs:81`. Crate lib path `…/📌️panels/📦️packages/🦀️rust/Cargo.toml:16` is `../../🦀️.rs`. |
| energie | dependency placement of `semio-s-artifact-energy-model-editor-panels` | WRONG | The lane says "dev-dependency". In `ENE/🧪️tests/🔬️mounted-bestest/📦️packages/🦀️rust/Cargo.toml` it is a regular `[dependencies]` entry at line 27 (the `[dependencies]` table starts at line 18; the manifest has no `[dev-dependencies]` table). Functionally equivalent for this test-only crate. The HEAD version does not contain the entry, so it is new. |
| energie | `command_from_action` round trip with typed `field` | OK | `SetStructureField { field, value }` maps to `SET_NODE_ACTION_ID` (`ENE/🎮️commands/🦀️.rs:234`). The variant is declared at `:148`. `command_from_action` is at `:308`; the `set-node` arm at `:318` reads `nodeId`, then falls back to `field`, then `id`. The reducer maps `"name"` to `rename_model` (`ENE/🧮️reduce-and-helpers/🦀️.rs:116-117`). Caveat: the args come from the derived `ToValue`, whose field-name encoding was assumed, not traced into the macro. The native-descriptor refusal gate (`:313`) was not traced against the mutation aggregate ids. |
| energie | cycle from the new dependency | OK | No `Cargo.toml` references `semio-s-artifact-energy-model-editor-mounted-tests` except its own `Cargo.lock` entry. `semio-s-artifact-energy-model-editor-panels` lock dependencies (`ENE/Cargo.lock:7394+`) do not include it. No cycle. |
| energie | lock refresh needed? | WRONG | Yes, a refresh is needed. `ENE/Cargo.lock:7376-7391` (the mounted-tests entry) lists its dependencies and has no `"semio-s-artifact-energy-model-editor-panels"` line. The coordinator must refresh `Cargo.lock`. |
| energie | `SetActiveExample { example_id: "bestest-600" }` id exists | OK | `ENE/📚️examples/🏛️bestest-600/🦀️.rs:7` (`ID = "bestest-600"`). Variant shape at `ENE/🎮️commands/🦀️.rs:175`. |

## Aggregator (puzzle, `PUZ/🧪️tests/🔬️pane-smoke/🦀️.rs`, own cycle)

| Pane | Item | Verdict | Correct value / evidence |
|---|---|---|---|
| aggregator | catalog `objects`, `references` en/de after load | OK | Labels: "Objects" / "Objekte" at `PUZ/🗣️terminology/🦀️.rs:38`, "References" / "Referenzen" at `:45`. Checked at `PUZ/🧪️tests/🔬️pane-smoke/🦀️.rs:38-42` before any dispatch. |
| aggregator | catalog `objects`, `references` en/de after redo | WRONG | The law requires the catalog check after load and after redo. After redo (`…/🔬️pane-smoke/🦀️.rs:95-101`) only the new row id is checked (`:101`), not the catalog strings. Fix: after `:101`, render `artifact::BODY_KEY` and assert `labels.objects.as_str()` and `labels.references.as_str()` are contained, with `labels = puzzle3d_labels(&ViewModel::new(locale, Native))`. |
| aggregator | import `puzzle3d_labels` in module scope | OK | `pub fn puzzle3d_labels` at `PUZ/🗣️terminology/🦀️.rs:155`, returning `Option` (hence the `.expect`). Imported in `PUZ/🧪️tests/🧰️prelude/🦀️.rs:36`. The prelude is `include!`d into `PUZ/🧪️tests/🔬️gestures/🦀️.rs` (line 4-ish), which declares `mod pane_smoke;` (`:13`) and the child reaches it through `use super::*`. |
| aggregator | locale carried by dispatches (`set_label_axes`) | UNVERIFIED | `set_label_axes` sets `self.view.locale` (`PUZ/🧪️tests/🧰️context/🏗️raw-app-fixture/🦀️.rs:125-128`). Whether the dispatch helper (`PUZ/🧪️tests/🧰️context/🎛️settle-and-dispatch-drivers/🦀️.rs:161`) uses that view was not traced. The helper takes no view argument. |
| aggregator | not using `PaneSmokeLanguage` | OK (by design) | The cycle is its own (`aggregator_cycle(locale)` at `:55`) as the lane states, so the law's `ActionMeta` language carry does not apply to it. |

## Summary

- Items checked: 30 table rows (law 2, generator 3, process3d 2, cad 2, gis 3, sourcing 3, fem3d 4, energie 7, aggregator 4; the aggregator's by-design row is counted, and UNVERIFIED rows are counted as checked).
- Items wrong: 5 (fem3d terminology import, fem3d panels import, energie dependency wording, energie lock refresh, aggregator catalog after redo).
- Unverified (not counted as wrong): generator composition copy import chain, sourcing `schema::curated_count` path, aggregator dispatch locale carry.
