# Invalid Canopy Material Provenance R79

Read-only exact arithmetic and fresh selected geometry audit. Current source SHA2561bad321155b30fcaed4d85f6ff15e0ea5102d93e71c0b367693d19cccf72bb34. The same oldquad is reproduced after fixture publication; no source/producer/test edits by reviewer. Retained portable minimalinput slab-invalid-minimal-r79.json contains only two operand triangles, arguments and exact failing output; no complete source copy. Reproduction slab-invalid-trace-r79.py; generated full trace/stages retained.

## Native Identity

SL-STAIR-CANOPY full_parts index41 of57, region REGION-SL-STAIR-CANOPY, MAT-RC, z9.03..9.26, thickness0.23. Exact output ring:

```json
[[2.811716948652,10.402239499262],[7.32701794427,9.987825055392],[7.32701794427,9.987825055393],[2.811716948659,10.402239499261]]
```

GEOS reports Self-intersection[4.00113325221129 10.2930748397082], unsigned reported area1.4503954624574418e-12m². Exact binary64 rational orientations prove edges0→1 and2→3 cross properly: endpoint side signs `[+,-,-,+]`. This is true self-crossing, not an is_valid tolerance artefact.

## Exact Original Geometry

The ring originates directly from `_analysis._clip(subject,clip,digits=12)` inside `_slab_surface` which pairs convex ears of the same expanded region and reference outline. Subject and clip are adjacent source triangles sharing byte-identical binary64 edge A=(2.8117169486516618,10.402239499262112),B=(7.3270179442704295,9.987825055392424). Subject third vertex is(7.27340137075032,10.044757618762205); clip third vertex is(7.376946909212996,9.927542597843773). Exact BigInt rational determinants place their third vertices on strictly opposite sides of AB. Hence exact triangle intersection is precisely AB, zero occupied area. No genuinely positive source material belongs to this generated sliver.

## Rounding Mechanism

`_half` computes ordinary floating intersections, then rounds every output coordinate to12digits and canonicalizes via `_ccw` at EACH clipping stage. Stage1 rounds the original shared vertices while retaining the full triangle. Stage2 retains those rounded vertices. Stage3 clips that already rounded triangle against the ORIGINAL unrounded shared edge B→A. The rounded shared endpoints no longer lie exactly on that edge; new floating crossings are constructed and rounded again, producing the four-vertex bow-tie. Generated slab-invalid-stages-r79.json records each actual stage, establishing the mechanism rather than inferring it solely from final coordinates. Five invalid intermediate clip outputs occur in the broader triangulation; only this one survives the existing area filter into full_parts.

The existing `_slab_surface` positive area>1e-12filter admits this artefact because computed area is slightly above threshold; increasing the threshold would conceal a topology defect and could discard true small material elsewhere. Correct handling must preserve topology/shared boundaries and all truly positive material. This exact zero-area input pair may safely produce no occupied polygon based on exact geometry, not because it is small. A long-term kernel correction should avoid repeated coordinate rounding against inconsistent unrounded constraints or use exact boundary classification/construction behind the existing interface. Root owns implementation and schema-first language-agnostic regression design; no proposed quick area cutoff, invalid-ring skip or source-specific exemption is endorsed.
