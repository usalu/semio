# Morph Toolkit Report (MORPH-TOOLKIT)

Status: ready. API documented in `api.md` section 7.

- Module `A/🔀️morph/🐍️.py`: `silhouette`, `proxy`, `morph` (class `Morph`), `scatter`, `gather`, `split`, `unfold` (class `Swarm`), `token` (item, cell, path, number, plane, text), `travel`, `tether`, `retext`.
- Look: a multi-part source fades its details (first 40 %) while one hull silhouette in its mean colour flows along the path arc into the target; a multi-part target is revealed in the last 50 % while the silhouette fades. Single shapes transform directly. Ghost leaves a faint copy (`anim.ghost_mobject`).
- Traps found: (1) Cairo caches "static" mobjects; fading ghosts and arriving targets carry a no-op updater so they are redrawn. (2) An `AnimationGroup` leaves its wrapper Group in the scene; `Swarm` dissolves it so only the targets remain. (3) `Scene(skip_animations=True)` is the way to skip rendering in tests (the config key alone is ignored in 0.22).
- Tests `A/🔀️morph/🧪️tests/🐍️.py`: 25 tests OK in about 16 s (hull equals SciPy ConvexHull, exact scene contents after every helper, ghost opacity, colours, point-count robustness over 6 sources x 6 tokens, 140-source scatter builds in about 0.15 s and is capped at 4 s). Wired into `📜️script.ts` (byte-level, 1 line) via `morph-toolkit/wire_script.py`.
- `source_files()` of both ClipScenes now list the morph module.
- Showcase: `morph-toolkit/showcase.py [chain cell gather scatter retext]` writes quarter frames to `🗑️generated/morph-toolkit/`; `montage.py` builds contact sheets. Reviewed: capsule to leaf to path chip and back, cell and face, 20 squares to number and back, 12-capsule scatter, tether, retext.
- Part-wise mode (`pairs=`, `pairs_fn=` in scatter, `dip=`): classes `PartMorph`, `Flight`, `Dim`; `item` tokens now carry a faint ring (`[ring, dot, text]`) as the outline partner. Showcase chain is part-wise; 30 tests OK.
- Robustness (empty or degenerate parts) and `capsule_pairs(group, token)`; 32 tests OK.
- Default is now automatic part matching (no hull blobs; `mode="hull"` opts in): shrink/grow of unmatched parts, per-part proxies above 200 shapes; 37 tests OK; showcase re-rendered.
- Cost: a 140-source scatter costs roughly 0.1 s per frame in Python; use `proxy`/per-capsule silhouettes, never raw towers.
