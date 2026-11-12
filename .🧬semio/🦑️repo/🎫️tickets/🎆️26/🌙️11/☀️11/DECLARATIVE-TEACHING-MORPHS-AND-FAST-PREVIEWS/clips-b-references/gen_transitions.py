"""One-shot writer of the declarative transitions of the Excel references clip (Beat2_References).

Row: (name, intent, relation, recipe, correspondence, duration, lag, limit, pathArc, dip, ghost, copy). Numbers are the arguments of the former `_morph.*` calls (library defaults spelled out), so the
clip's timing stays identical: `scene.play(..., run_time=...)` still stretches every stage animation uniformly exactly like it stretched the old Swarm/Morph objects.
Run: uv run --project <python project> python gen_transitions.py (from the repository root).
"""

import importlib
import json
from pathlib import Path

spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")

AUTO, ORDER, BODY, BODY_WHOLE, FAN_IN, FAN_OUT = ("recipe", "auto"), ("order",), ("recipe", "body"), ("recipe", "body-whole"), ("recipe", "fan-in"), ("recipe", "fan-out")
A, C, T = "abstraction", "concretization", "transformation"
O2O, M2M, M2O, O2M = "one-to-one", "many-to-many", "many-to-one", "one-to-many"

ROWS = [
    ("hook-lift", A, O2O, "parts", AUTO, 1.6, 0, None, 0.5, 0, 0.25, False),
    ("pointer-faces", A, M2M, "scatter", BODY, 1.8, 0.15, 4.0, 0.5, 0.15, 0, False),
    ("pointer-edges", C, M2M, "scatter", ORDER, 1.8, 0.3, 4.0, 0.5, 0, 0, False),
    ("pointer-marks", T, M2M, "scatter", ORDER, 1.0, 0.2, 4.0, 0.4, 0, 0, False),
    ("shift-retext", T, O2O, "retext", AUTO, 1.4, 0, None, 0, 0, 0, False),
    ("bar-shift", T, O2O, "retext", AUTO, 1.4, 0, None, 0, 0, 0, False),
    ("lights-faces", A, M2M, "scatter", BODY, 1.0, 0.5, 4.4, 0.5, 0.15, 0, False),
    ("back-lift", C, O2O, "parts", AUTO, 1.8, 0, None, 0.6, 0, 0.25, False),
    ("total-gather", A, M2O, "gather", FAN_IN, 1.6, 0.12, 3.0, 0.4, 0, 0, False),
    ("f4-second", T, O2O, "retext", AUTO, 1.0, 0, None, 0, 0, 0, False),
    ("f4-third", T, O2O, "retext", AUTO, 1.0, 0, None, 0, 0, 0, False),
    ("f4-fourth", T, O2O, "retext", AUTO, 1.0, 0, None, 0, 0, 0, False),
    ("absolute-split", C, O2M, "split", FAN_OUT, 1.6, 0.4, 3.0, 0.5, 0, 0, False),
    ("absolute-losses", C, M2M, "scatter", ORDER, 0.9, 0.55, 4.8, 0.5, 0, 0, False),
    ("slide-off-first", T, O2O, "replace", AUTO, 1.0, 0, None, 0, 0, 0, True),
    ("slide-off-second", T, O2O, "replace", AUTO, 1.0, 0, None, 0, 0, 0, True),
    ("repair-retext", T, O2O, "retext", AUTO, 1.4, 0, None, 0, 0, 0, False),
    ("bar-repair", T, O2O, "retext", AUTO, 1.4, 0, None, 0, 0, 0, False),
    ("repair-losses", C, M2M, "scatter", ORDER, 2.0, 0.1, 4.0, 0.5, 0, 0, False),
    ("mixed2-sync", A, M2M, "scatter", ORDER, 1.0, 0.08, 4.0, 0.5, 0, 0, False),
    ("mixed2-labels", A, M2M, "scatter", BODY, 1.8, 0.4, 4.0, 0.5, 0, 0, False),
    ("mixed2-bars", C, M2M, "scatter", ORDER, 1.6, 0.25, 4.0, 0.4, 0, 0, False),
    ("mixed-count", A, M2O, "gather", FAN_IN, 1.8, 0.06, 3.0, 0.5, 0, 0, False),
    ("fill2d-split", C, O2M, "split", FAN_OUT, 0.8, 0.04, 3.0, 0.5, 0, 0, False),
    ("fill2d-gather", A, M2O, "gather", FAN_IN, 0.8, 0.04, 3.0, 0.5, 0, 0, False),
    ("sheet-floor", A, O2O, "parts", BODY_WHOLE, 1.6, 0, None, 0.6, 0.15, 0, False),
    ("pertype-tabs", T, M2M, "scatter", ORDER, 1.6, 0.25, 4.0, 0.5, 0, 0, False),
    ("threed-chips", T, M2M, "scatter", ORDER, 1.0, 0.15, 4.0, 0.4, 0, 0, False),
    ("threed-values", A, M2M, "scatter", ORDER, 0.9, 0.15, 4.0, 0.3, 0, 0, False),
    ("threed-sum", A, M2O, "gather", FAN_IN, 2.0, 0.2, 3.0, 0.4, 0, 0, False),
    ("span-chips", T, M2M, "scatter", ORDER, 0.8, 0.1, 4.0, 0.0, 0, 0, False),
    ("span-values", A, M2M, "scatter", ORDER, 0.9, 0.15, 4.0, 0.3, 0, 0, False),
    ("span-tab", T, O2O, "parts", AUTO, 0.9, 0, None, -0.6, 0, 0, False),
    ("span-sum", A, M2O, "gather", FAN_IN, 2.0, 0.2, 3.0, 0.4, 0, 0, False),
    ("deleted-sum", A, O2O, "parts", AUTO, 1.4, 0, None, 0.5, 0, 0, False),
    ("guard-ref", T, O2O, "parts", AUTO, 0.9, 0, None, 0.5, 0, 0, False),
    ("guard-retext", T, O2O, "retext", AUTO, 1.6, 0, None, 0, 0, 0, False),
    ("bar-guard", T, O2O, "retext", AUTO, 1.6, 0, None, 0, 0, 0, False),
    ("names-retext", T, O2O, "retext", AUTO, 1.6, 0, None, 0, 0, 0, False),
    ("bar-names", T, O2O, "retext", AUTO, 1.6, 0, None, 0, 0, 0, False),
    ("recap-terms", A, M2M, "scatter", ORDER, 2.0, 0.3, 4.0, 0.5, 0, 0, False),
]


def item(row):
    name, intent, relation, recipe, corr, duration, lag, limit, arc, dip, ghost, copy = row
    prefix = f"excel.references.{name}"
    out = {
        "id": prefix,
        "intent": intent,
        "source": {"entity": f"{prefix}.source"},
        "target": {"entity": f"{prefix}.target"},
        "relation": relation,
        "correspondence": {"kind": corr[0], **({"name": corr[1]} if len(corr) > 1 else {})},
        "recipe": recipe,
        "timing": {"duration": duration, "easing": "smooth", "lag": lag, "limit": limit},
        "placement": {"space": "scene", "anchor": "center", "fit": "contain", "uniform": True},
        "final": {"pathArc": arc, "dip": dip, "ghost": ghost},
    }
    if copy:
        out["copy"] = True
    return out


def main(root):
    clip = Path(root) / "🎓️teaching" / "🏛️architecture" / "📊️excel" / "🎬️clip" / "📍️references"
    document = {"$schema": "../../../../🔀️morph/🧬️schema/🔣️.schema.json", "version": 1, "transitions": [item(row) for row in ROWS]}
    problems = spec.validate(document)
    assert not problems, [str(p) for p in problems]
    target = clip / "🔀️transitions" / "🔣️.json"
    target.parent.mkdir(exist_ok=True)
    target.write_bytes(spec.dumps(document).encode("utf-8"))
    print(f"wrote {len(ROWS)} transitions -> {target}")


if __name__ == "__main__":
    main(Path.cwd())
