# Fleet 1108: browser-e2e lane

Date 2026-11-08. Owner: lane `browser-e2e`. Status: harness extended and contract-green; hosted browser run NOT yet executed (host not buildable yet).

## Infrastructure found

- Harness: `@semio-tech/framework-os-dev:time-travel` (Playwright/Chromium, fresh context per locale en then de, React DOM and wgpu mirror, folder-bound persistence reload). A plugin owns a `timeTravelScenarioV1` module passed through `--scenario`.
- Norm scenario already existed for EN 1990 only (`norm/tests/time-travel`). The puzzle lanes use the same harness (`puzzle/tests/time-travel`).
- Host: variant ports come from `catalog/generated/playgrounds.json` (en1990 6094 ... vdi3805 6105 react, +100 wgpu). Start: `repo run @semio-tech/framework-os-dev:dev-<variant>-react-prototype`; URL `http://127.0.0.1:<port>/?plugin=<variant>&role=editor&example=<example>` (verified current in catalog, not yet served).

## Changes (all under `✏️s/🔌️plugins/📕️norm/🧪️tests/⏪️time-travel/`)

- `🧫️witness/🔣️.json`: now the language-agnostic acceptance matrix (schemaVersion 2): 15 artifacts x {variant, sample example, react/wgpu ports, pin}; `pin` (field path/key/owner/EN+DE labels, check id, before/after value and utilization) is null until measured against a native run.
- `🧬️schema/🔣️.json`: closed schema for the matrix (15 artifacts, locales en+de, journey boot/edit/undo/redo/reload, five windows).
- `🟦️.ts`: scenario selects its row by env `SEMIO_NORM_ARTIFACT` (default en1990); new step verdict `visual-editor-windows-mounted` (viewport, diagrams, properties, checks, outliner, ids `norm-<variant>-<role>`) for every artifact; unpinned rows fail loudly with `<step>-needs-pinned-witness` so they cannot read as passes.
- `🧪️contract/🟦️.ts`: 4 tests / 36 assertions pass (Ajv oracle on the matrix, ports equal the generated playground catalog, pinned utilization = value/limit, scenario steps). Run: `repo --service=off tools exec --light bun -- test ./<path>` (log deleted). It is also in the package-runner suite list.
- `🥒️.feature`: windows-mounted and edit/undo/redo/persist scenarios x en/de.

Run one artifact: `SEMIO_NORM_ARTIFACT=en1993 repo run @semio-tech/framework-os-dev:time-travel -- --scenario "✏️s/🔌️plugins/📕️norm/🧪️tests/⏪️time-travel/🟦️.ts" --renderer react --locales en,de --chords en,de --out <ticket>/🗑️generated/browser-e2e`

## Artifact x gate matrix

Gates: native = wasm component builds; windows = five windows mounted; edit = numeric edit updates check; undo/redo; persist = reload keeps value; en/de = both locales.

| artifact | sample | react port | pin | native | windows | edit | undo/redo | persist | en+de |
|---|---|---|---|---|---|---|---|---|---|
| en1990 | road-bridge-compliant | 6094 | pinned (0.2 to 0.4 m/s2, 40% to 80%) | not run | not run | not run | not run | not run | not run |
| en1993 | heb240-compliant | 6097 | unpinned | not run | not run | not run | not run | not run | not run |
| en1991 | de-office-compliant | 6095 | unpinned | not run | not run | not run | not run | not run | not run |
| en1992 | compliant-prestressed-beam | 6096 | unpinned | not run | not run | not run | not run | not run | not run |
| en1994 | composite-floor-beam | 6098 | unpinned | not run | not run | not run | not run | not run | not run |
| en1995 | glulam-floor-beam | 6099 | unpinned | not run | not run | not run | not run | not run | not run |
| en1996 | loadbearing-wall | 6100 | unpinned | not run | not run | not run | not run | not run | not run |
| en1997 | compliant | 6101 | unpinned | not run | not run | not run | not run | not run | not run |
| en1998 | seismic-rc-frame | 6102 | unpinned | not run | not run | not run | not run | not run | not run |
| en1999 | aluminium-roof-purlin | 6103 | unpinned | not run | not run | not run | not run | not run | not run |
| din4108 | failing-thin-insulation | 6091 | unpinned | not run | not run | not run | not run | not run | not run |
| din16798 | compliant-office | 6092 | unpinned | not run | not run | not run | not run | not run | not run |
| din18599 | compliant-two-zone | 6093 | unpinned | not run | not run | not run | not run | not run | not run |
| iso16757 | demo | 6104 | unpinned | not run | not run | not run | not run | not run | not run |
| vdi3805 | blatt-3 | 6105 | unpinned | not run | not run | not run | not run | not run | not run |

## Blockers

- No `semio_hub_norm_component.wasm` exists; the host needs the norm component, which needs every artifact crate green. Other lanes' native builds are still running/red (e.g. shared `norm-artifact-contract-rs:build` exit 1 in its latest log). My host run (`dev-en1990-react-prototype`) is queued behind ~11 builds on the capacity gate (swap above ceiling).
- Rows other than en1990 need pins: after the first successful host run for an artifact, take the discovered numeric field/check from the `--explore` inventory and fill `pin`; until then the edit gates fail by design.
- Not verified: German-locale labels in the rendered editor, wgpu renderer, persistence reload for any artifact.

## Live job

My queued host run (`dev-en1990-react-prototype`, log `🗑️generated/browser-e2e/host.txt`) was left queued (killing it was denied). When it serves, run the command above for en1990; delete the log afterwards.

## Update: pins derived without a native run (2026-11-08)

13 of 15 artifacts now carry a pin in the matrix (field, EN+DE labels from field meta, check id, before/after, `basis`, `linear`). Utilizations were re-derived in Python from the Rust check formulas and example DSL/data; none observed in the app yet. Browser compares displayed percent within 0.5 points.

| artifact | field | check | before to after | confidence |
|---|---|---|---|---|
| en1990 | bridgeSls.deckAcceleration | en1990.a2.acceleration.deck-D1.sls-D1 | 40% to 80% | pinned earlier |
| en1993 | members.deflectionLimitRatio 300 to 600 | en1993.7.sls.sls-char-q-imposed | 30.3% to 60.6% | medium-high |
| en1991 | selfWeightElements.thickness 0.20 to 0.22 | en1991.1-1.self-weight.slab-rc | 83.3% to 91.7% | high |
| en1992 | members.span 12 to 9 | en1992.7.4.ld.beam-PS1 | 110.8% to 83.1% (example starts above 100%) | medium |
| en1994 | beams.barSpacingM 0.15 to 0.24 | en1994.7.4.crack.beam-B1 | 69.2% to 80% (non-linear) | medium-high |
| en1995 | members.dampingXi 0.04 to 0.005 | en1995.7.3.velocity.beam-B1 | 0.5% to 14.3% (non-linear, small) | medium |
| en1996 | walls.bedJointThicknessM 0.012 to 0.0135 | en1996.2.bedjoint.wall-north | 80% to 90% | medium-high |
| en1997 | upliftCases.porePressure 40000 to 20000 | en1997.10.hyd.upl-U1 | 66.7% to 33.3% | high |
| en1998 | joints.beamMomentRdNm 4e6 to 6e6 | en1998.1.bldg-office.capacityDesign.joint.joint-1 | 57.8% to 86.7% | medium-high |
| en1999 | fatigueDetails.deltaSigmaEd 35e6 to 25e6 | en1999.1-3.fat.range.fat-1 | 84.8% to 60.6% | high |
| din4108 | thermalBridges.psi 0.4 to 0.2 | din4108-bb2.delta-u | 100% to 50% | high |
| din16798 | zones.noiseDb 34 to 24.5 | din16798-1.noise.zone-office | 97.1% to 70% | high |
| din18599 | elements.uValueWM2k 0.14 to 0.40 | din18599.geg.ht-prime | 59.8% to 80.4% (affine) | medium-high |
| iso16757 | not pinned | only candidate is singleton scriptLimits.maxSteps (156x utilization, no owner scope, mutation takes 3 values) | | blocked |
| vdi3805 | not pinned | candidate heatExponentN needs a whole-configuration payload, owner scope guessed | | blocked |

Contract suite still green (4 tests, 102 assertions). Risks to confirm on first host run: en1992 starts above 100%; en1995 u0 is 0.5%; nested/non-numeric edit paths for vdi3805 and iso16757.
