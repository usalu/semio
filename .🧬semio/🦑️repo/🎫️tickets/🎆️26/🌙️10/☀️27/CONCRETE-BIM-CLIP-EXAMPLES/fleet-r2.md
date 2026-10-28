# Fleet Round 2 (Session 28198dbf, 2026-10-27 18:15)

Goal: finish all twelve new clips (six BIM, six optimization) end to end at the energy-clip bar, then render and deliver en + de, speaker + spectator.

## State At Start

- BIM: real Villa Savoye drawings exist; scenes have 10–11 beats each (optimization: 27–40). `👁️representations` crashes (`_level` gets a tuple from the model), `🔌️exchange` crashes (Manim Transform family mismatch). README still describes the old "one south wall" story.
- Optimization: all six scenes capture in en + de; open issues are numeric presentation (signs, chained ratios), label/line overlaps, clutter.
- Tests: logic passes; failures only from `SEMIO_TEACHING_OUTPUT_ROOT` missing outside `bun …/📜️script.ts test`.
- Rendering runs as `bun 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render …` (no `repo.exe`); a full render aborts when sources change mid-render, so the final render waits until every scene agent is done.

## Waves

| Wave | Model | Work | Output |
| --- | --- | --- | --- |
| 1 audit | Haiku 5.5 | one auditor per clip (BIM 📊️ 🧬️ 🧩️ 🔗️, optimization all six) | `audit-r2-<topic>.md` in each ticket |
| 1 fix | Sonnet 5.5 | BIM 👁️representations, 🔌️exchange crash + depth | `scene-<topic>.md` Round 2 |
| 2 build | Sonnet 5.5 | one owner per remaining clip implementing its audit | `scene-<topic>.md` Round 2 |
| 3 verify | Haiku 5.5 | fresh frame audits en + de | `verify-r2-<topic>.md` |
| 4 docs | Sonnet 5.5 | BIM README + transcripts for Villa Savoye, optimization transcripts refresh | README, `🗣️transcript.*.md` |
| 5 render | coordinator | full render en/de × speaker/spectator, delivery | `render-r2.md` |

## Pending Follow-Ups

- Optimization: promote the extrema `_sub` pattern into `ClipScene.section(en, de)` (per-beat `section_heading`, energy convention; the persistent title chip was removed on purpose by ENERGY-CLIP-STYLE, see optimization `title-convention.md`); refresh stale `energy-idioms.md` notes.
- BIM model data: align `🧱️model/🔣️.json` basis flags with the scenes and research (LoadBearing "inferred", material "assumed", levels/layers "assumed").
- BIM README and transcripts: rewrite for Villa Savoye (README still says "one south wall").

## Coordinator Decisions

- **Piloti count (research 2.11, conflict 15 vs 25):** the teaching model draws 25 pilotis on the 5 × 5, 4.75 m grid. Any count on screen or in narration reads "25 in this teaching model" with the assumption star and the legend "sources differ: 15–25". Never present 25 as a historical fact; never hide that the drawing has 25.
- **Piloti diameter and height, slab thickness, levels, window sizes (research 2.12, 14):** teaching-model assumptions, always starred.
- **Per-occurrence vs per-type values:** tributary area, reaction and placement are occurrence values; diameter, material, LoadBearing are type values. Psets sit where the STEP excerpt and the type beat put them — one consistent binding across 🧬️objects and 🔌️exchange.
- **Derivation vs apportioned parts (optimization):** a part that is derived on screen (÷ COP, ÷ EER, kW · h · share) prints its own correctly rounded result; where it also appears in a stacked sum, show parts and total with one decimal so both the derivation and the sum hold exactly as printed (largest remainder only on the last digit shown). One precision per quantity per frame.
- **Hanging garden vs solarium (research 2.7–2.9, 3.5, 6.1):** the jardin suspendu (`SP-TERRACE`, about 110 m²) is the open-air terrace on the **first floor** (Le Corbusier's "garden 3.5 m above the earth"); the **roof storey** carries the solarium (about 70 m²) with its curved screen walls and planting. Never move the 110 m² garden to the roof. The 🔗️consistency build-up change sits on the roof slab SL-R and uses the model's roof planting area, named "roof planting (solarium level)", not the 110 m² garden.
- **Titles:** no persistent title chip (ENERGY-CLIP-STYLE decision); per-beat `section_heading` only.

## Shared-File Rule

Scene agents own only their scene file. Additive helpers in shared model/toolkit files are allowed only with the Edit tool (never whole-file Write), never renaming or removing existing names, and with a matching test/case.
