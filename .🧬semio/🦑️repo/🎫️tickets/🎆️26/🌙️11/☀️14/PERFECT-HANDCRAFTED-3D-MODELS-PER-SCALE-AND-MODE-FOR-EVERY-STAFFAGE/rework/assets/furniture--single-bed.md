# `furniture/single-bed` — rework report

## Pass 1 (second round), 2026-11-22

Start state: source `volumes/furniture/single-bed.json` and catalog `volumes` were identical. The earlier judgement is in `rework/14.md` section 5 (measured, one rig imperfection left open). This pass closed that rig imperfection and three further defects found by rendering.

### Reference table (pine single bed 90 x 200 with adjustable slatted base, own knowledge, mm)

| Part | Real | Model 1:50 | Verdict |
|---|---|---|---|
| Mattress | 900 x 2000, 160 – 220 thick, cover 8 – 12 quilted, piped edges | 900 x 2000 x 200, cover 10, piping top and bottom, three parts 720 / 600 / 680 | right |
| Lying height | 450 – 500 | 485 | right |
| Outer size of frame | mattress + 2 x rail + post offset: about 1000 x 2100 – 2120 | 1010 x 2120 | right |
| Posts | 55 – 70 square | 60 square, eased edges, chamfered tops | right |
| Head board / foot board height | 850 – 1000 / 450 – 600 | 950 / 550, frame and V-jointed panel 18 | right |
| Side rails | 25 – 30 x 150 – 190, ledger 25 x 25 – 40, underside 120 – 200 above floor | 30 x 160, top at 300, underside 140, ledger 25 | right |
| Slatted base | frame rails 25 x 40 – 60, 28 slats 36 – 38 x 8 – 9, camber 10 – 20, pitch 65 – 70, rubber caps | rails 25 x 40, 28 slats 38 x 8, camber 15, pitch 65, caps | right |
| Head section | 650 – 750 long, pivot at the frame rail (bolt or ratchet fitting), 0 – 60/65 degrees | 720, pivot on two bolts at x = 780, y = 230, 0 – 65 degrees | right |
| Centre support | cross beam 40 x 60 – 70 with a leg (usual from 1400 wide, optional at 900) | 40 x 65 beam, 60 leg | plausible |
| Bone of the head section | from the pivot to the head end | pointed from the pivot towards the FOOT end (through the floor when raised) | **wrong, fixed** |

### Defects found and what was done

1. **Rig, all scales: `slat-head` and `mat-head` pointed the wrong way.** Both bones started at the hinge but ran towards the foot end; in `head-raised`, `reading-in-bed`, `half-raised-key` and the two raising clips the drawn bones went down through the floor, and the uniform bone masses and the occupant load span were written with negative coordinates. Fixed at the root with `rework-single-bed-head-bones.py`: `slat-head.angleDeg` 0 → 180 (u from the hinge to the head board, planar rig path kept), every coordinate in the frame of the two bones turned by the same half turn about w: solids of both bones in body, thermal and structure at 1:50, 1:100, 1:200, 1:500 (lofts, extrusion, boxes, ellipsoids, repeats), all elevation shapes and `solid` profiles in front and plan at four scales, bone masses (`comX` 0.36), the placement of `mat-head` under its parent (`y` -0.055), its prismatic travel (parent normal now points down into the slats: `axisDeg` 90, 0 – 0.06 m, pose `dy` positive = sinking; three poses), the occupant load span (0 – 0.72 m). Joint limits and the pose angles of `slat-head` are unchanged (-65 – 0, -50, -25). Proof that nothing moved: `rig-lateral-compare.ts` before / after `largest move 0.0000 mm` over every pose and clip instant; sheets of `head-raised`, `asleep`, `reading-in-bed` and `sit-up-to-read` t = 3.6 s read after the change (bones lie along the raised section, mattress sinks towards the slats).
2. **1:500 body: head and foot end were slabs standing on the floor** while the tray between them floats 140 mm above it on the centre leg and the structure list of the same scale has posts. Rebuilt as one inverted-U extrusion each (posts 60 wide, lower edge at 140, true heights 950 and 550) in `rework-single-bed-500-ends.py`; the 1:500 front drawings `head` and `foot` got the same notch. Sheet (flat and `head-raised`) and `r3-2d` 1:500 read.
3. **1:500 structure: the centre leg carried nothing** (it ended 65 mm under the slat plates; `volumes-connected` reports it now). Added the 40 x 65 mm cross beam at its true section (`slender`) on the leg between the side rails, as at the finer scales (`rework-single-bed-500-cross-beam.py`).
4. **Thermal 1:100, 1:200, 1:500: the mattress had lost its layers** (one solid without layer; the sheet showed it grey and the two-layer region was not nested). Added the cold-foam core per mattress part, set in by the 10 mm cover on the section and at the two outer ends and running through at the inner joints, and bound the outer solids to the layer `quilt` (`rework-single-bed-coarse-cores.py`). Cut sheets of 1:500 (flat) and 1:100 (`head-raised`) read: core inside cover, slats hidden, frame outside.
5. **Decisions** of 1:100, 1:200 and 1:500 rewritten where the model changed (en, de); the 1:500 body text no longer claims soft edges on the mattress blocks.

Not changed after looking: 1:50 body (flat, six directions), 1:50 structure and thermal cut, 1:100 and 1:200 body and structure, 1:200 thermal (before the core was added). They agree with the table.

### Validation

- `r3-validate.ts furniture/single-bed --volumes`: `VALID furniture/single-bed (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`
- after publishing, `r3-validate.ts furniture/single-bed --catalog-only --volumes`: `VALID furniture/single-bed (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`

Side effect: the non-volume part of the catalog file is now written with plain two-space indentation throughout (it was a mix of inline and expanded lists); content unchanged apart from the edits above.

### What remains imperfect (for the next pass)

- **All scales, body and thermal, raised poses: the head mattress part runs into the middle part at the fold.** The pivot lies 55 mm under the mattress, so at 50 degrees the joint end of the head part swings up to 195 mm into the middle part (visible in the cut thermal sheet of 1:100 `head-raised`, hidden from outside as a crease). A real foam mattress compresses there; the model has no fold solid. A clean solution needs a mitred or rounded joint end on `mat-head` and `mat-mid` (body, cover, core, 2D elevation) that still reads as one flat mattress in `flat`.
- **1:50: no ratchet stay / prop of the raised head frame** in body or structure (decisions call it hidden). In `head-raised` the lifting frame is held by the two hinge bolts alone, which is not the real load path of a raised section (ratchet fitting at the pivot or a prop onto the base rail).
- **Sheets not read in this pass after the last change:** 1:50 thermal uncut; 1:100 and 1:200 thermal uncut with the new cores (1:100 was read cut in `head-raised`, 1:500 cut in `flat`); `half-raised-key`; clip instants `raise-head-section` 1.5 / 3.6 s, `get-into-bed` 1.0 / 2.4 s, `sit-up-to-read` 1.5 s (rendered after the rig change, not read; they interpolate the poses that were read); `r3-2d` at 1:100 and 1:200 (their shapes were only turned with the bones, placement proven unchanged by `rig-lateral-compare`).
- The head mattress part is 3 mm narrower per side than the other two at every scale (447 against 450; 894 against 900 at 1:500). Left as found; it is not visible and probably keeps the side faces from coinciding when the part is raised, but it is not a real feature.

### Fundamental issues (outside this asset)

- `volumes-insert.py` runs `main()` at import (line 118), so its `dump` cannot be reused to write a catalog file in the house format; authors who change non-volume members rewrite the file with another layout.
- `r3-sheet.ts --list thermal` paints a solid of a multi-layer region that has no `layer` in grey without a legend entry; it reads like a missing region.
- `rig-lateral-compare.ts` reported `largest move 0.0000 mm` although the 1:500 front shapes `head` and `foot` changed from `rect` to a notched `poly`: it compares placement, not outline, which its summary line does not say.
