# Round-2 author brief (finish assets to done)

Read in this order: `/Users/ueli/Documents/semio/CLAUDE.md`; `T/author-brief.md` (incl. addendum); the amendments at the end of `T/design.md` (B1–C16 are normative); the rulings at the top of `T/author-requests.md`; `T/kit.md` if it exists (shared kit `kit-*`: use it instead of writing helpers); `T/volumes-tools.md` (its top says which rulings the check implements today); the report of the previous author of your assets (named in your prompt) and its scripts.

Your assets already have a source `T/volumes/<category>/<slug>.json` from a previous author, possibly spliced although unfinished. "Done" for an asset means all of:
1. every authored scale has handcrafted `body`, `thermal`, `structure` and real bilingual `decisions`;
2. `volumes-check.ts <asset> --product` (the catalog validator; the tool's default engine is older) is green under the rulings (where the tool lags a ruling, you name the measurement and show it passes by the ruling's own definition);
3. you opened the render sheet of EVERY authored scale with the Read tool after the last change, and what you saw is right: overlays green, axonometric body reads as the real object in the third dimension, thermal shells nested and regions complete, structure a believable load path inside the body;
4. spliced with `volumes-insert.py` after 1–3;
5. a section for the asset in your report `T/author-<NN>.md` (counts per scale and list, final check line, what you changed by hand, what you saw, anything imperfect stated plainly).

Principles learnt in round 1 (binding):
- Generator or drafter output is a starting point. Flat extruded plates for organic forms, fill capsules, unreviewed lists are not done.
- Never scale thermal copies, never shrink a region's coverage, never thicken or drop parts to hit a number. Model the object honestly; a remaining deviation is a finding you report with numbers.
- `areaM2`: C16 — never touch leaf-model regions or foliage/fur/feather/hair kinds; others only within a factor 1.5; larger discrepancies go to `T/area-review.md`. If the previous author changed values beyond that in your assets, restore the committed value (`git show HEAD:"<asset path>"` is a read-only way to see it).
- 2D drawing defects (missing shape, stray line, whole-asset extents that contradict between views, `filled: false` on a body shape) are repaired at the root in the asset and listed in the report.
- Work asset by asset; finish one before the next; update the report after each. Iterate with `--scale N`, one check at a time, no unattended background checks. Tool output only under `T/🗑️generated/author-<NN>/`.
- Do not weaken rules or shared tools. Rule problems → `T/author-requests.md`, tool defects → `T/tool-requests.md` (measurements included), then continue.
- No git command that modifies anything. Others edit the tree live; keep going. Don't stop to ask.
Return only the report path plus five lines.

- `T/kernel-traps.md` lists silent geometry traps found in current models (loft section overlaps, revolve profiles closed through the axis where a shell was meant, solids the kernel validator rejects, uniform tapers that make boxes): repair every row of your assets and say so in the report. Kernel rules now: a revolve profile whose first point equals its last is a closed loop (shell); any other profile is closed through the axis; `taperTo` is [scale at from, scale at to] about the outer ring's centroid.
- `T/catalog-check.md` (live worklist) and `T/area-decisions.md` (the area reviewer's decisions) are authoritative for what fails and which `areaM2` values stand.

- C29: `areaM2` is the region's TOTAL outer area (union of the region's own solids; covering by other regions, riders, ground or walls does not reduce it). Exposure is the pose's `exposedFraction`/`contact.fraction` and is only reported. Never tile, split or shrink thermal solids to steer the burial test.

- Git: only read commands (`git show HEAD:<path>`, `git diff`, `git log`). `git checkout`, `restore`, `stash`, `reset`, `clean` and the like are forbidden even for a single file — the working tree is shared live with 50 machines. To undo your own edit, write the previous content back with an ordinary file write.
- D2: thermal emitters are outlet patches ON the surface (nostril, mouth, exhaust tip, vent, a patch of the back), never discs floating in the air at an anchor. D3: `structure` is never the body or a copy of it — at every scale, 1:500 included, it is members, joints, supports and masses of its own; `{ of }` only where the body solid really is the member.
- Viewer outage (since about 23:10 on 14 Nov): the Read tool may fail with "PreToolUse hook did not respond" — then you cannot open render sheets. Do not claim a sheet as viewed that you could not open; retry later in your run; list every unviewed sheet in your report under "Not viewed (viewer outage)" so a reviewer can catch up; read text files with the shell meanwhile. Geometry work continues; a model is not "done" until its sheets have been seen.
