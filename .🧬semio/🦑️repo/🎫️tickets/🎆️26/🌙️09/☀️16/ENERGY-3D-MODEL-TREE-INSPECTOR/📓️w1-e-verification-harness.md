# W1-E — verification harness (probes + scripts) for the energy 3d / tree / inspector ticket

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
SEMIO_PROBE_EXAMPLE="BESTEST 900" \
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
