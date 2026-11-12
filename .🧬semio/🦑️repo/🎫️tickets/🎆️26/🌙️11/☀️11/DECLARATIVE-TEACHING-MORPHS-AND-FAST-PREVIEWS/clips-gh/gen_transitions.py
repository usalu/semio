"""Generate the capsule and casting transition documents (canonical text via spec.dumps) from one table."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "AGENTS.md").exists())
sys.path.insert(0, str(ROOT))
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
CLIP = ROOT / "🎓️teaching" / "🏛️architecture" / "🦗️grasshopper" / "🎬️clip"

AUTO = {"kind": "recipe", "name": "auto"}
FAN_IN = {"kind": "recipe", "name": "fan-in"}
FAN_OUT = {"kind": "recipe", "name": "fan-out"}
ORDER = {"kind": "order"}
PAIRS_01 = {"kind": "pairs", "pairs": [["0", "0"], ["1", "1"]]}


def row(prefix, name, intent, source, target, *, recipe="parts", relation="one-to-one", correspondence=AUTO, duration, lag=0.0, limit=None, arc=0.0, dip=0.0, copy=False):
    out = {
        "id": f"{prefix}.{name}", "intent": intent,
        "source": {"entity": f"{prefix}.{source}"}, "target": {"entity": f"{prefix}.{target}"},
        "relation": relation, "correspondence": correspondence, "recipe": recipe,
        "timing": {"duration": duration, "easing": "smooth", **({"lag": lag} if lag else {}), **({"limit": limit} if limit else {})},
        "final": {"pathArc": arc, **({"dip": dip} if dip else {})},
    }
    if copy:
        out["copy"] = True
    return out


def write(folder, rows):
    raw = {"$schema": "../../../../🔀️morph/🧬️schema/🔣️.schema.json", "version": 1, "transitions": rows}
    problems = spec.validate(raw)
    assert not problems, [str(p) for p in problems]
    target = CLIP / folder / "🔀️transitions"
    target.mkdir(exist_ok=True)
    (target / "🔣️.json").write_text(spec.dumps(raw), encoding="utf-8", newline="\n")
    print(folder, len(rows))


c = "grasshopper.capsule"
capsule = [
    row(c, "hook.capsule-to-box", "abstraction", "hook.capsule", "hook.box", duration=1.4),
    row(c, "parametric.box-to-solid", "concretization", "parametric.box-chip", "parametric.box-solid", duration=1.6),
    row(c, "parametric.width-to-line", "concretization", "parametric.width-knob", "parametric.width-line", duration=1.0),
    row(c, "sliders.dimensions-to-card", "abstraction", "sliders.dimension-lines", "sliders.card-values", recipe="scatter", relation="many-to-many", correspondence=ORDER, duration=1.2, lag=0.3, limit=2.0),
    row(c, "plane.axes-to-origin", "abstraction", "plane.base-axes", "plane.origin-panel", duration=1.6),
    row(c, "box.centre-to-position", "abstraction", "box.centre-dot", "box.centre-panel", duration=1.4),
    row(c, "lift.length-to-half", "abstraction", "lift.length-line", "lift.length-half", duration=1.4, arc=0.4),
    row(c, "lift.height-to-half", "abstraction", "lift.height-line", "lift.height-half", duration=1.4, arc=-0.4),
    row(c, "rounded.helpers-to-fillet", "abstraction", "rounded.helper-nodes", "rounded.fillet-node", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.2, lag=0.1, limit=2.2, arc=0.4),
    row(c, "rounded.box-to-shell", "transformation", "rounded.box-drawing", "rounded.shell-drawing", duration=1.6),
    row(c, "rounded.radius-to-tag", "concretization", "rounded.radius-token", "rounded.radius-tag", duration=1.4, arc=0.3),
    row(c, "frontplane.ring-to-plane", "abstraction", "frontplane.red-ring", "frontplane.plane-token", duration=1.6, arc=0.6),
    row(c, "frontplane.plane-to-ring", "concretization", "frontplane.plane-token", "frontplane.front-ring", duration=1.4),
    row(c, "cut.nodes-to-cutter", "abstraction", "cut.nodes", "cut.cutter-node", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.2, lag=0.1, limit=2.0, arc=0.4),
    row(c, "cut.ring-to-prism", "transformation", "cut.ring", "cut.prism", duration=1.4),
    row(c, "cut.prism-to-hole", "transformation", "cut.prism", "cut.hole", duration=1.4),
    row(c, "bolts.rings-to-points", "abstraction", "bolts.rings", "bolts.point-panel", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.0, lag=0.2, limit=2.4, arc=0.4),
    row(c, "domains.range-to-edge", "concretization", "domains.range-text", "domains.edge-line", duration=1.4),
    row(c, "types.minis-to-table", "abstraction", "types.minis", "types.table-cells", recipe="scatter", relation="many-to-many", correspondence=ORDER, duration=1.2, lag=0.3, limit=2.4, arc=0.5),
    row(c, "orient.capsule-to-plane-a", "abstraction", "orient.start-capsule", "orient.plane-a-token", correspondence={"kind": "recipe", "name": "capsule-item"}, duration=1.2, dip=0.15),
    row(c, "orient.plane-a-to-capsule", "concretization", "orient.plane-a-token", "orient.end-capsule", correspondence={"kind": "recipe", "name": "item-capsule"}, duration=1.4, dip=0.15),
    row(c, "rotate.panels-to-rotation", "abstraction", "rotate.panels", "rotate.rotation-node", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.2, lag=0.2, limit=2.0, arc=0.4),
    row(c, "rotate.angle-to-arc", "concretization", "rotate.angle-token", "rotate.angle-arc", duration=1.6, arc=0.4),
    row(c, "listitem.items-to-capsules", "concretization", "listitem.item-tokens", "listitem.placed-capsules", recipe="scatter", relation="many-to-many", correspondence={"kind": "recipe", "name": "capsule-items"}, duration=1.2, lag=0.5, limit=3.0, arc=0.5),
    row(c, "outside.stray-to-null", "transformation", "outside.stray-index", "outside.null-token", duration=1.0),
    row(c, "wrap.index-to-item", "transformation", "wrap.index-token", "wrap.again-item", duration=0.8),
    row(c, "recap.orient-to-legend", "abstraction", "recap.orient-born", "recap.orient-legend", duration=1.2),
    row(c, "recap.list-item-to-legend", "abstraction", "recap.list-item-born", "recap.list-item-legend", duration=1.2),
    row(c, "recap.index-to-legend", "abstraction", "recap.index-born", "recap.index-legend", duration=1.2),
]
write("💊️capsule", capsule)

k = "grasshopper.casting"
casting = [
    row(k, "types.sources-to-chips", "abstraction", "types.sources", "types.chips", recipe="scatter", relation="many-to-many", correspondence=ORDER, duration=1.4, lag=0.3, limit=4.0, arc=0.5),
    row(k, "text-number.quoted-to-chip", "transformation", "text-number.quoted", "text-number.chip", recipe="retext", duration=0.7),
    row(k, "text-number.chip-to-capsule", "concretization", "text-number.chip", "text-number.built", correspondence=PAIRS_01, duration=1.5, arc=0.4, dip=0.15),
    row(k, "storeys.slider-to-integer", "transformation", "storeys.slider-chip", "storeys.integer-chip", recipe="retext", duration=0.7),
    row(k, "storeys.integer-to-bands", "concretization", "storeys.integer-chip", "storeys.bands", recipe="split", relation="one-to-many", correspondence=FAN_OUT, duration=1.2, lag=0.15, limit=3.0, arc=-0.3),
    row(k, "ties.bands-to-dot", "abstraction", "ties.bands", "ties.dot", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.1, lag=0.05, limit=1.6, arc=0.4),
    row(k, "deliberate.storey-to-lost", "transformation", "deliberate.eleventh", "deliberate.lost", duration=1.2, arc=0.5),
    row(k, "point-plane.origin-to-chip", "abstraction", "point-plane.origin-dot", "point-plane.origin-chip", duration=1.3, arc=-0.5, copy=True),
    row(k, "wrong-way.plane-to-red", "concretization", "wrong-way.world-plane", "wrong-way.red-plane", duration=1.4, arc=0.3),
    row(k, "line-plane.line-to-axes", "transformation", "line-plane.line", "line-plane.axes", duration=1.4, arc=0.2),
    row(k, "explicit.inputs-to-plane", "abstraction", "explicit.inputs", "explicit.built-plane", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.2, lag=0.02, limit=3.0, arc=0.4),
    row(k, "domain.anchor-to-line", "concretization", "domain.anchor", "domain.number-line", duration=1.5),
    row(k, "domain.length-to-interval", "concretization", "domain.length-chip", "domain.interval", duration=1.4, arc=0.4),
    row(k, "geometry.rings-to-chip", "abstraction", "geometry.rings", "geometry.curve-chip", duration=1.4, arc=0.4),
    row(k, "geometry.body-to-chip", "abstraction", "geometry.body", "geometry.brep-chip", duration=1.4, arc=-0.4),
    row(k, "bool-int.one-to-ring", "concretization", "bool-int.one", "bool-int.ring", duration=0.9),
    row(k, "int-bool.true-to-ring", "concretization", "int-bool.true-chip", "int-bool.ring", duration=0.9),
    row(k, "locale.comma-split", "transformation", "locale.comma", "locale.pieces", recipe="split", relation="one-to-many", correspondence=FAN_OUT, duration=0.8, lag=0.1, limit=3.0, arc=0.2),
    row(k, "fail.capsule-to-null", "abstraction", "fail.ghost-capsule", "fail.blank", correspondence={"kind": "recipe", "name": "capsule-item"}, duration=1.4, arc=0.4, dip=0.15),
    row(k, "fix.type-to-chip", "transformation", "fix.type-chip", "fix.value-chip", correspondence=PAIRS_01, duration=1.2, arc=0.4),
    row(k, "fix.chip-to-capsule", "concretization", "fix.value-chip", "fix.built", correspondence=PAIRS_01, duration=1.5, arc=0.4, dip=0.15),
    row(k, "matrix-fill.chips-to-cells", "abstraction", "matrix-fill.chips", "matrix-fill.cells", recipe="fade", relation="many-to-many", correspondence=ORDER, duration=1.0, lag=0.1),
    row(k, "recap.cells-to-habit-one", "abstraction", "recap.cells-one", "recap.habit-one", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.4, lag=0.1, limit=3.0, arc=0.4),
    row(k, "recap.cells-to-habit-two", "abstraction", "recap.cells-two", "recap.habit-two", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.4, lag=0.1, limit=3.0, arc=0.4),
    row(k, "recap.cells-to-habit-three", "abstraction", "recap.cells-three", "recap.habit-three", recipe="gather", relation="many-to-one", correspondence=FAN_IN, duration=1.4, lag=0.1, limit=3.0, arc=0.4),
]
write("🫗️casting", casting)
