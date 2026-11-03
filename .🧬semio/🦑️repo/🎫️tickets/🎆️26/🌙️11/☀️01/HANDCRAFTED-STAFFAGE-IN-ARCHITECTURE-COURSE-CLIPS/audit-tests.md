# 🧪 Audit: tests, fixtures, schemas, docs and configs referencing the ad-hoc silhouettes

Read-only audit. No repo file was modified and no modifying git command was run.

- Repo root: `C:\git\semio`. Paths below are relative to it. `V` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, `I` = `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains`, `A` = `I/🏃️activity`.
- Excluded: `.venv`, `node_modules`, `.🧬semio/🦑️repo/🎫️tickets`.
- Method: ripgrep over the whole repo for `person_glyph`, `seated_person_glyph`, `object_silhouette`, `radiator(`, `sun_glyph`, `moon_glyph`, `_ACTIVITY`, `_activity_figure`, `🫟️`, `silhouettes/`, `🏃️activity`, `visuals/🔣️.json`, `visuals/🧬️schema`, the JSON keys `standing`, `seated`, `standing_height`, `seated_height`, `radiator_height`, `radiator_width`, and the schema titles. Each hit was read in context.
- Snapshot time: about 01:46 local. The working tree is changing during this audit. Other agents are migrating clip scripts to `staffage` right now. Several `radiator(` and `person_glyph` call sites seen at the start of the audit were gone by 01:46. Line numbers are current as of the last scan. Re-run the grep before editing.

## 0. Summary

- Definitions to remove: `V/🐍️.py` (`radiator`, `object_silhouette`, `person_glyph`, `seated_person_glyph`, the `_SILHOUETTES` loader, plus `sun_glyph`/`moon_glyph` because they call `object_silhouette`). Note that `sun_glyph`/`moon_glyph` are NOT in the requested list. See section 9.
- Asset and schema files to delete: `V/🫟️silhouettes/` (whole folder), `A/` (whole folder: `🔣️.json`, `🧬️schema/🔣️.schema.json`, `🧪️tests/`, three SVGs).
- Files to edit: `V/🔣️.json`, `V/🧬️schema/🔣️.schema.json`, `V/🧪️tests/*` (visuals tests), `V/🧪️tests/🔣️scenes.json`, `V/🧪️tests/🔣️integration.json`, `V/🧪️tests/🔣️.cases.json`, `V/🧪️tests/🐍️silhouettes.py`, `A/🧪️tests/*`.
- Pipeline dependency lists that name the silhouette JSON: `V/🎬️clip/🪟️presentation/🐍️.py:37` and `🎯️optimization/🎬️clip/🎨️visuals/🐍️.py:39`.
- Clip modules still calling the removed helpers: about 20 files, listed in section 5.
- Existing stale tests: `V/🧪️tests/🐍️silhouettes.py:96-106` and `A/🧪️tests/🐍️.py` already call names that no longer exist in the scene module (section 8).
- Replacement already in place: `V/🧍️staffage/🔣️.json` (new catalog with `panel-radiator`, `walking-commuter`, etc.) and the `staffage`/`staffage_motion` helpers (`V/🐍️.py` imports; physics `_radiator`/`_person`, heating fundamentals `_radiator`/`_occupant`, internal-gains `_figure`, summer `_person`).

## 1. Definitions in `V/🐍️.py` (to delete or rewrite)

| Line (current) | Symbol | What it does | Must change |
|---|---|---|---|
| 26-28 | `_ARCHITECTURE` / `_SILHOUETTES` loader | Reads `V/🔣️.json` (kept) and `V/🫟️silhouettes/🔣️.json` (to delete) | Remove the `_SILHOUETTES` line (28). Keep line 26 only if the visuals JSON stays. |
| 387 | `watt_anchor(..., compare="laptop")` calls `object_silhouette(...)` | Device icon per watt comparison (`_WATT_DEVICES`) | Replace the icon with a handcrafted staffage device or drop the icon. |
| 1140-1151 | `radiator(center, color, height, width)` | Ad-hoc filled panel from `_ARCHITECTURE_DIMENSIONS["radiator_height"/"radiator_width"]` (1144-1145) | Delete. Callers move to `staffage("...panel-radiator")`. |
| 1165-1169 | `sun_glyph` | `object_silhouette("sun", width=1.4)` | Not in the request. Decision needed (section 9). |
| 1172-1176 | `moon_glyph` | `object_silhouette("moon", width=0.48)` | Not in the request. Decision needed (section 9). |
| 1195-1197 | `object_silhouette(kind, ...)` | Draws `_SILHOUETTES[kind]` from the silhouette library | Delete. |
| 1201-1206 | `person_glyph(pos, color, scale)` | Standing figure from `_ARCHITECTURE["standing"]` and `dimensions.standing_height` (1205) | Delete. |
| 1211-1217 | `seated_person_glyph(pos, ..., chair_color)` | Seated figure from `_ARCHITECTURE["seated"]` and `dimensions.seated_height` (1215), plus chair lines | Delete. |
| (helper) | `_filled_outline` | Shared by person and object silhouettes | Delete only if nothing else calls it after the change. |

Not affected: `window_silhouette_outline` (line 62, used at 153). It is window chrome, and it does not read `_SILHOUETTES`. `lamp_glyph` (1300), `cloud_glyph` (1229), `window_glyph`, `room_section`, `house_section` and `_ARCHITECTURE["section_material"]` (942) stay.

## 2. Assets and fixtures to delete

| Path | Content | Reference that keeps it alive |
|---|---|---|
| `V/🫟️silhouettes/🔣️.json` | `outlines`: laptop, bulb, toaster, heater, vacuum, sun, moon, tree, factory, flame, monitor, microbe | `V/🐍️.py:28`, `V/🐍️.py:1197`, `V/🧪️tests/🐍️silhouettes.py`, `V/🎬️clip/🪟️presentation/🐍️.py:37`, `🎯️optimization/🎬️clip/🎨️visuals/🐍️.py:39` |
| `V/🫟️silhouettes/🧬️schema/🔣️.schema.json` | Schema "Architectural Silhouette Library" (title line 3; 12 required outline names) | none outside the folder |
| `V/🫟️silhouettes/README.md` | Source and licence notes (lines 1-5: CC0 Wikimedia "Man standing silhouette" and "Silhouette of Muybridge walking", adapted by pitr and Mette Aumala) | none. See section 7. |
| `A/🔣️.json` | `reference_height`, `bed`, `poses.sleep/walk/sprint` with `asset` paths (sprint asset at line 28) | `A/🧪️tests/🐍️.py`, `A/🧪️tests/🔣️.cases.json`, `V/🧪️tests/🐍️silhouettes.py:97-101` |
| `A/🧬️schema/🔣️.schema.json` | "Cooling Activity Silhouettes" (title line 3; requires `reference_height`, `bed`, `poses` with sleep/walk/sprint) | none outside the folder |
| `A/🧪️tests/🐍️.py` | Activity tests (see section 3) | none |
| `A/🧪️tests/🔣️.cases.json` | `reference_size` 1.205, `scales`, `poses`, `metabolic_watts` [80,100,200,800] | `A/🧪️tests/🐍️.py`. The metabolic values (line 68) are still needed; move them to an internal-gains test. |
| `A/😴️sleep/🖼️.svg`, `A/🚶️walk/🖼️.svg`, `A/🏃️sprint/🖼️.svg` | Activity SVGs | `A/🔣️.json`, `V/🧪️tests/🐍️silhouettes.py:101` |

## 3. Tests that must change or go

Rows give file:line, what the test asserts, and the change needed.

### 3.1 `V/🧪️tests/🐍️.py` (class `ArchitectureTest`)

| Line | Test | Asserts | Change |
|---|---|---|---|
| 17-27 | `test_standing_person_is_one_opaque_closed_silhouette` | `person_glyph` (line 21): one closed opaque contour, height = 0.66 x scale (CASES.standing) | Delete. Replace with a staffage test for the handcrafted standing cast. |
| 29-41 | `test_seated_person_is_one_solid_body_and_anchored_at_floor` | `seated_person_glyph` (33): one contour, height, bottom at -0.8, chest above mouth | Delete. Replace with a seated staffage test (same anchor and ordering checks). |
| 88-95 | `test_radiator_and_person_share_a_plausible_building_scale` | `radiator([0,0,0])` (90), size 0.52 x 0.26 (CASES.radiator), height ratio to `person_glyph` 0.35-0.45 (93-94), fill opacity (95) | Delete or port: take height and width from the staffage height metadata, compare to the handcrafted person. |
| 157-180 | `test_silhouette_rasters_match_independent_component_oracle` | `person_glyph` (161) and `seated_person_glyph` (162): one connected component, raster oracle (CASES.raster) | Port to staffage casts (raster of the staffage body). |
| 43-86, 97-106, 108-155 | house, room, window, lamp tests | Not silhouette-related | No change. |

### 3.2 `V/🧪️tests/🐍️silhouettes.py` (class `SilhouetteTest`, whole file)

| Line | Test | Asserts | Change |
|---|---|---|---|
| 7 | `import xml.etree.ElementTree as ET` | Used only at 101 | Remove with the activity test. |
| 12 | `CASES` from `🔣️silhouettes.json` | Keys: `people`, `devices`, `celestial`, `activities`, `activity_minimum_cubic_fraction`, `backdrop_fade` | Remove `people`, `devices`, `celestial`, `activities`, `activity_minimum_cubic_fraction`. Keep `backdrop_fade` if the test stays. |
| 15-41 | helpers `raster`, `component_count` | Generic raster and 8-connected count | Keep. The staffage tests can reuse them. |
| 45-50 | `test_human_contours_use_flowing_anatomical_curves` | `V._ARCHITECTURE["standing"/"seated"]` segments: at least 75% cubic | Delete (the JSON keys go away). Port to the staffage outline data if that format is kept. |
| 52-55 | `test_standing_entourage_has_a_natural_asymmetric_pose` | `person_glyph` (53), asymmetry >= 0.06 | Port to the handcrafted standing cast. |
| 57-73 | `test_people_are_solid_connected_and_preserve_architectural_height` | `person_glyph` (64), `seated_person_glyph` (65), height `V._ARCHITECTURE_DIMENSIONS[kind+"_height"]` (66), one connected component, max 3 hole pixels (CASES.people) | Port to staffage. The `_height` key must come from staffage metadata. |
| 75-86 | `test_equipment_is_closed_opaque_silhouette_art` | `object_silhouette` for CASES.devices (laptop, bulb, toaster, vacuum, heater), width 0.65, one part | Delete. The devices must move to staffage or be removed. |
| 88-94 | `test_sun_and_moon_are_single_filled_profiles...` | `getattr(V, kind+"_glyph")` for sun and moon (91) | Depends on the sun/moon decision (section 9). |
| 96-106 | `test_activity_art_has_brush_contours_without_speed_lines` | `scene._ACTIVITY["poses"]`, `scene._ACTIVITY_ROOT`, SVG path curve fraction >= 0.7, sprint has no speed lines. Imports `I/🐍️` (97). | Delete. It is already broken (section 8). The curve-fraction rule can go to the staffage tests. |
| 109-118 | `test_annotation_frames_and_leaders_replace_thought_bubbles` | `cloud_glyph`, `cloud_trail`, physics `_cloud` | No change. Not silhouette-related. |
| 120-129 | `test_natural_and_industrial_profiles_are_connected_filled_contours` | `object_silhouette` for tree, factory, flame, monitor, microbe (125): closed, opacity 1, one component | Delete or port to staffage. |
| 132-140 | `test_silhouette_backdrops_can_fade...` | `room_section`, `lamp_glyph`, `FadeIn`, duration 0.2 | No change. The name says silhouette but it does not use the library. |

### 3.3 `V/🧪️tests/🔣️.cases.json` (fixture for 3.1)

- `standing` (line 2), `seated` (line 8), `radiator` (line 14): delete with their tests.
- `house`, `room`, `window`, `lamp`, `raster` block: keep. `raster.connected_components` is still wanted for the staffage raster test.

### 3.4 `V/🧪️tests/🐍️integration.py` (class `SceneIntegrationTest`)

| Line | Test or helper | Asserts | Change |
|---|---|---|---|
| 31-35 | `assert_radiator(radiator, floor, room_height)` | Radiator height, width and clearance from `CASES.physical` (`radiator_height_m`, `radiator_width_m`, `clearance_m`) | Keep the helper. Feed it the staffage radiator's height, width and bottom. |
| 37-50 | `test_demand_emitters_and_occupants` | Demand chapters `certificate`, `recap`, `practice` (`_heater`, `_stand`), person height 0.7 x room height (49), bottom on floor (50) | Keep. `_stand`/`_heater` are local helpers. Check each after the migration. |
| 52-62 | `test_system_losses_and_regulation_stages` | `plant["radiator"]`, `plant["person"]`, regulations `_stage(with_radiator=True)` | Keep. Port the helpers. |
| 64-70 | `test_summer_occupant_follows_house_scale` | Summer `_person` (already staffage, line 93 of summer module): height 0.7 x house half-height, bottom | Keep. |
| 72-79 | `test_seated_room_occupant_has_human_proportions` | Internal-gains `_seated` (already staffage): height `height * 1.3 / 2.5` (77), bottom on `y_f` (78), chest above lap (79) | Keep the assertions. The value `1.3` should come from `CASES.physical.seated_m` (line 5), not be hard-coded. |
| 81-101 | `test_physics_scene_constructor_bounds` | Physics `person` (AST-exec, height 1.75/2.5 x room), `fe_heater` (`assert_radiator`), `radiator` with `5.45` (height 0.855, width 1.38, `CASES.physics`) | Keep. Re-check the AST lookups after the physics staffage change. |
| 103+ | `test_windows_text_native_allocations_are_bounded` | Windows text | No change. |

### 3.5 `V/🧪️tests/🐍️scenes.py` and `V/🧪️tests/🔣️scenes.json`

| Line | Test | Asserts | Change |
|---|---|---|---|
| 16-21 | `test_physics_uses_canonical_architecture` | `assertIs(getattr(physics, local), getattr(shared, canonical))` for every entry in `physics_shared` | Rewrite the mapping. Entries `_radiator: radiator` (scenes.json:13) and `_person: person_glyph` (:14) cannot match, because physics now defines its own staffage-based `_radiator` (`🧲️physics/.../🐍️.py:484`) and `_person` (:501). Remove `_build_sun`/`_moon` (:15-16) if sun/moon are removed. |
| 23-32 | `test_heating_plant_uses_one_physical_scale` | `plant["person"]` height 1.75/2.5 x scale (27), radiators height 0.65 and width 1.3 (30-31), clearance 0.12 (32) | Keep. Read the staffage height and width metadata. |
| 34-42 | `test_overview_emitters_scale_with_house` | Demand-power `rads` height (41), clearance (42) | Keep. Same change. |
| 60-66 | `test_heating_closeup_retains_real_dimensions` | `stage["occupant"]` height 1.75 (65), `stage["radiator"]` height 0.65 (66) | Keep. Same change. |
| `scenes.json:5-6` | `house.radiator_height_m` / `radiator_width_m` | Physical sizes (0.65, 1.3) | Keep. They are physical targets, not the visuals JSON. |
| `scenes.json:4` | `house.person_m` 1.75 | Physical height | Keep. |

### 3.6 `A/🧪️tests/🐍️.py` (whole file, `ActivityTest`)

| Line | Asserts | Change |
|---|---|---|
| 12 | Imports `I/🐍️` as `SCENE` | Delete with the file, or keep the module for the metabolic test. |
| 17-32 | Profile closed and opaque, same human size across scales, bottom at `case.bottom x scale`, heat point (`_activity_figure(case["kind"], s=scale)`, line 23) | Delete. The call uses the wrong kwarg (`s=`); the current signature is `unit=`, `at=` (`I/🐍️.py:164`). |
| 34-55 | Connected solid body, heat point inside the torso (`_activity_figure(..., s=scale)`, line 41) | Delete or port to staffage. |
| 57-63 | `_activity_figure("sit", s=1)` (60) equals `SCENE.seated_person_glyph(ORIGIN, scale=2.2)` (61) | Delete. `seated_person_glyph` is not defined in the scene module. |
| 64-68 | Metabolic watts `[80,100,200,800]` from the `acts` list in `Beat2_HumanFactor` (`I/🐍️.py:341`) | Keep. Move to an internal-gains test that does not depend on the SVG poses. |

### 3.7 Other test-like files

- `I/🐍️` is not a test but the test in `❄️cooling/🎬️clip/🧪️tests/🐍️.py:151` has a docstring "Compare real silhouette placement". It references no removed symbol. No change is required. Reword only when touched.
- `V/🧪️tests/🎨️semio-style/🐍️.py:46-67` and `🔣️.cases.json` `silhouettes` key: window silhouette outlines (`window_silhouette_outline`). Not affected. Do not rename.
- `🎬️clip/🪟️presentation/📐️geometry/🧪️tests/🐍️.py:167-170` and `🔣️.cases.json:282` (`silhouetteCases`): window presentation silhouettes. Not affected.

## 4. Schemas and JSON contracts

| File | Lines | Current contract | Change |
|---|---|---|---|
| `V/🧬️schema/🔣️.schema.json` | 8-9 | `required`: `standing`, `seated` | Remove both. |
| same | 20-23 | `dimensions.required`: `standing_height`, `seated_height`, `radiator_height`, `radiator_width` (plus `house_wall`, `storey_height`) | Remove the four silhouette and radiator keys. |
| same | 32, 35 | `properties.standing` and `properties.seated` (`$ref` outline) | Delete both properties and the `outline` def if unused. |
| same | 3 | Title "Architectural Energy Drawing Contract" | Keep. Still covers `section_material`. |
| `V/🔣️.json` | 4-7 | `dimensions.standing_height` 0.66, `seated_height` 0.55, `radiator_height` 0.26, `radiator_width` 0.52 | Remove the four keys. Keep `house_wall` and `storey_height`. |
| same | 11 | top-level `standing` outline | Delete. |
| same | 9763 | top-level `seated` outline | Delete. |
| same | (keys) | `version`, `section_material`, `dimensions.house_wall`, `dimensions.storey_height` | Keep. `_ARCHITECTURE` still loads this file (`V/🐍️.py:26`, `:942`, `:1055`, `:1096`). |
| `V/🫟️silhouettes/🧬️schema/🔣️.schema.json` | 3 and outlines `required` list | Silhouette library, 12 names | Delete with the folder. |
| `A/🧬️schema/🔣️.schema.json` | 3 and `poses.required` | Activity silhouettes | Delete with the folder. |
| `V/🧍️staffage/🔣️.json` | whole file | New catalog. Already references `panel-radiator` (lines 8-23), `walking-commuter` (line 21) | Target. The folder also holds `🐍️.py` and `🧪️tests/`. No schema file for this catalog exists in the folder (checked at 01:47). Add one if the catalog is to be validated. |

## 5. Clip modules that still call removed helpers

Current line numbers. Each row needs its call replaced by a `staffage` or `staffage_motion` cast from the matching `🧍️staffage` catalog entry.

| File | Lines | Symbol |
|---|---|---|
| `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` | 58, 338 | `sun_glyph` (decision, section 9) |
| same | 438 | `object_silhouette("microbe")` replace with staffage |
| `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | 43, 142 | `sun_glyph` |
| `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | 57, 107 | `sun_glyph` |
| `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py` | 39, 109 | `sun_glyph` |
| `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py` | 64, 302 | `moon_glyph` (alias and call) |
| same | 605 | `object_silhouette("laptop")` |
| same | 669 | `object_silhouette("monitor")` |
| same | 813 | `person_glyph` in `mini_person` |
| same | 816 | `object_silhouette("laptop")` |
| same | 944 | `object_silhouette("laptop")` for `laptop_screen` |
| same | 98, 161-167, 351 | `_ACTIVITY_UNIT`, `_ACTIVITY_CAST`, `_activity_figure` are staffage-based and stay. Only the test calls them incorrectly. |
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 71-72, 778, 1330, 1332 | `sun_glyph`, `moon_glyph` |
| `🌬️ventilation/🎬️clip/❓️need/🐍️.py` | 71, 399, 524 | `sun_glyph` |
| `📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` | 55, 202, 218, 656, 853 | `sun_glyph`; `object_silhouette("factory")` (202); `object_silhouette("flame")` (218) |
| `📊️demand/🎬️clip/♨️system-losses/🐍️.py` | 301 | `object_silhouette("flame")` |
| `📊️demand/🎬️clip/🧪️practice/🐍️.py` | 57-58, 142, 148, 357, 1090 | `person_glyph` (142); `radiator(` (148); `sun_glyph` (aliases and 357, 1090) |
| `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` | 68-69, 151 | `sun_glyph`, `moon_glyph` (alias) |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 69-70, 692, 921-922, 1102 | `sun_glyph`, `moon_glyph` |
| `🔥️heating/🎬️clip/🌬️convection/...` and `🧱️conduction/...` | (not searched in detail) | Imports `V`; no removed symbol found |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 50, 134 | `moon_glyph` |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 74, 626 | `sun_glyph` |
| `📊️demand/🎬️clip/🔁️recap/🐍️.py` | 51, 268, 1285 | `sun_glyph` |
| `📊️demand/🎬️clip/⚡️demand-power/🐍️.py` | 57, 732 | `sun_glyph` |
| `📊️demand/🎬️clip/⚖️regulations/🐍️.py` | 66, 392 | `sun_glyph` |
| `📊️demand/🎬️clip/⚖️regulations/🧪️tests/☀️sun/🐍️.py` | 39, 60 | `sun_glyph` in a test oracle |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 495, 498 | `_build_sun = sun_glyph`, `_moon = moon_glyph` aliases. `_person` (501) and `_radiator` (484) are already staffage. |
| `🎯️optimization/🎬️clip/🏠️room/🐍️.py` (via `_art`) | 585, 1298 | `sun_glyph` |
| `🎯️optimization/🎬️clip/⚖️index/🐍️.py` | 133 | `object_silhouette(GLYPHS[criterion])`, GLYPHS = bulb, sun, factory (line 25) |
| `🎯️optimization/🎬️clip/📉️extrema/🐍️.py` | 440 | `_art.radiator(...)`. Seen at start of audit; no longer present at 01:46 (migrated). Verify. |
| `📊️demand/🎬️clip/🏷️certificate/🐍️.py`, `🌬️ventilation/...`, `📊️demand/🎬️clip/🔁️recap/...`, `📊️demand/🎬️clip/⚡️demand-power/...`, `🌬️ventilation/🎬️clip/🌡️heat-flow/...` | (`person_glyph`, `radiator(`, `object_silhouette("laptop")` seen at start) | Gone by 01:46. Migrated mid-audit. Verify each is staffage now. |

Sun and moon call counts, for scale: `sun_glyph` is used in about 16 modules or tests, `moon_glyph` in 5. Re-run `rg -n '\bsun_glyph\b|\bmoon_glyph\b' 🎓️teaching` for the current list.

## 6. Pipeline, cache and manifest references

| File | Line | Reference | Change |
|---|---|---|---|
| `V/🎬️clip/🪟️presentation/🐍️.py` | 37 | `source_files()` includes `visuals / "🔣️.json"` and `visuals / "🫟️silhouettes/🔣️.json"` | Remove the silhouette JSON. Keep the visuals JSON only if it still exists. This is render provenance, so the change invalidates cache keys (expected). |
| `🎯️optimization/🎬️clip/🎨️visuals/🐍️.py` | 38-39 | `source_files()` includes `_art.__file__.with_name("🔣️.json")` and `parent / "🫟️silhouettes/🔣️.json"` | Remove the silhouette entry at 39. |
| `V/🎬️clip/🧪️tests/🔣️.cases.json` | 507 | `shared_render_dependency.profile = "...visuals/🔣️.json"` | Keep if the visuals JSON stays. Otherwise repoint. |
| `V/🎬️clip/🧪️tests/🐍️.py` | 244, 285, 642 | Lists `🎨️visuals/🐍️.py` as a common render input | No change. |
| `🎓️teaching/.../⚡️energy/🎬️clip/🔣️.json` | 260-399 | Clip manifest; scripts such as `...internal-gains/🐍️.py`, scene `Beat2_HumanFactor` (388) | No change unless scene names change. |
| `🎯️optimization/...` `🔣️taxonomy` (`🧰️framework/.../📚️library/🔣️taxonomy.json:8916`) | chapter slug `teaching-clip-chapter-internal-gains` | Chapter name, not the silhouette | No change. |
| `V/🏗️authoring/📜️script.ts` | 18 | Test command (unittest module paths, presentation browser closure test) | No silhouette path found in this line. Keep. Re-check after deleting `V/🧪️tests/🐍️silhouettes.py` or `A/🧪️tests/🐍️.py` in case a runner lists them. |

## 7. Documentation and attribution

- `V/🫟️silhouettes/README.md:1-5`: attribution for CC0 Wikimedia silhouettes. If any new handcrafted staffage is derived from those traces, keep the credit (move it to the staffage catalog README). Otherwise delete with the folder.
- `🎓️teaching/🏛️architecture/🔀️morph/README.md:3, 5, 11` mention `silhouette` as a morph helper (`🔀️morph/🐍️.py:96`). Unrelated. No change.
- `🎯️optimization/🎬️clip/README.md:23, 72` link `🎨️visuals/🐍️.py`. No removed name. No change.
- Prompts and notes: `staffage-launch-kit/prompt.txt:44, 139` and `followup-prompt.txt:119` ask for "recognizable silhouettes" (generic). `.🧬semio/.../💬️prompts/🐙️ueli.md:3067` is a UI silhouette remark. `✍️notes/semio_brep_kernel_audit_7ad363f.md:1048` is BREP. No change.
- `🧍️staffage/🧬️schema/🔣️.json:1148-1158` and `🧬️schema/🟦️.ts:192, 252` define a `silhouette` text role in the line-hierarchy schema. That is a different concept. Do not remove.

## 8. Pre-existing stale references (broken before this change, found by static reading only; no tests were run)

1. `V/🧪️tests/🐍️silhouettes.py:97-106`: `importlib.import_module(...internal-gains...)` then `scene._ACTIVITY["poses"]`, `scene._ACTIVITY_ROOT` and `scene._activity_figure(kind)` (one positional argument). The scene module defines `_ACTIVITY_UNIT` (98), `_ACTIVITY_CAST` (161) and `_activity_figure(kind, *, unit, at)` (164). `_ACTIVITY` and `_ACTIVITY_ROOT` do not exist.
2. `A/🧪️tests/🐍️.py:23, 41, 60`: `_activity_figure(..., s=scale)`. Wrong keyword (`unit`/`at` only). Line 61 `SCENE.seated_person_glyph` does not exist in the scene module.
3. `V/🧪️tests/🔣️scenes.json:13-14` plus `V/🧪️tests/🐍️scenes.py:16-21`: `physics._radiator` and `physics._person` are staffage-based defs (physics 484, 501). The identity assertion `assertIs(... shared.radiator / shared.person_glyph)` cannot pass.

## 9. Decisions needed

1. `sun_glyph` and `moon_glyph` (V/🐍️.py:1165-1176) are built on `object_silhouette`. They are not in the request, but deleting `object_silhouette` breaks about 16 modules and 2 tests (section 5). Choose: (a) replace them with handcrafted staffage sun and moon casts, or (b) keep them as separate handcrafted vector helpers. Do not keep a hidden dependency on the deleted library.
2. `object_silhouette` device kinds used elsewhere: laptop, monitor, vacuum, bulb, heater, toaster (watt comparison and `_WATT_DEVICES`), factory, flame, tree, microbe, and bulb/sun/factory in `⚖️index`. Each needs a staffage replacement or removal.
3. Attribution (section 7): keep or drop the CC0 credits.
4. Physics identity test (section 8.3): rename the scene mapping to the staffage helpers instead of the visuals functions.
5. Whether `V/🔣️.json` keeps `house_wall` and `storey_height` as the only dimensions, or moves them to the staffage catalog.

## 10. Checked, no reference found

- Nx and project files (`nx.json`, `package.json`, `pyproject.toml`, `📋️project.json`, `📜️script.ts` at root, `🔒️dependencies.json`, `🎓️teaching/**/package.json`, `tsconfig.json`): none name the silhouette files, activity folder or removed helpers.
- Packaging lists, snapshot files and schema catalogs (`🧰️framework/.../📚️library/🔣️schema-catalog.json`, `🗂️schema-document-index.json`): no entry for `V/🧬️schema`, `V/🫟️silhouettes` or `A/🧬️schema`.
- Quiz JSON (`🧲️physics/❓️quiz/🔣️.json`, `🔥️heating/❓️quiz/🔣️.json`): "radiator" is quiz wording only.
- `V/🧪️tests/🔣️integration.json:96-99`: "radiator" in German and English text only. No change.
- `🎓️teaching/🏛️architecture/📊️excel` and `🏢️nakagin` define their own visuals modules. No import of the energy `V` module that uses removed names. Not affected (except the `object_silhouette` in `⚖️index`, which imports `_art`, i.e. `V`).
- `🏗️bim`: its visuals are separate. No removed names.

## 11. Ordered change list (for the implementer)

1. Add the staffage replacements and their tests first (standing, seated, radiator, devices, sun/moon if kept, activity poses). Use the existing staffage helpers and the `🧍️staffage/🔣️.json` catalog.
2. Migrate the clip callers in section 5 (re-run the grep first; many are already migrated).
3. Delete `V/🐍️.py` definitions at 28, 387 (replace), 1140-1217 (and `_filled_outline` if unused).
4. Edit `V/🔣️.json` and `V/🧬️schema/🔣️.schema.json` (sections 4).
5. Delete `V/🫟️silhouettes/` and `A/` after the replacements exist.
6. Edit `V/🧪️tests/🐍️.py`, `🐍️silhouettes.py`, `🐍️integration.py`, `🐍️scenes.py`, `🔣️scenes.json`, `🔣️.cases.json`, `🔣️silhouettes.json`, `🔣️integration.json` (section 3). Keep the window silhouette tests.
7. Edit the two `source_files()` lists (section 6) and check `V/🎬️clip/🧪️tests/🔣️.cases.json:507`.
8. Re-run the grep for every removed symbol and path (section 0) and confirm zero hits outside this ticket.
