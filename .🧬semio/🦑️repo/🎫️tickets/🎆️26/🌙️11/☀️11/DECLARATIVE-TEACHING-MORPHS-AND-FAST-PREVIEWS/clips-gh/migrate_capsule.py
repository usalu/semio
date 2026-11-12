"""Replace the toolkit morph calls of the capsule clip by declarative `hop(...)` transitions (one-time hand migration; kept as the audit trail)."""
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "AGENTS.md").exists())
FILE = ROOT / "🎓️teaching" / "🏛️architecture" / "🦗️grasshopper" / "🎬️clip" / "💊️capsule" / "🐍️.py"
text = FILE.read_text(encoding="utf-8", newline="")

PAIRS = [
    ('INK, MUTED, GOOD, WARN, DATA, SERV = (getattr(_visuals, name)',
     'TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\nRECAP = {5: "orient", 6: "list-item", 7: "index"}\nINK, MUTED, GOOD, WARN, DATA, SERV = (getattr(_visuals, name)'),
    ('        d = self._d\n        types = _nk.capsule_types()',
     '        d = self._d\n        morphs = _morph.stage(self, TRANSITIONS)\n        hop = lambda name, source, target: morphs.transition(f"grasshopper.capsule.{name}", source=source, target=target)\n        types = _nk.capsule_types()'),
    ('_morph.morph(chosen, center, run_time=1.4)', 'hop("hook.capsule-to-box", chosen, center)'),
    ('_morph.morph(hold["chip"], hold["solid"], run_time=1.6)', 'hop("parametric.box-to-solid", hold["chip"], hold["solid"])'),
    ('_morph.morph(knob, hold["line"], run_time=1.0)', 'hop("parametric.width-to-line", knob, hold["line"])'),
    ('_morph.scatter(hold["src"], hold["dst"], lag=0.3, run_time=1.2, limit=2.0)', 'hop("sliders.dimensions-to-card", hold["src"], hold["dst"])'),
    ('_morph.morph(base_axes, origin_panel, run_time=1.6)', 'hop("plane.axes-to-origin", base_axes, origin_panel)'),
    ('_morph.morph(centre_dot, centre_anchor, run_time=1.4)', 'hop("box.centre-to-position", centre_dot, centre_anchor)'),
    ('_morph.morph(half_lines[0], half_tokens[0], path_arc=0.4, run_time=1.4)', 'hop("lift.length-to-half", half_lines[0], half_tokens[0])'),
    ('_morph.morph(half_lines[1], half_tokens[1], path_arc=-0.4, run_time=1.4)', 'hop("lift.height-to-half", half_lines[1], half_tokens[1])'),
    ('_morph.gather(helper_nodes, fil["fil"].view, run_time=1.2, lag=0.1, limit=2.2)', 'hop("rounded.helpers-to-fillet", helper_nodes, fil["fil"].view)'),
    ('_morph.morph(hold["box"], hold["shell"], run_time=1.6)', 'hop("rounded.box-to-shell", hold["box"], hold["shell"])'),
    ('_morph.morph(radius_token, hold["tag"], path_arc=0.3, run_time=1.4)', 'hop("rounded.radius-to-tag", radius_token, hold["tag"])'),
    ('_morph.morph(red_ring, plane_token, path_arc=0.6, run_time=1.6)', 'hop("frontplane.ring-to-plane", red_ring, plane_token)'),
    ('_morph.morph(plane_token, front_ring, run_time=1.4)', 'hop("frontplane.plane-to-ring", plane_token, front_ring)'),
    ('_morph.gather(cur_nodes, cut["cut"].view, run_time=1.2, lag=0.1, limit=2.0)', 'hop("cut.nodes-to-cutter", cur_nodes, cut["cut"].view)'),
    ('_morph.morph(hold["ring"], hold["prism"], run_time=1.4)', 'hop("cut.ring-to-prism", hold["ring"], hold["prism"])'),
    ('_morph.morph(hold["prism"], hold["hole"], run_time=1.4)', 'hop("cut.prism-to-hole", hold["prism"], hold["hole"])'),
    ('_morph.gather(hold["rings"], point_panel, run_time=1.0, lag=0.2, limit=2.4)', 'hop("bolts.rings-to-points", hold["rings"], point_panel)'),
    ('_morph.morph(hold["range"], hold["edge"], run_time=1.4)', 'hop("domains.range-to-edge", hold["range"], hold["edge"])'),
    ('_morph.scatter(hold["src"], hold["dst"], lag=0.3, run_time=1.2, path_arc=0.5, limit=2.4)', 'hop("types.minis-to-table", hold["src"], hold["dst"])'),
    ('_morph.morph(start_capsule, plane_a_token, pairs=_morph.capsule_pairs(start_capsule, plane_a_token), dip=0.15, run_time=1.2)', 'hop("orient.capsule-to-plane-a", start_capsule, plane_a_token)'),
    ('_morph.morph(plane_a_token, end_capsule, pairs=[(t, s) for s, t in _morph.capsule_pairs(end_capsule, plane_a_token)], dip=0.15, run_time=1.4)', 'hop("orient.plane-a-to-capsule", plane_a_token, end_capsule)'),
    ('_morph.gather([move_panel, apl["pa"].view, apl["pb"].view], spin["pr"].view, run_time=1.2, lag=0.2, limit=2.0)', 'hop("rotate.panels-to-rotation", [move_panel, apl["pa"].view, apl["pb"].view], spin["pr"].view)'),
    ('_morph.morph(angle_token, hold["arc"], path_arc=0.4, run_time=1.6)', 'hop("rotate.angle-to-arc", angle_token, hold["arc"])'),
    ('_morph.scatter(item_tokens, placed, lag=0.5, run_time=1.2, path_arc=0.5, limit=3.0, pairs_fn=_morph.capsule_pairs)', 'hop("listitem.items-to-capsules", item_tokens, placed)'),
    ('_morph.morph(stray, nothing, run_time=1.0)', 'hop("outside.stray-to-null", stray, nothing)'),
    ('_morph.morph(index_token, again, run_time=0.8)', 'hop("wrap.index-to-item", index_token, again)'),
    ('*[_morph.morph(born[i], legend[i], run_time=1.2) for i in born]', '*[hop(f"recap.{RECAP[i]}-to-legend", born[i], legend[i]) for i in born]'),
]
for old, new in PAIRS:
    if text.count(old) != 1:
        sys.exit(f"expected exactly one occurrence of: {old[:80]} (found {text.count(old)})")
    text = text.replace(old, new)
FILE.write_text(text, encoding="utf-8", newline="")
print("migrated", len(PAIRS), "replacements")
