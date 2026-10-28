# Initial Read-Only Architectural Energy Clip Audit

## Scope and Evidence

Read only source inspection of all Python chapters under `🎓️teaching/🏛️architecture/⚡️energy`. No authored source modified, no render run, no git mutations, and no worktrees. Findings below distinguish mathematical defects from visual candidates requiring rendered review. Full callsite inventory is in [explorer-callsites.md](explorer-callsites.md). No nested AGENTS.md was found beneath energy; the root instructions apply.

## Architecture

The language-neutral curriculum descriptor `🎬️clip/🔣️.json` enumerates five ordered videos, chapters, and segments. Shared authoring lives under `🏗️authoring`: fonts, visuals, speech, clip schema, runtime, renderer, and contract tests. Each chapter owns its scene script and optional composition, speech, timing, and trace facets.

`🏗️authoring/🎨️visuals/🐍️.py` is the intended visual vocabulary: window_glyph 868, open_window 890, house_section 897, room_section 950, person_glyph 1026, seated_person_glyph 1057. Layout guards at 2332 check text collision and frame constraints; they do not validate building geometry or human dimensions.

Physics duplicates these implementations in `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`: window 463, house 488, room 534, person 605. Fixing only shared glyphs leaves physics inconsistent.

## Confirmed Geometry Weaknesses

1. Shared house uses arbitrary screen geometry: wall width 3.6, total two-storey height 2.4, floor height 1.2, window height .5, roof rise 1.1 (visuals 903–938). It has no metres-to-scene contract. A physically coherent example with storey 2.5 m maps 1.2 scene units to 2.5 m, so 1.75 m occupant is .84 units high (70% storey); .5-unit window is about 1.04 m high; house width becomes 7.5 m. Whether these dimensions are desired needs explicit schema, rather than scaling independent glyphs.

2. Shared window geometry scales incompletely. The house passes depth .42*s and opening height .5*s (937–938), but window_glyph reveal half-width .1 and frame clearance .04 remain fixed (876,879). At small s, frame clearance changes proportion and eventually exceeds opening. Uniformly scaling a canonical window would avoid this.

3. Shared dictionaries expose snapshots (x, center, hinge, bottom_left, etc.) alongside mutable VGroups (886,941–946). Transforming the group later does not automatically transform these NumPy anchors. Any integration that scales/repositions a completed house must update anchors or derive them from geometry.

4. Shared room is truly cut around glazing (961–966); ventilation local _room is solid walls with separate opaque dark window rectangle laid over them. See ventilation natural 104–122, need 106–120, mechanical 118, volume 262. These are structurally different drawing models, so window clipping, thickness, and opening behaviour cannot stay consistent.

5. Shared person_glyph height is normalized to .66 then independently multiplied (1053–1054). Its default position is the figure center. Seated person uses a floor anchor and an unnormalized .55-ish-unit figure (1057–1085). These anchor and dimensional conventions differ.

## Scale Review Candidates

- Cooling heating-comparison _internal_gains at 126 uses scale=1.1, giving person height .726 in 1.2-unit storey, ratio .605. The same series house uses canonical scale=1 at 96.
- Demand primary-energy _stand at 126–129 enforces .7*room_h, and _furnish supplies room_h=1.2*s at 143. This gives correct .84*s person with feet at floor.
- Ventilation natural _stand 130–134, need 123–128, volume 275–279 and mechanical 159–163 use 70% inner height; these are good behaviour to preserve centrally.
- Cooling internal-gains has seated scale=3.6 in room h=3.4 at 272–273, seated scale=2.6 in another room at 959–963, and generic standing scale=1.25 at 448. Heating internal-gains uses seated scale=3.6 in room h=3.3 at 378–379, and scale=3.2 in room h=3.2 at 462–464. These manually selected values need geometry-driven seated dimensions.
- Cooling transmission-humidity _build_house 121 accepts scale but air inset t=.1 and offsets .02,.06,.16 stay fixed (122–125); shell and inset proportions diverge across scales.
- Cooling summer-strategy _person 89–91 and ventilation natural _person 125–127 preserve historic factor 1.09. Remove arbitrary factors when physical height becomes explicit.
- Demand certificate 835–839 has house scale=.55 and people scale=1.5. These may be symbolic buyers/tenants outside a diagram rather than occupants: do not force architecture dimensions onto pictogram comparisons without inspecting intent.

## Tests and Rendering

Existing `🏗️authoring/🎬️clip/🧪️tests/🐍️.py` validates manifests, order, scheduling, cancellation, path portability, caches, UTF-8, and a real Manim segment test. It contains no person, room, window, or architectural-scale contract. Add language-neutral geometry cases plus Manim comparison checks.

Pinned environment at `🏗️authoring/📦️packages/🐍️python/pyproject.toml`: Python >=3.12,<3.13; Manim .22.0; NumPy 2.5.4; imageio-ffmpeg .6.0; test fonttools 4.63.0.

Repository entry:
```text
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video ventilation --quality l --jobs 4 --output-root <ticket>/🗑️generated/render --no-play
```

Renderer supports video selection, l/m/h quality, even pixel resolution, FPS, force, concat-only, bounded jobs, per-segment logs, and cancellation (`🖌️render/🐍️.py` 23–63). Runtime Manim interface uses active Python -m manim (84–86); FFmpeg resolves configured binary, PATH, then imageio interface (89–101). Entry script provisions uv project env with pinned Python 3.12, disables bytecode, sets UTF-8 and teaching output/cache variables, and terminates descendant process trees on cancellation.

For previews route SEMIO_TEACHING_OUTPUT_ROOT, SEMIO_TEACHING_CACHE_ROOT, and SEMIO_TEACHING_CACHE_DIR into ticket generated. Set LAYOUT_CHECK=1 for authored text/layout diagnostics. Full render output includes complete MP4 and private intermediate media under requested root.

## Tooling Observations

Native repo executable was found via `.🧬semio/🦑️repo/native-repo-path`; it is not on initial PATH. repo tools exec rg is rejected because rg is absent from registry. Used repo tools exec node for read-only traversal and reading. repo search energy returned no output for several minutes and was cancelled; targeted node traversal completed immediately.

## Suggested Integration Sequence

Central schema should declare dimensions, floor datum, room bounds, wall/slab thickness, glazing opening, sash hinge and opening state, and physical person heights. Keep symbolic icons explicitly separate from architectural inhabitants. Test model geometry first; compare projection/Manim bounds; then migrate physics duplicates and chapter-local shell/occupant helpers. Preserve narration order and inspect windows, seated/standing figures, small overview houses, and air tracks in renders.
