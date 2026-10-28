from pathlib import Path

path = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir()) / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py"
text = path.read_text(encoding="utf-8")
swaps = [
    ("        rings = _engine.VGroup(self._ring(a), self._ring(b))\n", ""),
    ("_engine.FadeIn(rings), ", ""),
    ("_engine.Indicate(plan.floor, color=GOOD, scale_factor=1.0)), lag_ratio=0.25), seconds=5.0)", "_engine.Indicate(plan.parts[\"CW-1\"], color=GOOD, scale_factor=1.0)), lag_ratio=0.25), seconds=5.0)"),
    ("_engine.Indicate(plan.bx[a[1:]], color=DATA), _engine.Indicate(plan.bx[b[1:]], color=DATA),", "_engine.Indicate(plan.parts[\"label_x:\" + a[1:]], color=DATA), _engine.Indicate(plan.parts[\"label_x:\" + b[1:]], color=DATA),"),
    ("_engine.Indicate(plan.by[a[0]], color=DATA)),\n            _engine.AnimationGroup(_engine.FadeIn(table.rows[4])", "_engine.Indicate(plan.parts[\"label_y:\" + a[0]], color=DATA)),\n            _engine.AnimationGroup(_engine.FadeIn(table.rows[4])"),
    ("        ring_a = rings[0].get_center()\n", "        ring_a = self._at(*self._xy(a))\n"),
    ("_engine.FadeOut(rings[1]),\n                                   plan.pilotis[b].animate.set_fill(STRUCT)),", "plan.parts[b][1:].animate.set_opacity(0), plan.parts[b][0].animate.set_fill(CONCRETE, 0.9)),"),
    ("        axes = (plan.axis_x[a[1:]], plan.axis_y[a[0]])\n        every = [plan.pilotis[t] for t in", "        axes = (plan.parts[\"axis_x:\" + a[1:]], plan.parts[\"axis_y:\" + a[0]])\n        every = [plan.parts[t] for t in"),
    ("_engine.Indicate(plan.floor, color=tone[\"storey\"], scale_factor=1.0)),", "_engine.FadeIn(storey_tint)),"),
    ("_engine.Indicate(plan.bx[a[1:]], color=DATA), _engine.Indicate(plan.by[a[0]], color=DATA)), lag_ratio=0.3)", "_engine.Indicate(plan.parts[\"label_x:\" + a[1:]], color=DATA), _engine.Indicate(plan.parts[\"label_y:\" + a[0]], color=DATA)), lag_ratio=0.3)"),
    (", plan.pilotis[a].animate.set_fill(DATA)), _engine.FadeIn(people)", "), _engine.FadeIn(people)"),
    ("_engine.Indicate(rings[0], color=DATA), lag_ratio=0.25)", "_engine.Indicate(plan.parts[a], color=DATA), lag_ratio=0.25)"),
    ("        axes = (plan.parts", "        box_points = [self._at(0, 0), self._at(g[\"box\"][\"length\"], 0), self._at(g[\"box\"][\"length\"], g[\"box\"][\"width\"]), self._at(0, g[\"box\"][\"width\"])]\n        storey_tint = _draft.polygon(box_points, GREEN_GROUND, 0.3)\n        axes = (plan.parts"),
    ("_engine.FadeOut(part) for part in (relations, *relations.rows, count_note, slab_shape)", "_engine.FadeOut(part) for part in (relations, *relations.rows, count_note, slab_shape, storey_tint)"),
    ("_engine.AnimationGroup(_engine.FadeIn(table.rows[3]), _engine.Indicate(plan.outline, color=GLASS, scale_factor=1.0)),", "_engine.AnimationGroup(_engine.FadeIn(props.rows[3]), _engine.ShowPassingFlash(plan.outline.copy().set_color(GLASS).set_stroke(width=5), time_width=0.7)),"),
]
for old, new in swaps:
    if old not in text:
        print("MISSING", old[:70])
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
