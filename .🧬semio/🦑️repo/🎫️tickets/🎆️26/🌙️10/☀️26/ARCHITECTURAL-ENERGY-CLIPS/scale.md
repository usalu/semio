# Physical Scale Integration

Demand, cooling, and physics scene sources now size occupants and room emitters from their surrounding room/storey geometry: a 2.5 m reference clear height, 1.75 m standing adult, 1.3 m seated adult, 0.65 m by 1.3 m panel radiator, and 0.12 m radiator floor clearance. Standalone conceptual heat-pump radiator dimensions are explicitly 0.855 by 1.38 drawing units so the explanatory diagram retains readable scale.

## Implementation

- Demand system losses uses room-derived panel geometry, and both flow endpoints follow actual panel bounds.
- Demand regulations uses the house storey for the occupant and panel. Certificate, recap, and practice have tested local emitter helpers; all three place people exactly on floor anchors. Practice hydraulic supply/return connections follow panel edges rather than assumed offsets.
- Cooling summer strategy uses the house storey for its adult. Heating comparison removes the occupant floor gap. Internal gains uses physical seated dimensions and semantic lap anchors, replacing invalid references to numbered limbs after the shared silhouette change.
- Physics uses room-derived adult height and floor placement, a house-derived final-energy panel, explicit conceptual panel dimensions, and head/mouth coordinates relative to the filled standing silhouette.
- Authored wall group usages were checked in owned chapters. Existing left-group, right-wall and deck grouping remains appropriate in those owned usages.

## Test Contract

`🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json` supplies language-neutral physical dimensions and scale cases. `🐍️integration.py` independently compares actual third-party Manim bounds against the contract at house scales 0.52, 0.8, and 1.05 and room heights 2.25 and 3.4. The authoring test command includes this module without removing any existing suite module.

The first run failed before implementation on the expected panel size mismatch, missing per-scene helpers and the previous summer helper interface. The subsequent initial suite passed all five cases in 4.905 seconds. An additional Physics scene-constructor case was then added. The final suite passed all six tests in 5.341 seconds using the managed Python 3.12 environment through `repo tools exec uv`. Standard third-party pydub warnings were non-fatal.

## Files

- `📊️demand/🎬️clip/♨️system-losses/🐍️.py`
- `📊️demand/🎬️clip/⚖️regulations/🐍️.py`
- `📊️demand/🎬️clip/🏷️certificate/🐍️.py`
- `📊️demand/🎬️clip/🔁️recap/🐍️.py`
- `📊️demand/🎬️clip/🧪️practice/🐍️.py`
- `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`
- `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py`
- `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py`
- `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`
- `🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py`
- `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json`
- `🏗️authoring/📜️script.ts`

All listed source/test paths are relative to `🎓️teaching/🏛️architecture/⚡️energy`. No numerical scientific subject values were changed.

## Complete Scale-Agent Source Footprint

- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`

## Caption Count Reconciliation

The portable contract contains **39 canonical scene/key cases and 44 previous-language strings**; five keys have both EN and DE previous strings. This is **43 overflows from the all-source caption sweep plus one earlier Physics comparison failure**. It is not a claim that the sweep found 44 failures.

The retained records establish the partition: `caption-repairs.json` contains 42 unique language-specific sweep repairs; the separately registered ventilation `Beat3_LatenteLast` / `dx` / EN case is the 43rd. The remaining previous string is Physics `Beat4_Waermepumpe` / `compare2` / DE, whose earlier three-line failure is still recorded in `🗑️generated/smoke/results.json`. Every retained repair's previous text matches its portable case. The original caption oracle has validated all 44 previous strings across the 39 keys.

The later **Opaque envelope** qualifier restoration changes one English narration datum and its matching portable case/repair input. It adds no new overflow occurrence or canonical key. The targeted original caption oracle check establishes the restored wording still fits.
