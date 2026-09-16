# W1-E — verification harness (probes + scripts) for the energy 3d / tree / inspector ticket

> §1–§5 are the pre-wave-1 harness and its baselines. **§6 is the wave-1 outcome** and **§7 is the results-recolour
> timeline** (why DoD 4 is still red) — read those first if you only want to know what the restaged build does today.

Lane W1-E, session ⚪c7d63d40, 2026-09-16 16:40–17:00. No Rust touched, no builds run, no servers started.
Everything below was authored in this ticket folder and run against the **predecessor's already-serving react dev
app on `http://127.0.0.1:6106/?plugin=energy`** (screen session `energy-serve`, started by the 26/09/06 ticket) —
i.e. against a build that has the OLD three windows and none of wave 1's work.

## 1. Files landed in this ticket folder

| File | What it is |
| --- | --- |
| `📜️activate-energy-react.sh` | copy of the predecessor's restage script; `LOG` now points at **this** ticket's `🗑️generated/activate-energy-react.txt`. Screen session name kept: `energy-activate`. |
| `📜️serve-energy-react.sh` | copy of the predecessor's serve script; `LOG` → this ticket's `🗑️generated/serve-energy-react.txt`. Screen session name kept: `energy-serve`. Still 6106 / `SEMIO_PLUGIN=energy` / `SEMIO_RENDERER=react`. |
| `📜️describe-energy.sh` | copy; `LOG` → this ticket's `🗑️generated/describe-energy.txt`. |
| `🐍️energy-console-dump-probe.mjs` | copy of the predecessor's boot probe, unchanged (it already resolves its out-dir from `import.meta.dir`, so it writes into this ticket). |
| `🐍️energy-3d-probe.mjs` | **new** — DoD 1 + the 3d half of DoD 2. |
| `🐍️energy-panels-probe.mjs` | **new** — DoD 2 + DoD 3. |
| `🐍️energy-results-probe.mjs` | **new** — DoD 4. |
| `🐍️energy-dom-dump.mjs` | **new** (wave 1) — read-only diagnostic: dumps every panel tab, every `panel:<ns>/<row>` row and every form control per step. Run this first when a panel assertion fails. |
| `🗑️generated/baseline/` | boot-probe baseline (console, `state.json`, `final.png`). |
| `🗑️generated/baseline-3d/`, `baseline-panels/`, `baseline-results/` | the three new probes run once against the current (pre-wave-1) build. |

All three new probes share one shape, on purpose:

- **they never throw.** Every check goes through `assert(id, ok, detail)`; a missing window / row / control is a
  `FAIL` line with a plain-English explanation, not a stack trace. The whole body is wrapped in try/catch, and a
  crash is itself recorded as `probe.completed: false`.
- **stdout is the summary**: one `PASS`/`FAIL`/`SKIP` line per assertion, then
  `RESULT=PASS|FAIL passed=N failed=M`, then the failing assertions repeated. Exit code 0 only when `failed === 0`.
- **artefacts**: `🗑️generated/<SEMIO_PROBE_OUT>/report.json` (assertions + every raw sample), `console.txt`
  (full browser console + pageerrors, timestamped), and a `.png` per step.
- **env knobs** like the predecessor probes: `SEMIO_PROBE_URL`, `SEMIO_PROBE_OUT`, `SEMIO_PROBE_SECONDS`, plus
  per-probe knobs documented in each file's header.
- chromium is launched with `--use-angle=metal` (headless chromium otherwise falls back to SwiftShader and the
  WebGL numbers stop meaning anything).

## 2. How to run each probe after the restage

Restage + serve first (coordinator, main session — subagent-started servers die with the agent):

```zsh
cd /Users/ueli/Documents/semio
screen -S energy-activate -dm ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR/📜️activate-energy-react.sh"
# wait for `exit=0` in 🗑️generated/activate-energy-react.txt, then (only if 6106 is not already serving):
screen -S energy-serve  -dm ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR/📜️serve-energy-react.sh"
curl -s -o /dev/null -w '%{http_code}\n' "http://127.0.0.1:6106/?plugin=energy"
```

### 2.1 Boot / regression baseline

```zsh
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR"
SEMIO_PROBE_OUT=boot-w1 SEMIO_PROBE_SECONDS=120 bun "🐍️energy-console-dump-probe.mjs"
```

### 2.2 3d model window (DoD 1)

```zsh
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR"
SEMIO_PROBE_OUT=energy-3d-w1 \
SEMIO_PROBE_WINDOW_3D=energy.model.3d \
SEMIO_PROBE_EXAMPLE="BESTEST 620" \
SEMIO_PROBE_MIN_INSTANCES=8 \
SEMIO_PROBE_DOMAIN=energyModel \
SEMIO_PROBE_SECONDS=120 \
  bun "🐍️energy-3d-probe.mjs"
```

> If lane B names the window kind something other than `energy.model.3d`, pass the real id in
> `SEMIO_PROBE_WINDOW_3D` — the probe also *finds* any World3d surface on its own and, when the id differs,
> fails with `a World3d surface exists under a DIFFERENT id (<what it found>)` so the rename is obvious.

### 2.3 Tree panel + inspector (DoD 2, 3)

```zsh
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR"
SEMIO_PROBE_OUT=energy-panels-w1 \
SEMIO_PROBE_TREE_PANEL=framework.panel.artifact \
SEMIO_PROBE_INSPECTOR_PANEL=framework.panel.inspection \
SEMIO_PROBE_FIELD='u.?value' \
SEMIO_PROBE_VALUE=1.5 \
SEMIO_PROBE_SECONDS=120 \
  bun "🐍️energy-panels-probe.mjs"
```

### 2.4 Results mode (DoD 4)

```zsh
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR"
SEMIO_PROBE_OUT=energy-results-w1 \
SEMIO_PROBE_WINDOW_3D=energy.model.3d \
SEMIO_PROBE_RESULT_FIELD=solarGain \
SEMIO_PROBE_RESULT_ACTION=set-result-field \
SEMIO_PROBE_SECONDS=120 SEMIO_PROBE_RUN_SECONDS=240 \
  bun "🐍️energy-results-probe.mjs"
# add SEMIO_PROBE_REQUIRE_FIELD_SWITCH=1 once lane D actually ships the result-field action
```

## 3. Baseline results (current pre-wave-1 build, 2026-09-16 ~16:50)

### 3.1 `🐍️energy-console-dump-probe.mjs` → `🗑️generated/baseline/`

Green, and identical in substance to the predecessor's closing probe:
`data-semio-os-ready=energy`, `data-semio-os-error=null`, title `semio · energy · model`, picker on `Demo`,
BESTEST 600 loaded (1 zone / 6 surfaces / 2 fenestrations / 12 materials / 4 constructions / 1 thermostat /
1 ideal-loads HVAC), zones table has its row, simulation window says `No energy simulation run`.

**One thing to know about the DOM**: the only `[data-surface-id]` host in the whole page today is
`window:framework.window.table` (the zones table). The tree and simulation windows are plain UI documents, not
scene surfaces. So "surface count" is a real signal for the new 3d window — it will be the second host to appear.

### 3.2 `🐍️energy-3d-probe.mjs` → `🗑️generated/baseline-3d/` — `RESULT=FAIL passed=2 failed=5`

| assertion | baseline | message |
| --- | --- | --- |
| `boot.ready` | PASS | `data-semio-os-ready=energy` |
| `world3d.present` | FAIL | `window:energy.model.3d absent — the served build publishes no World3d surface at all (surfaces seen: window:framework.window.table). Restage with 📜️activate-energy-react.sh once wave 1 lands.` |
| `world3d.instances` | FAIL | `skipped — no World3d surface to read` |
| `world3d.canvas` | FAIL | `skipped — no World3d surface to read` |
| `world3d.exampleRerender` | FAIL | `skipped — no World3d surface before/after the switch (picker said ok:BESTEST 900)` |
| `world3d.pick` | FAIL | `selectedIds=[] · aria-selected rows=[] · domain 'energyModel' seen=false · console hits=0` |
| `no.faults` | PASS | 0 fault lines |

Note the picker step still *works* on the baseline (`picked: "ok:BESTEST 900"`, model text follows) — so the
example-switch half of the probe is proven; only the 3d half is unmeasurable.

### 3.3 `🐍️energy-panels-probe.mjs` → `🗑️generated/baseline-panels/` — `RESULT=FAIL passed=2 failed=9`

`tree.panel` FAILs with
`no panel rows found — the artifact tree panel is absent (open attempt: id:framework.panel.artifact; panel namespaces in the DOM: none)`,
and the five family assertions plus both inspector assertions and `history.entry` report "skipped / not met yet".

Useful baseline detail for lane C: the shell **already renders the tab buttons** `framework.panel.artifact`
("Artifact"), `framework.panel.toolRun` ("Tool runs") and `framework.panel.history` ("History") — the Artifact
button exists and clicks fine; it just has no guest body behind it. So the only missing piece on the DOM side is
`.panel_tab_def(...)` + a rendered body, and the probe's discovery will pick the rows up the moment they publish.

### 3.4 `🐍️energy-results-probe.mjs` → `🗑️generated/baseline-results/` — `RESULT=FAIL passed=3 failed=3 skipped=1`

| assertion | baseline | message |
| --- | --- | --- |
| `boot.ready` | PASS | ready |
| `results.world3d` | FAIL | `window:energy.model.3d absent … DoD 4 cannot be measured until wave 1 lands the 3d window` |
| `results.run` | **PASS** | tool `auto-armed by the Tool category`, `mod+enter` → `Finalized`, run text `Finalized Run: 1`, full annual run (`Final: 9509.739 kWh after 8760 of 8760 timesteps`) in ~40 s |
| `results.recoloured` | FAIL | `skipped — no World3d surface to compare` |
| `results.legend` | FAIL | `no legend caption inside the world host and no element with a legend id (world host found=false)` |
| `results.fieldSwitch` | SKIP | `no action.set-result-field row (action rows: action.category.actions, action.set-simulation-settings, action.set-run-period, action.setActiveExample, action.copy, action.cut, action.paste, action.category.history, action.undo, action.redo, action.commitCheckpoint, action.createAlternative)` |
| `no.faults` | PASS | 0 fault lines |

The run half is therefore **already proven end to end on this build** — the tool arms, the chord starts it, and it
finalizes. Only the colouring and the legend are unmeasurable today.

## 4. DoD → assertion checklist

| DoD line | Probe · assertion | What it actually proves |
| --- | --- | --- |
| **1** 3d viewport renders the whole model as geometry | `energy-3d` · `world3d.present` | a `[data-surface-id="window:energy.model.3d"]` host exists with `data-meshes-json`/`data-instances-json` — i.e. a real `World3dScene` was published through `scene_surface`. |
| **1** … zones + 6 surfaces + 2 fenestrations present | `energy-3d` · `world3d.instances` | `data-instances-json` parses to **≥ 8** instances for BESTEST 600; the report records the first 12 instance `{id, meshId, label, color}` so the coordinator can eyeball that surfaces AND windows are both there. **Fenestrations as real polygons** is only partly covered here — the probe cannot tell a polygon from a rectangle; it records `meshIds` and vertex counts, so use lane A's Rust tests for the polygon shape and this for "they exist and are separate instances". |
| **1** … the scene actually paints | `energy-3d` · `world3d.canvas` | ≥ 1 `<canvas>` inside the host (0 = three.js never mounted, which is how a published-but-dead scene looks). |
| **1** example switch re-renders | `energy-3d` · `world3d.exampleRerender` | picks `BESTEST 900` through the navbar combobox (Escape afterwards — the listbox stays open and covers the middle window), then asserts the meshes **or** instances lane digest/count changed. |
| **2** tree ⇄ 3d pick (3d → selection) | `energy-3d` · `world3d.pick` | clicks the canvas centre (spiralling to 6 further points if the centre is sky), then requires either a non-empty `selectedIds` in the host's `data-selection-json` **or** an `aria-selected="true"` **panel** row. `mode-dock-tab-*` is explicitly excluded — those window tabs are permanently `aria-selected` and made the assertion pass vacuously in the first draft. The report also records console lines mentioning `interactionSelect`/`energyModel` and the full `data-interaction-json`. |
| **2** tree lists every family with human labels | `energy-panels` · `tree.panel` + `tree.family.{zones,surfaces,fenestrations,materials,constructions}` | the artifact panel publishes `panel:<ns>/<row>` rows, and each family has ≥ 1 row whose **text** (not id) carries a word of ≥ 3 letters. |
| **2** tree pick → selection | `energy-panels` · `inspector.surface` (the click half) | a surface row is clicked by its real DOM id and the inspector reacts. |
| **3** inspector shows the selected entity | `energy-panels` · `inspector.surface` | after the tree click, the inspection panel has rows **and** either echoes the clicked row's text or exposes form controls. |
| **3** inspector edits through retained actions | `energy-panels` · `inspector.fenestrationEdit` | selects a fenestration row, finds the control whose id/`data-ui-path`/name/aria-label matches `/u.?value/i`, fills `1.5`, commits with Enter + blur, then asserts the new value reads back from the control **or** appears in an inspector/tree row. |
| **3** … the edit is a real document mutation | `energy-panels` · `history.entry` | a `[DEBUG] history patch applied` console line (`ShellHost/🟦️.tsx:1997`) with a **non-empty `labels` array** — this is what separates "the input accepted a keystroke" from "a retained mutation committed and the undo stack grew". This is the assertion that would catch the known `model_edit`-has-no-`diff_fenestrations` silent-no-op. |
| **4** run colours the scene by a result field | `energy-results` · `results.run` then `results.recoloured` | arms the tool by opening `#framework.category.tool` (never clicking the tool row — that is a re-press that disarms), `mod+enter`, waits for `Finalized`; then compares the **before/after colour fingerprint**: mesh-lane digest, instance-lane digest, an 8-bucket luminance histogram over every `data.colors` triple, and the set of distinct per-instance `color` strings. A canvas pixel histogram is attempted as a secondary signal and reported, but never fails the probe alone (a WebGL canvas without `preserveDrawingBuffer` usually refuses readback). |
| **4** with a legend | `energy-results` · `results.legend` | a legend caption **inside the World3d host** or an element with a legend id/class. The simulation window's own `Final: … kWh after 8760 of 8760 timesteps` text is explicitly excluded — a document-wide "kWh" scan passed on the baseline build that has no 3d window at all, which would have been a false green. |
| **4** result-field switch | `energy-results` · `results.fieldSwitch` | unfolds the window's Actions pane (force-click: a window header can overlay its own toggle), submits `set-result-field` with `SEMIO_PROBE_RESULT_FIELD`, asserts the colours changed **again**. Optional by default (reported as `SKIP` when the action is absent); `SEMIO_PROBE_REQUIRE_FIELD_SWITCH=1` makes it a hard assertion once lane D ships it. |
| **5** browser probes prove 1–4 with logs + screenshots | all three | each writes `report.json`, `console.txt` and a PNG per step under `🗑️generated/<out>/`; `no.faults` on every probe greps the console for `pageerror\|trapped\|panicked\|unreachable\|dropped action\|Unknown action\|fixed-capacity\|surface-render\|window-context-required`. |

DoD 5's Rust half (native + wasm32-wasip2 checks, existing laws) is **not** covered by this lane — no cargo was run here.

## 5. Honest limitations / things the coordinator should know

1. **The window kind id is a guess.** `energy.model.3d` came from the task, not from code; the existing energy
   windows are `framework.window.tree`, `framework.window.table` and `energy.simulation`, so lane B may well pick
   a different id. The probe degrades to a clear "exists under a DIFFERENT id" message; re-run with
   `SEMIO_PROBE_WINDOW_3D=<id>` and tell me (or just edit the default) once it is fixed.
2. **The fem2d probe's panel namespace is wrong, do not copy it.** `🐍️fem2d-panels-probe.mjs` reads
   `panel:fem2d-play-document/<row>`, but the shell prefixes panel rows with the **panel-tab kind id**
   (`ShellHelpers/🟦️.tsx:2040`, `new UiDocumentStore(\`panel:${node.key}\`)`), i.e. `panel:framework.panel.artifact/<row>`.
   That fem2d probe has no output under its ticket's `🗑️generated/`, so it looks like it was never run. My probe
   therefore **discovers** every `panel:<ns>/` namespace present and picks the artifact-ish one, rather than
   trusting a name.
3. **`world3d.pick` can only prove "something got selected", not "the RIGHT thing got selected".** The React host
   dispatches `interactionSelect` without logging it, so there is no console line naming the picked id. The probe
   records `data-selection-json`, `data-interaction-json` and any console line mentioning `energyModel`; proving
   tree-pick ⇄ 3d-pick agree on one id needs the tree panel to publish `aria-selected` rows, which it cannot do
   until lane C lands. Once it does, tightening this to "the 3d pick selects the same row id the tree shows" is a
   two-line change and worth making.
4. **"Fenestrations as real polygons" (DoD 1) is not browser-provable.** The probe can count instances and record
   mesh ids, but not distinguish a derived rectangle from an authored polygon. That belongs in lane A's Rust tests.
5. **`results.recoloured` assumes the colours ride in the scene lanes.** If lane D paints results via
   `selection_json`/a side lane instead of `data.colors`/instance `color`, the digest comparison still catches it
   (the whole meshes/instances attribute is digested), but the histogram will read 0 channels — read the
   `coloursBefore`/`coloursAfter` blocks in `report.json` before believing a FAIL.
6. **Known-benign console noise**, whitelisted in the 3d probe's fault filter and present on every baseline run:
   `[DEBUG] contributions push refused empty pack {"plugin":"energy", … "chars":2,"kinds":[],"consumes":[]}`
   (routed through `console.error`), plus one `404` for a resource the playground always misses and a handful of
   `[DEBUG]` lines that happen to use the error channel. Nothing else appears.
7. **The results probe runs a full 8760-timestep annual simulation** (~40 s on the baseline). Do not run it
   concurrently with a cargo wave if the host is loaded.
8. **I did not restage, build, or start any server**, per the lane rules. Everything above was measured against
   the predecessor's build already serving on 6106.

---

# 6. Wave-1 results (2026-09-16, after the coordinator's restage)

The restaged build serves on 6106 with **two** World3d-capable surfaces in the DOM
(`window:energy.model.3d`, `window:framework.window.table`) and four panel tabs
(`framework.panel.artifact`, `framework.panel.inspection`, `framework.panel.toolRun`, `framework.panel.history`).

## 6.1 3d probe — effectively green (`🗑️generated/energy-3d-w1c/`)

| assertion | w1 | w1b | w1c |
| --- | --- | --- | --- |
| `boot.ready` | PASS | PASS | PASS |
| `world3d.present` | FAIL (no surface) | PASS — `868×907px, 1 canvas` | PASS |
| `world3d.instances` | — | PASS — `8 instances / 8 meshes`, ids `40,41,42,43,44,45,50,51` | PASS |
| `world3d.canvas` | — | PASS | PASS |
| `world3d.exampleRerender` | — | **FAIL** — `BESTEST 900`, digests identical | **PASS** — `BESTEST 620`, instance digest `4170712899→4208044579`, mesh digest `1612346085→2670008559` |
| `world3d.pick` | FAIL | PASS — `selectedIds=["44","40"]`, domain `energyModel` seen, 12 console hits | PASS |
| `no.faults` | FAIL (`useThree is not defined`) | FAIL (`setCamera` dropped) | FAIL (`setCamera` dropped) |

**Change made**: the probe's default example is now **`BESTEST 620`**, with the reason written at the declaration —
900 is 600's geometry with heavyweight constructions, so its mesh/instance lanes are byte-identical to 600 and the
re-render check could never differ; 620 keeps the envelope but moves the two windows from the south wall to the east
and west walls, so the fenestration instances genuinely move. w1c is the same build as w1b with only that override,
and it turned a false FAIL into a PASS.

The one remaining 3d fault is a **product defect, not a probe defect** (see 6.4).

## 6.2 Panels probe — repaired against the live DOM (`🗑️generated/energy-panels-w1h/`)

`RESULT=FAIL passed=9 failed=3`, and all three failures are one product defect.

| assertion | result |
| --- | --- |
| `boot.ready` | PASS |
| `tree.panel` | PASS — 48 rows under `panel:energy-model-artifact/` |
| `tree.family.zones` | PASS — `energy-model-artifact.zones=Zones (1)`, `1=Zone · 129.60 m³ Zone` |
| `tree.family.surfaces` | PASS — 15 rows, e.g. `40=South Wall · exteriorWall Surface` |
| `tree.family.fenestrations` | PASS — 5 rows, e.g. `50=South Window West · 6.00 m² Window` |
| `tree.family.materials` | PASS — 14 rows, e.g. `10=Wood Siding · 0.01 m · 0.14 W/mK Material` |
| `tree.family.constructions` | PASS — 5 rows, e.g. `30=🧱Lightweight Exterior Wall · 3 layers Construction` |
| `inspector.fenestration` | PASS — tree click on row `50` → **31** `.fenestration.` form fields, incl. `…fenestration.u-value` |
| `inspector.surface` | PASS — tree click on row `40` → **19** `.surface.` form fields |
| `inspector.fenestrationEdit` | **FAIL** — `…fenestration.u-value.input` `3 → 3`; the dispatch is refused (6.4) |
| `history.entry` | **FAIL** — 0 history lines, *because* the dispatch was refused |
| `no.faults` | **FAIL** — 8 lines, all the same refusal |

### Four DOM facts the old probe had wrong, and what changed

1. **The two panels share one dock anchor.** Opening Inspection *unmounts every* `panel:energy-model-artifact/` row.
   The old probe clicked a tree row, switched to Inspection, and then reported the next tree row "absent" — the row was
   fine, the panel was gone. Every step now re-opens the tab it needs.
2. **A panel body can take > 10 s to publish** after its tab is clicked (measured `ok after 10500ms (1 re-click(s))`;
   one earlier run did not publish within 40 s at all). The old fixed `waitForTimeout(2500)` was the reason
   `inspector.surface` reported "published no rows" while the surface form was in fact rendering. `openPanel` now polls
   for the namespace up to `SEMIO_PROBE_PANEL_SECONDS` (default 60) and **re-clicks the tab every 10 s**, since a click
   that lands while the other panel is still committing gets swallowed.
3. **A fallback regex of `/artifact|model/` matched the *inspection* namespace** — `energy-model-inspection` contains
   "model". The probe then read the inspector's rows as the tree's, failed to find row `50` in them, and blamed the
   tree. `nsFor` now takes an explicit `exclude` regex; `openTree`/`openInspector` are two named helpers.
4. **Selecting a surface makes its own window rows unreachable.** At boot the tree has 48 rows with `50`/`51` listed
   under `40`; after picking `40` it has 44 rows and those two are replaced by a single
   `energy-model-artifact.zones.40.windows.more` marker. Clicking that marker returns "ok" but the tree stays at 44
   rows and `50` never comes back — it is an **overflow marker, not an expander**. The probe therefore now selects the
   **fenestration first**, while the tree is still in its pristine boot state, and the surface second; it still tries
   `.more` expansion, then the sibling row (`51`), then a 3d pick, and reports which escalation worked. *This is worth
   a look from lane C on its own: as it stands a user cannot reach a window in the tree once its host surface is
   selected.*

Also added: `selectEntity` records its escalation attempts; the edit step now reports the **console refusal verbatim**
instead of letting a refused dispatch read like a silent no-op; `no.faults` now also greps
`does not own action|refused: dispatch-failed|[DEBUG] action failed` (and whitelists the baseline
`contributions push refused empty pack` noise).

## 6.3 Results probe — parsing fixed, not re-run yet

Per the coordinator, the recolour check cannot be measured until lane B's `DuplicateSiblingKey` per-tick fault is
fixed, so the probe was **not** re-run. Two changes landed:

- **Run-state parsing is no longer one regex over `document.body.innerText`.** `runStateRead()` now builds its haystack
  from the visible body, the whole document's `textContent` (which still contains a currently tab-hidden panel —
  `innerText` drops it), every `aria-live`/`role="status"` node, and every `panel:framework.panel.toolRun/…` row; the
  busy/state capture is `busy=(true|false)\s*·\s*([^·|]+?)(?:\s*·\s*|\s*\|\s*|\s+mod\+|\s+Run:|$)`. It also captures the
  new `Surfaces coloured by: …` caption. A run counts as finished when `Finalized` appears **and** that is not merely
  the text that was already there before the chord (`pre` is sampled first), so a stale finalized run cannot pass it.
- **The legend assertion accepts the `Surfaces coloured by: …` caption** wherever it lives, since the wave-1 caption is
  in the simulation window rather than inside the World3d host. The earlier exclusion of `Final: … kWh …` stays — a
  bare "kWh" scan passed on a build with no 3d window at all.

Both files were smoke-run end to end after every edit (outputs left at `🗑️generated/smoke-parse-*`), so they parse and
complete; the results probe's *assertions* are still unverified against a healthy run.

## 6.4 Product defects the harness found (for lanes B and C)

1. **`set-fenestration-property` is refused, so no inspector edit can ever commit.** The inspector dispatches correctly —
   `{field: uValueWM2K, id: 50, value: 1.5, gesture: energy-model-inspection.fenestration.u-value.input:…, commit: false}`
   then the same with `value: 3, commit: true` — and the shell answers
   `SemioFaultError: window kind energy.model.3d does not own action set-fenestration-property`,
   logged as `input #N set-fenestration-property refused: dispatch-failed (user window=energy.model.3d)`.
   A panel's action is dispatched in the context of the **active window**, and `energy.model.3d` does not declare it.
   Declare the inspector's `set-*-property` actions on every window kind (`.window_kind_actions(...)`), or dispatch them
   app-scoped. Until then DoD 3 is unreachable and the control visibly snaps back (`3 → 3`).
2. **`setCamera` is refused on the same window kind** — `dropped action "setCamera" dispatched from window kind
   "energy.model.3d": no window kind declares it … Declare it with .window_kind_actions()`. This is why `no.faults`
   fails in `energy-3d-w1b`/`w1c` even though every other 3d assertion passes; camera orbit will not persist.
3. **The tree's `…windows.more` overflow marker is a dead end** (see 6.2 §4).
4. Earlier in the wave, `energy-3d-w1` failed with a `pageerror: useThree is not defined` — fixed by the time of w1b.

## 6.5 Incidental: a peer broke the shell mid-session

Between ~17:41 and ~17:53 the playground would not boot at all:
`pageerror ReferenceError: setActiveUtilityForWindow is not defined` from `<FrameworkOsShellInner>`. The declaration
`const setActiveUtilityForWindow = useCallback(` had been removed from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` while two call sites still
referenced it (an engine-contract law at `🧪️tests/🔬️engine-contract/🟦️.ts:8897` pins that declaration). A peer lane
finished the refactor at 17:53 and the app recovered on its own; nothing in this lane was changed to fix it. Worth
knowing because any probe run inside that window reports a completely empty DOM.

## 6.6 Updated run commands

The 3d and results commands in §2 are unchanged except that `SEMIO_PROBE_EXAMPLE` now defaults to `BESTEST 620`.
The panels command gained knobs:

```zsh
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR"
SEMIO_PROBE_OUT=energy-panels-w2 \
SEMIO_PROBE_SECONDS=150 SEMIO_PROBE_PANEL_SECONDS=60 \
SEMIO_PROBE_SURFACE_ROW=40 SEMIO_PROBE_FENESTRATION_ROW=50 SEMIO_PROBE_FENESTRATION_ROW_ALT=51 \
SEMIO_PROBE_FIELD='u.?value|u_value' SEMIO_PROBE_VALUE=1.5 \
  bun "🐍️energy-panels-probe.mjs"
```

A new diagnostic, `🐍️energy-dom-dump.mjs`, dumps every panel tab, every `panel:<ns>/<row>` row and every form control
per step — use it first whenever a panel assertion fails, before editing the probe:

```zsh
SEMIO_PROBE_OUT=dom-dump SEMIO_PROBE_ROWS=40,50 bun "🐍️energy-dom-dump.mjs"
```

---

# 7. Results recolour timeline (restage #4, `🗑️generated/energy-results-w5/`)

`RESULT=FAIL passed=6 failed=1` — the only red is `results.recoloured`, and the timeline says exactly why.

## 7.1 The finding: the scene IS recoloured, and Finalized REVERTS it

```
t=  20.2s pre-chord    run=false/…     meshDig=1612346085 bytes=2833 hist=[0,0,0,12,0,12,24,0] textNodes=0 kWhInHost=false refresh+=0
t=  22.2s running      run=true/…      meshDig=  47763835 bytes=4777 hist=[0,0,48,0,0,0,0,0]  textNodes=0 kWhInHost=false refresh+=4
t=  24.4s running      run=true/…      meshDig=  47763835 bytes=4777 hist=[0,0,48,0,0,0,0,0]  textNodes=0 kWhInHost=false refresh+=0
t=  26.6s running      run=true/…      meshDig=  47763835 bytes=4777 hist=[0,0,48,0,0,0,0,0]  textNodes=0 kWhInHost=false refresh+=0
t=  28.7s running      run=true/…      meshDig=  47763835 bytes=4777 hist=[0,0,48,0,0,0,0,0]  textNodes=0 kWhInHost=false refresh+=0
t=  30.8s running      run=true/…      meshDig=  47763835 bytes=4777 hist=[0,0,48,0,0,0,0,0]  textNodes=0 kWhInHost=false refresh+=0
t=  32.3s finalized    run=false/final meshDig=1612346085 bytes=2833 hist=[0,0,0,12,0,12,24,0] textNodes=0 kWhInHost=false refresh+=0
t=  35.3s settled      run=false/final meshDig=1612346085 bytes=2833 hist=[0,0,0,12,0,12,24,0] textNodes=0 kWhInHost=false refresh+=0
t=  47.7s field-switch run=false/final meshDig=1612346085 bytes=2833 hist=[0,0,0,12,0,12,24,0] textNodes=0 kWhInHost=false refresh+=0
```

Read it left to right:

- **While the run is in flight the 3d window really does recolour.** `data-meshes-json` goes from 2 833 → **4 777 bytes**
  and its digest from `1612346085` → `47763835`, and the vertex-colour histogram changes shape completely:
  `[0,0,0,12,0,12,24,0]` (three distinct class-colour bands: 12 + 12 + 24 vertices) → `[0,0,48,0,0,0,0,0]`
  (all 48 vertices in one luminance bucket). That is a result ramp being applied — lane D's colouring works.
- **At `Finalized` it reverts, exactly, to the pre-chord bytes.** `2833` and digest `1612346085` again, byte-identical
  to `t=20.2s`. Not a partial refresh, not a stale frame: the neutral per-class palette is back.
- It stays reverted through `settled` (+3 s) and through the field switch (+15 s).

So `results.recoloured` is not "the colouring never happens" — it is **"the colouring only lives for the duration of
the tick payload and the finalize path throws it away"**. The per-surface results ride the `ToolRunView.payload` tick
channel; once the run finalizes, the render falls back to the committed document, which carries no per-surface results.
The probe's before/after comparison is around the run, so it correctly reports DoD 4 unmet — but the fix is on the
persist/finalize side, not the colour-ramp side.

Same picture one sample coarser: `[0,0,48,0,0,0,0,0]` all-one-bucket during the run also suggests the ramp is
currently mapping every surface to nearly the same value — worth a look once the revert is fixed.

## 7.2 The other channels, and what they did NOT show

| channel | what the timeline recorded |
| --- | --- |
| `data-status-json` on the 3d host | **absent at every sample** (`status=none`, `counters={}`). The World3d scene publishes no `status` lane at all, so there is no revision/generation/rendered-at counter to read. If lane D wants one, `World3dScene.status_json` is the lane the host stamps into `data-status-json`. |
| text nodes inside the 3d host | **0 at every sample**, `hostTexts: []`. There is **no caption, legend or HUD rendered inside the 3d window** at any point of the run. |
| a `kWh` / `W/m²` text inside the host | **false at every sample.** |
| the legend that *does* exist | `Surfaces coloured by: Conduction loss` — in the **simulation window**, not in the viewport. `results.legend` passes on it. So DoD 4's legend exists as a caption but there is no colour ramp / min-max scale anywhere. |
| console lines naming `energy.model.3d`, `window_bodies`, a refresh or dirty scope | **4 lines in total, all at t=22.2s, and none of them names the window body**: `performInvocation {actionId:"toolRunStart"}`, `command ingress lane {actionId:"toolRunStart", seq:633, lane:"Interactive"}`, `command ingress crossed {actionId:"toolRunStart", bytes:582}`. Grepping the whole console for `window_bodies` or `energy.model.3d` returns **zero** lines in this build — the shell logs no per-body refresh, so the DOM attribute digests above are the only usable signal. |
| Tool runs panel steps | at `finalized`: `Finalized · Encoding the final result (6/6) … Final: 9509.739 kWh after 8760 of 8760 timesteps · Coarse timestep (provisional): 9509.739 kWh … · Steady-state estimate (provisional): 0 kWh facility electricity · No data · No data` — note the two trailing **`No data`** tiers. |
| canvas pixel readback | `available`, but `nonBlank 0 → 0` and histogram `[4096,0,0,0,0,0,0,0]` — the WebGL drawing buffer reads back fully black in headless chromium, so it is useless as a signal here. The lane digests are authoritative; the readback stays in the report only to say so. |

## 7.3 `set-result-field` now works, and the probe drives it

`results.fieldSwitch` **PASSES**:
`toggle=ok row=ok pick=ok:Solar transmitted; options=["Conduction loss","Conduction gain","Solar transmitted","Solar absorbed"]; caption "Surfaces coloured by: Conduction loss …" → "Surfaces coloured by: Solar transmitted …"`.

Three things had to change, all of them facts about the live DOM:

1. **It is the simulation window's engagement, never the 3d one.** The old `/simulation|result|3d|model/` match picked
   `framework.window.energyModel3d.engagement` first, whose Actions pane lists `setCamera` and the `set-*-property`
   actions and no result field — which is why the step used to report "no `action.set-result-field` row". The probe now
   takes `framework.window.energySimulation.engagement` explicitly.
2. **There is no execute button.** The action form has exactly one control — a shadcn Select trigger `#field`
   ("Coloured by", a `BUTTON` with `role="combobox"`, chevron icon) — and **picking an option IS the commit**.
   `fill()` can never work on it; the probe opens it and clicks the option.
3. **`solarGain` is not a field.** The live options are `Conduction loss`, `Conduction gain`, `Solar transmitted`,
   `Solar absorbed`. The default is now `Solar transmitted`, and the matcher is fuzzy (case/space-insensitive, camelCase
   humanised); if the env value names no option the probe picks the first option that differs from the one currently in
   the caption, and says so in `chosenBecause`.

The assertion is on the **caption flipping**, which is the honest proof that the action committed. `colours changed
again=false` is reported but deliberately not required — the colours are already reverted by then, which is 7.1's
finding, not this step's.

## 7.4 How to re-run

```zsh
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR"
SEMIO_PROBE_OUT=energy-results-w6 \
SEMIO_PROBE_SECONDS=150 SEMIO_PROBE_RUN_SECONDS=240 \
SEMIO_PROBE_RESULT_FIELD="Solar transmitted" \
  bun "🐍️energy-results-probe.mjs"
# the timeline is printed at the end and written to 🗑️generated/<out>/timeline.txt (+ report.json .timeline)
```

Once lane D makes the colouring survive finalization, `results.recoloured` flips green with no probe change: it
compares the pre-chord sample against the post-`Finalized` one, which is precisely the property that is missing.
