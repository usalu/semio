# Live Ownership Audit

Read-only comparison of source 120 count-transition ghost mechanism with Grasshopper 122–124 and Excel 125–128; no render or failure reproduced here.

- Grasshopper Tower (122), tower/🐍️.py 38–52 replaces the image child with a new object on each discrete count change. At 379–387 growing is installed in stage then count animates; 438–446 repeats with rising_display. Both are actual keyed image child replacements under a persistent display group, matching the ownership-sensitive structural pattern. Inspect native frames across both count transitions and later stage removal. A ghost has not been established by this source inspection.
- Grasshopper Casting (123), casting/🐍️.py 104–120 similarly replaces the cached image child on a boolean transition; its window display and toggle transition deserve targeted native QA after the correction. This is an actual matching mechanism, not evidence that it produces the 120 ghost.
- Grasshopper Capsule (121), capsule/🐍️.py 106–123 replaces all children on key change via _MixedLive; 817–820 install wrapstage. This differs in mixed vector/image contents but shares child replacement and merits native transition/removal QA.
- Grasshopper Anatomy (120) keyed snapshots at 41–51 now update existing image pixel_array in place. The source owner is separately addressing the reproduced issue.
- Excel Program (127), program/🐍️.py 33–47 and 326–331 uses identical keyed image child replacement; repaint calls 594 and 602 change the key during animation while the tower group is scene-owned (553–560). Inspect count transitions and removal. Excel References (126), references/🐍️.py 21–31 and 602–618 uses the same replacement inside a parent tower_live group; inspect mixed-reference count updates and the FadeOut at 654. These are actual structural matches; neither is a reproduced defect.
- Excel Anatomy (125) and Energy (128) use self.live capsules, with full towers snapshot before display. No explicit keyed image child replacement was found in their lesson source; shared workbook uses self.live for several child groups, but source inspection alone does not establish matching image ownership or a ghost.
- Grasshopper Trees (124) did not show a keyed image child-replacement helper in this scan. Ordinary live groups remain and cannot be declared safe solely from source.

## Concrete dynamic Live oracle candidates

Anatomy 125: capsule scene-added 359–360, keyed on s.iw at 145–152, then recalc 471 ramps width for three seconds while bound B5/B9/B10 values change at 465–467. Energy 128: capsule 270–275 keys tint/grow/vs.t/vent/vol; on-screen ramps at 485 and 540–541 change it, and recalc/fade-in/grow coincide at 524–525. Both are actual dynamic Live play paths; no ghost has been reproduced here.

Old Casting 123 partial uncached_00026 native frames 30–55 show two slider handles and overprinted values while the tower count changes. Saved casting26-30.png, casting26-45.png and casting26-55.png under generated; rerender needed after the ownership fix. 122 count-transition partial has not yet been produced in the inspected generation.

## Native focused oracles

The selected 125 capsule width test without hatch produced two frame max-channel deltas [0,48] against freshly drawn Cairo. The selected real B5 bound cell produced six deltas [0,23,11,17,17,17]. These prove pixel differences, but the 320×180 evidence is too small to establish readable stale text. The selected actual 128 capsule tint produced [0,55]; at the midpoint the actual capsule remains untinted while the fresh oracle shows tinted faces. Scripts excel-live-oracle.py and excel-cell-oracle.py retain the test; actual/fresh PNGs were saved to generated. All are focused tests, not complete-scene coverage. The full-hatch six-frame run was canceled after three frames due to excessive duration.

## Trees 124 path progress

Current attempt started 08:51:26 UTC. Native first/mid/last frames of current partials 17 (mtime 09:49:26) and 18 (09:53:01) show Building and then Tower A; no premature Storey 3 label or slice in those inspected points. Existing partials 19 and 20 were from 08:38, before the attempt, and were excluded; earlier tentative concern based on those files is retracted. Await current stage 3 and 4.

Current Trees partial 19 (mtime 10:04:30 UTC) first/mid/last native frames begin at Tower A, transition to Storey 3 and end with only the Storey 3 label and selected level. No premature Facade label in those frames. Stage 4 awaits current output.

Current Trees partial 20 (mtime 10:19:38 UTC), 27 frames: native first/mid/last show Storey 3 first, transition to Facade 3 (west), and final facade group and full path/index. The four sampled stages now follow Building, Tower A, Storey 3, Facade 3; muted future address is intentional. This is not full temporal coverage.

## Corrected Casting partial 26

Current 123 attempt began 09:52:36 UTC; partial 26 mtime 10:23:06 UTC. Native frames 0/15/30/45/55 show the old duplicate slider handles and panel values resolved. At frame 45 input 2.5 and output 2 are legible while the title overprints prior values (9 and 8) and prior tower highlights remain. This is a concrete remaining discrepancy in the current partial, not a final master review.
