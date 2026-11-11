# Published Master Visual Audit

Checked 2026-11-07 UTC. Freshness is from canonical lib.video_status and lib.master_fresh against the current manifest and inputs; each returned True. These are point samples and do not establish full temporal coverage.

| Master | Published mtime UTC | Duration | Fresh segments | Visual observations at 25%, 50%, 75% |
|---|---|---:|---:|---|
| Optimization_Extrema_speaker_en_480p15.mp4 | 05:57:20 | 295.8 s | 1/1 | No blank frames or obvious clipping. Diagrams, graphs and captions visible; fine explanatory labels are small at output resolution. |
| Optimization_Sampling_speaker_en_480p15.mp4 | 06:02:28 | 265.1 s | 1/1 | No blank frames or obvious clipping. The 75% sample shows overlapping labels during a scene transition; temporal check below. |
| Full_Physical_Fundamentals_speaker_en_480p15.mp4 | 06:06:43 | 1085.2 s | 8/8 | No blank frames or obvious clipping. Heating schematic and thermal mass diagrams remain in view. The equation at 25% is dim at that sampled instant. |

## Sampling transition check

Native 854×480 frames at 193.85, 197.85, 198.85, 199.85, and 203.85 seconds show a transition: the endpoints are clean and intermediate samples contain temporary overlaps. Persistent label collision is not confirmed. Camera focus and restoration cannot be comprehensively established by these isolated frames. No staffage appears in these sampled frames, so staffage proportion and clipping remain unchecked.

No confirmed visual defects in the reviewed samples. Full playback and scene-level coverage remain necessary for broader conclusions.

## Optimization Experiments

Published mtime 2026-11-07 06:07:35 UTC; duration 322.1 s; canonical freshness True, 1/1 segment fresh. Samples at 25%, 50%, and 75% show diagrams and staffage within frame; no obvious disproportion or blank scene. The sampled 75% overlap was checked at native 854×480 at 236.55, 240.55, 241.55, 242.55 and 246.55 seconds: endpoints and 240.55 s are clean, while intermediate animation shows changing labels. Persistent collision is not confirmed. This remains a limited point-sample inspection.

## Optimization Index

Published 2026-11-07 06:12:54 UTC; duration 301.9 s; canonical freshness True, 1/1 segment fresh. At 25%, 50%, and 75% there are no blank frames or obvious clipping; staffage is proportionate. The 25% view contains overlapping scene labels. Native 854×480 samples at 70.48, 74.48, 75.48, 76.48, and 80.48 s show an animated transition with clean endpoints; persistent overlap is not confirmed. Limited point sampling does not establish full coverage.

## Optimization Pareto

Published 2026-11-07 06:17:53 UTC; duration 291.4 s; canonical freshness True, 1/1 segment fresh. Samples at 25%, 50%, and 75% show the graphs and room in frame, proportionate staffage, readable captions and no obvious blank or clipping. Fine chart labels are small at output resolution. This is limited point sampling.

## Optimization Algorithms

Published 2026-11-07 06:25:59 UTC; duration 361.1 s; canonical freshness True, 1/1 segment fresh. Sampled 25%, 50%, and 75% show no blank frames or obvious clipping; staffage is proportionate. Native 854×480 samples at 85.27, 89.27, 90.27, 91.27 and 95.27 s show overlapping values while the simulation animates; both endpoints are clear. Persistent collision is not confirmed. Fine labels are small; point sampling cannot establish full coverage.

## Full Ventilation

Published 2026-11-07 06:32:08 UTC; duration 1876.6 s; canonical freshness True, 22/22 segments fresh. Samples at 25%, 50%, and 75% show diagrams and captions in frame, with no obvious blank, clipping, or overlap. No staffage is visible in these samples. Isolated samples do not cover all scenes or camera transitions.

## Full Heating Demand

Published 2026-11-07 06:55:25 UTC; duration 1930.0 s; canonical freshness True, 35/35 segments fresh. Samples at 25%, 50%, and 75% show diagrams and captions in frame, no obvious blank, clipping, or overlap. No staffage is visible in these samples. The sampled red formulas are small but distinguishable at this contact-sheet scale; samples do not cover all scenes or camera transitions.

## Full Energy Balance

Published 2026-11-07 07:24:35 UTC; duration 2400.6 s; canonical freshness True, 26/26 segments fresh. Samples at 25%, 50%, and 75% show captions and building diagrams in frame, with proportionate staffage. The dim 75% frame is a transition: native 854×480 samples at 1795.45, 1799.45, 1800.45, 1801.45, and 1805.45 s show the chart fade out and the building scene appear. No persistent defect is confirmed. Point sampling does not cover all scenes or camera transitions.

## Full Cooling Demand

Published 2026-11-07 07:53:42 UTC; duration 1148.6 s; canonical freshness True, 22/22 segments fresh. Samples at 25%, 50%, and 75% show diagrams, formulae and captions in frame, without obvious blank, clipping or overlap. No staffage is visible in these samples. Isolated samples do not cover all scenes or camera transitions.

## BIM attempt failures

Source 114 current attempt 1794043437341272600 exited 1 at 10:02:34 UTC: IndexError from empty ring in savoye plan/🐍️.py:493 to sheet/🐍️.py:121. Source 115 current attempt 1794043432926723500 exited 1 at 10:02:23 UTC: ValueError Open exposed material boundary in drafting surfaces/🐍️.py:37. Attempt logs contain full traces. Existing v clips date from October and were not accepted as fresh. First seven current 114 partials cover opening number/unit frames only; no staffage was visible there.

BIM114 later attempt 1794057566080466100 exited 1 at 14:10:48 UTC with ValueError Open exposed material boundary in surfaces _regions:124 → _trace:38. The earlier empty-ring failure is distinct. The complete attempt log is retained in the cache pending cleanup.

## Grasshopper Tower

Published 2026-11-07 17:26:47 UTC; duration 296.4 s; canonical master_fresh True, 1/1 segment fresh. Three sampled frames show tower and nodes in frame. The 75% overlap is a transition: native frames at 217.3, 221.3, 222.3, 223.3 and 227.3 s show clear endpoints; no persistent overlap confirmed. Separate source partial QA checked the formerly resurfacing note. Full playback and all semantic details remain outside point-sample coverage.

Current accepted sampled count: 12 of 26 masters (11 energy/optimization plus Grasshopper Tower); 14 detailed lessons remain pending. Global course/full-suite verification remains pending.

## Grasshopper Trees

Corrected master published 2026-11-07 17:36:42 UTC; duration 281.5 s; canonical master_fresh True, 1/1 segment fresh. At 25/50/75% selected capsule, branches and tower remain in frame without obvious clipping or overlay. Independent targeted source-path and corrected recap samples are recorded in visual-audit.md; full temporal and course acceptance remain pending.

Both Tower and Trees publishers were confirmed exit 0 by root. Current accepted sampled count is 13 of 26 masters; 13 pending. Full course and suite gates remain pending.

## Source 114 remaining QA

Human-reference QA is not applicable to source 114 or its shared Savoye plan/axon: they contain no human cast. The isolated actual driver probe passed metre scale, displacement and 0°/90° bounds against Shapely; see bim114-projection-performance.md. Full fresh moving-plan visual QA remains required after BIM admission.

## GH 120 native receipt

Current GH source 120 native receipt appeared at 21:21:46 local; drawing verified all 88 recorded source hashes current. The published master is not yet accepted, so current reviewed-master count remains 13/26. Cohort 49488 continues to source 121; v4 gate 60843 remains active. Excel 125 partial 86 was still open without a moov atom and is not a closed reviewed partial. Identify native workers by source ordinal and argv, since GH 120 and Excel 125 both use Beat1_Anatomy.

GH 120 native metadata: one manifest segment with no clause-selected excerpts; all 24 NARRATION beats occur in order, with bounds 0.333–370.717 s inside 373.33 s, a current 88-source receipt and version 1 empty cursor sidecar. Drawing retained check-120-trace.py and its audit. Publication and canonical master freshness remain pending; reviewed master count stays 13/26.

## GH 121 viewport repair pause

Coordinator stopped GH capacity root 37384 and descendants including native 27460; cohort 49488 reaped exit 1 intentionally incomplete. Source 120 native receipt remains reusable. Drawing owns the source 121 viewport correction and permanent bounds tests; canonical dependency inspection found no other lesson depending on that source. Excel 1851 and v4 gate 60843 remain active. A small source 120 publication and a fresh source 121 single-source run are planned after their respective prerequisites; neither implies master acceptance.

## Grasshopper Anatomy master

Publisher session 38650 exited 0; 5,730 frames, duration 382.0 s. Canonical master_fresh True, 1/1 segment fresh, checked after publication with a 1 GiB metadata grant. At 25%, 50%, and 75%, sampled panels, values, perspective objects and captions are in frame with no obvious blank or clipping; these three point samples do not establish full temporal coverage. GH 120 native trace and 88/88 source-pin checks are recorded above. Current accepted sampled count is 14/26; full course and suite verification remain pending.

## Excel 125 native receipt

Drawing verified 854×480 at 15 fps, 262.93 s, current 89-source receipt, version 1 empty cursor sidecar, and one whole manifest segment without excerpts. All 23 NARRATION beats are ordered and valid at 0.333–262.333 s; retained input check-125-trace.py. The master remains pending publication and review; accepted sampled count stays 14/26.

## GH 121 second layout correction

Coordinator stopped only session 5721, capacity root 70108 and native 24404, exit 1 intentionally incomplete, after confirming the types table overlaps the still-visible slider column. Drawing owns the source 121 stage visibility and table/minis centering fix; only source 121 is affected. Excel 1851 and v4 gate 60843 continue. A small concat-only publication for native-complete Excel 125 and independent casting 123 are planned; acceptance remains 14/26 pending 125 master QA.

## Excel Anatomy master

Publisher session 1188 exited 0; 4,074 frames, duration 271.6 s. Canonical master_fresh True, 1/1 segment fresh. At 25%, 50%, and 75%, spreadsheet, capsule, error state and captions are in frame with no obvious blank or clipping. Point samples do not establish full temporal coverage; native cue and 89-source receipt checks are recorded above. Current accepted sampled count is 15/26; full course and suite verification remain pending.
