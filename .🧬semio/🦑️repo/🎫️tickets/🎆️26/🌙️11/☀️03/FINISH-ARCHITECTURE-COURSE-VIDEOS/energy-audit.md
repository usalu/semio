# Energy Clip Audit

- Ventilation room occupants now use the clear-storey metre rather than normalizing the visible bounds of an animated person. Existing architecture geometry and a language-neutral metre fixture pass against Manim and NumPy.
- Cooling house occupants and heating calculation furnishings now share the architectural storey metre. Demand room people and the recap pendant use the same room metre as surrounding furnishings.
- Removed person enlargement animation in ventilation room sequences.
- Caption selection and reading budgets now use the explicit display language; the language-neutral English/German fixture passes against NumPy. VO timing selection remains independent.
- Verified the ventilation architecture suite (6 tests) with the teaching Python environment and the caption fixture with system Python. Full video renders and visual review remain with the coordinator.
- Additional tests: cooling animation suite 4/4 with explicit English and ticket output root; demand primary-energy scale suite 5/5.

- Corrected passive-room MVHR and house cooling AHU to their architectural metre; both validated by seven passing ventilation architecture fixture tests. Scene sources changed while the batch was running; verify current fingerprints before delivery.
- Inspected six frames of corrected ventilation Beat2: English captions and Dwelling label render; passive-house chart label remains within frame. Source scene was re-rendered after correction. Passive room MVHR uses the room metre.

## Changed and Created Files

- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🧪️architecture/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🧪️architecture/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚡️demand-power/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🌍️captions/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🌍️captions/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media/course_preview.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/energy-audit.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/preview-audit.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/preview-cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/preview-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/inspect-energy.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/inspect-ahu.py`

## Cooling AHU Semantics

The previous scene used a split AC indoor appliance for a labelled ventilation unit. It now combines existing elevation casts of a 0.6 m wall-mounted heat-recovery ventilator and a 0.52 m finned cooling coil, each at the architectural room metre. Supply and extract paths target the actual coil and fan centers. Static Manim review checked the assembly; a rendered-scene review remains after the cooling batch. No shared cast manifest was edited.

- Controlled publisher restart after tested staging scanner fix: terminated only workers 67316 and 14248; preserved all source clips and ongoing source queues, relaunched fail-closed concat-only physical lesson and volume chapter.

- Created ticket input script `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/watch-browser-logs.cjs`. The physical, volume and energy-balance publishers failed at the browser silhouette settle assertion. Full captured diagnostic was relayed to the browser owner; no source clips were discarded.

- Inspected six frames of rendered cooling Beat3: unit and coil remain at room scale, airflow reaches both components, labels fit, and captions are English.

- Cooling sizing s84 failed in Manim/Pango on a raw equation caption. The original caption also rendered successfully in isolated default and 854×480 probes, so no individual unsupported glyph was established. Reworded the clause as natural English, retained the diagram formula, and added a passing real caption-bar regression. Affected source and tests: 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py and its 🧪️tests/🐍️.py and 🔣️.cases.json.

- Writer exclusion investigation: the current per-source write boundary is inside runtime/🐍️.py, which segment_state explicitly fingerprints. Repo tool admission controls machine capacity rather than keyed source writers. A correct lock requires changing the runtime and invalidating existing source certificates; no compatibility/freshness bypass was introduced. This run uses disjoint producer sets and explicit completion barriers.

- Source audit found heating s43 failed because its English comparison exceeded the two-line caption limit. Reworded without changing the three comparison values; a real caption-bar test passes and the shared conduction script’s chapter is rerendering. Changed files: 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py and its 🧪️tests/🐍️.py and 🔣️.cases.json; registered in the canonical authoring tests.

- Media helper freshness now checks canonical current presentation_state and all saved input hashes in addition to scene certificates. Portable changed-frontend fixture fails before and passes after, using NumPy. Changed/created files: .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media/lib.py and this ticket’s media-freshness-test.py and media-freshness-cases.json.

- Created ticket input script verify-energy.py to verify each energy master using the strengthened canonical frontend/content/cursor/source freshness gate and compute SHA-256 digests.

## Source Midpoint Review

Reviewed source-review sheets 01–18 (ordinals 1–108, including repeated excerpts): the sheets cover current-plan source ordinals through 108; the sampled corrected cooling AHU and English caption are visible, and the rewritten heating comparison was separately reviewed. Several concrete remaining English-localization defects appear in diagram labels: `außen` remains in heating fundamentals ordinals 31–35 and 39, heating solar ordinals 54 and 56; `Boden` appears in demand ordinal 89. These are visible diagram labels, not the translated captions. Potential layout collisions at midpoint include stacked small readouts in ventilation heat-capacity ordinal 15 and heat-bridge dimension labels at heating ordinal 45; check the full-time sequence before deciding whether a transition is intentional. Source-review contact sheets sample one frame and do not prove temporal continuity or complete scene coverage.

- Temporal inspection of s15 showed the readout collision only during a TransformFromCopy animation, while the later held frames are clear; no timing change made. In s45 the dimension labels persistently overlapped their numeric column, so English labels were shortened to Outside/Centreline/Inside; authored en/de label bounds pass Pango tests. Localized outside labels in heating fundamentals, heating solar, and ventilation heat-flow, and Floor in the demand recap. Updated/created files: 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py plus its 🧪️tests, 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py, 📊️demand/🎬️clip/🔁️recap/🐍️.py, and this ticket’s label-test.py and label-cases.json. Representative diagrams require fresh rerenders after these source edits. The source-review sheets 01–18 are current energy plan sources; older orphan ordinal files elsewhere are excluded from the current plan.

- Localized the practice comparison label calculated/berechnet in 📊️demand/🎬️clip/🧪️practice/🐍️.py. The isolated s111–113 source rerender and chapter assembly completed; publication stopped because presentation inputs changed while rendering. The heating correction source chapters likewise completed, while its presentation publisher failed during ongoing browser changes. Final energy masters remain subject to full current frontend and source freshness checks after the browser fix is stable.

## Complete Owned File Inventory

Updated existing files:

- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚡️demand-power/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🧪️architecture/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🧪️architecture/🔣️.cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media/course_preview.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media/lib.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🎨️finishes/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🎨️finishes/🛝️ramps.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🎨️finishes/🧱️layers/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🏗️facade/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🏗️facade/🔗️junctions.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🏗️structure/🔗️joints/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/➖️difference/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/📐️frames/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔧️services/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️profiles/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧮️analysis/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧮️analysis/📏️distance/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🪟️curtain/🐍️.py`

Created files:

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🌍️captions/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🌍️captions/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/♻️cache/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/♻️cache/🔣️.schema.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/♻️cache/🔣️.manifest.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/♻️cache/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/♻️cache/🧪️tests/🔣️.cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/energy-audit.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/preview-audit.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/preview-cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/preview-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/inspect-energy.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/inspect-ahu.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/watch-browser-logs.cjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/media-freshness-cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/media-freshness-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/verify-energy.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/label-cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/label-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/slab-cache.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/slab-cache.schema.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/slab-cache-cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/slab-cache-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/slab-native-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/slab-cache-design.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/bim-cache-dependencies.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/bim-globals-inventory.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/bim-globals-values.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/bim-classes-inventory.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/module-identity-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/production-cache-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/production-context-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/production-slab-test.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/cache-context-check.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/cache-source-audit.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/cache-env-probe.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/full-slab-validation.py`

Removed tracked source files: none. Only isolated generated test records and corrupt Manim partials for sources 83 and 84 were removed during controlled recovery. Other agents maintain separate inventories for their own files.

## Additional capsule, interop and BIM surface inventory

Updated:
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🐍️.py`
- This ticket's existing `test-capsule-stage.py`.

Created:
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🧪️tests/👻️ghost.py`
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🧪️tests/🎚️ui.py`
- `🎓️teaching/🧰️interop/🎬️scene/🐍️.py`
- `🎓️teaching/🧰️interop/🎬️scene/🧪️tests/🐍️.py`
- `🎓️teaching/🧰️interop/🎬️scene/🧪️tests/🔣️.schema.json`
- `🎓️teaching/🧰️interop/🎬️scene/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🔣️tap.schema.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🔣️tap.json`
- Ticket reports `capsule-stage-review.md` and `bim-boundary-audit.md`.
- Ticket inputs `capsule-stage-cases.json`, `capsule-preview-test.py`, `capsule-ghost-test.py`, `capsule-phase-smoke.py`, `capsule-lifecycle.py`, `capsule-hook-check.py`, `core-preview-pose.py`, `measure-capsule-preview.py`, `measure-capsule-later.py`, `measure-capsule-sprite.py`, `measure-core-preview.py`, `family-fingerprint.py`, `live_family_prototype.py`, `family-boundary-test.py`, `capsule-ui-family-test.py`, `capsule-ghost-family-test.py`, `capsule-family-morph-test.py`, `capsule-family-wait-test.py`, `capsule-ui-family-fade-test.py`, `capsule-ui-ghost-test.py`, `bim-boundary-capture.py`, `bim-boundary-reduce.py`, `bim-boundary-minimize.py` and `bim-boundary-verify.py`.

No tracked files removed. The coordinator registered permanent test modules in the already-listed authoring script; the drawing owner created the separate keyed-image helper. The final bilingual capsule run remains incomplete after controlled cancellation under critical memory pressure.

Additional created ticket report: capsule-lifecycle-equivalence.md.

Additional created read-only investigation: excel-anatomy-geometry-audit.md.

Additional BIM 114 projected-boundary files, fixtures and ticket input inventory are recorded in bim-boundary-audit.md.
