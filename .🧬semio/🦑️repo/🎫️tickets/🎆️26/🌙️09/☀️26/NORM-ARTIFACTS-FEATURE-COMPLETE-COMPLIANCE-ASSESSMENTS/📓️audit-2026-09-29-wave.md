# Norm audit wave 2026-09-29

Read-only Composer 2.5 fleet. Lines marked confirmed were re-read in current source. The rest is the auditor’s blocking list, not yet re-read line by line. No execution agents: Grok 4.7 High is not an available model slug.

Shared report path: PASS. Check, remedy, bilingual results chrome, and the compliance gate (option 0 must leave Fail; annex De vs En except din4108, din18599, iso16757, vdi3805) are wired. [Shared report](ba99a4cd-47b0-4139-bfb9-f912d986558a).

## Confirmed in source

- DIN 18599: `din18599.2.cooling-need` limit is `q_c.max(1.0)`. Net-floor remedy only past 5%. See `📓️audit-2026-09-29-din18599.md`.
- DIN 4108: `check_surface_mass` forces Pass. Empty-zone and empty-element fails set `applicable = false`.
- EN 1990: `betaComputed` is an input. SLS 6.14b–6.16b compare service effect to `rdStr`. Annex B DSL/IL stores computed 0/1 and utilization 1 or 2. Category utilization is `category/5`.
- EN 1992: EN punching `vRd,max` is commented “conservative stand-in”. Prestress utilization is `stress_pa(util * lim_c)`.
- EN 1998: elevation regularity uses stiffness and mass, remedy only raises `stiffnessX`. Bridge `periodRatio > 3` fails with no remedy; only `< 1` gets one.
- EN 1999: remedies write resistances into characteristic `nK`. Section options include `sec-i160` and similar ids. Explanations embed `actions_digest`.
- ISO 16757: several `OneOf` remedies use `options: Vec::new()`. Empty constraint ids are rewritten to `constraint-{ci}` and then pass.
- VDI 3805: `limits.maxNestingDepth` uses `actual_depth = 4`. Historical sheets Pass when `strictMode` is off; the strict remedy sets `strictMode` false.

## Reported, not re-read line by line

- DIN 16798: fan-flow pass band `1 + 1e-6`; filter utilization is a rank, then forced Pass when overspecified.
- EN 1991: `lm1-remaining` can Fail with no remedy; `c_pe`, opening factor, and notional lanes use synthetic scores; `alpha_a` and `lm3_axle_n` branches are identical for De and En.
- EN 1995: members or connections with no actions emit no strength checks; `finish()` can leave only `applicable = false`; fastener-count remedies are non-applicable.
- EN 1996: qualitative checks use fixed 0.2/0.5/1.5/2.0 utilization; basement thickness remedy is `t * 1.2`.
- EN 1997: governing-situation rollup never fails; earth-pressure check is hard-coded Pass; UPL γ factors are identical for De and En; settlement remedy raises the limit to the computed settlement.

## Confirmed after the first write

- EN 1993: bending `en1993.6.2.5.my` adds a section remedy only when `next_section_options` is non-empty. HSS utilization uses `m_rd.max(1.0)`.
- EN 1994: class and shear remedies write `steel.twM`, and `SteelSection::resolve` replaces plate geometry from the catalogue designation. §7.2.2 writes stress magnitudes into `qAreaPa`. Bending `one_of` lists every heavier HEB; LTB filters to sections that clear.

The audit fleet is complete. All 15 families are GAPS. The shared report path is PASS.
