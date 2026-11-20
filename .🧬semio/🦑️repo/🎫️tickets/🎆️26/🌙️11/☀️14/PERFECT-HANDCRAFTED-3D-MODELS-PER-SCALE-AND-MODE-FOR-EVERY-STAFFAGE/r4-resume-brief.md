# Round 3 resume brief (2026-11-20)

You are one round-3 author for exactly ONE staffage asset (named in your prompt). Model every solid by hand; the bar is "perfect".

1. Read `round3-brief.md` in this ticket folder completely and follow it to the letter (it names every other file to read and every tool). `r3-tools.md` lists the render and validation tools; all exist.
2. The previous session was interrupted on 2026-11-20 around 04:10. Before starting, look for earlier round-3 work on your asset: `r3-<slug>-*` / `r3_<slug>_*` scripts, the report `r3-<category>-<slug>.md`, `🗑️generated/r3-<slug>/`. If it exists, RESUME it (re-run its ship chain, then continue with look-and-fix) instead of starting over. If none exists, start at step 1 of the brief.
3. Between 03:40 and 04:10 bulk passes of another agent (`living-*`, `equipment-*`, `digital-*`, `hu-*` scripts; reports `living-models.md`, `equipment-audit.md`) wrote into many catalog files (new 1:500 / 1:200 digital scales, rebinding). Compare the catalog `volumes` with `volumes/<category>/<slug>.json` and your scripts' output before the first splice; keep what is better and real, and make sure every scale the catalog now declares ends up handcrafted by you.
4. Calibration of the bar: `🗑️generated/r3-sheet/humans/standing-adult.s50.body.png` is an accepted round-3 result (80 solids, garments, hands, shoes). A 9-solid picnic table of plain bars or a toaster as one rounded box with two sticks is the sloppiness the dev rejects: real construction, real joints and fixings, real proportions, real skeleton, at every scale and in body, thermal and structure.
5. Look at your renders with the Read tool again and again; do not finish below grade A in any list at any scale. Finish with `volumes-check.ts --product` and `r3-validate.ts` green, spliced into the catalog, and the report complete with a final state line `State: DONE` (or the plain list of what is still imperfect).
6. Others edit the tree live; never run a modifying git command; never stop to ask. The machine is shared (10 cores, 16 GB): run one render or check at a time, never loops of parallel bun processes.

Return only: the report path plus at most five lines.

Addendum 05:15: `r3-sheet.ts` and `volumes-render.ts` work at digital-only scales (a scale whose 2D representation is `unavailable`); do not write your own sheet copies for that. Open every sheet you render after your last change, at every scale and list, before you write `State: DONE`.
