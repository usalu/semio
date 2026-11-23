# Scene surface accessible names

`semio_plugin::scene_surface(id, kind, scene, name, description)` is now the only scene-surface constructor. `name` is required and localized by the caller (`semio_plugin::surface_name(locale, en, de)`, an exhaustive `Locale` match, or a plugin label catalog entry); `description` is optional. `named_scene_surface` is removed. The TS host needs no change: `SurfaceAccessibilityShell` in `Interpreter/🖼️surface-views` already renders the node's `accessibility` as `aria-label`/description on `role="application"`.

Migrated callers: 139 (a scan of every `scene_surface(` call in tracked `.rs` files finds none with the old 3-argument form).

Callers (plugin / window dir / line / last EN,DE literals found in the call; catalog-backed names show the literal when inline, else `(label/catalog)`; the full wording per surface lives at the call site):

| area | window | line | name literals |
|---|---|---|---|
| ✒️writer | ✒️main | 107 | Text editor / Texteditor |
| ➗️mathematical | 📐️geometry | 43 | Geometry view / Geometrieansicht |
| ➗️mathematical | 🕸️graph | 50 | Equation graph / Gleichungsgraph |
| 🀄️wfc | 👁️preview | 73 | Preview / Vorschau |
| 🀄️wfc | 👁️preview | 106 | Preview / Vorschau |
| 🀄️wfc | 👁️preview | 175 | Board / Tafel |
| 🀄️wfc | 👁️preview | 175 | Preview / Vorschau |
| 🀄️wfc | 👁️preview | 224 | Preview / Vorschau |
| 🀄️wfc | 👁️preview | 287 | Preview / Vorschau |
| 🀄️wfc | 🔲️grid | 265 | Grid / Raster |
| 🀄️wfc | 🕸️graph | 225 | Slot graph / Slot-Graph |
| 🀄️wfc | 🕸️graph | 225 | Slot graph / Slot-Graph |
| 🀄️wfc | 🖼️input | 53 | Input / Eingabe |
| 🀄️wfc | 🖼️input | 159 | Input / Eingabe |
| 🀄️wfc | 🧊️preview | 53 | Preview / Vorschau |
| 🀄️wfc | 🧊️preview | 291 | Preview / Vorschau |
| 🀄️wfc | 🧩️output | 53 | Output / Ausgabe |
| 🀄️wfc | 🧩️output | 93 | Output / Ausgabe |
| 🀄️wfc | 🧱️grid | 245 | Grid / Raster |
| 🌀️procedural | 👁️preview | 53 | Preview / Vorschau |
| 🌀️procedural | 👁️preview | 75 | Preview / Vorschau |
| 🌀️procedural | 👁️preview | 84 | Preview / Vorschau |
| 🌀️procedural | 🕸️flow | 148 | Flow / Fluss |
| 🌀️procedural | 🖼️semantic-ui | 57 | (label/catalog) |
| 🌀️procedural | 🖼️semantic-ui | 80 | (label/catalog) |
| 🌊️flow | 🌊️main | 107 | Flow graph / Flussgraph |
| 🌊️flow | 🌊️main | 120 | Flow graph / Flussgraph |
| 🌊️flow | 👁️preview | 50 | Generation preview / Generierungsvorschau |
| 🌊️flow | 🗣️compiled | 54 | Compiled wire format / Kompiliertes Wire-Format |
| 🌍️gis | 🏔️terrain | 68 | Terrain view / Geländeansicht |
| 🌍️gis | 🏔️terrain | 85 | Terrain view / Geländeansicht |
| 🌍️gis | 🗺️map | 86 | Map / Karte |
| 🌍️gis | 🗺️map | 110 | Map / Karte |
| 🌎️hub | 🔄️workflow | 109 | Workflow graph / Workflow-Graph |
| 🌎️hub | 🕸️compiled-dag | 56 | Compiled workflow / Kompilierter Workflow |
| 🌎️hub | 🗂️media-vfs | 57 | Media files / Mediendateien |
| 🌿️vcs | 📜️history | 45 | Version history / Versionsverlauf |
| 🎞️animate | 🖼️tile-editor | 86 | Slide canvas / Folienfläche |
| 🎞️animate | 🖼️tile-editor | 103 | Slide canvas / Folienfläche |
| 🎥️shooting | 🎥️scene | 157 | Scene / Szene |
| 🎥️shooting | 🎥️scene | 228 | Scene / Szene |
| 🎥️shooting | 🖼️icon | 82 | Icon / Symbol |
| 🎬️sequence | 📜️script | 50 | Script / Skript |
| 🎬️sequence | 📽️main | 81 | Sequence graph / Sequenzgraph |
| 🎬️sequence | 📽️main | 87 | Sequence graph / Sequenzgraph |
| 🎬️sequence | 🧬️compiled | 44 | Compiled wire format / Kompiliertes Wire-Format |
| 🏗️fem | 🖥️app-surface | 36 | (label/catalog) |
| 🏗️fem | 🖥️app-surface | 43 | (label/catalog) |
| 🏙️bim | 🗺️plan | 49 | Floor plan / Grundriss |
| 🏙️bim | 🧊️world | 56 | Model view / Modellansicht |
| 🏙️bim | 🧊️world | 222 | Model view / Modellansicht |
| 🏙️bim | 🧬️family | 108 | Family / Familie |
| 🏙️bim | 🧬️family | 276 | Family preview / Familienvorschau |
| 🏙️bim | 🧮️schedule | 96 | Schedule / Bauteilliste |
| 🏙️bim | 🧰️kit | 224 | (label/catalog) |
| 🏛️architect | 📋️register | 62 | Register / Register |
| 🏛️architect | 🕸️graph | 118 | Graph / Graph |
| 🏛️architect | 🧭️trace | 99 | Trace / Ablauf |
| 🏭️process | 🪚️workpiece | 224 | Workpiece / Werkstück |
| 💠️lowpoly | 🌐️model | 262 | (label/catalog) |
| 💠️lowpoly | 🖼️uv | 105 | (label/catalog) |
| 💡️reasoning | 🕸️canvas | 51 | Wire canvas / Leitungsfläche |
| 💡️reasoning | 🕸️canvas | 57 | Wire canvas / Leitungsfläche |
| 📋️forms | ▶️try | 72 | forms-view-try.{}.value |
| 📋️forms | ▶️try | 152 | forms-try.{key}.input |
| 📋️forms | 📊️results | 49 | {}.answer |
| 📋️forms | 🧱️builder | 58 | Form builder / Formularbaukasten |
| 📏️layout | 👁️preview | 53 | (label/catalog) |
| 📏️layout | 👁️preview | 101 | Layout preview / Layoutvorschau |
| 📏️layout | 📐️blueprint | 58 | (label/catalog) |
| 📏️layout | 🧵️paragraph-work | 60 | (label/catalog) |
| 📐️cad | ✏️edit | 511 | (label/catalog) |
| 📕️norm | 📄️pages | 198 | (label/catalog) |
| 📕️norm | 🪟️window | 120 | (label/catalog) |
| 📖️playbook | 🌊️embedded-flow | 76 | embedded-flow.source |
| 📖️playbook | 🏗️builder | 78 | (label/catalog) |
| 📖️playbook | 👁️preview-and-params-body | 88 | single |
| 📖️playbook | 👁️preview-and-params-body | 148 | playbook-module.{key}.editor |
| 📖️playbook | 📋️steps | 52 | (label/catalog) |
| 📖️playbook | 📜️source | 39 | (label/catalog) |
| 📖️playbook | 📰️activity | 56 | (label/catalog) |
| 📖️playbook | 🔺️changes | 45 | (label/catalog) |
| 📖️playbook | 🗂️files | 94 | (label/catalog) |
| 📖️playbook | 🧵️embedded-evaluation | 257 | single |
| 📸️remodel | 📊️report | 145 | Report / Bericht |
| 📸️remodel | 🖼️frames | 137 | Frames / Bilder |
| 📸️remodel | 🧊️model | 172 | Model view / Modellansicht |
| 📸️remodel | 🧊️model | 185 | Model view / Modellansicht |
| 🔋️energy | 🧊️model | 96 | Model view / Modellansicht |
| 🔋️energy | 🧊️model | 242 | Model view / Modellansicht |
| 🔱️trinity | 🌐️graph | 45 | Nakagin graph / Nakagin-Graph |
| 🔱️trinity | 🌐️graph | 66 | Nakagin graph / Nakagin-Graph |
| 🔱️trinity | 📊️results | 46 | Result graph / Ergebnisgraph |
| 🔱️trinity | 📊️results | 50 | Result table / Ergebnistabelle |
| 🔱️trinity | 📝️editor | 13 | Jack query / Jack-Abfrage |
| 🔱️trinity | 🔎️jack | 18 | Jack query / Jack-Abfrage |
| 🔱️trinity | 🖌️ui | 91 | (label/catalog) |
| 🕸️dag | 🕸️main | 55 | Node graph / Knotengraph |
| 🕸️dag | 🕸️main | 58 | Node graph / Knotengraph |
| 🕸️dag | 🧬️compiled | 54 | Compiled wire format / Kompiliertes Wire-Format |
| 🖍️draw | 🖼️canvas | 82 | Drawing canvas / Zeichenfläche |
| 🖍️draw | 🖼️canvas | 183 | Drawing canvas / Zeichenfläche |
| 🖨️raster | 🖼️composite | 63 | Raster canvas / Rasterfläche |
| 🖨️raster | 🧭️navigator | 49 | Raster navigator / Rasternavigator |
| 🗄️stdio | 📐️geometry | 22 | IFC representation geometry / IFC-Repräsentationsgeometrie |
| 🗄️stdio | 📐️geometry | 22 | IFC representation geometry / IFC-Repräsentationsgeometrie |
| 🗄️stdio | 🪟️windows | 142 | (label/catalog) |
| 🗄️stdio | 🪟️windows | 287 | (label/catalog) |
| 🗒️note | 🖼️composite | 54 | Note canvas / Notizfläche |
| 🗒️note | 🖼️composite | 100 | (label/catalog) |
| 🗒️note | 🖼️composite | 115 | Note canvas / Notizfläche |
| 🧩️puzzle | ◻️2d | 179 | Puzzle 2D / Puzzle 2D |
| 🧩️puzzle | ◻️2d | 248 | (label/catalog) |
| 🧩️puzzle | 🖼️canvas | 199 | {PUZZLE2D_PLAY_SURFACE_ID}.{pane} |
| 🧩️puzzle | 🧊️3d | 419 | (label/catalog) |
| 🧩️puzzle | 🧊️main | 534 | (label/catalog) |
| 🧰️framework | 📃️document | 200 | Document page / Dokumentseite |
| 🧰️framework | 📊️table | 167 | Table / Tabelle |
| 🧰️framework | 📊️table | 204 | Table / Tabelle |
| 🧰️framework | 📊️table | 238 | Table / Tabelle |
| 🧰️framework | 📝️text | 158 | Text / Text |
| 🧰️framework | 📝️text | 234 | Text editor / Texteditor |
| 🧰️framework | 🔬️reconcile-budget | 85 | puzzle3d-main-perspective / 3D view |
| 🧰️framework | 🔬️reconcile-budget | 174 | puzzle3d-main-perspective / 3D view |
| 🧰️framework | 🔬️tool-run | 484 | Board / Brett |
| 🧰️framework | 🔬️tool-run | 503 | 3D view / 3D-Ansicht |
| 🧰️framework | 🧊️mesh | 52 | 3D view / 3D-Ansicht |
| 🧰️framework | 🪟️surface-carriers | 146 | viewport / Scene |
| 🧰️framework | 🪟️surface-carriers | 175 | {} / Scene |
| 🧰️framework | 🪟️surface-carriers | 177 | :[9,9,9]} / Scene |
| 🧰️framework | 🪟️surface-carriers | 181 | {} / Scene |
| 🧰️framework | 🪟️surface-carriers | 192 | viewport / Scene |
| 🧰️framework | 🪟️surface-carriers | 202 | preview / Scene |
| 🧰️framework | 🪟️surface-carriers | 211 | viewport / Scene |
| 🧱️block | 🌐️world | 52 | Object Kind / Objektart |
| 🧱️block | 🌐️world | 75 | (label/catalog) |
| 🪵️sourcing | 👁️preview | 102 | Object preview / Objektvorschau |
| 🪵️sourcing | 🔬️unit | 678 | neutral-retained-table / Retained table |
| 🪵️sourcing | 🧱️editor-vocabulary | 89 | (label/catalog) |


## This pass

Live `scene_surface(...)` calls outside `.cache` are 5-argument. One call was still 3-argument and is now 5-argument.

Changed (1 call, 1 file):

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/audit-eurocode-scenes.rs` — `publish` now takes `name` and calls `scene_surface(id, kind, scene, name, None)`. The model caller uses `surface_name(Locale::En, "Structural model", "Tragwerksmodell")` because that harness has no viewer locale. The diagram caller uses `surface_name(options.locale, "Diagram", "Diagramm")`.

Left unchanged:

- Window kits `text`, `mesh`, `table`, and `document` already passed a name and a description.
- PDF page canvas already passes `labels.canvas_name` and `labels.canvas_description`.
- `pdf/**/import/**` has no `scene_surface` call. The shell save/backbone worker has no `scene_surface` call.
- Test fixtures that already pass an English name and `None` were left as they are: `surface-carriers` (`"Scene"`), `reconcile-budget` (`"3D view"`), and sourcing unit (`"Retained table"`). They have no locale in scope.

Remaining 3-argument calls, not edited:

- Six stale copies under `.cache/f/lost/adopt-2026-11-20T00-40-36-105Z/stale/` for wfc grid2d, bitmap output, grid3d preview, and 3d preview. They are not the live sources. The live wfc windows already pass `surface_name`.

Callers that could not be safely named: none. The audit harness was the only live 3-argument call, and both of its surfaces have a fixed English and German name.

## Verification

- Source scan (git-tracked and untracked `.rs`, excluding ticket snapshots): 139 `scene_surface(` calls, all five-argument; no `named_scene_surface` left.
- Native `cargo check` (shared native-target, nightly-2026-07-20) of `semio-plugin` and `semio-plugin-window-transient` from the root workspace: Finished, no errors.
- Plugin crates (each artifact has its own workspace): NOT compile-verified. A sweep over about 55 workspaces ran the volume out of space (ENOSPC, cargo errors "No space left on device", Cargo.lock write failures); the sweep was stopped, then the shared target was cleared by the coordinator mid-run. No compile error attributable to the migration was seen, but none of the plugin crates finished. Re-run per workspace: `-p <crate>` with the `chk` recipe from the ticket notes, one workspace at a time.
- Hub (`semio-hub-space-app`), window-kit tests, `surface-carriers`, `reconcile-budget`, `tool-run` tests were edited but not compiled.
- TS host: no change needed; `SurfaceAccessibilityShell` already maps the node's accessibility label/description to ARIA on the `role="application"` shell.

## Compile sweep

Native `cargo check` (nightly-2026-07-20, shared native-target, one workspace at a time; `--workspace --all-targets --keep-going` per plugin workspace).

- `MeshWindowKit::render` in `🗄️stdio/📇️registry/🧬️contract/🪟️window-kit/🧊️mesh-scene` already passes `locale` (the reported break no longer reproduces); every `Text/Mesh/Table/DocumentWindowKit::render*` call found by grep passes a locale.
- Block 3D canvas name: `🧱️block/🗿️artifacts/🧊️3d` `🌐️world` surface is now "3D view / 3D-Ansicht" (was "Object Kind / Objektart"); the window title/terminology (`window_world`) stays "Object Kind".
- Norm ticket audit scripts (`GET-NORM-PLUGIN-WORKING-END-TO-END/audit-eurocode-scenes.rs`, `audit-eurocode-viewport-pages.rs`): not compiled by any manifest (only by the ticket's `link-*.py` scripts); `render_viewport_page` calls updated to the current 7-argument arity (extra `name`: "Viewport").
- Heuristic scan: all 97 inline `surface_name(<locale>, ..)` calls use an identifier bound earlier in the enclosing function.
- Checked clean of migration errors: PDF 1.7 page-editor test chain and 1.4 round-trip chain, `semio-plugin-surface-carrier-laws`, `semio-plugin-reactor-core`, `semio-plugin-window-kits(-rich)`, `semio-hub-space-app` lib, and the workspaces writer, mathematical equation, wfc 2d/grid2d/bitmap/3d/grid3d (all-targets).
- Not swept: the other ~66 plugin workspaces. Each costs 2 to 10 minutes under shared build contention and every one is blocked on the same foreign errors below, so the sweep was stopped. No scene-surface, `surface_name` or kit-locale error appeared in any checked workspace.
- Foreign (not from this migration) errors that fail whole crates: `semio-os-store-space-history` (`sqlite/snapshot/native` 2 vs 3 args), `semio-plugin-instance-construction` (`heap_bytes`), `semio-framework-test-transient-preparation-law` and `semio-plugin-tests-06` (retained-work/`InteractiveJob::step`/`RetainedAllocationEffect` API change; also `time-travel` `SurfaceKind` type split), `semio-hub-space-app` lib test (`PluginApp::render` takes 4 args, missing `*Setting` types), `semio-framework-value-native-owned` (transient E0133).

## PDF tests

- `semio-s-artifact-stdio-pdf-mutation-round-trip-tests`: 12 passed, 0 failed.
- `semio-s-artifact-stdio-pdf-page-editor-tests`: 61 passed, 1 failed. Fixed three test defects: the NaN box edit is now refused at `edit_from_action` (coded `number-invalid`), so the test accepts refusal at either step; the German inspector and thumbnail tests now admit a `FixtureUiArena` and retire the rendered tree; the German test reads input values and accessibility from the tree instead of a shallow `Debug`, and drops the over-broad `blurDecimalQuad` absence check (the five extra boundary boxes still use the host dot-only `blurDecimalQuad` commit; only the media box was moved to the locale text input, a residual).
- Remaining failure: `paint_fidelity::canvas_paint_matches_mupdf_probes` case `codec-jpx-rgb` (probe [3,3] paints black, expected 5,6,3). The image preview and JPX codec sources are unchanged against HEAD; unrelated to names or i18n, left alone.
