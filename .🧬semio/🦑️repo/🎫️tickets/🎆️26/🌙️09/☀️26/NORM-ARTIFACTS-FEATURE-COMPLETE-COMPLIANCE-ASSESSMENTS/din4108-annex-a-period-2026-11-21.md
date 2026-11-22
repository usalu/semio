# DIN 4108-3 Annex A period balance

The moisture diagram and report share the March 2024 winter profile, with the fixed boundary pressures of 1,168 Pa and 321 Pa and the prescribed thermal surface resistances. Drying uses 1,200 Pa on both sides and 1,700 Pa at the condensation location for walls and shaded or light roofs. An unshaded dark roof uses 2,000 Pa. The calculation changes the diffusion paths when the first wet plane dries. [DIN Media edition history](https://www.dinmedia.de/de/norm/din-4108-3/373925063) · [DIN 4108-3:2024-03, Annexes A and B](https://de.scribd.com/document/929890470/DIN4108-3)

The Annex B wall reproduced both condensation interfaces and 3.197 kg/m² potential evaporation. Its winter mass was 0.869 kg/m² using the unrounded calculated saturation pressures, compared with the published 0.872 kg/m². Independent SQLite reproduced the unrounded calculation and checked the pressures substituted in the published worksheet. Six altitude boundary cases and both roof summer variants passed.

The live report screens exterior opaque stacks. The site case records elevation and room conditions and excludes the Annex A calculation above 700 m. The roof case applies an exposure to the selected element ID. A roof without recorded exposure uses the shaded or light roof value for screening. Construction-case exemptions and material-specific condensate limits remain open in D-07. Native and fresh signed browser verification remain pending.

## Construction cases

The two new design cases persist a room-side floor slab assessment and a Figure 17 attic ceiling assessment. The slab reports the 20 m vapour diffusion recommendation. The attic case checks the outdoor opening area, the two upper-layer limits, the airtight layer resistance and the room-side share of thermal resistance. Both require the specified assembly conditions to be confirmed. D-07 still covers other exemptions and material-specific limits. [DIN 4108-3:2024-03 §5.3.3.3 and §5.3.5](https://de.scribd.com/document/929890470/DIN4108-3)

All 23 construction cases matched an independent SQLite calculation. The DIN 4108 archive suite passed 26 tests and 1,100 assertions, including restoration through the TypeScript snapshot parser.

The construction catalogue also checks the 20% resistance share above waterproofing for perimeter-insulated floor slabs, the two thermal-resistance bands for internally insulated exterior walls and the three diffusion bands for Figures 4 and 5 roofs. The latter apply the 20% room-side resistance limit using the least-resistive path. All 60 cases in the combined construction fixture matched an independent SQLite calculation. The archive suite passed 26 tests with all seven period and construction cases included in the persisted record. [DIN 4108-3:2024-03 §5.3.2–5.3.5](https://de.scribd.com/document/929890470/DIN4108-3)

## Material and airtightness evidence

Element-specific condensate evidence now selects the 0.5 or 1.0 kg/m² cap and verifies material durability and the 5% and 3% wood moisture increases. A recorded case names the element ID. Duplicate rows with that ID leave the assessment unverified. Independent SQLite and Rust agreed across 19 material cases. The existing design-case owner now routes every period, construction and material entry through the same declared catalogue. [DIN 4108-3:2024-03 §5.2.2 and §5.2.3](https://de.scribd.com/document/929890470/DIN4108-3)

The 2026 airtightness concept collects the four planning and quality-assurance references for technical review. Seven independent evidence cases passed. The recorded evidence alone does not verify execution or pressure results. [DIN 4108-7:2026-04](https://www.dinmedia.de/de/norm/din-4108-7/398456216) · [FLiB planning guide](https://flib.de/publikationen/09_leitfaden/FLiB_Leitfaden_Luftdichtheitskonzept.pdf)

## Ventilated roofs

Case DINA-09 takes the declared ventilated assembly, dimensions, openings and diffusion inputs. At roof slopes below 5°, it checks opposing free openings, the airflow length and cross section, room-side diffusion thickness and resistance share. At 5° and above, it applies the separate minimum free height and distinguishes eaves or the mono-pitch termination from a ridge or hip. Both branches apply the corresponding sloped roof area in the opening formulas. The calculation uses free areas after the effect of obstructions. [DIN 4108-3:2024-03 §5.3.4](https://de.scribd.com/document/929890470/DIN4108-3)

The independent SQLite projection and production Rust evaluator agreed on 18 vectors covering both slope ranges, threshold values and missing or ineligible geometry. Native and browser execution remain pending.
