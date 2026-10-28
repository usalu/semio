# Source Reuse Audit and Validation

## Implemented

The manifest now has 118 ordered occurrences and 110 canonical script/scene sources. One source render serves all complete and partial occurrences. Every repeated selection is cut once, including forced runs.

- Heating solar selects physics thermal mass `store` through `room`, then `winter`.
- Cooling transmission selects physics thermal mass `day` through `lag`, then `summer`.
- Energy-balance system losses selects the earlier heating calculation's `AnlagenVerluste`, `chain` through `sum`.
- Cooling solar selects the earlier heating opening/glass share (`a`–`ff`), external Raffstore (`unshaded`–`reduced`) and glass energy split (`split`–`gval`). The final cooling example uses the same 1.92 m² opening, 0.70 glass share and 0.60 glazing transmittance, with summer 800 W/m² and 0.15 external shading, producing 96.768 W.
- The seven obsolete independent scene definitions are removed with their local speech references and stale timing/trace entries.
- All five lesson wrappers and the four affected chapter composition tools delegate to the same manifest runtime. Chapter identifiers select chapters through `--chapter`; no independent mega-scene or playlist reconstructs whole source beats behind a partial selection.

Narration marks are copied into private source provenance bound to the exact source fingerprint and quality. Selection rejects missing, stale, unknown, reversed or overlapping boundaries. FFmpeg re-encodes the exact video interval and optional audio on the same clock, then publishes atomically. Complete source renders remain reusable after interruption.

## Validation

Language-neutral JSON cases were added before implementation. The first semantic test failed because the new range interface did not exist; the obsolete-source test failed while duplicate definitions remained. Both passed after implementation.

The managed Python 3.12 environment ran all 30 clip contract tests successfully; one existing Windows symbolic-link test skipped. Real Manim and FFmpeg integration verified:

- Source discovery for all 110 unique declared sources.
- A repeated source rendered once and two selected ranges cut once each.
- Fifteen decoded frames appeared in the independently expected red/blue/green/red/blue sequence, including repeated ranges.
- A second chapter-only concat run performed no rendering or cutting.
- FFmpeg excerpt output matched an independently decoded PyAV reference, and its 48 kHz audio duration matched the selected interval.
- A real source render consumed three existing clause WAV recordings (440/880/1320 Hz), attached them on its measured clock before source publication, and preserved the expected reordered frequencies through repeated excerpts and chapter concatenation.
- NumPy independently validated the shared solar-window multiplication.
- Imported source changes, shared visual-profile changes and recorded-audio changes invalidated cached clips. Incomplete clause recording sets failed explicitly. Canonical framework theme tokens, font catalog and font binaries remain in the fingerprint.
- Existing whole-source rendering, source-change invalidation, failure isolation, cancellation and output-order tests passed.

## Wider Content Audit

| Topic | Earlier source | Later appearance | Assessment |
|---|---|---|---|
| Thermal mass | Physics `Beat6_ThermischeMasse` | Heating storage and cooling time lag | Canonical excerpts implemented. Active-depth example stays consistent: 20 m² × 0.10 m × 2,400 kg/m³ = 4,800 kg, approximately 2.7 kWh at 2 K. |
| System loss chain | Heating `AnlagenVerluste` | Energy balance system-loss opening | Earlier-source excerpt implemented; later detailed equipment and energy-factor analysis remains distinct. |
| Ventilation | Ventilation heat-flow/recovery | Heating winter bridge and cooling summer bridge | Distinct seasonal balances and totals. The later scenes explicitly recall the earlier explanation, then apply different numerical boundary conditions. |
| Cooling airflow sizing | Ventilation hygienic duct sizing | Cooling system sizing | Distinct requirement: size air supply against cooling load and temperature lift, then compare with hygiene airflow. Keep. |
| People | Heating `Beat2_PersonenPhiP` | Cooling `Beat2_HumanFactor` | Heating uses approximately 80 W sensible heat; cooling adds total heat, latent/sensible division and activity. Keep the distinct treatment. |
| Equipment and lighting | Heating sources | Cooling usage factors and mitigation | Shared origin but distinct simultaneous-use and reduction calculations. Keep quantitative applications; later introductory sentences can use existing source excerpts. |
| Window frame factor | Heating solar `Beat2_BestrahlungUndFlaeche` | Cooling solar `Beat2_FrameFactor` | Implemented: earlier `a`–`ff` sequence; cooling final equation uses the same canonical glass-share notation and value. |
| External shading | Heating solar `Beat5_Verschattung` | Cooling solar `Beat3_ShadingFactor` | Implemented: earlier `unshaded`–`reduced` sequence; cooling applies the canonical 0.15 external-blind factor. |
| Glass energy split | Heating solar `Beat3_GWert` | Cooling solar `Beat4_GlassTransmittance` | Implemented: earlier `split`–`gval` sequence; cooling uses glazing `g = 0.60` with a separate shading factor, avoiding double counting. |
| Solar irradiance | Heating facade/season angle | Cooling solar orientation curves | Cooling adds a different maximum-power/orientation analysis. Keep those applied diagrams. |

## Limits

The validation above proves source selection, source discovery and media composition; it does not substitute for watching every full lesson. The parent task runs representative geometry renders and a dry construction pass over all declared scenes. The clear solar-definition repetitions above are reconciled with earlier-source references and obsolete local definitions removed.

## Narration Availability

A source-tree scan found zero canonical MP3/WAV/OGG/M4A/FLAC assets in the current energy corpus. Current authored lessons therefore remain silent. No speech provider was invoked and no narration was fabricated. When canonical per-clause recordings exist, the runtime aligns and attaches them to the source before any excerpt is cut; partial recording sets and recordings longer than their measured subtitle windows fail explicitly. The real audio integration fixture validates this path with generated test tones and does not claim to supply production narration.

## Final Verification

The final managed Python 3.12 run completed 30 clip contract tests in 30.355 seconds with one existing Windows symlink skip. Source code was held stable for this run.
