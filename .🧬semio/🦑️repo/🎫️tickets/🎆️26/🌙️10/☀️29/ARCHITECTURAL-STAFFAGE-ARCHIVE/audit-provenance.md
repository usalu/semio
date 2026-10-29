# Provenance and Claims Audit (haiku-reader, web-verified sample)

~115 distinct reference ids across 48 assets; ~20 verified by web search, the rest known-real; `"source": "measured"` occurs 0 times;
no "validated/accurate/CFD/calibrated" claims.

## Errors to fix
- `hoy-zernicke-1985`: real paper, title wrong ("Modulation of limb dynamics in the swing phase of locomotion").
- humans cyclist `iso-8090-2007`: id/text disagree (ISO 8090 = terminology 2019; text describes ISO 8098:2014 / EN 14764).
- humans jogger `cavanagh-1982` → 1980; wheelchair `cooper-2008` → 2014; `mcmahon-1986` and `rain-wetting` are not citations.
- humans jogger: Dalleau 1998 speeds misquoted (paper ≈ 5.1 m/s).
- vehicles city-bus: 68 kg passenger mass from Directive 97/27/EC Annex I, not 2001/85/EC (repealed).
- Two Wood Handbook editions under different ids (objects GTR-190 2010 vs vegetation GTR-282 2021) — unify; Table 5-3 unverified.
- horse `halter-webbing` tagged literature without reference.
- Hutchinson & Brown 1969, Gebremedhin & Wu 2001: unconfirmed pages/DOI → re-tag authored-estimate if unconfirmed.
- vehicles: "condensed from manufacturer data sheets" without cited sheets → label authored-estimate; pseudo-ids
  (`typical-b-segment`, `typical-bev-12m`, `soil-density`) are authored bases, not references.
- components: process notes ("per brief", "to be verified") in shipped citations.
- running-child: rounded Jensen 1986 fractions must be authored-estimate ("guided by Jensen 1986").
- labrador-dog summary "correct footfall order" → "footfall order after Hildebrand 1965".
- ISO 7730 metabolic table is Annex A, not B.
