# Corrected Raw Scene Visual Audit

This spot check visually inspected eight decoded frames from completed corrected Physical Fundamentals raw units, including the precise temperature-label scene that previously showed white replacement boxes. No remaining missing-glyph boxes, clipped labels or black frames were observed in these samples. This does not certify unsampled footage or the final assembled movies.

## Inputs and Method

Read `glyph-coverage.md`, `visual-movies.md`, and the nested current `video_manifest.json`. The retained `corrected-visual-probe.py` runs through the repo launcher and `run.ts`, reads completed MP4s only when their `.render.json` sidecar records the frozen font helper SHA256 `00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2`, then uses existing PyAV to decode the first frame at or after each requested timestamp. Existing Pillow builds the generated contact sheet. All output stays under `🗑️generated/corrected-visual`; `frames.json` records requested/actual timestamps and exact paths. No rendering was started.

## Inspected Samples

| Raw ordinal | Scene | Actual local frame time (s) | Visual observation |
| --- | --- | --- | --- |
| 6 | Beat5_Strahlung | 68.465690 | Red room-temperature readout renders ϑ clearly; solar diagram, equation and caption fit. |
| 7 | Beat6_ThermischeMasse | 67.599674 | Room convection diagram and German labels fit; no boxed glyphs. This selected frame has no temperature symbol to assess. |
| 8 | Beat7_SensibelLatent | 85.664388; 101.264388 | Humidity chart horizontal ϑ axis label renders correctly; chart, saturation curve and captions fit. |
| 8 | Beat7_SensibelLatent | 116.864063 | Sensible/latent bar chart and German captions are intact; no symbol target in this moment. |
| 8 | Beat7_SensibelLatent | 132.397070 | Exterior/interior ϑ temperature labels render correctly; lower formula is in its authored fade-in transition. |
| 8 | Beat7_SensibelLatent | 146.397070 | Winter/summer diagrams and existing visible temperature labels remain clean while the summer diagram builds. |
| 8 | Beat7_SensibelLatent | 153.397070 | Both finished diagrams show clean exterior/interior ϑ labels and complete lower comparisons: winter surface/dew point and summer register/dew point. The previously observed solid white boxes are absent. |

Opened the complete generated `contact.jpg` and the final 854×480 `8-0.98.png` at native resolution for the detailed comparison. Raw ordinal 8 is `energy/demand/1_physical_fundamentals/scene_1.py::Beat7_SensibelLatent`; the corrected winter/summer symbols are authored around lines3295–3338. All timestamps above are raw-unit local times; first-pass whole-movie timestamps cannot be reused as corrected movie positions.

## Pending Targets for Final QA

| Raw ordinal | Exact source/scene | Required check | Current status |
| --- | --- | --- | --- |
| 80 | energy/demand/4_cooling/3_transmission_humidity/scene_3.py::Beat3_LueftungslastImSommer | Blue `← Lüftung · Kapitel 3:` chip at authored lines531/542/567, plus later chapter5/4 variants | No completed raw clip available during this audit; not visually certified. |
| 115 | energy/demand/5_energy_balance/7_praxis_vs_theorie/scene_7.py::Beat1_BerechnungVsAbrechnung | Horizontal-axis `berechnet →` at authored line444 | No completed raw clip available during this audit; not visually certified. |

No `\oint` command or ∮ literal occurs in any authored intro/segment script referenced by the manifest. It is a declared shared math symbol, so no actual curriculum frame exists to target. Its actual native Pango/SVG coverage and supporting-font geometry are already validated by the all44-symbol glyph tests and tiny rendered probe in `glyph-audit.md`; this raw-course sample adds no claim of an integral appearing in authored footage.

## Process and Ownership

The first sampling command saved its files then encountered cp1252 encoding when printing Unicode paths; its output was changed to an ASCII-only progress count and the full probe reran successfully with exit0. The final eight-frame probe also ended0. All owned diagnostic processes finished. No source/index, renderer/controller, font helper, or live generated media/cache was changed. Final movie QA remains required after all117 raw units and five movies complete.


## Follow-up: Completed Cooling Arrow Chip

Raw ordinal80 (`energy/demand/4_cooling/3_transmission_humidity/scene_3.py::Beat3_LueftungslastImSommer`) is now complete, with MP4 and `.render.json` recording the frozen helper SHA256. This supersedes its pending status above. The retained `corrected-arrow-probe.py` sampled six frames through the repo launcher, completed with exit0, and saved all images plus exact timestamps under `🗑️generated/corrected-arrows/`.

| Actual raw-local time (s) | Authored moment | Visual result |
| --- | --- | --- |
| 14.599674 | Sensible-load chip fading in | Left arrow is present; low opacity is the authored transition. |
| 23.333008 | Chapter3 sensible-load reminder, line531 | Blue `← Lüftung · Kapitel 3:` followed by sensible-load formula is clear. The former boxed prefix is gone. |
| 32.066341 | Chapter3 latent-load reminder, line542 | Left arrow and latent formula are clear. |
| 40.799674 | Combined sensible/latent reminder, line567 | Left arrow and both formula terms are clear. |
| 49.533008 | Chapter5 recovery and Chapter4 ventilation strategies, lines576–577 | Both separate blue chips begin with readable left arrows; labels fit within the frame. |
| 55.999674 | Final recovery/strategy hold | Both arrows and remaining formulas/caption remain clear. |

The completed corrected raw unit duration is58.266341s. Opened `contact.jpg` and the full native854×480 `80-0.40.png`; none of the sampled frames has a replacement box, black/missing frame, clipping or new layout defect. Scene diagrams, German captions and stacked load bars remain intact.

Raw ordinal115 (`Beat1_BerechnungVsAbrechnung`, `berechnet →` axis) still had no completed MP4/sidecar at the final check of this follow-up and remains pending. No claim is made about that target yet. All owned diagnostic processes have ended; product/source/index/render-controller files and generated render media remain untouched.


## Follow-up: Completed Energy Balance Axis

Raw115 `energy/demand/5_energy_balance/7_praxis_vs_theorie/scene_7.py::Beat1_BerechnungVsAbrechnung` is complete with the frozen font helper digest in its provenance sidecar. This supersedes its earlier pending status. The repo-driven `corrected-balance-probe.py` finished0 after sampling six frames at local52.399349,64.065690,75.665690,87.332357,104.799023 and112.932357s. Results remain under `🗑️generated/corrected-balance/`.

Opened the full contact sheet and the native854×480 `115-0.65.png`. The authored `berechnet →` axis at scene line444 shows a distinct right arrow, with no white box, at75.665690s and remains correct in the later87.332357,104.799023 and112.932357s frames. The64.065690s frame is an authored axis build before the arrow appears. Scatter points, reference line, statistic cards, German captions and labels fit within the frame. No remaining glyph or layout defect was found in these targeted samples. These checks complete the three originally affected PF/Cooling/Energy Balance raw-scene targets. Final whole-movie contact-sheet QA follows when all five final movies and sidecars are published.


## Final Corrected Movie Visual Certification

All five final corrected MP4s and their `.render.json` sidecars were present under `🗑️generated/g/rendered/` before inspection. The retained `final-visual-previews.ts` invoked the existing `make-previews.py` through repo, using ticket FFprobe and existing PyAV/Pillow, and finished with exit0. The preview tool checked the exact five expected manifest basenames and successfully selected all15 requested frames. Opened and visually inspected each generated three-panel10%/50%/90% contact sheet.

**Result: all15 final sampled frames pass visual inspection.** None is black/missing or shows a replacement glyph box, clipped caption or new layout defect. The originally affected PFϑ, Cooling← and Energy Balance→ labels are now visibly correct in the final movies as well as the separately sampled raw scenes. This certifies the listed visual samples; independent whole-frame decoding and assembly-provenance checks are separate audits.

| Final movie | Actual decoded10%,50%,90% frame times (s) | Observation |
| --- | --- | --- |
| Physical Fundamentals | 100.465039; 502.388932; 904.249089 | Kilowatt-hour scale, heat-pump energy balance, and condensation diagrams are visible. The 90% temperature labels contain readable ϑ characters; no former white rectangles remain. |
| Ventilation | 162.266016; 811.124870; 1459.987305 | Roadmap, winter airing comparison and seven-card handover sequence are visible. The final sample is the intended empty-card build-in, verified by the first populated card five seconds later. |
| Heating | 172.799349; 863.928776; 1555.123568 | Room convection, thermal-bridge comparison and heat-gain utilization graph are visible. German labels, equations, captions and legend fit; no missing glyphs observed. |
| Cooling | 107.666016; 538.062435; 968.524544 | Cooling air circuit, summer ventilation load and sorption-assisted strategy diagram are visible. The 50% blue reminder chip begins with the corrected readable ←. |
| Energy Balance | 212.199674; 1060.732357; 1909.326823 | Envelope/ventilation loss diagram, annual load curve and measured-vs-calculated scatter plot are visible. The 90% horizontal label ends in the corrected readable →. |

### Authored Build-in Confirmation

`final-visual-details.py` recorded exact first-frame-at-or-after timestamps for all15 targets and decoded four adjacent samples five seconds later; it finished0. All19 timestamp records are in generated `corrected-final-previews/frames.json`. Opened the four additional native854×480 frames:

- PF909.249089s: the summer duct/register diagram builds beside the intact winter diagram and readableϑlabels. Its earlier904.249089s sample intentionally introduces the summer half before those objects appear; it is not missing rendered content. The earlier raw8 local153.397070s inspection shows the finished combined diagram.
- Ventilation1464.987305s: the first outlined card is now populated with `V / Volumen / 440 m³`, confirming the empty-card1459.987305s frame is an authored reveal sequence.
- Energy Balance217.199674s: the ventilation coefficient n remains intentionally highlighted in a yellow outline during the staged equation explanation. This is an authored formula highlight, not a missing-font white box.
- Energy Balance1065.732357s: the annual-load area is filled and reads `Qₕ = 24 600 kWh`, confirming the earlier1060.732357s zero/empty-area state is the beginning of that animation.

### Retained Evidence and Process Completion

Contact sheets, native supplemental frames and timestamp JSON remain solely under `🗑️generated/corrected-final-previews/` for the parent final review and eventual generated cleanup. Contact-sheet SHA256s:

- `Full_Physical_Fundamentals_480p15.jpg`: `3cea59918604ee4f6f7e0b02c3956a24eba0aac02d4198772c39c84b8d8ea190`
- `Full_Ventilation_480p15.jpg`: `46e875dd221fda839ce0d94a582bf45b659ece61996619bf650d72ebda663354`
- `Full_Heating_Demand_480p15.jpg`: `47ec526a61fd152355a671165c5c476d125826a514cbe2f3de788eb0f8e9ebaf`
- `Full_Cooling_Demand_480p15.jpg`: `0516954ea06aa8716c6b0755a2a4feeefa2ff5f777e717c5c00008c3d3cfdde8`
- `Full_EnergyBalance_480p15.jpg`: `3310893681952213f06795630fc9ef152ed2333e6ecead6df49827bc7e21eb9e`

All owned preview/probe subprocesses ended successfully before this report was completed. No renderer/controller, source/product/index, font helper, or original render cache/media was changed. The frozen font helper SHA256 is still `00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2`. No visual QA blockers remain among the inspected corrected final samples.
