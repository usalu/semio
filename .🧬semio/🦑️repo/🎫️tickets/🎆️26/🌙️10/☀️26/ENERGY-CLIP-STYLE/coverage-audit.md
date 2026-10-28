# Energy Style Coverage Audit

Read-only audit of authored local chrome while shared theme, typography, glyph, and scene work proceeds. Line references describe the source observed during implementation and may move. Findings were sent to root and clip_style.

## Highest Priority Coverage

- `🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py`: local background and palette at lines 31–35, explicit camera background at 197, and literal font sizes 42/19/31/20/18/16/36 at 213–237. This is separate from ordinary scene title helpers and must inherit the common theme explicitly.
- `🔥️heating/🎬️clip/🧮️calculation/🐍️.py`: `_heading` at 170 defaults size 34 and edge buffer 0.6; `_label` at 175 defaults 16. Standalone titles at 655 and 823 use 34 and 28, respectively. Formula rows also choose literal 26–38 sizes. Shared font setup covers face selection, but consistent heading spacing needs these helpers to delegate or use common tokens.
- Demand series contains independent vivid semantic palette constants: system-losses 60–66; regulations 79–82; primary-energy 73–78; certificate 66–70; recap 85–90; practice 88–94 and 924. These must be mapped centrally or changed to semio theme roles. Preserve pedagogical semantic distinctions while adopting theme colors.

## Local Chip And Chrome Duplication

- Cooling summer-strategy `_back_chip` 113–119, transmission-humidity 187–193, and system-sizing 157–163 construct independent rounded boxes with 0.1 padding and 1.8 stroke; summer and transmission also declare AIR #60A5FA.
- Ventilation mechanical `_math_chip` 166–169, heat-flow `_handover_chip` 249–253, volume `_handover_chip` 290–293, and handoff around 142–145 duplicate chip chrome. Heating convection chip helpers 110–121 use 0.09/0.1 padding and 1.6 stroke. Shared chip currently uses another padding/stroke convention, so delegating these prevents further spacing drift.
- Cooling solar-radiation has dark panel fills #161A21 at 647/651. Summer-strategy panel fill #12151C at 447; internal-gains card/control fills #12151C around 678/850/1004.
- Ventilation mechanical comparison cards use dark fills #1a1215/#101825/#102018 around 763/777/798; MVHR equipment fill #2C3E50 at 779. Differentiate illustrative equipment materials from interface panels when applying shared surface colors.

## Verification Boundaries

The manifest covers physical fundamentals, ventilation, heating, cooling, and energy balance. Include intro, a demand scene, heating calculation, and a cooling local backlink chip in representative visual checks; checking shared helpers alone misses these authored paths. Existing shared runtime source fingerprints include fonts and visuals, so updated shared style invalidates segment reuse. No people-glyph audit was repeated because clip_style owns that work. No source was edited by this audit.
