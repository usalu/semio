# Standalone Instructional Headings

Persistent video-name chips are removed from the authored bodies. The actual chapter name remains in the presentation window chrome. Instructional content now uses a real standalone section_heading(text) chip positioned directly in the first top-left content row. The obsolete scene_title, play_scene_title and beat_subtitle APIs, anchor variables, persistent _title state and lifecycle keep entries are removed, together with unused title timing exports and the removed entrance animation budget.

Calculation headings and the main heating equation remain instructional content. BIM nested heading anchors are replaced by direct standalone headings. Equipment labels, axes, diagram titles, standards notes, captions and narration remain. The native note-band explanation and existing typography fixture now describe the standalone contract.

## Test Evidence

Before editing, actual ventilation header execution exposed both “Warum lüften?” and “Was Lüftung leisten muss”; the persistent title violated the intended body. Independent native geometry tests also failed because the standalone API was absent. RED: 2 tests in 27.062 seconds, one real title failure plus missing-API English/German subcases.

After migration, five tests passed in 22.042 seconds: all 38 canonical modules import and contain no obsolete header API or state, actual authored native header retains instruction and omits the video title, independent NumPy/Manim geometry matches exact standalone positioning, and updated style fixtures preserve the primary instructional chip. The five bounded readout-template regressions passed in 8.588 seconds; template cache logic was untouched.

The read-only visual audit passed English/German heading rasters and the actual 480×270 source header, including its right-hand DIN note. Evidence: generated/ih/en-heading.png, de-heading.png and actual-source-header.png; review: inspection-body-title-removal-review.md.

A Windows mapped-file restriction interrupted truncating the shortened solar-gains source, leaving a duplicate three-line tail. The exact duplicate tail was removed through a guarded atomic edit; all scoped sources parse. No source backup or migration script was retained.

## Provenance And Rendering

Shared font changes intentionally stale every one of the 117 canonical source certificates. The production guards must rerender them; no certificate was rewritten. The existing publisher also needs a fresh Python process after wrapper changes, since disk fingerprints alone cannot prove that an already imported wrapper executes current code. Root owns the exact process cancellation and restart. No duplicate full source render was started here.

## Changed Paths

- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚡️demand-power/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🌬️convection/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/👥️internal-gains/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py
- 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🐍️.py
- 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py
- 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧬️algorithms/🐍️.py
- 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py
- 🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️experiments/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🧪️tests/📑️heading/🔣️.json
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🧪️tests/📑️heading/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🔣️.cases.json
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts

## Current Shared Hashes

- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py: b1469a74596c2f37002464d11a6c3a0c7a9ecbd3a51859302b5e953a435a69d7
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py: f774231bd4f4ed46cb915d28e8f754bd86f7d8832166ef4587fc7b56a0f15d1e
