# roe-deer (Capreolus capreolus) - authoring notes

- Fragments: `🧰️scripts/animals/roe-deer/{00-head,10-rig,20-anchors,30-geometry,40-poses,50-clips,60-thermal}.json`. Rig, anchors and geometry are expanded from hand-authored limb templates by `roe-deer-gen.py` (near/far mirroring only); poses and clips by `roe-deer-emit.py` (IK via `kin.py`); `roe-deer-run.sh` assembles and checks, `roe-deer-view.sh` previews. Check: `roe-deer` passes.
- Dimensions: 25 kg buck, height 1.08 m / width 1.46 m (alert pose bounds incl. ears and antlers), withers 0.75 m, rump 0.78 m, hip joint 0.68 m.
- Rig: 26 bones; spine pelvis-lumbar-thorax, neck, head, jaw, two ears; near and far chains hind (thigh, shank, cannon, hoof) and fore (scapula, humerus, forearm, cannon, hoof). Masses authored-estimate, sum 25.0 kg.
- Poses (named): alert, grazing, alert-foreleg-raised, bounding-gathered, bounding-pushoff. Key poses (`key: true`): walk-phase-0..7, bound-phase-0..7.
- Clips: walk-cycle (1.0 s, loop, plant, advanceM 0.7385, duty 0.65, order hn-fn-hf-ff), bound-cycle (0.9 s, loop, plant, advanceM 0.6775, duty 0.55, overlapping, always supported), alarm-freeze (7 s, non-loop, grazing to alert, ear flick, near fore raised and held, contact-fn ends 3.7 s).
- structure.supports is []; no ballistic phases are used.
- Physics sources: Meeh area 0.97 m2 (k 0.1135, Spector 1956, authored choice); insulation authored-estimate after Scholander 1950 / Cena-Monteith 1975 (Gebremedhin-Wu 2001 for wet nose); activity after Kleiber 1947 / Taylor 1970 / Weiner 1977; gait after Hildebrand 1976 / Dagg 1973 / Muybridge 1887. Joint limits and masses are authored estimates (Andersen et al. 1998 as guide).
- Simplifications: bound has no flight phase (overlapping stances); stance hoof keeps its sole flat (no heel-off); far-side hooves/toe tips can dip up to 2 cm below y=0 in swing; tail is a vestigial shape on the pelvis (white rump patch as detail); antlers are detail strokes; hoof cleft is a detail line only.
- Pose bounding-gathered / pushoff have the centre of mass outside the support footprint (snapshot of a dynamic phase); only the force residual is zero.
