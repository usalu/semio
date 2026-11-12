# Resource, Cache and Lifecycle Audit (haiku-resource-audit, transcribed by coordinator)

Static read-only audit, 2026-11-11; costs are hypotheses until measured. Abbreviations: R = `🎓️teaching/🏛️architecture`, A = `R/⚡️energy/🏗️authoring`, SF = `A/🎨️visuals/🧍️staffage/🐍️.py`, M = old `R/🔀️morph/🐍️.py`, N = `R/🏢️nakagin/🎨️visuals/🐍️.py`, RT = `A/🎬️clip/⚙️runtime/🐍️.py`, BM = `R/🏗️bim/🎬️clip/🧱️model/🐍️.py`.

## Preview-time savings (ranked) and routing

| # | Finding | Evidence | Owner |
| --- | --- | --- | --- |
| 1 | Tower template cache (2 entries) thrashes per storey/variant, rebuilding occlusion-sorted building | N:52, N:2041-2061; tower clip 171-176, 369-370, 398, 409 | perf (clips-a) |
| 2 | n:n morphs built eagerly at call time, quadratic `_plan`, double outlines | M:426-447, 357-368, 279-293, 219-224, 257-270 | final-morph (lazy build in `begin`) |
| 3 | Occlusion ordering always full detail; vertex hashing before cache lookup; `core_assembly(detail)` ignores detail | N:1926, 1344-1356, 1508-1526; R/🏢️nakagin/🐍️.py:1135 | perf |
| 4 | BIM information clip rebuilds Pango ruler text, car pose loop and staffage group every frame (`always_redraw`) | `🏗️bim/🎬️clip/📊️information/🐍️.py:402-411, 800, 256-279, 793, 233-236, 214-218, 712` | perf |
| 5 | BIM queries derive context and serialise whole model per call, deep copies | BM:2050-2058, 2903-2906, 2924, 2943; `♻️cache/🐍️.py:203-246, 295-350` | perf |
| 6 | Segment fingerprinting reparses/hashes everything per segment, even cached | RT:421-437, 398-418, 373-376; calls 512, 539, 549, 617, 634, 642, 749, 775, 779, 783, 801 | exports |
| 7 | Staffage no LoD, `_slot_count` reads all frames, per-slot rewrite | SF:984-989, 853-858, 1120, 1142-1160 | staffage |
| 8 | PartMorph copies source and builds Dim at ghost 0; `_live` forces redraw | M:360-368, 138-139 | final-morph |
| 9 | PreviewSwap live mode copies lane points twice per frame, walks family | `🔀️morph/🖼️preview/🐍️.py:133-142, 164-172` | preview |
| 10 | Snapshot/sprite captures render full frame then crop | `🏢️nakagin/🎨️visuals/🖼️snapshot.py:12-18`, `🖼️keyed.py:46`, `🖼️sprites.py:30-33` | perf |
| 11 | Unbounded accumulators / stale lru caches | SF:675, 702-711 (`_DRAWN`), N:50 (`_TEXTS`), heating fundamentals 375-387, optimization model 27-35 (`lru_cache(None)` + deepcopy), keyed images; SF:113-142 stale digests | lifecycle |
| 12 | Silhouette/hull pure-Python per point | M:44-71 | final-morph (vectorise) |
| 13 | Unbounded disk orders/slabs; whole-building JSON (~1 GB) on store | N:252-312; BM:2062; bim cache 195-246 | lifecycle |
| 14 | Profile-scoped `Job.key`/`BoundedCache` have no production callers | `🎚️profile/🐍️.py:69-71, 114-116`, `🗃️cache/🐍️.py:12` | profiles + lifecycle |
| 15 | Fixed `.partial.mp4`, `render.cfg`, concat list names in shared media dir; structural solver `subprocess.run` bypasses process-tree kill; staffage lock waits forever | RT:338, 600, 363-369, 800, 832; `R/🧍️staffage/🏗️structure/🐍️.py:168`; SF:546-553, 579-597 | lifecycle |

## Contamination / correctness risks

1. Final morphs of groups above `HEAVY=200` fly bounding rectangles (`proxy`) — a cheap substitution labelled final (M:246-270, 308-310, 364). Owner: final-morph.
2. Unbounded disk growth (#13). Owner: lifecycle.
3. New stage: `Subject` lacks frozen part bounds, so preview placement measures the resolved LoD part (`🎭️stage/🐍️.py:24-34`, `🖼️preview/🐍️.py:40-45`). Owner: schema + preview.
4. Stale process caches (#11).
5. PreviewSwap restores source only in `finish()`, not in `clean_up_from_scene`; exception leaves source suspended (`🖼️preview/🐍️.py:196-222`). Owner: preview.
6. Stage reads fps from `manim.config.frame_rate`; profile fps unused → two sources of truth (`🎭️stage/🐍️.py:195`). Owner: profiles (Job sets config; stage reads Job; assert equal).
7. Runtime env (RT:534-537) lacks `SEMIO_TEACHING_PROFILE`. Owner: profiles.
8. `BoundedCache` default `sizeof=len` does not bound bytes. Owner: profiles.

## Lifecycle that is already sound

Exporter process tree (job object / session group, parent watchdog, 16 writes in flight), `ToolSession`/`execute_jobs` cancellation, browser calls via session. Gaps: `.narrated.mp4` left on failure (RT:491-498), interrupted morph leaves `_live` updaters, global patches (SF:1060-1084 scene clock, fonts `text2svg`/`Text.__init__`), import-time work (font registration, 60-step solar bisection in heating calculation 138-147).

## Existing cheap levers

`tower_axon(segments=…)`, `capsule_assembly(detail="exterior")`, staffage `window=`, exporter frame thinning + SHA-256 dedup, Manim TeX SVG cache (`tex_file_writing.py:60-63`; per-source media dirs may recompile the same formulas — unverified).
