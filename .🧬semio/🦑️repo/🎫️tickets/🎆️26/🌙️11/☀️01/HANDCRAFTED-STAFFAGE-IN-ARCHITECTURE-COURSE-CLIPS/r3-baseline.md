# 🧭 Round 3 baseline: open items and owners

Worker `doc` (W20), round 3. Source of the open list: `r2-open-items.md` (status `ol`, final) and `r2-coordinator-rulings.md`. Spot checks were run on 2026-11-03 with Grep, Glob and Read against the current tree. Paths are relative to `A` = `🎓️teaching/🏛️architecture` unless shown otherwise. Quoted evidence is the grep output or the file line.

Status codes in the last column:

- **open**: still present in source, or no evidence of a fix.
- **resolved**: the source no longer shows the problem (evidence quoted).
- **changed**: the source differs from the r2 text, and the fix is not confirmed by a run.
- **ruled**: r2 coordinator ruling says no change. The owner should confirm or close.
- **unverified**: needs a run or render. Grep cannot decide it.

## Summary

The 60 items below are the r2 ledger items that are still listed as open, ruled or verification-pending. The 42 mapped items are the brief map. The other 18 are unmapped.

| class | count | ids |
|---|---|---|
| resolved in source (grep evidence) | 4 | OI-23, OI-48, OI-124, OI-125 |
| wording corrected in summary.md | 2 | OI-74, OI-109 |
| changed, fix not yet confirmed | 2 | OI-113, OI-114 |
| ruled no-change, owner to confirm or close | 15 | OI-04, OI-21, OI-27, OI-41, OI-46, OI-47, OI-54, OI-73, OI-90, OI-93, OI-116, OI-117, OI-120, OI-122, OI-129 |
| open (including unverified and dispatched) | 37 | the rest |

The OI-xx ids are the r2 ids. Items that r2 raised as new (OI-110 to OI-135) use the same ids.

## Owners from the brief map

### W05 contract (provenance), W06 thermal, W07 structural, W17 media

No r2 OI is mapped to these workers. New r3 items only.

### W08 people

| OI | area | evidence (current tree) | r3 state |
|---|---|---|---|
| OI-47 | Excel plan occupants, one relaxed pose | `📊️excel/🎬️clip/📋️program/🐍️.py:59` `def occupant_spots`, `:358` call. Pose count not re-read. | ruled (rulings: round-1 ruling stands). Owner to confirm, then close. |
| OI-117 | physics clerk at 0.72 of house scale | `⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:956` `scale(0.72)`, `:986-987` `1.75 * 0.72 * house["w_height"] / 5.0` | ruled (rulings: plausible as designed). |

### W09 vegetation

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-10 | cast exporter peak memory (1206 MB, 757 MB) | No investigation or fix report in T. Not grep-checkable. | open |

### W10 objects

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-04 | practice building store as `energy_tank` | `⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py:1144` `energy_tank(np.array([TANK_X, ...]), height=1.30, width=0.80, ...)` | ruled keep (`r2-coordinator-rulings.md`: abstract "Quelle" gauge). Conflicts with the r2 ledger, which says the coordinator should overrule. W10 to confirm or reopen. |
| OI-23 | kettle mass convention | `🧍️staffage/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json:51` `"massKg": 1.22` | **resolved** (dry mass 1.22, the ruling kr). r2 text said 2.22. |
| OI-48 | humidifier category | Found only at `🧍️staffage/🗂️catalog/🏗️machinery/evaporative-humidifier/🔣️.json`. Grep for `evaporative-humidifier` in `🗂️catalog` lists no `🔌️appliances` path. | **resolved** (moved to machinery, ruling ap). |
| OI-87 | previews of touched assets not rendered | Not grep-checkable. Dispatched to `av`. | open (unverified) |
| OI-113 | toaster casing 180 W | `🔌️appliances/toaster/🔣️.json`: literal `180` absent. `heatW` values now 95, 25, 150 (lines 1799, 1836, 1873). | **changed**. Ruling ap applied (cool-touch casing, about 60 C). Render and temperature not confirmed. |
| OI-114 | pumpjack skid and motor in one bone | `🏗️machinery/oil-pumpjack/🔣️.json`: literal `1100` absent. Separate `skid` bone (`:141`, `:429-430`) and `motor` bone (`:302`, `:1014-1015`). `heatW` 300, 720, 1800 at `:9503`, `:9573`, `:9643`. | **changed**. Ruling ap applied (skid split). Bone mass about 250 kg not confirmed. |
| OI-116 | laptop vent jet | Ruled plausible (rulings). | ruled |
| OI-123 | visual weak spots (open laptop, chalkboard, gas-hob slab) | Dispatched to `av`. | open (unverified) |

### W11 buildings

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-46 | Nakagin podium furniture as 3D boxes | `🏢️nakagin/🐍️.py:1419`, `:1428-1441` (`reception_desk`, `cafe_chair`, `office_desk`, `office_chair`), `:2457-2461` (`bed-platform`, `bed_drawer`). All still present. | ruled keep (rulings: round-1 ruling stands). The r2 ledger still lists it as an owner decision. W11 to confirm. |

### W12 timing

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-121 | invisible `_stand` prototype motion in `_room_occupants` | `⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py:70` imports `staffage_height`; `:136-140` `_stand` builds `staffage_motion(...)` with `staffage_height(cast)`; `:148-159` `_room_occupants` returns those motions as the drawn occupants. | open (partly). Dispatched to `cl`. The motions are drawn, not invisible. W12 to decide. |

### W13 player

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-128 | React viewer has no `span` range mode | Grep `span` under `🧍️staffage/🌐️site` returned 0 files. The React viewer location is not confirmed here. | open (unverified). Dispatched to `rv2`. |

### W14 heating and cooling clips

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-41 | thermometers as diagram glyphs | `thermometer_glyph` occurs 34 times in 14 files under `A` (r2 said about 25 call sites). | ruled (rulings: thermometers stay glyphs). Judgement decided once. |
| OI-112 | sorption narration says solar regenerates the wheel | `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:292` `"Solarwärme regeneriert das Rad: ..."` still present. | open. Dispatched to `cl`, which said "solar-fed regeneration coil". Narration not changed. |

### W15 ventilation, demand, physics, optimization clips

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-67 | volume Beat1 dimension overlap | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:454` narration "10 m breit" still present. Dimension overlap needs a render. | open (unverified). Dispatched to `cl`. |
| OI-115 | renders after bp, xc, ts edits | No render evidence in T. Dispatched to `ne`. | open (unverified) |

### W16 other series

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-01 | BIM reserve `RES-FIN-ROOF-GRAVEL-SL-R-705` | `🏗️bim/🎬️clip/🧱️model/🔣️.json` and `🧭️cutter-winding.json` still contain the id. | open. Dispatched to `br`. |
| OI-02 | BIM setUpClass builds the 2 h geometry | `🏗️bim/🎬️clip/🧪️tests/🐍️.py:277` `class ModelCase(unittest.TestCase)`, base of `BimInformationContract` (`:1041`) and `BimConsistencyContract` (`:2390`). | open (same root as OI-01). |
| OI-80 | BIM information and consistency suites never ran | Same classes at `:1041` and `:2390`. | open (blocked by OI-01). |
| OI-81 | BIM clip contract timed out at 560 s | `BimClipContract` at `:2543`. No later report in T. | open |
| OI-85 | excel `test_native_lifecycle` ran 25 min, killed | `📊️excel/🎬️clip/🧪️tests/🎭️presentation/🐍️.py:235` `def test_native_lifecycle`. Not rerun. | open. Dispatched to `xl`. |
| OI-118 | BIM information car never viewed in full scene | Not grep-checkable. | open (unverified). Dispatched to `br`. |

### W18 validation

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-82 | optimization full 75-test suite not rerun | `🎯️optimization/🎬️clip/🧪️tests/🐍️.py:1184` `class SchemaFirstStudy` present. No full run in T. | open |
| OI-86 | kettle and humidifier clip unittests not run after edits | No run reported. Dispatched to `cr2`. | open |
| OI-89 | slow module sweep not rerun in one pass | No run reported. | open |
| OI-90 | sweep module list (27, 33, 34, 35 without tests) | `🧰️ht/modules.txt` in T now lists 32 entries, and its numbering has changed. Rulings say `round.sh` already skips 27, 33, 34 and 35. `round.sh` not read here. | ruled (rulings). W18 to confirm the skip in `round.sh`. |
| OI-119 | phase-change vitest, kettle plateau balance 1.7e-5 vs 1.9e-6 | `🧍️staffage/🧪️tests/🫕️phase-change/🟦️.ts` exists. No pass report. Dispatched to `rr/tr/pr`. | open (verify) |
| OI-124 | undeclared imports in test and oracle files | `⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml` `[dependency-groups] test` (lines 19-38) now declares `anastruct==1.7.1`, `ezdxf==1.4.5`, `pandas==2.3.3`, `psychrolib==2.5.0`, `pyet==1.5.0`, `pythermalcomfort==4.8.0`, `reportlab==5.1.0`, `skia-pathops==0.9.2`, `pillow==13.0.0`. Every module the r2 row named is declared. | **resolved** (declared). W18 to confirm with an import run. |

### W19 registry

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-102 | cache key folders incomplete | `C:\git\semio\.🧬semio\🎓️teaching\⚡️cache\staffage\` has key folders. Runtime state, not audited. | open (self-healing) |
| OI-125 | oracle docstrings say `uv run --with` | Grep `uv run --with` under `A`: 0 files. Grep `--with ` under `⚡️energy/🏗️authoring/🎨️visuals`: 0. | **resolved** by grep. W19 to confirm the `--project` wording. |
| OI-127 | reports cite removed ids and counts | `r2-g5-report.md` and `r2-g6-report.md` still in T, not corrected. | open. Coordinator, reports only. |

### Coordinator

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-73 | German review by reading only | Method limit. | ruled (rulings: recorded as a limit). |
| OI-74 | summary says meters are glyphs | Gas and electricity meters are casts: `🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` has 2 entries for `"gas-meter"`/`"electricity-meter"`. | **wording corrected** in `summary.md`, section "Round 3 corrections". |
| OI-109 | summary says every device is now a cast | Still in `summary.md:7`. OI-04 (`energy_tank`) and OI-41 (`thermometer_glyph`) show the claim is too broad. | **wording corrected** in `summary.md`, section "Round 3 corrections". Original text kept. |
| OI-110 | speech for changed clauses has no audio, no TTS key | No `.env` under `⚡️energy` (Glob `**/*.env`: 0 files). | open (user) |
| OI-111 | speech tooling German only, per-clip modules | `subtitle_clauses` and `synthesize_clause_audio` in `⚡️energy/🏗️authoring/🔊️speech/🐍️.py` and in per-clip `🔊️speech/🐍️.py` modules (grep limit 10 files reached). | open (speech owner, outside staffage scope) |

### Infra

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-83 | POSIX cancellation, Bun half not run | No POSIX run in T. | open (infra) |
| OI-84 | devcontainer not run, Docker stopped | r2 status. Not re-checked. | open (infra) |
| OI-100 | `repo dependencies ensure` refuses | r2 status. Not re-run. | open (infra) |
| OI-101 | `repo ffmpeg` broken, PyAV and PyMuPDF used | r2 status. Workaround in place. | open (infra) |

## Unmapped open items (no owner in the r3 brief map)

These items are open or ruled in r2 but are not in the brief map. The coordinator should assign an owner or close them.

| OI | area | evidence | r3 state |
|---|---|---|---|
| OI-11 | cast size budget, per frame or per cast | r2 judgement, no owner. | open |
| OI-21 | cylinder 1.48 m high | Ruled accepted (rulings). | ruled |
| OI-27 | shower spray 300 W | Ruled plausible (rulings). | ruled |
| OI-34 | wall in panel-radiator asset | Dispatched to `ap` (rulings: move wall out). Not grep-checked. | open (unverified) |
| OI-54 | pen-series off ISO steps | Ruled (rulings: no policy). | ruled |
| OI-91 | staffage and React vitest totals | No single final library run. | open |
| OI-93 | `.feature` coverage for Python | Ruled (rulings: JSON cases suffice). | ruled |
| OI-106 | deleted test file, missing silhouettes | `Glob **/silhouettes.py` under `⚡️energy/🏗️authoring/🎨️visuals`: 0 files. Still missing. | open (git state) |
| OI-120 | phase-change scope | Ruled (rulings: plausible). | ruled |
| OI-122 | hob pot as heat source | Ruled (rulings: plausible). | ruled |
| OI-126 | ticket data untracked | Git state. Not committed (correct per rules). | open (infra) |
| OI-129 | search facets | Ruled (rulings: outside this ticket). | ruled |
| OI-130 | React `tsc --noEmit` 12 errors in `🧰️framework` and visual check | Not checked here. | open (out of staffage scope) |
| OI-131 | heading renders not done | Not checked here. | open (unverified) |
| OI-132 | conduction Beat4 height dimension | Dispatched to `cl`. | open (unverified) |
| OI-133 | hob plan dimension labels show height | Dispatched to `av`. | open (unverified) |
| OI-134 | HRV emitter reference regime | Dispatched to `ap` (rulings: state the reference). | open (unverified) |
| OI-135 | `taskkill` may have stopped another agent's python | Check that no render was lost. | open |

## Next steps

- W10 confirms OI-04 (ruling keep or reopen), OI-113, OI-114.
- W11 confirms OI-46 ruling.
- W14 confirms the OI-41 ruling is recorded.
- W18 confirms OI-124 by import run and OI-90 by `round.sh`.
- W19 confirms OI-125 and handles OI-127 with the coordinator.
- Coordinator assigns the 18 unmapped items above.
