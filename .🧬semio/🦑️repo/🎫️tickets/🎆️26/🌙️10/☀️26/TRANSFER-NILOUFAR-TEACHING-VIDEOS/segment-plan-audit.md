# Segment Render Plan Audit

Independent read-only inspection of original Niloufar tutorial sources supports separately constructed beat Scenes, followed by ordered chapter/movie concatenation. No actual cross-beat object/state dependency was identified. This is a static audit; final full rendering remains a separate check.

| Video | Chapters | Beats by chapter | Total beats |
| --- | --- | --- | --- |
| Physical Fundamentals | 1 | 8 | 8 |
| Ventilation | 6 | 3,3,4,6,4,2 | 22 |
| Heating | 6 | 9,6,1,5,8,6 | 35 |
| Cooling | 6 | 3,5,3,5,2,3 | 21 |
| Energy Balance | 7 | 4,4,4,4,4,3,3 | 26 |

Total: 26 chapters, 112 beats; five intros yield 31 logical intro/chapter clips and five complete movies.

## Original Builder Evidence

The original source chapter playlists explicitly enumerate the above beat order. Physical Fundamentals delegates its only chapter to list(_m1.BEATS). Its original build_full_video.py renders intro plus every beat in BEATS separately, passing each scene class name to a separate Manim invocation before concat. The recommended full-physical script likewise documents beat renders plus FFmpeg concat.

Ventilation's six section builders and Energy Balance's seven builders enumerate scenes in exactly playlist order and invoke Manim once per scene. Heating's first five builders do likewise. Cooling's six builders do likewise: sections 1 and 5 use uppercase SCENES and inline subprocess arrays; their alternate builder shape still creates each beat independently.

Heating final calculation has no original build_full_video.py. Its local FullFinalCalculationVideo wrapper lists ReviewingHeatLosses, Scene2, ReviewingHeatGains, Scene4, UltimateEnergyBalance, AnlagenVerluste, exactly matching the full Heating playlist. Each of these classes uses its own NARRATION and ordinary Scene methods; none reads custom self state left by another beat. Thus missing an original individual-builder file here does not imply a cross-beat requirement.

## State Audit

AST inspection of every authored scene construct found normal Scene methods/properties, own class NARRATION, own class constants and own helper methods. Regulatory-framework beats refer to helper methods declared on the corresponding classes, which independent Scene construction preserves. Their renderer time calculations use a captured local t0 and elapsed differences. Energy Balance Beat4_RichtigAuslegen uses self.time for an updater phase; its original section builder renders this beat independently, so this is authored within-beat behavior rather than a prior-beat dependency.

Scenes create their own local visual objects. No cross-beat self object, class handoff, or prior scene instance requirement was identified. Separate render processes also preserve the original per-section builders' module/class initialization semantics.

The original Cooling system-sizing playlist intentionally starts Beat2_VolumeFlowEquation then Beat3_IsolateAirflow; there is no missing Beat1 to invent. Preserve these explicit names/order.

## Manifest Comparison

The new explicit manifest was inspected after video_portability saved it. The independent AST comparator resolved original module paths, original playlist class references and PF BEATS, then compared all 112 ordered {script,scene} pairs against every chapters[].segments list. All five videos matched exactly. Every intro script and selected intro class also exists in the original source. Counts match the table above: 26 chapters, 112 beats, five intros, 117 independent render units assembled into 31 logical clips and five movies. The comparator passed without importing runtime or scenes. The retained independent comparator is `compare-segments.py`; its derived JSON is transient generated output.

The retained original playlist/builder audit input is `segment-audit.py`; its derived JSON is transient generated output. No runtime, manifest, authored scene, or index edits were made.
