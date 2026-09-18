# Plugin & Artifact Inventory Audit — 2026-09-18

Read-only audit of every `✏️s/🔌️plugins/*` directory plus `✏️s/🔌️plugins/🔒️policy-allowlist.json`. No files were edited, no git state was changed, no builds were run beyond the read-only greps/finds captured below. Raw command captures are under `🗑️generated/plugins-catalog.txt`, `🗑️generated/plugins-inventory.txt`, `🗑️generated/plugins-tests.txt`, `🗑️generated/plugins-wasm-artifacts.txt` in this ticket folder.

## Method

- Canonical registry: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json` (60 `pluginId`/`directoryName` pairs — 34 top-level plugins + 26 nested extension/module sub-plugins).
- Workspace membership: root `Cargo.toml` `[workspace] members`.
- Per-plugin disk facts: `find`/`grep` over `🗿️artifacts/*`, `🎛️apps/*`, `Cargo.toml` package names, `component-app-assembly` feature, `*.wit` files, `wasm32-wasip2` references, `📋️project.json`/`launch.json`.
- Tests: `grep -rl "#\[test\]"` / `#\[ignore` per plugin directory (no `cargo test` run repo-wide; no builds triggered).
- Built wasm evidence: `find … -name '*.wasm'` with mtimes (no new builds triggered by this audit).
- Ticket recency: directory names and `📓️status*.md` content under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/*`.

**Important scoping note on `🔒️policy-allowlist.json`**: this file is *not* an os boot registry. Its own `$comment` states it is a merged, per-area contribution to "allowlists this area contributes to repository policy rules" (e.g. a `semantic-vocabulary` rule listing individual `.rs`/`.ts` file paths per plugin that are exempt from some naming/lint rule). It plays no role in whether a plugin boots or is registered; it is referenced below only where relevant.

## Per-plugin table

Legend: **Cat.**=registered in `🗺️catalog.json`; **WS**=root `Cargo.toml` workspace member; **CAA**=`component-app-assembly` feature present on ≥1 crate; **Wasm built**=most recent `*.wasm` mtime found under the plugin (via `dist/component-dev|component-release`); **Sept ticket**=most recent ticket folder under `26/09` that substantively concerns the plugin; **Boot**=classification from Q1 below (BOOTS / BROKEN / UNTESTED / IN-PROGRESS).

| Plugin | Artifacts | Apps | Crates | CAA | Wit | Wasm built | Cat. | WS | Sept ticket (latest) | Boot |
|---|---|---|---|---|---|---|---|---|---|---|
| ✒️writer | 1 (writer) | 0 | 2 | N | N | none found | Y | Y | none dedicated (08/17 zero-warnings report only) | UNTESTED |
| ➗️mathematical | 1 (equation) | 0 | 4 | N | N | none found | Y | Y | ☀️02 FIX-MATHEMATICAL-JS-PACKAGE-METADATA (metadata only) | UNTESTED |
| 🀄️wfc | 5 (bitmap,2d,grid2d,3d,grid3d) | 0 | 7 | Y (all 5 artifacts + plugin) | N | 09-17 17:23 | Y | Y | ☀️18 EXTRACT-WFC-PLUGIN (**today**, brand-new extraction from procedural) | IN-PROGRESS |
| 🌀️procedural | 2 (generation3d, generation2d) | 0 | 3 | Y (both) | Y (8 files) | 09-17 15:08 | Y | Y | ☀️15 PROCEDURAL-3D-HISTORY-CAMERA-SPURIOUS (after ☀️03/☀️09/☀️14 marathon) | BOOTS (with recurring vite wedge) |
| 🌊️flow | 1 (flow) + 9 extensions | 0 | 11 | N | N | **today** 20:38 (actively building) | Y (flow + 9 `flow-extension-*`, drifted names — see §2) | Y | ☀️15 FLOW-GRAPH-SLIDER-KNOB-SIZE / FLOW-MINIMAP-SQUARE-CORNERS | embedded in procedural/wgpu work; no standalone E2E ticket |
| 🌍️gis | 2 (gismap, gisterrain) | 0 | 3 | Y (both) | N | 09-17 13:44 | Y | Y | ☀️17 DEMONSTRATOR-GIS-MAP-STATIC-TILES (after ☀️16 GIS-2D-END-TO-END-BUILD) | IN-PROGRESS |
| 🌿️vcs | 1 (vcs) | 0 | 2 | N | N | none found | Y | Y | none dedicated | UNTESTED |
| 🎞️animate | 1 (presentation) | 1 (🎬️presentation) | 2 | N | N | none found | Y | Y | ☀️06 RESTORE-REVEAL-JS-PRESENTATION-PRODUCT-FRAMEWORK (reveal.js infra, not plugin boot) | UNTESTED |
| 🎥️shooting | 1 (shooting) | 0 | 2 | N | N | 09-16 23:28 | Y | Y | ☀️17 SHOOTING-ICON-CAMERA-PROJECTION-PARITY (after ☀️16 SHOOTING-PLUGIN-END-TO-END, port 6019) | BOOTS |
| 🎪️demonstrator | 1 (playground) | 0 | 2 | Y (plugin) | N | 09-17 20:12 | Y | Y | ☀️17 DEMONSTRATOR-* cluster (BUILD-CLOSURE, GENERATOR-SHIP-PLUGIN-BOOT, GIS-MAP-STATIC-TILES, NX-BUILD-CACHE, REMOVE-DEBUG-CONSOLE, STATIK-DEFAULT-BETONWALD) | IN-PROGRESS |
| 🎬️sequence | 1 (sequence) | 0 | 4 | N | Y (1) | none found | Y | Y | none dedicated | UNTESTED |
| 🏗️fem | 2 (3d, 2d) | 0 | 5 | Y (3d,2d) | N | 09-17 22:52 | Y | Y | ☀️17 FEM-3D-DISTRIBUTION-BUILD (after ☀️06 FEM-PLUGIN-END-TO-END closed 09-16, ☀️16 2D/3D-INTERACTIVE-FEATURE-COMPLETE) | BOOTS (fem2d proven, port 6086) |
| 🏛️architect | 1 (program) | 0 | 2 | N | N | none found | Y | Y | none dedicated | UNTESTED |
| 🏭️process | 1 (process3d) | 0 | 6 (+4 material extensions) | N | N | 09-17 22:17 | Y (process + 4 `process-extension-*`, drifted names) | Y | ☀️16 PROCESS-CONCRETE-FOREST-EXAMPLE ("vite on 6222 wedges on peers' registry regen") | IN-PROGRESS (known wedge) |
| 💠️lowpoly | 1 (lowpoly) | 0 | 2 | N | N | **today** 16:04 | Y | Y | no Sept E2E ticket (last proven boot recipe is 26/08/29, port 6078); 1 `#[ignore]` = framework gap (`attach_backbone` fail-closed) | BOOTS (last verified 08/29, not re-verified in Sept) |
| 💡️reasoning | 1 (wires) | 0 | 2 (plugin crate literally named `semio-s-plugin-reasoning-mindmap` — **id drift**, see §2) | N | N | none found | Y | Y | none dedicated | UNTESTED |
| 📋️forms | 1 (forms) | 0 | 2 | N | N | 09-16 19:21 | Y | Y | ☀️16 FORMS-PLUGIN-END-TO-END (after ☀️03 tool-job-factory ticket) | BOOTS (port 6058, known `VITE_SEMIO_APP_ID` pin caveat) |
| 📏️layout | 1 (layout) | 0 | 2 | N | N | **today** 16:58 | Y | Y | ☀️18 LAYOUT-PDF-EXPORT-END-TO-END (**today**, after ☀️16 LAYOUT-PLUGIN-END-TO-END, port 6079) | BOOTS, active work today |
| 📐️cad | 1 (cad) + 4 extensions | 0 | 6 | N | N | 09-17 22:56 | Y (cad + 4 `cad-extension-*`, drifted names) | Y | ☀️15 DEV-CAD-REACT-E2E (port 6020) | BOOTS (09-16 baseline), extensions unverified |
| 📕️norm | 15 (en1990–en1999, din4108, din16798, din18599, iso16757, vdi3805) | 0 | 17 | N | N | none found | Y | Y | none dedicated | UNTESTED (reference-data artifacts; 15 playground entries declared but never booted) |
| 📖️playbook | 1 (playbook) | 0 | 3 (+ procedural extension) | N | N | none found | Y (playbook + `playbook-module-procedural`, drifted name) | Y | none dedicated | UNTESTED |
| 📜️imperative | 1 (procedure) | 0 | 6 (+5 extensions) | N | N | none found | Y (imperative + 5 `imperative-extension-*`, drifted names, 3/5 emoji mismatches) | Y | none dedicated | UNTESTED |
| 📸️remodel | 1 (remodeling) | 0 | 2 | N | N | **today** 16:58 | Y | Y | ☀️06 REMODEL-PLUGIN-END-TO-END (port 6063) | BOOTS |
| 🔋️energy | 1 (model) | 0 | 2 (+ python oracle pkg) | N | N | 09-17 20:59 | Y | Y | ☀️17 ENERGY-MESH-SELECTION-HOVER-PARITY (after ☀️06 E2E, ☀️16 tree inspector) | BOOTS |
| 🔱️trinity | 2 (jack, rewriting) | 0 | 5 | Y (jack, rewriting, plugin) | N | **today** 01:35 | Y | Y | ☀️17 TRINITY-PLUGIN-END-TO-END | BOOTS (jack :6054 / rewriting :6056, two activations share one wasm — 5 historical boot faults) |
| 🕸️dag | 1 (dag) | 0 | 2 | N | N | none found | Y | Y | none dedicated | UNTESTED |
| 🖍️draw | 1 (drawing) | 0 | 6 | N | N | **today** 16:11 | Y | Y | ☀️05 DRAW-PLUGIN-END-TO-END; feeds ☀️18 PDF export tickets today | BOOTS |
| 🖨️raster | 1 (raster) | 0 | 2 | N | N | 09-16 23:32 | Y | Y | ☀️17 RASTER-DEFAULT-EXAMPLE-COMPOSITE (after ☀️05 E2E: "boots in the react renderer (and wgpu wasm) … confirmed with console logs") | BOOTS |
| 🗄️stdio | 39 file-format codecs | 0 | 65+ | N (library plugin, no UI app) | N | none (not a UI-boot plugin) | Y | Y | ☀️18 PDF-ARTIFACT-SPEC-COMPLETE (**today**) | N/A (I/O library; heaviest test debt, see §3) |
| 🗒️note | 1 (note) | 0 | 3 | N | N | 09-17 22:43 | Y | Y | ☀️17 NOTE-PLUGIN-END-TO-END (port 6080, 9 faults incl. `ink.document` wire drift, all fixed) | BOOTS |
| 🗟️**artifacts** | **0** | **0** | **0 (no Cargo.toml at all)** | — | — | — | **N — absent from catalog.json** | **N — not a workspace member** | **none, ever** | **DOES NOT EXIST as a plugin** (empty directory skeleton, see §2) |
| 🧩️puzzle | 3 (3d, 5d, 2d) | 0 | 4 | Y (all) | N | **today** 19:38 | Y | Y | ☀️17 PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D (after ☀️02/☀️06 E2E + 10 interaction tickets ☀️13–16) | BOOTS (most mature plugin; 3d/2d proven, 5d catching up, port 6012/6013/6014) |
| 🧱️block | 3 (3d, 5d, 2d) | 0 | 4 | Y (all) | N | none found (no recent rebuild evidence) | Y | Y | ☀️05 BLOCK-PLUGIN-END-TO-END ("Each of the three playgrounds boots in the react renderer … no bounded-factory faults") | BOOTS per ☀️05, not reconfirmed since, no fresh wasm build |
| 🪐️space | 2 (home, space) | 0 | 3 | Y (all) | N | none found | Y | Y | none dedicated | UNTESTED (despite CAA + workspace membership) |
| 🪵️sourcing | 1 (curation) + 3 extensions | 0 | 5 | N | N | 09-17 20:55 | Y (sourcing + 3 `sourcing-module-*`, drifted names) | Y | ☀️17 SOURCING-PREVIEW-CATALOGUE-STAGING (after ☀️01 E2E, ☀️16 grid-curation-only) | IN-PROGRESS/BOOTS (moderate confidence) |

## Q1 — Boot status today (evidence-based)

**BOOTS (proven, with a Sept ticket stating console-log-confirmed or battery-passing boot):** raster, forms, note, remodel, fem (fem2d specifically), draw, energy, trinity, puzzle, shooting, layout, cad (09-16 baseline), block (09-05, unconfirmed since), lowpoly (08-29, unconfirmed since).

**IN-PROGRESS / known-unstable (boots but hits wedges, faults, or is mid-rework):** procedural (recurring vite wedge, biggest single ticket in the repo), process (vite wedges on 6222 during registry regen), demonstrator (active fault-fixing cluster ☀️17), gis (built but boot maturity thin), sourcing (moderate confidence), wfc (extracted **today**, a driving-job bug just found).

**UNTESTED (declared as a plugin with a playground entry, registered in catalog + workspace, but zero Sept ticket and zero boot evidence found):** writer, mathematical, vcs, animate, sequence, architect, reasoning, norm (15 reference-code artifacts), playbook, imperative, dag, space. Twelve of the thirty-four real plugins — over a third — have never been driven through a React/wgpu boot in the month captured by the ticket log.

**Does not exist / N/A:** 🗟️artifacts (empty skeleton, not a plugin); 🗄️stdio (I/O codec library, has no UI app to boot by design — it is consumed by the other 34, not booted itself).

## Q2 — Registry inconsistencies

1. **`🗟️artifacts` is a dead stub.** `find` over the whole directory returns zero files — no `Cargo.toml`, no `🗿️artifacts/`, no `🎛️apps/`, nothing except a couple of empty nested directories (`◻️2d/🏅️standards/🔖️1`). It is absent from `🗺️catalog.json`'s 60-entry module list and absent from the root `Cargo.toml` workspace members. `git log --diff-filter=A` finds no history for the path (git never tracked it — it holds no files). The only repo references are two unrelated audit docs from other tickets. **This is the one plugin in the requested list of 35 that is not a plugin at all.**

2. **Systemic `directoryName` drift for every nested extension/module sub-plugin in `🗺️catalog.json`.** All 34 top-level entries match disk exactly (e.g. `cad` → `📐️cad`, `wfc` → `🀄️wfc`). But all 26 nested `*-extension-*`/`*-module-*` entries use a flat `"<parent>-extension-<name>"` naming scheme that has never existed on disk — the real layout is `<parent>/🧩️extensions/<emoji><name>`. Examples:
   - `cad-extension-aec-building` → catalog says `🏢️cad-extension-aec-building`; disk path is `📐️cad/🧩️extensions/🏢️aec-building`.
   - `imperative-extension-logic` → catalog emoji `⚖️`; disk emoji is `🧠️` (`📜️imperative/🧩️extensions/🧠️logic`) — **the emoji itself is wrong, not just the path shape.**
   - `imperative-extension-math` → catalog `➕️` vs disk `🧮️`; `imperative-extension-text` → catalog `🔡️` vs disk `📝️`.
   - `process-extension-wood` → catalog `🪓️` vs disk `🪵️`; `process-extension-concrete` → catalog `🏙️` vs disk `🧱️`.
   - `sourcing-module-beams` → catalog `🪜️` vs disk `🪵️`; `sourcing-module-slabs` → catalog `🧇️` vs disk `🧱️`.
   - `flow-extension-brep`/`-bim`/`-dictionary`/`-logic`/`-draw` → 5 of 9 flow extensions have mismatched emoji too.
   
   Roughly half of the 26 nested entries have a wrong emoji *and* every one of the 26 has a directory-name shape that cannot resolve on disk. If anything downstream trusts `catalog.json`'s `directoryName` literally to find a sub-plugin's descriptor, every extension/module lookup fails; if nothing trusts it (i.e. it's purely a display/id list), it is stale documentation that will mislead the next person who edits it. Either way it needs a regeneration pass. **P0/P1** — see gap list.

3. **No duplicate-emoji collisions among the 60 catalog entries themselves** (checked programmatically) and **no id drift found** for the plugins actually checked in depth (`wfc` → `package = "semio:wfc"` matches `pluginId: "wfc"`; `draw` → `"semio:draw"` matches `"draw"`). The taxonomy/builder-id convention (`package_id.strip_prefix("semio:")` in `🧰️framework/…/🔌️plugin/🏗️builder/🦀️.rs:618`) appears respected everywhere it was sampled.

4. **One crate-name/plugin-id drift found:** `💡️reasoning`'s plugin crate is named `semio-s-plugin-reasoning-mindmap` while the plugin id/directory/artifact are all `reasoning`/`wires` — a leftover from an earlier "mindmap" name that was never renamed to match its folder.

5. **`✏️s/🔌️plugins/🔒️policy-allowlist.json` is not a registry** — see the scoping note above. It is a per-rule file-path allowlist (56 KB, keyed by rule id like `semantic-vocabulary`), unrelated to boot/registration.

## Q3 — Test coverage gaps

Counted `#[test]` and `#[ignore]` occurrences per plugin directory (no `cargo test` run repo-wide). Full detail in `🗑️generated/plugins-tests.txt`.

**Thin test coverage (candidates for P2 backfill), ≤20 `#[test]`s in the whole plugin tree:**
`✒️writer` (13), `➗️mathematical` (5), `🌿️vcs` (12), `🎥️shooting` (13), `🏛️architect` (23), `💠️lowpoly` (13), `💡️reasoning` (15), `📋️forms` (16), `📜️imperative` (6), `📖️playbook` (8), `🪐️space` (7), `🪵️sourcing` (10), `🗒️note` (10), `🎬️sequence` (20). `🗟️artifacts` has 0 of everything (it has no source files).

**Heaviest test owners (for contrast, no action needed):** `🗄️stdio` 2143 tests / 627 files, `🏗️fem` 1783/297, `🧩️puzzle` 1868/280, `🀄️wfc` 1343/207, `🌀️procedural` 626/127, `📸️remodel` 400/24.

**`#[ignore]`d tests found (24 total), triaged:**
- **Diagnostic-by-design (no action)**: most of `📸️remodel`'s 5 (regenerates committed fixtures / minutes-long stereo probes), `🏭️process`'s 2 and `🔋️energy`'s BESTEST case (documented "run explicitly" one-shot authoring/EnergyPlus comparisons), `🗄️stdio`'s fixture-derivation ones, `🧩️puzzle`'s 2 fixture-regeneration cases.
- **Real known-broken test (P1)**: `🗄️stdio` — `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/…/🏄️surface-surface/🧪️tests/🔬️unit/🦀️.rs:246` is ignored with `"hangs indefinitely (5+ min, real CPU burn) on skew-cylinder general_marching — owner: W2-A, needs profiling"` — an actual unresolved defect, not a diagnostic convention.
- **Framework-gap-by-design (tracked elsewhere)**: `💠️lowpoly`'s single ignore (`attach_backbone` fail-closed on remote snapshot merge — a documented framework limitation).
- **Stray/undocumented (P2 cleanup)**: `🔋️energy`'s second ignore, `…/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:318`, is only tagged `"[DEBUG] W3-1b probe"` with no rationale — looks like a forgotten debug skip rather than an intentional exclusion; worth a maintainer look.
- 4 unresolved `🀄️wfc` ignores are pre-extraction leftovers (the plugin was carved out of `procedural` **today**, ☀️18) and 8 `🗄️stdio` ignores besides the hang are documented one-off derivation/regeneration tools.

## Q4 — Which plugins have a built wasm component right now

No wasm build was triggered by this audit; the following is purely `find *.wasm` with existing mtimes under each plugin's `dist/component-dev|component-release`. 20 of the 34 real plugins have at least one built `.wasm`; 14 do not.

| Plugin | Latest wasm mtime | # wasm files |
|---|---|---|
| 🌊️flow | 2026-09-18 20:38 (mid-build at audit time) | 20 |
| 🧩️puzzle | 2026-09-18 19:38 | 3 |
| 🀄️wfc | 2026-09-18 17:23 | 1 |
| 📏️layout | 2026-09-18 16:58 | 1 |
| 📸️remodel | 2026-09-18 16:58 | 1 |
| 🖍️draw | 2026-09-18 16:11 | 1 |
| 💠️lowpoly | 2026-09-18 16:04 | 1 |
| 🔱️trinity | 2026-09-18 01:35 | 2 |
| 📐️cad | 2026-09-17 22:56 | 10 (incl. 4 extensions) |
| 🏗️fem | 2026-09-17 22:52 | 7 |
| 🗒️note | 2026-09-17 22:43 | 2 |
| 🏭️process | 2026-09-17 22:17 | 10 (incl. 4 material extensions) |
| 🔋️energy | 2026-09-17 20:59 | 7 |
| 🪵️sourcing | 2026-09-17 20:55 | 8 |
| 🎪️demonstrator | 2026-09-17 20:12 | 2 |
| 🌀️procedural | 2026-09-17 15:08 | 2 |
| 🌍️gis | 2026-09-17 13:44 | 2 |
| 🖨️raster | 2026-09-16 23:32 | 1 |
| 🎥️shooting | 2026-09-16 23:28 | 1 |
| 📋️forms | 2026-09-16 19:21 | 1 |

**No built wasm found:** ✒️writer, ➗️mathematical, 🌿️vcs, 🎞️animate, 🎬️sequence, 🏛️architect, 💡️reasoning, 📕️norm, 📖️playbook, 📜️imperative, 🕸️dag, 🧱️block (surprising — its sibling `🧩️puzzle` rebuilt today but `block` has no recent artifact despite an equivalent 3-crate `component-app-assembly` layout), 🪐️space. (🗄️stdio and 🗟️artifacts excluded: not applicable / not a plugin.)

This lines up almost exactly with the Q1 UNTESTED list — the plugins nobody has booted this month are the same ones nobody has even compiled to wasm this month.

## Q5 — Prioritized gap list for "all plugins and artifacts working in the os"

**P0 — not registered / does not compile / does not exist**
1. `✏️s/🔌️plugins/🗟️artifacts` is an empty directory shell: no `Cargo.toml`, not in `🗺️catalog.json`, not a workspace member. Either delete it or give it real content — right now it silently fails every audit that assumes the requested 35-name list are all real plugins. File to touch: the directory itself; decision belongs to whoever owns the plugin roster (no ticket currently owns it).
2. `🗺️catalog.json` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`) has systematically wrong `directoryName` values for all 26 nested extension/module entries (13+ with outright wrong emoji, all 26 with an unresolvable path shape). Regenerate this file from disk (or fix whatever generator produced it) before anything downstream trusts it to locate a sub-plugin's descriptor.

**P1 — registered/compiles but has a known boot fault or real test defect**
3. `🌀️procedural` / `🌊️flow`: recurring vite dev-serve wedges under load (documented root causes across multiple ☀️09 sub-tickets — chokidar/cargo write storms, stale transforms). Needs the structural watch-policy fix to be verified as durably landed, not just patched per-incident.
4. `🏭️process`: vite on port 6222 wedges on peers' registry regen (☀️16 PROCESS-CONCRETE-FOREST-EXAMPLE, open).
5. `🗄️stdio`: `…/🧊️brep/…/🏄️surface-surface/🧪️tests/🔬️unit/🦀️.rs:246` — real hang (5+ min CPU burn) on skew-cylinder `general_marching`, owner tag `W2-A`, unresolved.
6. `🀄️wfc`: freshly extracted today (☀️18 EXTRACT-WFC-PLUGIN); a job-driving bug ("EVERY SLICE DRIVING `WfcJob` FROM A PARENT JOB") was just found via a wedged test binary — needs to land before the extraction ticket closes.
7. `🧱️block`: last confirmed booting 26/09/05; no wasm rebuilt since despite `puzzle` (its structural sibling) rebuilding today — worth a fresh boot check before assuming it still works.

**P2 — missing tests / features / cleanup**
8. Thin-coverage plugins (≤20 tests): `✒️writer`, `➗️mathematical`, `🌿️vcs`, `🎥️shooting`, `🏛️architect`, `💠️lowpoly`, `💡️reasoning`, `📋️forms`, `📜️imperative`, `📖️playbook`, `🪐️space`, `🪵️sourcing`, `🗒️note`, `🎬️sequence` — same plugins that are also boot-UNTESTED; a boot probe + a handful of unit tests would retire both gaps together.
9. `🔋️energy`'s undocumented `#[ignore = "[DEBUG] W3-1b probe"]` (`…/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:318`) needs either a real rationale comment or removal.
10. `💡️reasoning`'s plugin crate name `semio-s-plugin-reasoning-mindmap` should be renamed to match its `reasoning` directory/id (cosmetic but confusing to anyone grepping crate names).
11. The 12 UNTESTED plugins (`writer`, `mathematical`, `vcs`, `animate`, `sequence`, `architect`, `reasoning`, `norm`, `playbook`, `imperative`, `dag`, `space`) all declare `[[package.metadata.semio.playground]]` entries in their `Cargo.toml` (so an app/port *is* intended) but have never had a dedicated `-PLUGIN-END-TO-END` ticket, never been wasm-built recently, and have no committed activate/serve scripts anywhere outside a ticket folder. Each needs its own boot-recipe ticket following the pattern already proven for `raster`/`forms`/`note`/`layout` (`📜️activate-<plugin>-react.sh` + `📜️serve-<plugin>-react.sh` + a console-dump probe).

## Files referenced
- Registry: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`
- Workspace manifest: `Cargo.toml` (root)
- Dependency ledger: `🔒️dependencies.json` (root)
- Builder id convention: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:618`
- Policy allowlist (scoping note only): `✏️s/🔌️plugins/🔒️policy-allowlist.json`
- Raw captures: `🗑️generated/plugins-catalog.txt`, `🗑️generated/plugins-inventory.txt`, `🗑️generated/plugins-tests.txt`, `🗑️generated/plugins-wasm-artifacts.txt`
