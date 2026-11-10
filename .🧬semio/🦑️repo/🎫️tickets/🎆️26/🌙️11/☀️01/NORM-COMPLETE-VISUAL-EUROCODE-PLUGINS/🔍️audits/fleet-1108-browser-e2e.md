# Fleet 1108: browser-e2e lane

Date 2026-11-08. Owner: lane `browser-e2e`. Status: all 15 artifacts pinned and contract-green (4 tests, 129 assertions); hosted browser run NOT yet executed (builds busy).

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
| iso16757 | scriptLimits.maxSteps 10000 to 5000 | iso16757.5.8.scriptLimits.capacity | 15612.8% to 7806.4% (linear, singleton, empty owner) | medium |
| vdi3805 | catalog.products[id=RAD-600].configuration.attributes.heatExponentN 1.3 to 1.4 | vdi3805.3.n.RAD-600 | 86.67% to 93.33% | medium |

Contract suite still green (4 tests, 102 assertions). Risks to confirm on first host run: en1992 starts above 100%; en1995 u0 is 0.5%; nested/non-numeric edit paths for vdi3805 and iso16757.

## Update 2 (2026-11-08, continuation): all 15 pinned, re-validated, run sequence

No build or host was started. Everything below was derived from the current tree plus Python (`/home/deck/semio/.venv/bin/python -I`).

### New pins

- vdi3805 (example blatt-3, RAD-600): field `catalog.products[id=RAD-600].configuration.attributes.heatExponentN`, labels "Heat exponent n" / "Heizexponent n" (field-meta table), owner `scope:1:16:catalog.products7:RAD-600` (outliner collection `catalog.products`, row id RAD-600), argument `newConfiguration` (rule `catalog.products[].configuration` -> `change-product-configuration`, a leaf inside the typed rule is admitted by `can_set_field`). Check `vdi3805.3.n.RAD-600` utilization = n / RADIATOR_N_MAX (1.5): 1.3 -> 0.8667, 1.4 -> 0.9333. Only this check reads n (grep of `heat_exponent_n` in inferences); the diff rewrites the mirrored 210 record `n` in the same patch, so the sync check `vdi3805.3.sync.RAD-600` stays pass. Oracle: the artifact oracle `oracles/compliance/py` (`n:RAD-600` pass for both states) plus the n/1.5 ratio.
- iso16757 (example demo): field `scriptLimits.maxSteps`, labels "Max steps" / "Max. Schritte", rule `change-script-limits` (argument `newMaxSteps`, all three limits carried). Check `iso16757.5.8.scriptLimits.capacity` utilization = maxSteps / (max(maxRecursion,1) + timeoutMs/1000) = steps / 64.05: 10000 -> 156.128, 5000 -> 78.064 (displayed 15612.8% / 7806.4%, status forced pass). Only this check changes (recursion check reads maxRecursion only; part-number script `dn * 10 + 50` unaffected). No third-party oracle exists for the ratio; the value is an independent Python evaluation of the Rust formula.
- Why not geometry: the inspector keeps `geometry.objects.<id>.*` read-only (literal ids are not admitted by `can_set_field`; only viewport handles write them), and every geometry numeric edit moves at least two checks (volume/clearance/spaceVolume).
- Singleton facet: `scriptLimits` has no outliner row, so the pin has `owner: ""`. Schema: `owner` may be empty; contract test enforces that an empty owner has no `[id=` in its path, and that every non-empty owner equals the canonical `scope:1:<len>:<collection><len>:<id>` encoding of the path (now checked for all 15 pins, including the dotted collection `catalog.products`). Scenario: `selectOwner` returns true and the DOM matcher skips the owner suffix when `owner === ""`.
- Percent parsing in the scenario now accepts grouped numbers (`15.612,8 %`, `15,612.8 %`, narrow/no-break spaces) because iso16757 starts above 10000%; the `number()` parser distinguishes grouping from decimals (tested in bun for en/de forms).
- Script: `TICKET/🧰️scripts/browser-e2e-vdi-iso-pins.py` (re-derives both pins, `--write` patches the matrix).

### Re-validation of all 15 pins against the current tree

Checked mechanically (script in scratchpad, results below):
- Field meta: all 13 earlier labels (EN and DE) equal the current `FieldSpec` strings in the artifacts' field-meta/check files (en1997 `upliftCases[].porePressure` lives in `inferences/checks`, din16798 `noiseDb` in `field-meta` as a match arm).
- Edit rules: every path has a `SetFieldRule` and the `argument` equals the rule's `value` name (all 13 plus vdi3805 and iso16757).
- Owner scope: all strings equal the canonical encoding (contract test).
- Window ids: `ownerWindow` is `norm-<variant>-outliner` for all 15; vdi3805 outliner/properties ids match `norm-vdi3805-{outliner,properties}`. The outliner row is a select item only; the numeric input is in the properties inspector with row id `norm-inputs.<path>` (so the field key is stable even for nested paths).
- Example ids: all 15 equal the `ID` consts of the example sources (the catalog lists them with an icon prefix, the URL uses the bare id).
- Recomputed independently: din18599 H'T 0.5978 -> 0.8043 (Python from DSL elements, Fx and GEG limit 0.40, so still affine and correct after the balance-model changes), din4108 delta-u (single bridge tb-bad, limit 0.10, utilization psi/0.4), din16798 noise (limit 35 dB for category II office), en1993 sls check formula and id (`en1993.7.sls.<combinationId>`, delta/(L/ratio)).
- No stale pin found; nothing needed fixing besides the two new rows. din4108 `tIntC` is a zone field and does not touch the psi pin; the din18599 results -> checks rename does not touch the matrix (windows are listed by role `checks`).
- Residual risks (cannot be settled without a native run): en1992 starts above 100% (110.8%); en1995 u0 is 0.5%; the German decimal entry for nested attribute leaves; whether the inspector renders vdi3805 `configuration.attributes.*` leaves as editable numeric rows (meta labels exist and `can_set_field` admits them, row rendering unverified). Fallback if the vdi attribute row is absent: the geometry bbox grips (`geometry[id=G].bbox.max*`), but those are viewport-only.

### Contract gate

`repo --service=off tools exec --light bun -- test "./✏️s/🔌️plugins/📕️norm/🧪️tests/⏪️time-travel/🧪️contract/🟦️.ts"`: 4 pass, 0 fail, 129 expect() calls (2026-11-08, after all changes above).

### Exact run sequence once builds are green

1. Component and host (one shared norm wasm `semio_hub_norm_component.wasm`, cargo package `semio-hub-norm-component`, project `@semio-tech/norm-plugin`; each variant is a generated playground, catalog `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`):
   - `cd /home/deck/semio && repo --service=off run @semio-tech/norm-plugin:check` (optional gate; the component is built by the activate chain below).
   - `repo --service=off run @semio-tech/framework-os-dev:activate-en1990-react-prototype --catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` once (builds the norm component and host staging).
   - Serve (leave locale unlocked, do NOT set `SEMIO_LOCKED_LOCALE`), one variant at a time or in several shells: `repo --service=off run @semio-tech/framework-os-dev:dev-<variant>-react-prototype --catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` (ports: din4108 6091, din16798 6092, din18599 6093, en1990 6094, en1991 6095, en1992 6096, en1993 6097, en1994 6098, en1995 6099, en1996 6100, en1997 6101, en1998 6102, en1999 6103, iso16757 6104, vdi3805 6105).
2. Scenario per artifact (the scenario file carries its own default serve URL; omit `--serve` when the host above is already answering):
   `cd /home/deck/semio && SEMIO_NORM_ARTIFACT=<variant> repo --service=off run @semio-tech/framework-os-dev:time-travel -- --scenario "✏️s/🔌️plugins/📕️norm/🧪️tests/⏪️time-travel/🟦️.ts" --renderer react --locales en,de --chords en,de --out ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/browser-e2e/<variant>"`
   for `<variant>` in en1990 en1991 en1992 en1993 en1994 en1995 en1996 en1997 en1998 en1999 din4108 din16798 din18599 iso16757 vdi3805. Pass criteria per variant: verdict rows `visual-editor-windows-mounted`, `engineering-owner-selectable`, `engineering-before`, `folder-attach-preserves-engineering-state`, `engineering-after-edit`, `one-engineering-edit-one-document-entry`, `engineering-after-undo`, `engineering-after-redo`, `engineering-archive-written`, `engineering-persisted-after-reload`, `engineering-history-persists-after-reload` all true in en and de; no `*-needs-pinned-witness` row remains.
3. wgpu pass (optional, later): same command with `--renderer wgpu` (ports +100 are in the matrix).
4. Delete `🗑️generated/browser-e2e` logs after reading the verdicts; keep this report.
