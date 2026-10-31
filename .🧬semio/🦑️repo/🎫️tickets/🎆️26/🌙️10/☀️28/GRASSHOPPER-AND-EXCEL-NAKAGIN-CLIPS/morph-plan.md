# Morph Plan

## Request

"The videos are still extremely unsophisticated. Make sure to perfectly use morphs for explaining ideas. Always morph between abstract and concrete (e.g. the capsules turn into the data tree item, certain elements morph into the abstract concept or data point, etc.)"

## Principle

Every concept is born from the building and returns to it. The viewer sees the concrete thing (a capsule, a face, a storey, a core, a dimension) **physically transform** into its abstract stand-in (a tree leaf, a list item, a cell, a number, a path segment, a token on a wire) and, later, the abstract result transform back into the building (a computed list becomes placed capsules, a count becomes lit capsules, a cell value becomes a hatched face). No cut, no fade-swap where a morph can show identity: the same object changes form, so the viewer knows the abstract item *is* that capsule.

Each video has 8–14 **morph beats**, both directions at least three times each, chained where possible (capsule → leaf → path chip → back to capsule). Morphs are readable: 1.0–2.0 s, one idea each, a faint ghost or leader keeps the concrete origin visible while the abstract form is used, colours carry over (the token keeps the capsule's type colour), and many-to-many morphs are staggered (LaggedStart) so the eye can follow.

## Shared Morph Toolkit — `🎓️teaching/🏛️architecture/🔀️morph/🐍️.py` (owner MORPH-TOOLKIT)

Domain-neutral helpers used by both series (Manim only, no runtime deps), with tests in `🔀️morph/🧪️tests/🐍️.py`:

```python
silhouette(mob, *, color=None) -> VMobject            # one outline for a multi-face group (e.g. a detailed capsule) as morph source
proxy(mob) -> VMobject                                # cheap stand-in (bounding quad/outline) for heavy groups (towers with thousands of faces)
morph(src, dst, *, ghost=0.0, path_arc=0.0, run_time=1.4, keep_source=False) -> Animation   # concrete ↔ abstract, identity-preserving
scatter(srcs, dsts, *, lag=0.04, ghost=0.0, path_arc=0.6) -> Animation   # many → many (140 capsules → 140 leaves) in order
gather(srcs, dst, *, lag=0.02) -> Animation           # many → one (capsules collapse into a count / a list)
split(src, dsts, *, lag=0.06) -> Animation            # one → many (a list sprouts branches, a value fills cells)
token(label, *, kind="item|cell|path|number|plane|text", color, size) -> VGroup   # abstract stand-ins with consistent look
travel(token, path, *, run_time) -> Animation         # a token runs along a wire or leader
tether(a, b, *, color, dashed=True) -> Mobject        # live leader that follows both ends while they move (always_redraw)
unfold(faces, layout) -> Animation                    # a box's faces unfold into a flat net / into rows
retext(a, b) -> Animation                             # formula/text token morph (matching glyph shapes)
```

Performance: towers have thousands of faces; morph from `proxy()` or per-capsule `silhouette()`, never from the full face set.

## Per Video

Each MORPH agent writes `T/morph-<series>-<topic>.md` (list of morph beats: clause, concrete thing, abstract thing, direction, how it reads, which values carry over), then implements them in the scene file, keeping everything else. Seed ideas:

| Video | Seed morph beats |
| --- | --- |
| GH anatomy | slider knob value → dimension line on the capsule; the capsule → Box output token on the wire; panel lines ← storey capsules; stacked capsules collapse into the Series list; list items sprout into tree branches |
| GH capsule | numbers in sliders → dimension lines; box → rounded box → capsule (shape morph); capsule edges → plane axes; table rows ↔ the four type capsules; Orient as the capsule morphing from base plane to target plane |
| GH tower | core storeys → Series list items → planes on the facades; plane tokens → oriented capsules; capsules → count number and back to lit capsules; the extra two capsules → crossed tokens culled |
| GH casting | the token on the wire changes form (text glyph → number dot → integer square → boolean switch); point dot → plane square on the facade; circle → curve; box → brep shell |
| GH trees | capsules fly into tree leaves; facade/storey/tower highlights assemble the path chip segment by segment; Flatten = leaves fall into one list; Graft = items sprout branches; Path Mapper = branches merge; results fly back onto the tower |
| XL anatomy | capsule width dimension → cell B2 value; floor rectangle → area cell; capsule types → sheet tabs; formula tokens ↔ capsule edges; recalculated value → resized drawing |
| XL references | reference frame ↔ capsule face; filled formula rows ↔ faces lighting; type sheets stack ↔ type capsules (3D reference); deleted sheet → broken reference shards |
| XL program | capsules → table rows (140 rows flowing into the table); COUNTIF collapses counted capsules into a number; bars grow from stacked capsules; fittings in the plan → rows on the type sheet |
| XL energy | capsule unfolds into its net → surface rows; heat arrows → U·A cells → H_T; degree-day area → one number; per-type demand → building total → tower glow |

## Coordination

A codex session is remodelling the Nakagin drawings and model at the same time (measured layout: tower A 76, tower B 64 capsules, branches with one or two items) and also edits the Grasshopper tower and tree scenes and transcripts. Rules: re-read a file right before each edit, minimal Edit-tool diffs, never revert foreign changes, take every number from the model (the counts changed), do not edit `🏢️nakagin/**`; use its public drawing API (`capsules[(b,t,s,f,i)]` groups etc.) as it is now on disk.

## Verification

`T/capture_morph.py <series> <topic> <SceneClass> <lang> [keys]` saves the midpoint of every animation (`<key>-<n>-mid.png`) and each clause end (`<key>-end.png`) in `T/🗑️generated/morph/…`. Morph agents look at the mid frames to judge each morph; Haiku audits check that each listed morph beat is visible and identity-preserving.
