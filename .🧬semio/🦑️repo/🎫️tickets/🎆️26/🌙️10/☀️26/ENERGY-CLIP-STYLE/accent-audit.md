# Energy Clip Accent Hierarchy Audit

Read-only source audit on 2026-10-26. Line references describe the files before the parallel style edits.

## Canonical Contract

The canonical styling JSON assigns primary #ff344f, secondary #34d1bf, tertiary #fa9500, warning #fccf05, success #7eb77f and light #f7f3e3. Both appearances assign primary to selected board nodes, wires and handles; secondary represents selection exit, computing and some indirect handles. The palette does not itself require numerical primary dominance; the user explicitly requires that visual hierarchy for these clips.

## Shared Defaults Causing Teal Dominance

All paths below are relative to 🎓️teaching/🏛️architecture/⚡️energy.

- 🏗️authoring/🎨️visuals/🐍️.py:143 window_frame uses P_TEAL by default. This repeats teal on every generic box.
- The same file defaults window_chip/chip (762/771), note_line (1346), window_card/stat_card/card_grid (1369/1392/1409), formula_panel/math_panel (1617/1999) to P_TEAL. These generic teaching affordances should default to a named primary accent.
- Generic meter (700 P_CYAN) and clock_glyph (1241 P_CYAN) also inherit secondary. These should use primary unless callers explicitly pass scientific category colors.
- 🏗️authoring/🔤️fonts/🐍️.py:388-390 explicitly colors the subtitle P_TEAL, creating a persistent teal accent on title scenes.
- 🎬️clip/👋️intro/🐍️.py:44 defines P_TEAL directly from secondary; frame (151), divider (158-159), heading (206) and title frame (221) explicitly bypass shared defaults.

## Scene Overrides Worth Correcting

The root scene editor should preserve category colors while switching generic presentation accents. Broad replacement of every teal or cyan would erase scientific distinctions.

- 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:99 local _panel default, 720-731 meter label/frame/readout, 1352 note, 3319 closing frame and 3340 links remain teal even after shared default changes.
- 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:144 local _badge default, 1068 generic formula border, 1538 next-chapter navigation chip.
- 📊️demand/🎬️clip/⚖️regulations/🐍️.py:570 law card, 600 objective note, 632 revision note and 937 source card are generic secondary accents.
- 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:820 and 966 explanatory notes; 1214 reference chip.
- 📊️demand/🎬️clip/🏷️certificate/🐍️.py:550 schematic note, 771-774 mandatory-information card, 870-871 generic chips.
- ❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:533-553 and 594-606 decision chips, links and indication accents.
- Many scenes explicitly color source footnotes and unit labels teal. Source footnotes should be muted canonical foreground/gray rather than another saturated accent; unit labels can follow their associated quantity when semantic consistency is useful.

## Scientific Secondary Uses To Preserve

P_CYAN, P_BLUE, P_TEAL and all their PASTEL aliases currently resolve to the same secondary token (visuals lines 38-53). Explicit cold water, latent moisture, supply/exhaust air and cold-to-hot gradients depend on these colors. Preserve these role assignments in humid_air (278), person latent output (334-339), airflow cold gradient (518), window glass (969 and 1103), droplets (1316) and explicit scene thermal comparisons. Window glass may be pale/muted; it is architectural material rather than generic UI chrome.

Examples where teal is semantic rather than generic: mechanical ventilation exhaust-air ports and recovery denominator; summer-strategy exhaust-air paths; system-sizing volumetric flow/velocity comparisons; physics heat-pump valve gradient. Distinct data series also need coherent colors rather than automatic primary replacement.

## Suggested Verification

Add language-neutral contract fixtures for canonical primary/secondary identity and generic default roles. Runtime checks should call helpers without an explicit color and verify primary strokes/readout accents while exercising explicit secondary overrides. Sample one intro, one boxed equation and one cold/hot process frame; source token counts alone cannot prove screen-area or duration hierarchy.
