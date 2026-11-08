# 1107 Audit: What "Working End To End" Means For The Mit-Bestand Demonstrator

Lane `aud-workflow`. Read-only static audit: nothing was run (no repo, cargo, bun, Playwright or e2eready execution). "Asserts" means read from source.

Short names used below (repo root `/Users/ueli/Documents/semio/`):
- `D` = `♻️mit-bestand/🧺️demonstrator`
- `spec:` = `D/🧪️tests/🎭️acceptance/🟦️.ts`
- `probe:` = `C/1105b-tools/e2eready/probe.ts`
- `brand:` = `D/🪧️brand.ts`
- `locale:` = `D/🔨️modules/🌐️locale/🔣️.json` (texts keyed by `demonstrator.brand.*`; the en value is quoted)
- `landing:` = `D/🟦️.tsx`
- `zwischenbericht.tex` = `♻️mit-bestand/📋️bericht/📋️zwischenbericht/📋️zwischenbericht.tex`

## 1. Verdict

- As designed, the eight panes are independent live apps shown side by side in one overview. No pane reads or writes another pane's document. The only shared wiring is code, plugin distribution and policy (section 2).
- The only documented cross-pane story (Bauteilportal, Wizard, Generator, Bauteilobjekt, Entwurfswerkzeug) lives in the Zwischenbericht. It records that chain as not yet connected (`zwischenbericht.tex:598`). The demonstrator code, brand copy and tests do not implement or require it.
- The demonstrator's own definition of end to end is three separate checks: a passing prototype browser run, release build publication, and live-service readiness (`D/README.md:75`). The brief's definition (`C/1107-brief.md:6`: boot, show fixture, act with undo/redo, en and de, real browser) matches the per-pane acceptance and says nothing about cross-pane flow.
- Every pane has one primary acceptance flow (boot, content, one action with undo/redo, in en and de). Several features the brand copy describes have no test (sections 3 and 4).
- Status: verification is "pending" (`D/README.md:75`), and the brief records all pane smokes RED at stop (`C/1107-brief.md:11`). Not verified here.

## 2. Do Panes Share Data?

Evidence:
- Each pane is its own shell. `DemonstratorShell` mounts one `FrameworkOsShell` per pane with `pluginFilter={variants.runtime}`, `storageNamespace={pane.id}` and `shellId={pane.id}` (`landing:76-98`, esp. 85, 93-94).
- Every pane brand is `ephemeral: true` (`brand:360, 518, 567, 620, 663, 712, 778, 827`). The landing says so: "ephemeral brands → in-memory storage" (`landing:43-44`). Nothing persists across reloads and nothing is keyed to another pane.
- The page's own storage is used only for chrome appearance and locale (`landing:45, 50, 144-146`).
- The shared object is `FRAMEWORK_OS_HOST_PORTS = createFrameworkOsHostPortsV1()` (`landing:48`). Its bindings are plugin catalog admission, plugin distribution (hub source, caches, service worker), directory access policy for `document.write`, the preference vocabulary and the wasm bindings (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🪝️host-ports/🟦️.ts`, bindings block). No document store is among them.
- The runtime catalog `D/🔨️modules/🧩️runtime/🔣️.json` maps eight variants to runtime variants. It has no edges between panes.
- No test in `D/🧪️tests/` or `spec:` asserts data crossing panes. The one multi-pane test (`spec:728-742`, aussuchen revisit) visits `bearbeiten` only to prove that an edited aussuchen pane stays mounted (`spec:733`).

Conclusion: side by side. This matches the copy: "eight live panes" (`D/README.md:3`) and "pages of one LayeredOverview" (`landing:2`).

Documented intent outside the demonstrator (the mapping of panes to these components is inferred):
- `zwischenbericht.tex:414`: "Die Informationen einer Baukomponente werden in einem gemeinsamen Bauteilobjekt zusammengeführt." (one shared component object, fed from the Einspeiseplattform)
- `zwischenbericht.tex:424`: views inside the Entwurfswerkzeug "über die Daten des Test-Case miteinander verknüpft" (linked through test-case data)
- `zwischenbericht.tex:458` and `:601`: the chain is still to be joined to Bauteilportal, Wizard and Generator; the complete flow "bis zur Entwurfsvariante" is still to be checked.
- `zwischenbericht.tex:598`: "Eine durchgängige Verbindung mit Bauteilportal, Wizard und Generator lag noch nicht vor."
- `♻️mit-bestand/📋️bericht/📋️zwischenbericht/📎️anhang/👥️user-stories.tex:153-169` lists REQ-01 to REQ-17 against the Einspeiseplattform and the Entwurfswerkzeug (Katalog, Entwurf, Konfigurator).

Conflict to decide (inferred). The koordinator copy promises the pane "connects form, building, energy and structural models in shared views" (`locale:117`, also `:93`). The acceptance test asserts the opposite inside that pane: adding a box to `cad-play-shape` must leave `cad-play-building`, `cad-play-energy` and `cad-play-structure-classic` unchanged (`spec:995, 1000`; probe `catalogueAdd`, `probe:241-282`, with `unchanged`). The docs do not say which is right.

## 3. Intended Behaviour Per Pane

Order: the grid order is generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik (`brand:869-870, 911-968`). No narrative order between panes is documented.

### generator (runtime generation3d; default example `hexagonal-mushroom-column`, `brand:517`)
- Documented: "from rules and parameters to finished parts" (`locale:114`); "explore the flow editor in full screen and load examples" (`locale:114`); zoom, pan and orbit on the flow editor (`brand:531-540`).
- Tested: boot; node graph with seven widgets and six synapses by id (`spec:220, 938-950`); non-empty preview (`spec:225`); height 6 to 6.5 changes the preview geometry; undo and redo restore graph and geometry (`spec:932-975`; probe `graphValueEdit`, `probe:212-239`).
- Not tested: loading an example as an action; "finished parts" (no output or export check); the intro steps for zoom, pan, orbit and catalogue (`brand:523-553`) are only dismissed; Bauteilportal and Wizard intake, which the report says is still to come (`zwischenbericht.tex:399`).

### koordinator (runtime koordinator, cad; default `demo`, `brand:566`)
- Documented: "connects form, building, energy and structural models in shared views" (`locale:117`); "compare and merge several CAD windows side by side" (`locale:117`); "Click the Catalogue tab to browse the available building blocks" (`locale:94`).
- Tested: four windows carry content (`spec:233-240`); adding a box to the shape window changes only that window, and undo and redo restore it (`spec:977-1003`; probe `catalogueAdd`).
- Not tested: compare; merge; any catalogue kind other than the box (`D/🧪️tests/🎭️acceptance/🧬️edits/🧫️cases.json:24-33`); any propagation between models (this contradicts the isolation assertion, see section 2).
- Stale status text: `spec:236` says the cad-play-shape fix is "NOT yet compile-verified".

### aggregator (runtime aggregator, puzzle3d; default `concrete-forest`, `brand:359`)
- Documented: "assemble designs from existing components in three-dimensional space" (`locale:120`); "place parts from the catalogue and check the assembly" (`locale:120`); tour: "view, catalogue, transform, connection points and fill" (`locale:46`).
- Tested: both instances (top and perspective) carry content (`spec:245`); a catalogue kind added to both windows gives a new `puzzle3d.object.<hex>` with the seed's mesh, 11 vortices and a document row; undo and redo (`spec:1005-1038`; probe `catalogueAdd`, `probe:241-282`).
- Not tested: transform (move and rotate), described at `brand:440-450` and checked only as data in `D/🧪️tests/🏛️shell-brand/🟦️.ts:83-85`; "Show connection points" set to Always (`locale:64, 80`; `brand:452-462`); suggest and place with hover preview (`locale:82`; `brand:464-477`); the Fill tool, its count and its distribution (`locale:66-67, 86`; `brand:479-501`); playback of the four-minute tour (`brand:143-349`). No test in `D` references `ENTWERFEN_MIT_BESTAND_TUTORIAL`.
- Authors' own gap note: the tour's document track is empty, and "it just won't materialize the document edits themselves" (`brand:140-141`, `brand:291`).

### energie (runtime energy; default `bestest-600`, `brand:777`)
- Documented: "simulates the thermal behaviour of your design" (`locale:123`); "zones, envelope surfaces and boundary conditions" (`locale:106`); "view the model and the simulation run side by side" (`locale:123`).
- Tested: four windows carry content (`spec:249-273`); a run starts with Ctrl+Enter, shows progress, finalizes, reports the final step, accepts the result; a restart opens a new group; abort; no result accepted (`spec:788-824`; probe `toolRun`, `probe:338-361`). The restart rule is unit-tested as data (`D/🧪️tests/🎭️acceptance/🧬️simulation/🧫️cases.json`, consumed by `🧬️console/🧪️.ts:52-54`).
- Not tested: editing zones, envelope surfaces or boundary conditions (no action exists); result values (regex on text only, `spec:808`); "the simulation never writes the document" is asserted as text (`spec:808`, `simulation/🧫️cases.json`) but no check shows the model or zone table unchanged after a run (inferred gap); side-by-side layout (only presence of the windows is checked).

### aussuchen (runtime aussuchen, sourcing; default `demo`, `brand:619`)
- Documented: "survey the available existing components and compile a curation" (`locale:97`); the pool "lists all components found with their availability and typology" (`locale:97`); "filter by module and typology and place components in the selection" (`locale:126`).
- Tested: pool rows; the placeholder "No selection" (`spec:644`; probe uses substring match, `probe:490`); selecting a row shows that kind in the preview (`spec:637-658`; probe `rowSelectPreview`, `probe:363-374`); curation count by increment, ArrowUp, remove, undo and redo, with grid geometry matching the count (`spec:684-744`; probe `curationUndoRedo`, `probe:408-464`); `aria-valuemax` equals the availability of 24 (`spec:697`; fixture `🧬️curation/🧫️cases.json:4`); the count survives a 31-second revisit (`spec:732-742`).
- Not tested: filter by module or typology (no action exists); typology in the pool (only name and availability are asserted); count cannot exceed availability (only the attribute is read); any path by which the curated set reaches another pane (none documented or tested).

### bearbeiten (runtime bearbeiten, process3d; default `concrete-forest`, `brand:662`)
- Documented: "define the processing steps with which an existing component is adapted" (`locale:100`); "define and visualise processing chains on the workpiece" (`locale:129`).
- Tested: seven step rows (`spec:1047`; `D/🧪️tests/🎭️acceptance/🧬️edits/🧫️cases.json:44-57`); selecting a step shows id, label, machine, capability and enabled (`spec:1051-1056`); toggling Enabled changes the workpiece geometry; undo and redo (`spec:1040-1076`; probe `rowToggle`, `probe:284-313`).
- Not tested: defining, adding, removing or reordering steps; chain logic beyond the geometry change; validity of machine and capability values; the intro steps for zoom, pan and orbit (`brand:676-685`).

### verfolgen (runtime verfolgen, gis2d; default `demo`, `brand:711`)
- Documented: "shows spatially where existing components come from and where they go" (`locale:132`); "zoom to locations and explore the connections" (`locale:132`); "Zoom with the mouse wheel and pan the map" (`locale:103`).
- Tested: the map paints more than one colour (`spec:616-618`; probe `probe:492-495`); summary counts for positions, routes and regions; adding a feature at longitude 0, latitude 0 raises positions by one and leaves routes and regions unchanged; undo and redo (`spec:1078-1106`; probe `featureAdd`, `probe:315-336`).
- Not tested: where components come from or go (routes and regions are only compared, never read); zoom to a location; any real coordinate (fixture `🧬️edits/🧫️cases.json:59-65` uses 0, 0); the map cannot be graded on a software GPU (`D/README.md:29`, `probe:808`).

### statik (runtime fem3d; default `concrete-forest`, `brand:826`)
- Documented: "calculates the structure from reused building components" (`locale:110`, en "In Statics you calculate the structure from reused building components"); "edit the FEM model and check the structural response" (`locale:135`).
- Tested: Model and Results windows carry content (`spec:298-313`); the "Case: dead" caption settles; the phase slider and transport are paused; the deformation scale changes 300 to 600 and the result projection changes; undo and redo restore scale and projection (`spec:834-882`; probe `paramScale`, `probe:376-406`).
- Not tested: editing the FEM model (nodes, members, supports, loads), since no action exists; a new solve after a model change (inferred). The expected size for the brand default is written in a comment and not asserted (`spec:304`).
- Report: simplified structural systems and connection checks are listed as future work (`zwischenbericht.tex:441-444, 571-572`).

## 4. Intended Behaviour Across Panes And The Page (no test)

- Lifecycle: boots start 1.5 s after load, one per 35 s; a pristine app goes to its poster after 30 s offscreen, 5 min idle, or 60 s in a hidden tab (`landing:67-69` constants, comment at `landing:67-68`). Only edited-pane retention is tested (`spec:733`). Poster release, boot stagger and hidden-tab release are not.
- Responsive: swipe mode for touch phones (`landing:57, 141`) is untested. The e2e config uses Desktop Chrome only (`D/🔨️modules/🧪️e2e/🎚️config/🟦️.ts:57`); the probe uses 1600x1000 (`probe:138-141`). The repo rule orders desktop, then mobile, then tablet (`AGENTS.md`, repo rule, not demonstrator copy).
- Language: "with no default language" (`D/README.md:3`). Picker and stored choice are tested (`spec:513-532`; probe `runPicker`, `probe:584-614`). Resolution from browser-preferred languages is only implicit, because every test sets `locale` explicitly (`spec:68-71`). Inferred gap.
- Tutorial (Aggregator tour): no test. Intro demonstrations: checked only as data in `D/🧪️tests/🏛️shell-brand/🟦️.ts:54-97`, never played in a browser; acceptance dismisses intros (`spec:166-183`).
- Funding and overview cards are tested (`spec:482-510, 1108-1131`; probe `runLanding` and `runFunding`, `probe:543-582, 676-698`). Neighbour hints (`locale` keys `demonstrator.landing.neighbour*`) are not.
- Accessibility: tests check `lang` and `aria-pressed`; there is no accessibility audit (inferred gap against the repo rule).
- The Projektetage deck is outside this demonstrator; its slides show no pane workflow (grep finds only generic "Generator" slides).

## 5. Acceptance (`spec:`) Versus e2eready (`probe:`)

Shared: the same eight pane ids (`spec:573`; probe `brandPaneIds`, `probe:34-38`; `--check` at `probe:717-718`) and the same window kinds (`probe:719-727`, which is the only parity check). `spec:` has 25 cases per language project, which matches `D/README.md:75`.

Per pane, the probe runs one action; the spec runs the same flow plus extra checks:
- generator: spec compares widget and synapse ids and the full graph on undo and redo (`spec:948-949, 966-973`); probe checks the height value and preview geometry only (`probe:212-239`).
- statik: spec asserts phase zero changes the projection (`spec:851`) and the transport state after each step (`spec:878-879`); probe checks phase value, scale and transport (`probe:386-405`) but not the zero-phase projection.
- aussuchen placeholder: spec requires exact text (`spec:614, 644`); probe requires a substring (`probe:490`).
- energie restart rule: spec imports the shared `isNewToolRunGroup` (`spec:815`); probe repeats the rule inline (`probe:355`). Drift risk.
- Helpers: the probe copies `elementIdSegment` and `windowId` (`probe:47-58`) from the spec (`spec:96-118`, whose comment calls it a mirror). Drift risk.
- Equal in both: koordinator, aggregator, bearbeiten, verfolgen, and the funding, picker, switch and remount flows.

Neither covers anything in sections 3 and 4 that is not already listed.

## 6. Inferred Items (not documented)

- The order of the story between panes.
- The mapping of panes to report components: Generator to generator, Katalog to aussuchen, Entwurfsumgebung to aggregator or koordinator, Tragwerksanalyse to statik, energy to energie. Inferred from names.
- That an aussuchen-to-aggregator or statik handoff is intended.
- That koordinator "connects" means propagation between windows. The copy is ambiguous.
- That "never writes the document" should be verified with a projection check.
- The accessibility and mobile gaps, which come from repo rules rather than demonstrator copy.

## 7. Decisions For The Coordinator

1. Koordinator: propagate between windows, or stay isolated? The copy and `spec:995` disagree.
2. Is the Aggregator tour in scope for "working"? The authors mark its document track incomplete (`brand:140-141`).
3. Is FEM model editing in scope for statik? The copy says edit; no test does.
4. Is cross-pane handoff in scope for this wave? The docs make it a future step, and the demonstrator does not implement it. Recommendation: no.
