"""Writes the clip transition documents of the Grasshopper batch (anatomy, tower, trees) from the compact tables below.

Usage: gen_transitions.py   (no arguments; rewrites the three 🔀️transitions/🔣️.json files; ticket-local input, not a runtime dependency)
Row: (name, intent, relation, recipe, correspondence, duration, arc, dip, ghost, fit, extras)
Entities are `<series>.<name>.source` and `<series>.<name>.target` (None endpoints for vanish/appear).
"""
import importlib
import json
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
CLIPS = ROOT / "🎓️teaching" / "🏛️architecture" / "🦗️grasshopper" / "🎬️clip"
O2O, O2M, M2O, M2M = "one-to-one", "one-to-many", "many-to-one", "many-to-many"
IDENT, KEY, ORDER = {"kind": "recipe", "name": "auto"}, {"kind": "key"}, {"kind": "order"}


def recipe(name):
    return {"kind": "recipe", "name": name}


def pairs(*rows):
    return {"kind": "pairs", "pairs": [list(row) for row in rows]}


def endpoint(entity, sampling="snapshot"):
    return None if entity is None else {"entity": entity, "revision": 1, "sampling": sampling, "view": None, "scale": None, "pose": None, "scenario": None}


def row(series, name, intent, relation, kind, correspondence, duration, *, arc=0.0, dip=0.0, ghost=0.0, fit="contain", lag=0.0, limit=None, copy=False, source=True, target=True, sampling="snapshot", easing="smooth"):
    item = {
        "id": f"{series}.{name}", "intent": intent,
        "source": endpoint(f"{series}.{name}.source", sampling) if source else None,
        "target": endpoint(f"{series}.{name}.target", sampling) if target else None,
        "relation": relation, "recipe": kind, "copy": copy,
        "timing": {"duration": duration, "easing": easing, "lag": lag, "limit": limit},
        "placement": {"space": "scene", "anchor": "center", "fit": fit, "uniform": True},
        "final": {"pathArc": arc, "dip": dip, "ghost": ghost},
    }
    if correspondence is not None:
        item["correspondence"] = correspondence
    return item


def write(folder, rows):
    target = CLIPS / folder / "🔀️transitions" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    document = {"$schema": "../../../../🔀️morph/🧬️schema/🔣️.schema.json", "version": 1, "transitions": rows}
    target.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(len(rows), "rows written")


def anatomy():
    r = lambda *a, **k: row("anatomy", *a, **k)
    hop = dict(arc=0.5, dip=0.12, fit="none")
    return [
        r("script-capsule-to-chip", "abstraction", O2O, "parts", recipe("capsule-item"), 1.4, arc=0.4, dip=0.15),
        r("slider-width-hop", "continuation", O2O, "parts", IDENT, 1.4, **hop),
        r("box-width-hop", "continuation", O2O, "parts", IDENT, 1.4, **hop),
        r("box-length-hop", "continuation", O2O, "parts", IDENT, 1.4, **hop),
        r("box-height-hop", "continuation", O2O, "parts", IDENT, 1.4, **hop),
        r("box-dimension-width-to-wire", "abstraction", O2O, "parts", IDENT, 1.4, arc=0.4, dip=0.1),
        r("box-dimension-length-to-wire", "abstraction", O2O, "parts", IDENT, 1.4, arc=0.4, dip=0.1),
        r("box-dimension-height-to-wire", "abstraction", O2O, "parts", IDENT, 1.4, arc=0.4, dip=0.1),
        r("box-output-token", "continuation", O2O, "parts", IDENT, 1.4, arc=0.5, dip=0.1, fit="none"),
        r("sliders-length-hop", "continuation", O2O, "parts", IDENT, 2.0, **hop),
        r("sliders-height-hop", "continuation", O2O, "parts", IDENT, 2.0, **hop),
        r("volume-hop", "continuation", O2O, "parts", IDENT, 1.2, **hop),
        r("preview-eye-off", "transformation", O2O, "replace", IDENT, 1.2),
        r("volume-ghost", "concretization", O2O, "parts", IDENT, 1.4, arc=0.7),
        r("repair-text", "transformation", O2O, "retext", IDENT, 1.2, sampling="live"),
        r("repair-box", "transformation", O2O, "replace", IDENT, 1.2),
        r("repair-panel", "transformation", O2O, "replace", IDENT, 1.2),
        r("panel-hop", "continuation", O2O, "parts", IDENT, 1.2, **hop),
        r("params-hop", "continuation", O2O, "parts", IDENT, 1.4, **hop),
        r("toggle-true-to-window", "concretization", O2O, "parts", IDENT, 1.4, arc=0.5),
        r("valuelist-pick-hop", "continuation", O2O, "parts", IDENT, 2.4, **hop),
        r("plugin-ring-to-token", "abstraction", O2O, "parts", recipe("body-whole"), 1.6, arc=0.5, dip=0.12),
        r("item-capsule-to-token", "abstraction", O2O, "parts", recipe("capsule-item"), 1.2, arc=0.5, dip=0.15),
        r("series-capsules-to-tokens", "abstraction", M2M, "scatter", recipe("capsule-items"), 1.0, arc=0.5, dip=0.12, lag=0.1, limit=2.2),
        r("count-capsule-hops", "continuation", M2M, "scatter", ORDER, 1.6, arc=0.5, dip=0.12, lag=0.0, fit="none"),
        r("tree-capsules-to-leaves", "abstraction", M2M, "scatter", recipe("capsule-items"), 1.2, arc=0.5, dip=0.15),
        r("path-segments-to-chip", "abstraction", M2O, "gather", recipe("fan-in"), 1.0, arc=0.4, lag=0.2, limit=3.0),
        r("index-token-to-flock", "concretization", O2M, "split", recipe("fan-out"), 0.9, arc=0.3, lag=0.04, limit=2.4),
        r("legend-tool-discs", "abstraction", M2M, "scatter", ORDER, 1.3, arc=0.8, lag=0.15, limit=3.2),
        r("legend-data-discs", "abstraction", M2M, "scatter", ORDER, 1.3, arc=0.8, lag=0.15, limit=3.2),
    ]


def trees():
    r = lambda *a, **k: row("trees", *a, **k)
    many = dict(lag=0.012, limit=3.2, arc=0.3)
    rows = [
        r("flat-probe", "abstraction", O2O, "flight", IDENT, 1.4, arc=0.3),
        r("birth-tree", "abstraction", M2M, "scatter", KEY, 1.2, **many),
        r("birth-branch", "abstraction", M2M, "scatter", KEY, 1.2, arc=0.3),
        r("path-segment", "abstraction", O2O, "flight", IDENT, 1.75, arc=0.35),
        r("index-out", "abstraction", O2O, "flight", IDENT, 1.8, arc=0.3),
        r("chip-back", "concretization", O2O, "flight", IDENT, 1.8, arc=-0.3),
        r("letters", "abstraction", M2M, "scatter", KEY, 1.2, arc=0.3, lag=0.3),
        r("short-list", "transformation", M2M, "scatter", ORDER, 1.4, arc=0.3, lag=0.7),
        r("home-probe", "concretization", O2O, "flight", IDENT, 3.6, arc=0.3),
        r("head-retext", "transformation", O2O, "retext", IDENT, 1.2),
        r("thread-retext", "transformation", O2O, "retext", IDENT, 1.2),
        r("paint-mistake", "transformation", M2M, "scatter", KEY, 1.2, lag=0.012, limit=3.2),
        r("paint-result", "transformation", M2M, "scatter", KEY, 1.2, lag=0.012, limit=3.2),
        r("fan-fix", "transformation", M2M, "scatter", KEY, 1.2, arc=0.35, lag=0.012, limit=3.2),
    ]
    for name in ("flatten", "flatten-one", "graft", "graft-one", "simplify", "simplify-one", "mapper", "pertower", "flip"):
        rows.append(r(f"regroup-{name}", "transformation", M2M, "scatter", KEY, 1.2, **many))
    for index in range(3):
        rows.append(r(f"gather-storey-{index}", "abstraction", M2O, "gather", recipe("fan-in"), 1.2, arc=0.25, lag=0.012, limit=3.0))
    for index in range(2):
        rows.append(r(f"gather-tower-{index}", "abstraction", M2O, "gather", recipe("fan-in"), 1.2, arc=0.25, lag=0.012, limit=3.0))
    for term in ("tree", "branch", "path", "item", "index", "flatten", "graft", "simplify", "path-mapper", "flip-matrix"):
        rows.append(r(f"recap-{term}", "abstraction", O2O, "flight", IDENT, 1.2, arc=0.35))
    return rows




def tower():
    r = lambda *a, **k: row("tower", *a, **k)
    gather = dict(arc=0.4, lag=0.02, limit=3.0)
    split = dict(arc=0.3, lag=0.06, limit=3.0)
    rows = [
        r("note-retext", "transformation", O2O, "retext", IDENT, 1.2),
        r("count-gather", "abstraction", M2O, "gather", recipe("fan-in"), 2.0, **gather),
    ]
    for part in ("core", "capsules", "capital", "podium"):
        rows.append(r(f"parts-{part}", "abstraction", M2O, "gather", recipe("fan-in"), 2.0, **gather))
    rows += [
        r("parts-planes", "abstraction", O2O, "parts", IDENT, 2.0),
        r("core-envelope", "concretization", O2O, "parts", IDENT, 2.0, sampling="live"),
        r("shaft-assembly", "concretization", O2O, "parts", IDENT, 2.0),
        r("series-core", "continuation", O2O, "parts", IDENT, 2.0),
        r("facades-axes", "concretization", O2M, "split", recipe("fan-out"), 2.0, **split),
        r("axes-single", "abstraction", O2O, "parts", IDENT, 2.0),
        r("items-ports", "concretization", O2M, "split", recipe("fan-out"), 2.0, **split),
        r("orient-ports", "concretization", M2M, "scatter", ORDER, 2.0, arc=0.6, lag=0.04, limit=4.0),
        r("types-colour", "transformation", M2M, "scatter", KEY, 2.0, arc=0.0, lag=0.01, limit=4.0),
        r("count-total", "abstraction", M2O, "gather", recipe("fan-in"), 2.0, **gather),
        r("nowire-half", "continuation", O2O, "parts", IDENT, 3.0, arc=0.5),
        r("mistake-candidates", "concretization", M2M, "scatter", ORDER, 2.0, arc=0.4, lag=0.2, limit=4.0),
        r("fix-cull", "transformation", M2M, "scatter", ORDER, 2.0, arc=0.4, lag=0.2, limit=4.0),
        r("capital-core", "concretization", O2O, "parts", IDENT, 2.0),
        r("capital-height-tag", "abstraction", O2O, "parts", IDENT, 2.0, arc=0.5),
        r("capital-overhang-tag", "abstraction", O2O, "parts", IDENT, 2.0, arc=0.5),
        r("ontop-head", "continuation", O2O, "parts", IDENT, 2.0),
        r("ontop-storey", "abstraction", O2O, "parts", IDENT, 2.0, arc=0.5),
        r("podium-footprint", "concretization", O2O, "parts", IDENT, 2.0),
        r("why-base", "continuation", O2O, "parts", IDENT, 2.0),
        r("towers-a", "abstraction", M2O, "gather", recipe("fan-in"), 2.0, **gather),
        r("towers-b", "abstraction", M2O, "gather", recipe("fan-in"), 2.0, **gather),
        r("total-a", "concretization", O2M, "split", recipe("fan-out"), 2.0, **split),
        r("total-b", "concretization", O2M, "split", recipe("fan-out"), 2.0, **split),
        r("swap-program", "transformation", O2O, "parts", IDENT, 2.0),
    ]
    for part in ("planes", "core", "capsules", "capital", "podium"):
        rows.append(r(f"recap-{part}", "concretization", O2M, "split", recipe("fan-out"), 2.0, **split))
    return rows


if __name__ == "__main__":
    write("🦴️anatomy", anatomy())
    write("🌳️trees", trees())
    write("🗼️tower", tower())
