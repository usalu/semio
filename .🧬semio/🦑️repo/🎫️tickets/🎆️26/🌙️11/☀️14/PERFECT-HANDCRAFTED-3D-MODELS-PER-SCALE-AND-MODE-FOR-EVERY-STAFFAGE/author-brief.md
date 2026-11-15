# Author brief — handcrafted 3D models per scale and mode

You are one of 20 authors. Your assets are the lines of `T/assign/<NN>.txt` (`category/slug` after stripping the category emoji: the tools take e.g. `humans/standing-adult`; the catalog folder is `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/<emoji category>/<slug>/🔣️.json`).

Read first: `/Users/ueli/Documents/semio/CLAUDE.md`, `T/design.md` (all of it, incl. amendments B1–B4), `T/volumes-tools.md`, and the rows of your assets in `T/audit-volumes.csv` / the weaknesses named for them in `T/audit-volumes.md`. Frames and solid semantics: amendments A1–A6 of `../../☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/design.md` and the per-category `authoring-*.md` there (what the previous author of your category learnt).

## What to deliver per asset

A source file `T/volumes/<category>/<slug>.json` = `{ "volumes": { "50", "100", "200", "500" } }` in which every scale that the asset's `representations` author is an `AuthoredVolume` with handcrafted `body`, `thermal`, `structure` and bilingual `decisions` (en, de), and every unavailable scale is `{ "status": "unavailable", "denominator": N }`.

Quality bar — "perfect handcrafted" means:
- **Each scale is its own model.** 1:50 is the detailed model (start from the seed, then repair every weakness: missing bones, box-dominated organic forms, crude lateral depth, parts that float or interpenetrate wrongly). 1:100, 1:200 and 1:500 are each modelled anew against that scale's own elevation, front and plan drawings: merged masses, generalised sections, fewer but well-chosen solids — recognisably the same object, the same proportions, the same bones, never the 1:50 list with entries deleted. Smallest feature: 7.5 / 15 / 30 / 75 mm.
- **Body** matches the drawings from all three viewpoints and is right in the third dimension that no drawing shows: think about the real object (a thigh is not a cylinder, a car body has tumblehome, a crown is not a ball) and use lofts with real sections, revolves and extrudes rather than boxes where the subject is not boxy.
- **Thermal** is the asset's thermal model in space: every region has solids at every authored scale; at 1:50 multi-layer regions are nested shells per layer (B2); hidden regions are solids inside (B4); emitters are solids (B3); surface areas agree with `areaM2`. Use `{ of }` references where the thermal surface is exactly a body solid, real solids where it is not.
- **Structure** is the load path in space: member solids with real load-bearing sections along every member bone (B1) and every 2D `member` shape, joint solids at rig joints, support solids at the `structure.supports` anchors by support type (B3), compact mass solids where the model has concentrated masses. Coarser scales simplify sections and merge joints, they do not drop load-bearing members.
- **Consistency**: same bones, same proportions, same materials and tones across scales; ids stable where a part survives (`thigh-l` at every scale), unique per list.
- `decisions` per scale: what the solids stand for, what was merged or generalised at this scale and why, how thermal and structure were derived. English first, German second, real sentences.

## Loop per asset

```
cd T ; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN volumes-seed.ts <asset>
$BUN volumes-draft.ts <asset> --scale N --list …      # proposals; copy what is good, model the rest by hand
(edit T/volumes/<asset>.json — small python edit scripts named author-<NN>-*.py in T are fine)
$BUN volumes-check.ts <asset>                          # every rule, every scale must pass
$BUN volumes-render.ts <asset>                         # then READ every 🗑️generated/volumes-tools/render/<category>/<slug>.s<N>.png
```

You must look at the rendered sheet of every scale (overlays and the three axonometric renders) with the Read tool and fix what looks wrong even when the numbers pass: red/blue overlay areas, shapes that read wrong in axonometric view, shells that are not nested, members floating outside the body. A model that passes the check but looks crude is not done.

When an asset passes every rule at every scale and looks right: splice it — `python3 volumes-insert.py volumes/<asset>.json "<catalog asset file>"` (re-reads the asset, replaces `volume` by `volumes`, touches nothing else) — and go on to the next. Finish all your assets; do the hardest one first so problems surface early.

## Rules

- Do not weaken a rule, a threshold or a tool to make an asset pass. If a rule is wrong for an asset (measured, with numbers), write it into `T/author-requests.md` (append a section `## <NN> <asset> — <rule>` with the measurement and what you propose) and continue with the other assets; the coordinator answers there or by message.
- If the tools have a defect, append it to `T/tool-requests.md` the same way and work around it without editing the shared `volumes-*.ts` files (their owner fixes them).
- If the asset's 2D drawings, rig or thermal/structural data are themselves wrong, fix them at the root in the asset file (re-read right before editing; only your assets) and note it in your report.
- No git command that modifies anything. Others edit the tree live — keep going. Tool output only under `T/🗑️generated/`. Never delete others' files.
- Report: `T/author-<NN>.md` — per asset: solid counts per scale and list, check result line, what you modelled by hand, what you saw in the renders, anything left imperfect stated plainly. Return only the report path plus five lines.

## Addendum (coordinator)

- Rulings C1–C7 at the end of `T/design.md` are normative (exposed thermal area and `sectionFaces`, `slender`, tolerant cross-scale, fixed-quality monotonic, support `anchor`, lattices/`envelope`, resolution-independent measurements).
- One asset at this quality costs a large share of an agent's budget. Work asset by asset, finish and splice one completely before the next, update your report after each asset, and list the ones you did not reach as "not started" — the coordinator re-assigns them.
