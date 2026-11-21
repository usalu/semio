# Round 4 Review-and-Fix Brief (2026-11-20)

You are an independent art director and engineer for a small group of staffage assets (named in your prompt). Their round-3 authors reported them finished; the dev still finds the 3D staffage sloppy (wrong proportions, missing detail, wrong skeletons). Green validators prove nothing. Your eyes decide, and you FIX what you find.

Read first: `round3-brief.md` (the bar and all rules, ruling E1), `r3-tools.md`, `r4-resume-brief.md`, the asset's report `r3-<category>-<slug>.md` (its "still imperfect" and "not looked at" lines are your first worklist), and the rows about your assets in `r4-queue.md`.

Per asset, in this order:
1. Render and OPEN with the Read tool: `r3-sheet.ts <asset> --scale N` for N in 50, 100, 200, 500 in `body`; `--list structure` and `--list thermal --cut` at every scale; one non-default pose or clip frame at 1:50; `r3-2d.ts` at every scale that has a drawing. Use `--only <view>` to zoom when a detail is unclear.
2. Grade each scale and list A to D against the real subject: proportions against real reference numbers (state them), anatomy or construction, the skeleton or load path in `structure` (living: spine dorsal under the topline with real curvature, ribcage, scapulae, pelvis, long bones with joints at the real joints; things: the real frame, never the body or a block), thermal regions following the body, coarse scales authored for their scale (no blobs, no ragged patch seams, no stacked segments), no floating, interpenetrating or missing parts, poses staying closed.
3. Everything below A: fix it yourself at the root with the asset's own `r3-<slug>-*` scripts (extend them or add `r4-<slug>-*` scripts; every section stated by hand), including rig, dimensions and 2D where they are the cause. Re-render, look again, repeat until A. Then `volumes-check.ts <asset> --product` and `r3-validate.ts <asset> --volumes` green, spliced into the catalog.
4. Area values (`areaM2`) stay with the area rules C16, C18, C19, C27, C29; a red area measurement that the model honestly contradicts is listed in `area-review.md` with the hand sum, not forced.
5. Append to the asset's report a section `## Round 4 review` with: grades before and after per scale and list, what you changed, what you saw, and what is still not A with the reason. Add one line per asset to `r4-review.md` (`<asset> | before | after | open points`); append only, re-read before writing.

Rules: touch only your assets; others edit the tree live, re-read a file right before editing it; never run a modifying git command; never stop to ask; one render or check at a time (shared 10-core machine); tool output only under `🗑️generated/`; no comments inside definitions; docstrings start with a unique emoji. Never claim to have seen an image you did not open.

Return only: one line per asset (`<asset>: before -> after, open points`).

Addendum 16:55: `r3-validate.ts` now also reports `PRINT` lines (print suitability and valid-range ends, see the end of `r3-tools.md`). They must be clean for your assets too; fix the drawing or the declared range at the root.
