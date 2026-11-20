# Round 3 — `vegetation/small-leaved-lime` (park lime of about 60 years, *Tilia cordata*)

State: IN PROGRESS (2026-11-20).

## 0. Scripts and resume

No earlier round-3 work existed for this asset (no `r3-small-leaved-lime-*`, no report, no generated folder), so the work started at step 1 of the brief on 2026-11-20 04:40. The catalog `volumes` and `volumes/vegetation/small-leaved-lime.json` were identical at the start (no bulk pass had touched this asset's volumes). The state I found is kept in `🗑️generated/r3-small-leaved-lime/source-before-r3.json` until the ticket's generated folder is cleaned.

- `r3-small-leaved-lime-model.ts`: every stem and limb section, branch, burr, ridge, shoot, lobe, tuft and outlet per scale, in world metres; writes the volumes source and the candidate document `🗑️generated/r3-small-leaved-lime/🔣️.json`.
- `r3-small-leaved-lime-kit.ts`: tube and lobe conversion to bone-local solids and their exact orthographic outlines (my copy of the apple-tree kit).
- `r3-small-leaved-lime-draw.ts`: ruling E1, the 2D shapes of every scale and view as true views, and the plan symbol.
- `r3-small-leaved-lime-roots.py`: writes the named members of the candidate into the catalog file; every other member must stay equal.
- `r3-small-leaved-lime-areas.json`: the corrected bark areas (input of the model script).
- `r3-small-leaved-lime-cross.ts` (where two scales' outlines part), `r3-small-leaved-lime-look.ts` (zoomed views, wood only).
- Resume: `sh r3-small-leaved-lime-ship.sh` (build, validate the candidate, write roots and splice only when valid, validate the catalog), then `$BUN volumes-check.ts vegetation/small-leaved-lime --product`.

## 1. Reference

| quantity | real small-leaved lime | source | asset |
|---|---|---|---|
| height | 20 to 30 m when old; an open-grown park tree of about 60 years stands at 14 to 18 m | Pretzsch et al. 2015 (`pretzsch-2015`, height and crown against stem diameter for *Tilia cordata*); dendrology handbooks (Mitchell, Trees of Britain and Northern Europe) | 14.17 m, kept |
| crown spread | 8 to 10 m at a stem diameter of 0.5 to 0.6 m; about 0.6 of the height | Pretzsch 2015 | 9.0 x 8.95 m, kept |
| stem diameter at 1.3 m | 0.45 to 0.6 m at this size | Pretzsch 2015 | 0.57 m (rig section 0.56 m) |
| clear stem | 2.5 to 3.5 m in a park, crown base hanging to about 3 m | botany | stem 2.9 m, lowest foliage 2.85 m |
| habit | straight stem and one persistent leader to the top; limbs leave at 40 to 55 degrees above the horizontal, arch outwards and droop at the ends in the lower crown, and stand steeper towards the top; limbs sit all round the stem in loose tiers | botany (Mitchell; Roloff, Bäume Mitteleuropas) | leader in four parts; four tiers of four limbs each plus four top branches |
| crown form | dense, closed, broadly ovoid dome, widest in the lower third, blunt top; finely billowed surface without sky windows | botany | ovoid, widest at 6.5 m, 134 lobes |
| limb taper | cross-section roughly conserved over a fork | pipe model, Shinozaki 1964 | stem 0.246 m² against lower leader 0.196 + the four lowest limbs 2 x 0.038 + 2 x 0.011 = 0.29 m² |
| bark | grey-brown with long shallow ridges on old stems; burrs with epicormic shoots at the stem base and on the stem are typical of the species | botany | 16 ridges of 22 mm, three burrs, two shoot clusters |
| leaf | heart-shaped, 3 to 8 cm | botany | leaf marks of 7 cm |
| leaf area index | 5 to 7 for a closed lime crown | Jones 2014 (`jones-2014`) | 2.3 + 2.8 (sun + shade), kept |
| green wood density | about 750 kg/m³ | Wood Handbook 2021 (`wood-handbook-2021`) | 750, mass 1553 kg, kept |
