# Oracle Toolchain Research: EnergyPlus / OpenStudio / Ladybug-Honeybee / ASHRAE 140

Research date: 2026-09-06. Host verified: macOS 26.6.2 (Darwin 25.6), arm64 (Apple Silicon). `uv 0.11.15`, repo Python managed at 3.14.5 (arm64), with 3.11/3.12/3.13 downloadable on demand.

> **Note on org rename:** the `NREL` GitHub org (EnergyPlus, OpenStudio, BESTEST-GSR, …) now 301-redirects to **`NatLabRockies`** (e.g. `github.com/NatLabRockies/EnergyPlus`). Both old and new URLs resolve; the underlying repo/release/asset content is identical. OpenStudio's own license header now reads copyright "Alliance for Energy Innovation, LLC" (previously "Alliance for Sustainable Energy, LLC" i.e. NREL's operating contractor). Use whichever URL form; I use `NREL/...` below since it still resolves and is what most third-party docs reference, with `NatLabRockies/...` as the canonical redirect target.

---

## 1. EnergyPlus

**Latest stable: v26.1.0** (not 25.x — the tool moved a full major cycle since the ticket was scoped). Published 2026-03-31.
Release: https://github.com/NREL/EnergyPlus/releases/tag/v26.1.0 (→ redirects to `NatLabRockies/EnergyPlus`)

A slightly older but currently more relevant build is **v25.2.0** ("official"), because it is the exact version OpenStudio 3.11.0 bundles (see §2) — pin to whichever the OpenStudio side needs if running the Honeybee→OpenStudio→E+ chain, or 26.1.0 if running IDFs directly as a secondary/independent oracle.

### Release assets (archives, no installer/sudo needed) — v26.1.0

| Asset | Size |
|---|---|
| `EnergyPlus-26.1.0-6f2e40d102-Darwin-macOS13-arm64.tar.gz` | 209.9 MB |
| `EnergyPlus-26.1.0-6f2e40d102-Darwin-macOS12.1-x86_64.tar.gz` | 215.9 MB |
| `EnergyPlus-26.1.0-6f2e40d102-Linux-Ubuntu24.04-x86_64.tar.gz` | 240.3 MB |
| `EnergyPlus-26.1.0-6f2e40d102-Linux-Ubuntu24.04-arm64.tar.gz` | 232.4 MB |
| `EnergyPlus-26.1.0-6f2e40d102-Windows-x86_64.zip` | 242.4 MB |
| `EnergyPlus-26.1.0-6f2e40d102-Windows-arm64.zip` | 227.1 MB |

Note: 26.1.0 dropped Ubuntu 22.04 assets (only 24.04 now); Windows now ships an `arm64` build too. `.tar.gz`/`.zip` extract with no install step — just untar/unzip and reference the binary inside (`energyplus` on macOS/Linux, `energyplus.exe` on Windows) plus the bundled `Energy+.idd`, `WeatherData/`, `ExampleFiles/`, `PreProcess/`, `PostProcess/` (ReadVarsESO), `DataSets/` dirs — no `sudo`/registry writes required, everything is self-contained under the extracted root.

For v25.2.0 (the version OpenStudio 3.11.0 bundles), same asset-naming scheme, fetch via `https://github.com/NREL/EnergyPlus/releases/tag/v25.2.0`.

### CLI (confirmed from `src/EnergyPlus/CommandLineInterface.cc` on the `develop` branch)

```
energyplus [OPTIONS] [input_file]
  -w, --weather <path>          Weather file (default: in.epw in cwd)
  -d, --output-directory <dir>  Output directory (default: cwd)
  -i, --idd <path>              Input Data Dictionary (default: Energy+.idd next to binary)
  -r, --run-period (implicit, via -a/-D)
  -a, --annual                  Force annual simulation
  -D, --design-day              Force design-day-only simulation (excludes -a)
  -m, --epmacro                 Run EPMacro before simulating
  -x, --expandobjects           Run ExpandObjects before simulating
  -r, --readvars                Run ReadVarsESO after simulating (ESO/MTR -> CSV)
  -c, --convert                 IDF->epJSON or epJSON->IDF depending on input type
  --convert-only                Convert only, do not simulate
  -p, --output-prefix <PRE>     Output file prefix (default: eplus)
  -s, --output-suffix <mode>    Output suffix mode
  -j, --jobs <n>
  input_file                    Positional; .idf or .epJSON (default in.idf)
```
So the ticket's assumed invocation `energyplus -w <epw> -d <outdir> -r <idf>` is correct in spirit; in practice add `-x` (ExpandObjects, needed if the IDF has `HVACTemplate:*` objects — Honeybee's ideal-air-system IDFs typically don't need it) and `-a` to force annual runs regardless of the `RunPeriod` object's `Number of Times Runperiod to be Repeated`.

**epJSON**: fully supported as a first-class input format alongside `.idf` (schema `Energy+.schema.epJSON` ships next to `Energy+.idd`); `-c`/`--convert`/`--convert-only` round-trip IDF↔epJSON via the bundled converter. Honeybee's own IDF writer (`honeybee_energy.writer`) emits classic IDF text, not epJSON, so no epJSON involvement is required for the Honeybee path — only relevant if semio's own Rust model writer chooses to emit epJSON directly instead of IDF text.

### Bundled weather (from the EnergyPlus source `weather/` folder, which mirrors the installed `WeatherData/` dir)
- `USA_CO_Golden-NREL.724666_TMY3.epw` (+ `.ddy`, `.stat`) — confirmed present.
- **`USA_CO_Denver-Stapleton_TMY.epw`** (+ `.ddy`, `.stat`) — confirmed present. This is the historical BESTEST/ASHRAE-140 Denver TMY station (Denver-Stapleton, the classic reference weather file used in NREL's original BESTEST reports), still shipped today.
- The old literal filenames `DRYCOLD.epw` / `725650TY.epw` are **not** present anywhere in the current repo tree (checked full recursive tree, 14,643 paths) — those are legacy DOE-2/BLAST-era names from the original 1980s-90s BESTEST fileset. `USA_CO_Denver-Stapleton_TMY.epw` is the modern EnergyPlus-native equivalent and is what current ASHRAE-140 tooling (BESTEST-GSR, see §4) actually feeds to E+/OpenStudio.

### ASHRAE 140 / BESTEST case IDFs in `testfiles/`
**Not present.** I grepped the full recursive repo tree (1042 `testfiles/*` entries alone, 14,643 total) for `ashrae.?140`, `bestest`, `600`, `900`, `195`, etc. The only hits are the **ASHRAE 90.1-2019 prototype buildings** (`ASHRAE901_OfficeSmall_STD2019_Denver.idf`, `ASHRAE901_Hospital_STD2019_Denver.idf`, etc. — 19 files), which are a *different* benchmark (DOE/PNNL commercial prototypes), not BESTEST. The only "bestest" string anywhere in the repo is an unrelated Kiva (foundation heat transfer) unit-test fixture (`third_party/kiva/test/unit/fixtures/bestest-fixture.hpp`).
**Conclusion: EnergyPlus no longer ships ASHRAE 140 case files as example/test IDFs.** They must be sourced from NREL/BESTEST-GSR (see §4) or hand-authored from the Standard 140 spec.

---

## 2. OpenStudio SDK

**Latest stable: v3.11.0**, published 2026-01-15. Release: https://github.com/NREL/OpenStudio/releases/tag/v3.11.0
Bundles **EnergyPlus v25.2.0 "official"** (confirmed from the v3.11.0 changelog: multiple `V25.2.0-IOFreeze:` entries culminating in "Update to EnergyPlus v25.2.0 official", PR #5527). A `v4.0.0-alpha` prerelease exists (2026-09-01) but is not stable.

### Release assets (archives)

| Asset | Size |
|---|---|
| `OpenStudio-3.11.0+241b8abb4d-Darwin-arm64.tar.gz` | 329.2 MB |
| `OpenStudio-3.11.0+241b8abb4d-Darwin-x86_64.tar.gz` | 346.8 MB |
| `OpenStudio-3.11.0+241b8abb4d-Ubuntu-24.04-x86_64.tar.gz` | 400.2 MB |
| `OpenStudio-3.11.0+241b8abb4d-Ubuntu-24.04-arm64.tar.gz` | 378.2 MB |
| `OpenStudio-3.11.0+241b8abb4d-Ubuntu-22.04-x86_64.tar.gz` | 396.6 MB |
| `OpenStudio-3.11.0+241b8abb4d-Windows.tar.gz` | 326.8 MB |
| `OpenStudio-3.11.0+241b8abb4d-AlmaLinux-9.7-x86_64.tar.gz` | 391.9 MB |

**macOS arm64 tar.gz exists natively** — no Rosetta fallback is needed on Apple Silicon; ship the `Darwin-arm64` archive directly. (`.dmg`/`.exe`/`.deb`/`.rpm` installers also exist but the `.tar.gz` extracts cleanly without an installer or elevated privileges, matching the zero-touch requirement — confirm the extracted tree is relocatable, which it is for OpenStudio's typical layout.)

### CLI
`openstudio` ships inside the extracted tree at `<root>/bin/openstudio` (a self-contained Ruby+C++ interpreter with EnergyPlus embedded under `<root>/EnergyPlus/`). Confirmed usage patterns (standard OpenStudio CLI, unchanged in 3.11.0):
- `openstudio --version`
- `openstudio run -w workflow.osw [-m]` — runs an OpenStudio Workflow (`.osw`) file, which is how Honeybee's `to_openstudio_osw` + `run_osw` drive it (see §3).
- `openstudio labs` exists as a subcommand in recent OpenStudio CLIs (Streamlit/Jupyter-style scripting environment) — not needed for this pipeline.

### Python bindings (`openstudio` on PyPI)
Package `openstudio` v3.11.0 on PyPI — **not pure Python**, it's a compiled SWIG/pybind wrapper around the C++ SDK. Wheel matrix (from PyPI JSON API):
- **Windows**: per-CPython-version wheels `cp310`–`cp313` (`win_amd64`) — **no cp314 wheel yet**. 54.7 MB each.
- **macOS**: `openstudio-3.11.0-py3-none-macosx_11_0_arm64.whl` (61.3 MB) and `-macosx_10_9_x86_64.whl` (64.2 MB) — tagged generic `py3` (not CPython-ABI-specific), suggesting these wrap a version-agnostic loader; still needs empirical verification under 3.12/3.13 that the embedded native module actually loads (the `py3-none` tag does not by itself guarantee ABI compatibility across CPython minors for a package with a binary `.so`).
- **Linux**: `manylinux1_x86_64` and `manylinux2014_aarch64`, both tagged `py3` (74–79 MB).
- PyPI metadata says `requires_python: >=3.7.1` (a very loose constraint that likely predates 3.13/3.14 and shouldn't be trusted over the actual wheel tags above).

**Practical implication for repo Python 3.14**: since Windows wheels stop at cp313 and the repo's `uv`-managed Python is 3.14, **the honeybee/openstudio venv should pin `uv venv --python 3.12`** (a stable, downloadable, broadly-supported version across all three OS wheel matrices) rather than 3.14, confirming the ticket's own proposed design in §5.

---

## 3. Ladybug Tools ecosystem

All packages below are **pure-Python wheels** (`py3-none-any`, no C-extensions of their own — the only native code in the whole chain is the `openstudio` bindings package in §2). Sizes are tiny (11 KB–630 KB tarballs). Current PyPI versions as of 2026-09-06:

| Package | Version | Notes |
|---|---|---|
| `ladybug-core` | 0.44.59 | depends on `ladybug-geometry==1.35.4` |
| `ladybug-geometry` | 1.35.4 | |
| `ladybug-comfort` | 0.19.8 | optional `numpy==2.1.0` extra for `[mapping]` only |
| `honeybee-core` | 1.64.68 | depends on `ladybug-core==0.44.59`, `honeybee-schema==2.2.0` |
| `honeybee-schema` | 2.2.0 | pydantic-based HBJSON schema/validation |
| `honeybee-standards` | 2.0.7 | |
| `honeybee-energy` | 1.123.32 | depends on `honeybee-core==1.64.68`; optional extras `[standards]` → `honeybee-energy-standards==2.3.4`, `[openstudio]` → `honeybee-openstudio==0.7.2` |
| `honeybee-energy-standards` | 2.3.4 | DOE/ASHRAE 90.1 construction & program-type libraries |
| `honeybee-openstudio` | 0.7.2 | **pins `openstudio==3.11.0` exactly** — this is the pure-Python HBJSON→OSM translator that drives the compiled `openstudio` bindings |
| `lbt-recipes` | 0.28.9 | Queenbee/Pollination recipe layer — optional, not needed for a direct scripted pipeline |

None of the `requires_python` fields are set (all `None`/unconstrained at the metadata level for the pure-Python ones), so 3.12/3.13/3.14 are all nominally installable; the actual functional ceiling is set by the `openstudio` compiled package (3.13 max confirmed via wheels, 3.14 unconfirmed/unavailable). **Recommendation: use Python 3.12 for the whole venv** — it's the safe common denominator across `openstudio`'s Windows/macOS/Linux wheel matrix and has been out long enough that all these Ladybug packages are well-exercised on it.

### (a) Build a Model in Python
Confirmed on `ladybug-tools/honeybee-core@master`: `honeybee.room.Room.from_box(identifier, width=3.0, depth=6.0, height=3.2, orientation_angle=0, origin=Point3D(0,0,0))` classmethod exists exactly as expected, producing 6 `Face`s ordered `(Bottom, Front, Right, Back, Left, Top)`. `honeybee.face.Face`, `honeybee.aperture.Aperture` are the standard geometry primitives (add apertures via `Face.add_aperture` / punch-window helpers). Constructions/materials live in `honeybee_energy.construction.opaque`/`.window` + `honeybee_energy.material.opaque`/`.glazing`; schedules in `honeybee_energy.schedule.ruleset`/`.fixedinterval`; program types in `honeybee_energy.programtype`. `honeybee_energy/hvac/idealair.py` exists on `master` and is the `IdealAirSystem` module (confirmed path via GitHub contents listing of `honeybee_energy/hvac/`: `idealair.py`, `allair/`, `doas/`, `heatcool/`, `detailed.py`).

### (b) Write HBJSON
Standard honeybee-core `model.to_dict()` / `Model.to_hbjson(...)` — validated against `honeybee-schema` (pydantic models mirroring the same JSON schema Ladybug Tools publishes for Rhino/Grasshopper interop). This is the natural machine-readable interchange point if semio's Rust model wants to emit/consume the same schema for cross-checking against Honeybee's own model instead of only comparing final IDF/SQL output.

### (c) Translate to IDF
- **Without OpenStudio** (legacy path, still present): `honeybee_energy.run` / `honeybee_energy.writer` — the model can be written directly to IDF text via honeybee-energy's own writer logic (no OpenStudio dependency at all). This is the lighter-weight, pure-Python route.
- **With OpenStudio** (recommended, what Ladybug Tools itself has moved to): confirmed function list in `honeybee_energy/run.py` (fetched from `master`): `to_openstudio_osw(osw_directory, model_path, sim_par_json_path=None, ...)` builds an `.osw` workflow JSON referencing the model + simulation parameters; `run_osw(osw_json, measures_only=True, silent=False)` (with private `_run_osw_windows` / `_run_osw_unix` backends) invokes the `openstudio` CLI/bindings to execute that workflow, translating HBJSON→OSM→IDF and optionally simulating in one shot. `honeybee_openstudio` (0.7.2) is the newer **pure-Python-driving-the-compiled-bindings** translator that does the HBJSON→OSM step in-process via the `openstudio` Python package instead of shelling out to `openstudio run`.

### (d) Run EnergyPlus
`honeybee_energy.run.run_idf(idf_file_path, epw_file_path=None, expand_objects=True, silent=False)` (private OS-specific backends `_run_idf_windows` / `_run_idf_unix`) — this is the direct-IDF execution path, useful for the "secondary oracle" use case of running bundled/BESTEST IDFs straight through E+ without going through OpenStudio at all.

### (e) Read results
`ladybug.sql.SQLiteResult` (in `ladybug/sql.py`, confirmed on `master`) wraps the E+ `.sql` output (SQLiteResult requires `-r/--readvars`-independent SQL output, i.e. `Output:SQLite,SimpleAndTabular;` in the IDF/epJSON — Honeybee's simulation-parameter writer adds this automatically). Key methods: `values_by_output_name(output_name)`, `data_collections_by_output_name(output_name)` (returns Ladybug `HourlyContinuousCollection`s), `data_collections_by_output_name_run_period(...)`, `zone_cooling_sizes` / `zone_heating_sizes` (→ `ZoneSize` objects with `calculated_design_load`, `final_design_load`, `peak_date_time`, `peak_temperature` — this is your **peak load** source), `component_sizes_by_type`, `tabular_data_by_name` (for ASHRAE-140-style summary tables).

Confirmed exact E+ output-variable name strings from `honeybee_energy/simulation/output.py` (`Simulation­Output.add_*` methods), directly usable as `SQLiteResult.values_by_output_name(...)` arguments:
- **Zone air temperature (free-float cases 600FF/900FF etc.)**: `'Zone Mean Air Temperature'`, `'Zone Operative Temperature'` (via `add_comfort_metrics()`)
- **Ideal-loads heating/cooling energy**: `'Zone Ideal Loads Supply Air Total Cooling Energy'`, `'Zone Ideal Loads Supply Air Total Heating Energy'` (via `add_zone_energy_use()`); sensible-only variants `'Zone Ideal Loads Supply Air Sensible Cooling/Heating Energy'`
- **Solar gains**: `'Surface Window Transmitted Beam Solar Radiation Energy'`, `'Surface Window Transmitted Diffuse Solar Radiation Energy'`, `'Surface Window Transmitted Solar Radiation Energy'` (via `add_glazing_solar()`)
- **Peak loads**: via `SQLiteResult.zone_cooling_sizes` / `zone_heating_sizes` (`ZoneSize.calculated_design_load` in W, `.peak_date_time`, `.peak_temperature`), not a raw output-variable name — comes from the `Zone Sizing` tabular report the sizing run writes to SQL.

### Where Honeybee finds the E+/OpenStudio install
Read `honeybee_energy/config.py` (`master`, 820 lines) directly:
- **No environment-variable override exists.** The `Folders` class does *not* check `ENERGYPLUS_PATH`/`OPENSTUDIO_PATH` or any similar env var.
- Defaults come from `honeybee_energy/config.json` (bundled in the package, keys `"energyplus_path": ""`, `"openstudio_path": ""`). Empty string triggers auto-detection: `_find_energyplus_folder()` searches `/Applications/energyplus*` (macOS), `C:\energyplus*` (Windows), `/usr/local/energyplus*` (Linux), *and first checks alongside the resolved OpenStudio install* (`<openstudio_root>/../EnergyPlus`); `_find_openstudio_folder()` checks `ladybug_tools_folder` first (see below), then `/Applications`, `C:\`+`C:\Program Files`, `/usr/local`. Both pick the highest version number found.
- `Folders.config_file` can be repointed to any custom `config.json`; **or**, simplest for a repo-cache install, just set the properties programmatically at runtime before calling any run function — the module's own docstring example does exactly this: `folders.energyplus_path = "C:/EnergyPlusV9-0-1"`. Same pattern applies to `folders.openstudio_path`.
- `ladybug_tools_folder` itself is controlled by `ladybug/config.py`'s own `Folders` class, which **also has no env-var hook** — its default is `_find_default_ladybug_tools_folder()` → `os.getenv('HOME')/ladybug_tools` (mac/Linux) or `%PROGRAMFILES%\ladybug_tools` (Windows), again overridable via `ladybug/config.json` or by setting `ladybug.config.folders.ladybug_tools_folder = <path>` at runtime.

**Design implication**: to point everything at a repo-local cache with zero touch of the user's home directory or `/Applications`, the semio wrapper script must, at process start (before importing any `honeybee_energy.run` function), execute:
```python
from honeybee_energy.config import folders as hbe_folders
hbe_folders.energyplus_path = "<cache>/energyplus-26.1.0-<platform>"
hbe_folders.openstudio_path = "<cache>/openstudio-3.11.0-<platform>"
```
This avoids writing any config.json and avoids any dependency on `~/ladybug_tools` or `/Applications` existing.

---

## 4. ASHRAE 140 / BESTEST case data

Confirmed: EnergyPlus's own repo ships **no** BESTEST case files (see §1). The authoritative current source is:

**`github.com/NREL/BESTEST-GSR`** (redirects to `NatLabRockies/BESTEST-GSR`; an archived variant also exists at `NatLabRockies/BESTEST-GSR-ARCHIVE`) — "Building Energy Simulation Test - Generation Simulation and Reporting", maintained by NREL for exactly this purpose (automating ASHRAE 140 test-case generation/simulation/reporting for E+-based tools). Per its README (fetched 2026-09-06):
- Built against **OpenStudio 3.11.0 (bundling EnergyPlus 25.2.0)**, Ruby 3.2.2 — i.e. the exact same OpenStudio version identified in §2, so this is directly compatible with the toolchain proposed here.
- Structure: `measures/` (OpenStudio measures that *generate* the 97 test-case models programmatically — cases aren't hand-authored IDFs, they're parametrically built from `shared_resources/`: 9 base geometry OSMs + 11 EPW weather files + shared construction libraries), `integration_testing/workflow/<case>/data_point.osw` (per-case OSW workflow definitions — this is your ready-made list of the 97 case IDs), `results/` (pre-populated Standard-140-2020-template Excel spreadsheets: `RESULTS5-2A.xlsx` for envelope/fabric-load cases (the 600/900 series + variants), `RESULTS5-3A/B.xlsx` for cooling-equipment cases, `RESULTS5-4.xlsx` for heating-equipment cases), `results/bestest_zips/` (zipped per-case detailed E+/OSM/IDF outputs from a real run — this is a ready oracle-output dataset you can diff against without re-running anything), `run_all_generate_reports.rb` (master driver script).
- Covers **Section 5.2 (Building Thermal Envelope and Fabric Load Tests)** — this is where cases 600/600FF/610/620/630/640/650/900/900FF/910/920/930/940/950/960/195-series analytical cases live — plus 5.3 (cooling) and 5.4 (heating) equipment cases (179D scope).

**Complementary/background sources** (found but not deeply fetched — cite for the report reader to pull spec text/numeric tables themselves given copyright limits on reproducing report text verbatim):
- ANSI/ASHRAE Standard 140-2017/2020 itself (purchase from ashrae.org) — the normative case definitions (geometry, layer-by-layer construction conductivity/density/specific-heat/thickness, 0.5 ACH infiltration, 200 W internal gains, 20/27 °C thermostat setpoints, ground-coupling assumptions) originate here; BESTEST-GSR's `shared_resources/` encodes these as OSM/Ruby, which is the practical way to get them without buying the standard.
- NREL/TP-550-43827 (Sept 2008), *"EnergyPlus Testing With ANSI/ASHRAE Standard 140-2001 (BESTEST)"* — https://docs.nrel.gov/docs/fy08osti/43827.pdf — this is the classic report carrying the annual heating/cooling MWh, peak-kW, and free-float min/max-temperature **reference envelopes** (min/max across the reference programs BLAST/DOE-2/SRES/SUNCODE/etc. for each case) for the 600/600FF/900/900FF series specifically. I did not extract the numeric tables themselves (would require parsing the PDF's tables, and reproducing them verbatim risks the copyright-quote limit) — **recommend fetching this PDF directly and reading Table entries for Case 600/600FF/900/900FF** rather than relying on a paraphrase here.
- LBNL companion doc (same title) at `simulationresearch.lbl.gov/dirpubs/epl_bestest_ash.pdf` — appears to be a mirror/companion of the same NREL report.
- ANL 2020 update (`publications.anl.gov/anlpubs/2020/05/158451.pdf`) and IBPSA BS2021 paper (`publications.ibpsa.org/proceedings/bs/2021/papers/bs2021_30365.pdf`) cover the more recent Section 5.2 case additions (post-2017 revision) — relevant if targeting the current Standard 140-2020 case set rather than the original 1990s-2001 set.
- `github.com/Pamekitti/energyplus-bestest-validation` — a smaller third-party repo specifically for Case 600, bundling both E+ input files and BESTEST weather data + reference data from other tools; useful as a lightweight secondary cross-check but NREL/BESTEST-GSR is the authoritative/comprehensive one.

**Recommendation**: don't hand-transcribe the Standard 140 spec tables. Instead, either (a) run BESTEST-GSR's own Ruby/OpenStudio measures to generate the case IDFs+OSWs directly (gives you exact, currently-validated case definitions plus the reference `results/bestest_zips/` to diff against), or (b) read the case-generation measures' Ruby source to extract the parametrized geometry/construction/schedule values into semio's own Rust fixtures by hand (per CLAUDE.md's "handcraft all assets" rule — no migration scripts).

---

## 5. Provisioning design (proposal only — no downloads performed)

### Cache layout
```
.🧬semio/🦑️repo/⚡️cache/oracles/
  energyplus-26.1.0-darwin-arm64/         (extracted tar.gz)
  energyplus-25.2.0-darwin-arm64/         (if pinned to OpenStudio's bundled version instead)
  openstudio-3.11.0-darwin-arm64/         (extracted tar.gz)
  venv-honeybee-py312/                    (uv venv, --python 3.12)
  checksums.json                          (sha256 per archive, platform, version)
```
Platform key derived from `os.platform()`/`os.arch()` (Node) mapped to the exact release-asset suffixes captured above:
- macOS arm64 → `Darwin-macOS13-arm64` (E+) / `Darwin-arm64` (OpenStudio)
- macOS x64 → `Darwin-macOS12.1-x86_64` / `Darwin-x86_64`
- Linux x64 → `Linux-Ubuntu24.04-x86_64` / `Ubuntu-24.04-x86_64`
- Windows x64 → `Windows-x86_64.zip` / `Windows.tar.gz`
- devcontainer → same as Linux x64 (assume Ubuntu 24.04 base image; confirm against the actual devcontainer base)

### `📜️script.ts` command shape
`nx run <energy-plugin>:script oracle setup [--energyplus-version 26.1.0|25.2.0] [--openstudio-version 3.11.0] [--python 3.12]`
1. Detect platform/arch → resolve exact asset filenames from a small versioned table (the table above), matching GitHub's release-asset naming exactly (asset names are not perfectly systematic across OSes — e.g. macOS uses `Darwin-macOS13-arm64`, Linux uses `Linux-Ubuntu24.04-x86_64`, Windows omits an OS-name segment — so this must be a literal per-(tool,version,platform) lookup table, not a generated pattern).
2. Download each archive to the cache dir with a `.sha256` sidecar; verify against `checksums.json` (fetch the release's own `sha256sums.txt` asset — confirmed present in the EnergyPlus release asset list — as the source of truth, or pin known-good hashes fetched once and stored in the ticket/script).
3. Extract (tar/unzip via Node's built-ins or `tar`/`unzip` — cross-platform, no shell-specific `sudo`).
4. `uv venv --python 3.12 <cache>/venv-honeybee-py312` then `uv pip install --python <venv> honeybee-energy==1.123.32 honeybee-openstudio==0.7.2 ladybug-core==0.44.59` (pin exact versions used during this research so the pipeline is reproducible; bump deliberately, not via floating ranges, per the "no legacy/no drift" ethos).
5. Emit a small `oracle-env.json`/`.py` snippet in the cache dir recording the resolved `energyplus_path`/`openstudio_path` for the wrapper script to `hbe_folders.energyplus_path = ...` at runtime (per §3's config mechanism) — never write into `~/ladybug_tools` or `/Applications`.

### Estimated total download size per platform (E+ + OpenStudio only, venv installs are KB-scale)
- macOS arm64: 209.9 MB (E+) + 329.2 MB (OpenStudio, already bundles its own E+ so **only one of the two E+ copies is actually needed** if running solely through the OSW path — download standalone E+ only if also running bundled/BESTEST IDFs directly as a secondary oracle) ≈ **330–540 MB**
- macOS x64: 215.9 + 346.8 ≈ 350–563 MB
- Linux x64 (Ubuntu 24.04): 240.3 + 400.2 ≈ 400–640 MB
- Windows x64: 242.4 + 326.8 ≈ 330–569 MB

### Licenses
- **EnergyPlus**: BSD-3-Clause-style DOE license (confirmed header in `LICENSE.txt`: "Redistribution and use in source and binary forms... conditions are met", copyright UIUC/UC-LBNL/ORNL/Alliance for Energy Innovation LLC + DOE government-rights notice). Permissive, redistribution-friendly.
- **OpenStudio**: same BSD-3-Clause-style pattern (confirmed header in `LICENSE.md`, copyright "Alliance for Energy Innovation, LLC"). Permissive.
- **Ladybug Tools (`ladybug-core`, `honeybee-core`, `honeybee-energy`, `honeybee-openstudio`, etc.)**: **AGPL-3.0** (confirmed via GitHub's license API on all four checked repos). AGPL's network-copyleft clause is the concern to flag: if semio ever *serves* functionality built on these packages over a network (not just uses them as an internal offline validation oracle in CI/dev), AGPL requires offering the combined work's source. For the stated use case — a local/CI-only "run our engine, run theirs, diff the results" validation harness that is never itself distributed or served as a product — this is test/dev-tooling usage and should be fine, but **do not link or vendor honeybee/ladybug code into any shipped semio artifact**; keep it strictly as an external subprocess/venv invoked only by the oracle-comparison test harness, never imported into `s.energy.model`'s own runtime.
- `honeybee-schema` license not independently checked but is part of the same org/ecosystem — assume AGPL-3.0 as well pending direct confirmation if it becomes load-bearing.

### Fallback notes
- **No Rosetta fallback needed for OpenStudio on macOS arm64** — a native `Darwin-arm64` OpenStudio SDK tar.gz and a native `py3-none-macosx_11_0_arm64` `openstudio` Python wheel both exist for 3.11.0. The ticket's premise that OpenStudio might lack an arm64 build is outdated as of this version.
- **Python 3.14 is not yet viable** for the `openstudio` compiled bindings (wheels stop at cp313 on Windows; macOS/Linux wheels are tagged generically but unverified above 3.13) — pin the oracle venv to **3.12** regardless of the repo's own 3.14 default; this is an isolated venv so it doesn't conflict with the rest of the toolchain.
- No Docker/daemon dependency in this design — everything is plain archive-extract + `uv venv`, matching "no Docker daemon running" constraint and working identically in a devcontainer, native macOS/Linux/Windows.

---

## Key URLs (for the reader's own follow-up)
- EnergyPlus releases: https://github.com/NREL/EnergyPlus/releases (v26.1.0 latest stable, v25.2.0 = version OpenStudio 3.11 bundles)
- OpenStudio releases: https://github.com/NREL/OpenStudio/releases (v3.11.0 latest stable)
- OpenStudio Python bindings: https://pypi.org/project/openstudio/
- honeybee-energy: https://pypi.org/project/honeybee-energy/ · https://github.com/ladybug-tools/honeybee-energy
- honeybee-openstudio: https://pypi.org/project/honeybee-openstudio/ · https://github.com/ladybug-tools/honeybee-openstudio
- ladybug-core: https://pypi.org/project/ladybug-core/ · https://github.com/ladybug-tools/ladybug
- honeybee-core / honeybee-schema: https://pypi.org/project/honeybee-core/ · https://pypi.org/project/honeybee-schema/
- BESTEST-GSR: https://github.com/NREL/BESTEST-GSR (archive: https://github.com/NatLabRockies/BESTEST-GSR-ARCHIVE)
- BESTEST reference report: https://docs.nrel.gov/docs/fy08osti/43827.pdf
- ASHRAE 140 update (2020 cases): https://publications.anl.gov/anlpubs/2020/05/158451.pdf
