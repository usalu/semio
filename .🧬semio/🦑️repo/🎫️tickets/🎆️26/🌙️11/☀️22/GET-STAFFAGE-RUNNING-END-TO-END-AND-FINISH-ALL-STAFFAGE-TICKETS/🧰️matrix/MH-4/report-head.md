# 🧍️ Multiview Fixer MH-4: Twelve People

Fixer report of 2026-11-24. Batch `🧰️matrix/batches/MH-4.tsv` (12 assets of `humans`). Paths as in `📋️multiview-brief.md` (`S`, `T`, `U`); scripts in `U/🧰️matrix/MH-4/`, the recipe and the base tools are those of `multiview-MR.md` (`U/🧰️matrix/MR/`).

## 1. How it was measured and repaired

- **Before**: `gate.ts <id> --twins --soft` (the gate of MR with the worst subject per bone) over every pose, clip instant and transition of every authored scale of the 12 documents as they were at 13:39 (backups). The numbers of section 3 are these, not those of the batch file.
- **Repair**: one ops file per asset, `🧰️matrix/MH-4/ops/<id>.json`, applied by `🧰️matrix/MH-4/fit.ts` (the `fit.ts` of MR copied, its ops unchanged, with the ops this batch needed in addition, section 2). It is the last step of the chain of each asset; a second run changes nothing (`apply.sh` proves it: "0 changed something").
- **Dry run before every write** (`try.sh`, `do.sh`): the ops on a copy of the backup, what moved in a pose without any bone offset (`rest.ts`), the gate over the worst subjects and then over every subject, the silhouette guard, `validateAsset` and the three-views sweep of the copy against the backup (`measure.ts` of MR), and the authors' scan of the copy (`scan-file.ts`: `scanAsset` of `T/🔎️scan.ts` on a file, for the codes the gate does not measure: `paper-*`, `derived-view-*`, `unavailable-*`, the cue).
- **Where a repair moved the silhouette guard out of its limit**, `bisect.sh` measured the guard with the ops of one shape at a time; the op that did it was replaced by another construction or withdrawn, and the finding it was meant for is stated as open (section 4). No guard, test or tolerance was touched.
- **Proof** (`finish.sh`, `prove.sh` of MR): written file equal to the proven copy, gate, official scan, silhouette, `validateAsset`, three-views sweep, check CLI, the six suites with the name filter, the matrix of the asset. The sheets of the default pose at every authored scale were rendered before and after (`look.sh` of MR; `zoom.sh` draws a coarse band at 1:50 so that it can be read).
- One tool process at a time was kept for the long runs through a lock (`lock.sh`); short probes (`probe.ts`, `fit.ts` on a copy, `measure.ts` of one scale, `bisect.sh`) ran beside a matrix of the batch from 20:40 on, so at times two processes of this fixer.

## 2. Ops added to the recipe of MR (`🧰️matrix/MH-4/fit.ts`, header)

| Op or option | What it does | Why the people of this batch needed it |
|---|---|---|
| `prism` | a flat front or plan shape becomes an `extruded` solid that keeps its own outline: swept along the bone axis that points at the viewer at rest (`v` front of an upright limb, `u` its plan), span fitted to the elevation twin | trunk parts and heads: `front-sweep` exists for the front only, the plan had `plan-revolve` (a body of revolution, wrong for a coat) or `extrude` (a rectangle) |
| `prism` with `"refit": true` | refits the span of an existing prism along `u` to the elevation outline | `person-stretching`: plan prisms of an earlier round whose span was the full length of the part |
| `elevation-solid` (`respan`, `from`, `inset`) | a flat elevation shape (or a paper-thin solid) becomes the side outline swept over the width of the front or plan twin | the op `multiview-MR.md` 4.6 says is missing; `picnicker-sitting-on-ground` (feet that were three sheets of paper) |
| `extrude` with `"taper": false`, `"across": "front"` | the lateral span is the range the flat shape had, or the footprint of the front twin | feet (the fitted taper made a boot 14 mm narrower at rest), trunks that lean |
| `plan-revolve` with `"rebuild"` on a revolved twin, `"radius": "keep"`, `"reach": t` | mean radius of two half profiles; length only; part of the way to the elevation length | arms drawn as half profiles (0.20 m deep, 0.10 m wide), sleeves whose shoulder cap is hidden in the coat |
| `front-sweep` with `"view": "plan"` | refit of a plan prism along `v` | feet and heads drawn so |
| `mirror` | a half profile of a body of revolution becomes the whole body in the views named | forearms and hands that point at the front viewer (the meridian sheet is seen on edge, 8 mm high) |
| `copy` with `"create": "only"`, `"fromId"`, `"fromScale"`, `"as"` | adds a missing twin once | a letter, an umbrella shaft, a sole, the front hair of one band |
| `loft-thickness` | raises the half axes of a loft of the volumes to a minimum | `picnicker-sitting-on-ground` (section 3) |
| `despike`, `unlevel` (inside `front-sweep` and `prism`) | no sliver where the outline ran along the cut; no profile edge exactly along the bone | the union of the engine throws on both (`pregnant-person`; `U/requests.md`) |

Other tools of the folder: `auto.py` (first draft of the ops from a gate log by bone kind), `plan.ts` (every flat twin on a turning bone; not used for a write: prisms on bones that stand askew at rest move the rest drawing, 15 to 41 mm on a torso and neck), `probe.ts` (one subject: the shape at each end of each view, the shapes of a bone in three views), `culprit.ts` (which changed solid makes a subject throw), `suit.ts` (which shape violates a print profile), `frames.ts`, `singles.py`, `show.py`, `codes.py`, `worst.py`, `peek.sh`, `status.sh`, `short.sh`, `done.sh`, `report.py`.

What the batch showed beyond the recipe of MR:

- **Whole-figure extents are hard too.** A part that has no twin in a view, or sits on a bone with several shapes, fails `sync-forward` or `sync-lateral` as soon as it is the outermost part in a pose (a letter with an elevation shape only, an umbrella shaft without a plan shape at 1:500, a sole without a plan shape, a hem, a fill of an earlier silhouette round that covered the place of an arm).
- **A rectangle is not accepted by the silhouette guard at 1:50.** `extrude` (the elevation outline swept laterally) is exact for the gate and draws the hull of the sweep; for a foot at 1:100 and coarser it passes, for a round back or a hand at 1:50 it does not (section 4).
- **Painting order.** A solid is ranked by the plane of its profile; a front thigh that becomes a solid can come to lie over or under the hips and leave a sliver of them visible (`person-with-guide-dog` 1:200: `unavailable-scale-unsuitable`, feature 0.188 mm). The scan of the copy finds it, the gate does not.
- **The paper hacks of earlier rounds hide other findings.** A drawn shape 0.8 mm thick lowers the limit of `volumes-feature` for its bone (`picnicker-sitting-on-ground`).

## 3. Per asset

