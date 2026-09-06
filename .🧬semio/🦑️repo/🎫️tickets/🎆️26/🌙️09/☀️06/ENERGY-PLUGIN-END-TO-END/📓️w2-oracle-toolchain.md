# W-B — Honeybee → OpenStudio → EnergyPlus oracle for `s.energy.model`

Worker W-B. Everything below was RUN on this host (macOS 26.6.2, Darwin 25.6, arm64) and every number is
copied from the run that produced it. Nothing here is projected.

---

## 1. What landed

| Path | Role |
|---|---|
| `✏️s/🔌️plugins/🔋️energy/🧪️oracle/📦️packages/🐍️python/pyproject.toml` | standalone uv project, Python 3.12, four pinned packages in the `test` group |
| `…/🐍️python/uv.lock` | committed lock, 21 packages resolved |
| `…/🐍️python/🔣️.json` | pinned OpenStudio release-asset table with per-platform sha256 |
| `…/🐍️python/📜️script.ts` | `setup` / `status` / `run` / `native` / `emit` / `test` router |
| `…/🐍️python/📋️project.json` | nx targets `oracle-setup`, `oracle-status`, `oracle-run`, `oracle-native`, `oracle-emit`, `test` |
| `…/🐍️python/🧪️oracle/🐍️.py` | the whole oracle: toolchain binding, semio→honeybee translator, ASHRAE 140 native builder, E+ driver, result reader, selftest |
| `A/🧫️fixtures/🌦️denver-tmy/🌦️.epw` + `🔣️.json` | the committed BESTEST Denver weather year and its provenance sidecar |
| `A/🧫️fixtures/🏛️bestest-<case>/🔮️energyplus.json` | reference results for 600, 600FF, 610, 620, 640, 900, 900FF, 910, 920, 940 |
| `A/🔮️oracle/🔣️.json` | new `energyplus-25-2-0-via-honeybee-openstudio` registration, reversing the earlier DECLINE for the simulation capability only |
| `✏️s/🔌️plugins/🔋️energy/🧪️oracle/🔣️.json` | the Python package registered as an `oracleHostPackage` |
| `.vscode/launch.json` | six entries: `🔮️oracle🔋️energy⚙️setup / 🔍️status / 🏛️native / ▶️run / 🧫️emit`, `🧪️test🔋️energy🔮️oracle` |
| `🎫️…/ENERGY-PLUGIN-END-TO-END/🧫️bestest-<case>-semio-model.json` | the oracle's own reading of cases 600/600FF/900/900FF as semio `Model` documents (input files, for W-C to diff against) |

`A` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

---

## 2. Provisioning — what it downloads and what it verified

`bun nx run @semio-tech/energy-oracle-py:oracle-setup` resolves `<node platform>-<node arch>` against the
committed table in `🔣️.json`, downloads with `curl`, verifies sha256, extracts with `tar --strip-components 1`
into `.🧬semio/🦑️repo/⚡️cache/oracles/<tool>-<version>-<platform>/`, then runs `uv sync`.

Only ONE archive is needed: the OpenStudio SDK bundles EnergyPlus, so a separate EnergyPlus download would be a
second copy of the same binary.

| Field | Value |
|---|---|
| URL | `https://github.com/NREL/OpenStudio/releases/download/v3.11.0/OpenStudio-3.11.0%2B241b8abb4d-Darwin-arm64.tar.gz` |
| Size | 329 156 193 B (313 MiB), downloaded in 40 s at ~8 MB/s |
| sha256 | `b7f859038e2962371ff1e12ffe509235f01a7d54f631f04b458dfb5d7be36464` |
| Matches | GitHub's own release-API `digest` field for that asset |
| Extracted to | `.🧬semio/🦑️repo/⚡️cache/oracles/openstudio-3.11.0-darwin-arm64/` |

Present-tense proof that both binaries run (`bun nx run @semio-tech/energy-oracle-py:oracle-status`):

```
[oracle]   openstudio: …/openstudio-3.11.0-darwin-arm64/bin/openstudio -> 3.11.0+241b8abb4d
[oracle]   energyplus: …/openstudio-3.11.0-darwin-arm64/EnergyPlus/energyplus -> EnergyPlus, Version 25.2.0-cf7368216c
[oracle] python 3.12.13
[oracle] honeybee-energy 1.123.32, honeybee-openstudio 0.7.2, ladybug-core 0.44.59
[oracle] native ASHRAE 140 cases: 600, 600FF, 610, 620, 640, 900, 900FF, 910, 920, 940
```

The other platform rows in `🔣️.json` carry GitHub's published sha256 for `Darwin-x86_64`, `Ubuntu-24.04-arm64`,
`Ubuntu-24.04-x86_64` and `Windows` and are unverified by download on this host — they come from the release API,
not from a guess, and `setup` fails closed if a byte differs.

### Why a standalone uv project and not a root `[dependency-groups] test` pin

The coordinator's note asked for the ifcopenshell convention first. It does not work here, and this is measured,
not assumed. The root `pyproject.toml` pins `requires-python = ">=3.14,<3.15"`, and:

```
$ uv pip compile --python-version 3.14 --python-platform windows - <<< "openstudio==3.11.0"
  … you require openstudio==3.11.0, we can conclude that your requirements are unsatisfiable.
  hint: Wheels are available for `openstudio` (v3.11.0) on the following platforms:
    `manylinux1_x86_64`, `manylinux2014_aarch64`, `macosx_10_9_x86_64`, `macosx_11_0_arm64`
```

The compiled OpenStudio bindings publish no cp314 `win_amd64` wheel, so a root-group pin would break the Windows
leg of the zero-touch requirement. Python 3.12 is the widest common denominator across the three wheel matrices;
`uv sync` downloads that interpreter itself, so nothing on the host is touched. `sccache`'s download/extract
pattern in root `📜️script.ts` is the shape `setup` follows.

### Dependency gate

`bun ./📜️script.ts verify dependencies literal-external` is RED repo-wide and was already red before this work
(`current=184, oracle-conflicts=23`); none of the 23 conflicts names this package. The four new pins classify
correctly — this required moving them out of `[project] dependencies`, which the gate reads as production:

```
honeybee-energy      ['test-runner'] productionReachable= False literal-external
honeybee-openstudio  ['test-runner'] productionReachable= False literal-external
ladybug-core         ['test-runner'] productionReachable= False literal-external
openstudio           ['test-runner'] productionReachable= False literal-external
```

`productionReachable` in the whole-repo total fell from 105 to 101 as a result.

Licenses: EnergyPlus and the OpenStudio SDK are BSD-3-Clause; `ladybug-*` and `honeybee-*` are AGPL-3.0-only,
invoked only as an out-of-process subprocess/venv reference and never linked into or imported by a shipped semio
artifact. Recorded in the package docstring, in `pyproject.toml`, and in the oracle registration.

---

## 3. Weather

`A/🧫️fixtures/🌦️denver-tmy/🌦️.epw` is NREL's own BESTEST weather year, copied byte-for-byte from
`NREL/BESTEST-GSR@master:shared_resources/725650TYCST.epw` and renamed to the taxonomy filename.

```
LOCATION,DENVER INTL AP,CO,USA,TMY3,725650,39.83,-104.65,-7.0,1650.0
8 header lines + 8760 records          sha256 1d0402144460a26265555a18a9cdfe4f0f7d9b4f57d6194847af7959b518571f
1 611 228 B                            dry-bulb min −19.4 °C, max 40.0 °C, mean 10.88 °C
```

**This matters for anyone comparing against 1990s BESTEST tables.** The classic NREL/TP-550-43827 free-float
envelopes (600FF min −18.8…−15.6 °C) were produced against `DRYCOLD.TMY`, whose minimum is about −24 °C. The
current Standard 140 tooling uses this TMY3-derived file instead, so the old envelope numbers are NOT directly
comparable for free-float extremes. Comparison in §5 is therefore against NREL's own CURRENT reference run.

---

## 4. Runner CLI contract (for W-C's feature wiring)

All commands go through nx, or `bun ./📜️script.ts <cmd>` inside
`✏️s/🔌️plugins/🔋️energy/🧪️oracle/📦️packages/🐍️python`.

```
oracle-setup                                  provision + verify + uv sync   (idempotent, network only here)
oracle-status                                 print resolved paths/versions
oracle-run   <model.json> <weather.epw> <out.json>
             [--case ID] [--work DIR] [--keep]
             [--layer-order outside-in|inside-out]
             [--aperture-aspect F] [--sill-height M]
oracle-native <case> <weather.epw> <out.json> [--work DIR] [--keep]
oracle-emit   <case> <out.json>
test                                          selftest, no EnergyPlus run
```

* Exit code 0 on success; any failure is a non-zero exit with the tail of `eplusout.err` on stderr.
* `<out.json>` is exactly the `semio.energy.bestest-results/1` document from `📓️bestest-contract.md`:
  8760-long `hourly.zoneAirTemperatureC`, `heatingW`, `coolingW`, `transmittedSolarSouthWh`;
  `annual`/`peak` null for free-float cases; `freeFloat` null for controlled cases. Verified on every fixture.
* **Input shape.** `<model.json>` is a bare semio `Model` object; `model.schedules` (W-D0's new member) is read
  from the document itself, so a committed fixture is self-contained. An envelope
  `{"model": {…}, "schedules": {…}}` is also accepted, for the interval before that member existed. Only
  `schedules.constants` and `schedules.daily` are translated; `weekly`/`annual`/`time_series` exit with a message
  naming what was unsupported rather than silently defaulting.
* **`Infiltration` methods.** `PerExteriorArea` passes `flow_per_exterior_area_m3_s_m2` straight through;
  `ScheduledAch` is converted against the zone volume and the exterior area EnergyPlus itself uses for
  `Flow/ExteriorArea` (every surface whose outside boundary is the external environment — 171.6 m² for the
  BESTEST box, confirmed against `Zone Information` in the `.eio`). `EffectiveLeakageArea` and `WindAndStack`
  have no honeybee equivalent and are refused rather than approximated.
* Field spelling is the `ToValue` derive's default: **snake_case idents verbatim**, because `Model` and its entity
  structs carry no `#[value(rename_all = …)]`. Enums with only unit variants (`SurfaceClass`) are bare strings
  (`"ExteriorWall"`); `OutsideBoundary` has one data variant so it is externally tagged —
  `"OutdoorAir"` / `"Ground"` / `{"Interzone": 42}`. `EntityId`/`ScheduleId` are bare numbers (single-field tuple
  structs derive transparent automatically).
* `oracle-emit <case> <out.json>` writes the oracle's own reading of a case as a semio `Model`. Four are already
  written into this ticket folder (`🧫️bestest-600-semio-model.json` and 600FF/900/900FF) — **diff your
  `🔋️model.json` against those before debugging a physics disagreement.**
* Provisioning is the only step that touches the network; `oracle-run` and `oracle-native` are offline.
* `--work DIR` is wiped before each run (a stale work directory silently reused an old IDF once during this
  ticket and produced results that looked plausible; that is now impossible).

---

## 5. Reference results — and evidence the reference is itself right

`native` mode rebuilds each ASHRAE 140 §5.2 case from the standard's own geometry and layer tables, with no semio
input at all. The case data is hand-transcribed from NREL's own encoding (`NREL/BESTEST-GSR@master`:
`shared_resources/bestest_resources.osm`, `Bestest_Geo_South_12_0_0.osm`, `measures/…envelope…/measure.rb`),
not paraphrased from the copyrighted standard: 8 m × 6 m × 2.7 m single zone, floor `Outdoors`/NoSun/NoWind,
two 3.0 m × 2.0 m south windows at x 0.5–3.5 and 4.5–7.5, z 0.2–2.2, the exact double-pane
`WindowMaterial:Glazing` + 12 mm air stack, 200 W continuous gain at 60 % radiant, 0.5 ACH infiltration
(EnergyPlus reports back `ACH - Air Changes per Hour … 0.500` from a design flow of 1.800E-002 m³/s),
20/27 °C setpoints, unlimited-capacity ideal loads, `FullInteriorAndExterior`, TARP inside / DOE-2 outside,
terrain `Country`, 6 timesteps per hour, hourly reporting.

Compared against NREL's own published BESTEST-GSR run on the same OpenStudio 3.11.0 / EnergyPlus 25.2.0
(`results/workflow_results.csv`):

| case | metric | this run | NREL BESTEST-GSR | delta |
|---|---|---|---|---|
| 600 | annual heating MWh | 4.3247 | 4.3250 | −0.01 % |
| 600 | annual cooling MWh | 6.0441 | 6.0417 | +0.04 % |
| 600 | peak heating kW | 3.2042 | 3.2042 | +0.00 % |
| 600 | peak cooling kW | 6.3524 | 6.3524 | −0.00 % |
| 600FF | free-float min °C | −12.5606 | −12.5606 | 0.0000 K |
| 600FF | free-float max °C | 63.9200 | 63.9200 | 0.0000 K |
| 600FF | free-float mean °C | 24.9606 | 24.9606 | 0.0000 K |
| 610 | annual heating MWh | 4.3750 | 4.3750 | +0.00 % |
| 610 | annual cooling MWh | 4.3466 | 4.3444 | +0.05 % |
| 620 | annual heating MWh | 4.4828 | 4.4833 | −0.01 % |
| 620 | annual cooling MWh | 4.0742 | 4.0694 | +0.12 % |
| 640 | annual heating MWh | 2.6685 | 2.6583 | +0.38 % |
| 640 | annual cooling MWh | 5.7804 | 5.7778 | +0.04 % |
| 640 | peak heating kW | 4.6140 | 4.5465 | +1.48 % |
| 900 | annual heating MWh | 1.6611 | 1.6611 | +0.00 % |
| 900 | annual cooling MWh | 2.4982 | 2.4917 | +0.26 % |
| 900 | peak heating kW | 2.6875 | 2.6875 | +0.00 % |
| 900 | peak cooling kW | 3.0415 | 3.0415 | +0.00 % |
| 900FF | free-float min °C | 1.2467 | 1.2467 | 0.0000 K |
| 900FF | free-float max °C | 44.2992 | 44.2992 | 0.0000 K |
| 900FF | free-float mean °C | 25.1333 | 25.1333 | 0.0000 K |
| 910 | annual heating MWh | 1.9528 | 1.9528 | +0.00 % |
| 910 | annual cooling MWh | 1.3895 | 1.3861 | +0.24 % |
| 920 | annual heating MWh | 3.3315 | 3.3306 | +0.03 % |
| 920 | annual cooling MWh | 2.7407 | 2.7333 | +0.27 % |
| 940 | annual heating MWh | 1.0668 | 1.0639 | +0.27 % |
| 940 | annual cooling MWh | 2.4335 | 2.4278 | +0.23 % |

Free-float extremes reproduce NREL's to every published digit, and the peak-heating HOUR agrees too
(index 8759 here, `31-DEC-24:00` there, case 600). Most of the residual on the annual figures is the CSV's own
rounding to five significant digits; the one visible outlier (+1.48 % on the 640 peak) is the setback ramp
being sampled at hourly reporting rather than at the 10-minute schedule step.

Qualitative sanity, all in the expected direction: the 1 m overhang (610) cuts cooling 6.04 → 4.35 MWh and
raises heating 4.32 → 4.38; east/west glazing (620) cuts cooling to 4.07; high mass (900) cuts both
(1.66 / 2.50 against 4.32 / 6.04); night setback (640) cuts heating 4.32 → 2.67.

Not implemented as native cases and deliberately not faked: **630/930** (window fins), **650/950**
(night ventilation as a second infiltration object), **960** (two-zone sunspace), and the §5.2.4 ground-coupled
slab cases. `oracle-run` still handles any of these if W-C authors the `🔋️model.json`.

---

## 6. Translated-versus-native cross-check — one real finding

Same four cases, run twice: once built natively from the standard, once translated from the semio `Model`
documents in `🧫️bestest-<case>-semio-model.json`. This isolates translation error from physics error.

Translated with the window W-C's fixtures use — the standard's quoted `u_value_w_m2k 3.0`, `shgc 0.787`:

| case | metric | native | translated (U 3.0 / SHGC 0.787) | delta |
|---|---|---|---|---|
| 600 | annual heating kWh | 4324.7442 | 4382.0359 | +1.32 % |
| 600 | annual cooling kWh | 6044.0586 | 6386.0179 | +5.66 % |
| 600 | peak heating kW | 3.2042 | 3.2681 | +1.99 % |
| 600 | peak cooling kW | 6.3524 | 6.6434 | +4.58 % |
| 600 | hourly zone-temperature RMSE | — | — | 0.0509 K |
| 600FF | free-float min / max / mean °C | −12.5606 / 63.9200 / 24.9606 | −12.7207 / 66.1286 / 25.4215 | −0.16 / +2.21 / +0.46 K |
| 600FF | hourly zone-temperature RMSE | — | — | 0.6617 K |
| 900 | annual heating kWh | 1661.1379 | 1643.1729 | −1.08 % |
| 900 | annual cooling kWh | 2498.1614 | 2699.8237 | +8.07 % |
| 900 | peak heating kW | 2.6875 | 2.7370 | +1.84 % |
| 900 | peak cooling kW | 3.0415 | 3.2112 | +5.58 % |
| 900 | hourly zone-temperature RMSE | — | — | 0.1182 K |
| 900FF | free-float min / max / mean °C | 1.2467 / 44.2992 / 25.1333 | 1.1445 / 45.1862 / 25.5165 | −0.10 / +0.89 / +0.38 K |
| 900FF | hourly zone-temperature RMSE | — | — | 0.4239 K |

**The cause is a schema limit, not a bug, and it is the single most useful thing in this report for W-C.**
`Fenestration` (`⚙️engine/🔋️model/🦀️.rs`) carries only `u_value_w_m2k` / `shgc` / `vlt`, so a translated window
can only become an EnergyPlus `WindowMaterial:SimpleGlazingSystem`. The native path uses the standard's real
two-pane `WindowMaterial:Glazing` + 12 mm air stack. Simple glazing has a different angular transmittance curve,
so it admits more summer solar — the sign of every delta above. Everything else in the translation is faithful:
hourly zone-temperature RMSE is 0.05–0.12 K for the controlled cases, i.e. the two models track each other
within a tenth of a kelvin all year and diverge only in how much solar the window passes.

**Sensitivity, measured on the same four cases.** The same run with `u_value_w_m2k 2.740` / `shgc 0.760` — the
values EnergyPlus itself reports for the detailed BESTEST construction in its Exterior Fenestration table —
gives case 600 heating −1.41 % / cooling +2.51 %, case 900 heating −3.64 % / cooling +4.95 %, 600FF max
+1.52 K, 900FF max +0.75 K. So the window pair is worth about 2.5 percentage points of annual cooling, and
`2.740 / 0.760` is the closer surrogate under EnergyPlus's simple-glazing convention (where the U-factor is the
whole assembly's NFRC value, not a film-free conductance). W-C's fixtures currently quote `3.0 / 0.787`; the
emitted `🧫️bestest-<case>-semio-model.json` documents follow them so the two sides stay comparable, but
**whichever pair is chosen, both sides must use the same one** — this is a units/convention question about what
`Fenestration.u_value_w_m2k` means, and it is not currently written down anywhere.

Implication for the acceptance tolerances in `📓️bestest-contract.md`: the semio engine cannot do better than
this translation on solar-driven cooling, because it reads the same `Fenestration`. **Compare the engine against
the TRANSLATED numbers for cooling, or widen the cooling tolerance by ~8 %; do not hold it to the native ones.**
The committed `🔮️energyplus.json` fixtures are the NATIVE results, because those are the ones that reproduce the
standard.

---

## 7. Two things the semio schema does not state, declared rather than guessed

1. **Aperture geometry is under-determined.** `Fenestration` has an area and no vertices. The translator places
   `n` equal rectangles with aspect (width/height) `1.5`, sill `0.2 m`, one window centred in each of `n` equal
   horizontal bays — overridable with `--aperture-aspect` / `--sill-height`. For a 6 m² window that yields
   exactly 3.0 m × 2.0 m, which reproduces ASHRAE 140's own south windows at x 0.5–3.5 and 4.5–7.5 to the
   millimetre. It is still a convention, and a `🔋️model.json` with unusual window areas will get a defensible
   but not unique placement.
2. **`Construction.layer_material_ids` has no stated order.** The translator reads it OUTSIDE→INSIDE, the
   EnergyPlus convention (`--layer-order inside-out` flips it). **The engine appears to disagree with itself
   here**: `⚙️engine/🧠️precompute/🦀️.rs:467-473` overwrites `work.solar_absorptance` and `work.emissivity` on
   every layer, so a surface ends up with the LAST id's optical properties — which is only the exterior layer
   under the opposite (inside→outside) reading. Resistance and capacitance are order-insensitive sums, so this
   only shows up in solar gain. Worth settling explicitly in the schema.

Also worth recording: a semio `Material` with `density_kg_m3 * specific_heat_j_kg_k == 0` — which is exactly how
ASHRAE 140 tabulates its floor insulation — is translated to an EnergyPlus `Material:NoMass` with
`R = thickness / conductivity`, because EnergyPlus rejects a massive layer with zero density. This is the same
substitution NREL's own encoding makes (`R-25 INSULATION`, R 25.075).

---

## 8. Traps hit during this ticket, recorded so nobody pays for them twice

* **EnergyPlus SEGFAULTS (exit 139, no `.err` message) on `ZoneHVAC:IdealLoadsAirSystem` with `Autosize`
  capacities when there is no `Sizing:Zone` object.** honeybee's default ideal-air system autosizes, and
  `SimulationParameter` with zone sizing off writes no `Sizing:Zone`, so the two defaults combine into a silent
  crash right after `Initializing Simulation`. Fix: `honeybee.altnumber.no_limit` on both limits — which BESTEST
  requires anyway.
* **The OpenStudio-bundled EnergyPlus omits `PostProcess/ReadVarsESO`**, so `energyplus -r` fails with
  `ERROR: Could not find ReadVarsESO executable` after an otherwise successful run. It also ships no
  `WeatherData/` and no `ExampleFiles/`. Read results from the SQLite output instead; never pass `-r`.
* **`Room.from_box`'s face named `Front` faces NORTH** (azimuth 0). The south wall is `Back`. Select walls by
  `face.horizontal_orientation()`, never by index — an index-based first attempt put the BESTEST glazing on the
  north wall and produced 1.67 MWh of cooling instead of 6.04.
* **`honeybee_energy.config.folders` has no environment-variable hook.** Assign
  `folders.energyplus_path` / `folders.openstudio_path` at process start; nothing then touches
  `~/ladybug_tools` or `/Applications`.
* **ladybug's `SQLiteResult` returns energies already converted to kWh**, not the raw joules the E+ variable
  name implies.
* **A reused `--work` directory silently reruns the previous IDF.** Cost one wrong pair of 640/940 numbers
  before it was caught by the NREL comparison. `_run_energyplus` now wipes the directory first.

---

## 9. Left for others

* **W-C:** `A/🧫️fixtures/🏛️bestest-<case>/🔋️model.json` is still absent on disk; the dir was polled through the
  end of this work (`📓️w3-bestest-engine.md` §1.3 says that is where they land). When each lands,
  `oracle-run <model.json> <epw> <out.json>` regenerates that case's `🔮️energyplus.json` from the real fixture
  in about 25 s. The translator already tracks their current `Model`: `model.schedules`, `model.run_period` and
  the new `Infiltration` `method`/`design_flow_ach` fields are all handled, and the emitted
  `🧫️bestest-<case>-semio-model.json` documents were regenerated in that shape.
  Also owed by them: the `energy-model-1-simulate` capability and its `oracleRequirements` entry in
  `A/🔮️oracle/🔣️.json`'s `mutationManifests` (the oracle registration already declares that capability), and
  the `energy-model-1-bestest-compare-v1` comparison-profile spec the registration names.
* **The one open question between W-B and W-C** is §6's last paragraph: what `Fenestration.u_value_w_m2k`
  MEANS. `3.0 / 0.787` (the standard's quoted air-to-air pair, what their fixtures use) and `2.740 / 0.760`
  (what EnergyPlus reports for the same physical window) differ by ~2.5 points of annual cooling. Nothing in the
  repository states the convention. Pick one, write it into the `Fenestration` docstring, and both sides follow.
* **W-D0:** `Model::schedules` and `Model::run_period` exist but, per `📓️w3-bestest-engine.md` §3, nothing in
  the kernel reads them yet. The oracle reads them from the document today, so finishing that move on the Rust
  side removes the last reason for the `{model, schedules}` envelope.
* The `energy-model-1-mutate` external-oracle debt is untouched and still stands — the decline was reversed for
  the simulation capability only.
