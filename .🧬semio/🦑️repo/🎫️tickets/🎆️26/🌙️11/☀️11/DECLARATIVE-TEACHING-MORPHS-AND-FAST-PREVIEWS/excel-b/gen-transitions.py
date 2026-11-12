"""Emit the transition documents of the Excel anatomy, program and energy clips and the BIM information clip from compact tables.

Usage (repo root, PYTHONPATH=repo root): python <this file>. Output is canonical (`spec.dumps`), so rerunning is byte-stable.
"""
import importlib
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
ARCH = ROOT / "🎓️teaching" / "🏛️architecture"
EXCEL = ARCH / "📊️excel" / "🎬️clip"
SCHEMA = "../../../../🔀️morph/🧬️schema/🔣️.schema.json"
INTENTS = {"a": "abstraction", "c": "concretization", "k": "continuation", "t": "transformation"}


def side(entity):
    if entity is None:
        return None
    return {"entity": entity, "revision": 1, "sampling": "snapshot", "view": None, "scale": None, "pose": None, "scenario": None}


def corr(spec_):
    kind = spec_[0]
    if kind == "recipe":
        return {"kind": "recipe", "name": spec_[1]}
    if kind == "pairs":
        return {"kind": "pairs", "pairs": spec_[1]}
    return {"kind": kind}


def row(prefix, name, intent, src, tgt, relation, recipe, duration, correspondence="order", *, arc=0.0, lag=0.0, limit=None, copy=False, dip=0.0, ease="smooth"):
    item = {"id": f"{prefix}.{name}", "intent": INTENTS[intent], "source": side(f"{prefix}.{src}" if src else None), "target": side(f"{prefix}.{tgt}" if tgt else None),
            "relation": relation, "recipe": recipe, "copy": copy,
            "timing": {"duration": duration, "easing": ease, "lag": lag, "limit": limit},
            "placement": {"space": "scene", "anchor": "center", "fit": "contain", "uniform": True},
            "final": {"pathArc": arc, "dip": dip, "ghost": 0.0}}
    if recipe not in ("vanish", "appear"):
        item["correspondence"] = corr(("order",) if correspondence == "order" else (("recipe", correspondence) if isinstance(correspondence, str) else correspondence))
    return item


def swarm(run, total):
    return {"lag": round(total - run, 6), "limit": total}


def write(folder, items):
    target = folder / "🔀️transitions" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    ids = [item["id"] for item in items]
    assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]
    target.write_text(spec.dumps({"$schema": SCHEMA, "version": 1, "transitions": items}), encoding="utf-8")
    print(target.relative_to(ROOT), len(items))


A = "excel.anatomy"
anatomy = [
    row(A, "workbook.tower-to-file", "a", "tower", "file", "one-to-one", "hull", 1.6, arc=0.4),
    row(A, "workbook.file-to-count", "a", "file", "count", "one-to-one", "replace", 1.0, copy=True),
    row(A, "sheets.capsules-to-tabs", "a", "capsules", "tabs", "many-to-many", "scatter", 2.0, "capsule-items", arc=0.5, lag=0.25, limit=4.0),
    row(A, "address.value-to-cell", "a", "dimension-value", "cell-value", "one-to-one", "replace", 1.3, arc=0.5),
    row(A, "range.values-to-cells", "a", "dimension-values", "cell-values", "many-to-many", "replace", 1.6, arc=0.5, lag=0.3),
    row(A, "display.cell-to-stored", "t", "shown", "stored", "one-to-one", "retext", 1.4, arc=0.4),
    row(A, "display.stored-to-cell", "t", "stored", "shown", "one-to-one", "retext", 1.4, arc=0.4),
    row(A, "frames.values-to-cells", "a", "reference-values", "cell-values", "many-to-many", "replace", 1.6, arc=0.5, lag=0.4),
    row(A, "hatch.total-to-floor", "c", "total", "floor", "one-to-one", "replace", 1.5, arc=0.5),
    row(A, "precedence.names-to-edges", "c", "names", "edges", "many-to-many", "replace", 1.5, arc=0.5, lag=0.5),
    row(A, "precedence.bracket-to-corner", "c", "bracket", "landed", "one-to-one", "replace", 1.4, arc=0.5),
    row(A, "function.areas-to-total", "a", "areas", "sum", "many-to-one", "gather", 1.6, "fan-in", arc=0.4, lag=0.25, limit=3.0),
    row(A, "locale.english-to-german", "t", "english", "german", "one-to-one", "retext", 1.8),
    row(A, "recalc.strip-to-chip", "a", "strip", "change", "one-to-one", "parts", 1.5, "body-whole", arc=0.5),
    row(A, "trace.precedents-to-formula", "k", "precedents", "formula-landing", "many-to-many", "replace", 1.2),
    row(A, "trace.result-to-dependents", "k", "result", "dependents", "many-to-many", "replace", 1.2),
    row(A, "ref.deleted-fall", "t", "deleted", "fallen", "many-to-many", "replace", 1.3),
    row(A, "error.code-to-capsule", "a", "code-cell", "code-capsule", "one-to-one", "replace", 1.1, arc=0.5),
    row(A, "name.wrong-to-right", "t", "wrong", "right", "one-to-one", "retext", 1.4),
    row(A, "repair.value-to-line", "c", "value-cell", "value-line", "one-to-one", "replace", 1.3, arc=0.5),
    row(A, "legend.tab-to-capsule", "c", "tab", "capsule", "one-to-one", "parts", 1.8, "item-capsule", arc=0.4),
    row(A, "legend.frames-to-swatches", "a", "frames", "swatches", "many-to-many", "replace", 0.778, copy=True, lag=0.389),
]

E = "excel.energy"
energy = [
    row(E, "sheet.capsule-to-large", "c", "tower-capsule", "capsule-large", "one-to-one", "parts", 1.6, "auto"),
    row(E, "sheet.capsule-to-tab", "a", "tower-capsule-copy", "tab", "one-to-one", "parts", 1.6, "capsule-item", dip=0.15),
    row(E, "dims.lines-to-cells", "a", "dimension-lines", "dimension-cells", "many-to-many", "scatter", 1.4, arc=0.5, lag=0.04, limit=4.0),
    row(E, "areas.faces-to-cells", "a", "faces", "area-cells", "many-to-many", "scatter", 1.4, lag=0.08, limit=3.0),
    row(E, "ua.cells-to-arrows", "c", "u-cells", "heat-arrows", "many-to-many", "scatter", 1.2, arc=0.4, lag=0.04, limit=4.0),
    row(E, "ht.arrows-to-total", "a", "heat-arrows", "ht-total", "many-to-one", "gather", 1.0, "fan-in", arc=0.4, lag=0.02, limit=3.0),
    row(E, "degreedays.shortfalls-to-sum", "a", "shortfalls", "degree-days-sum", "many-to-one", "gather", 1.0, "fan-in", arc=0.4, lag=0.02, limit=3.0),
    row(E, "mistake.b-to-item", "t", "b-cell", "b-item", "one-to-one", "parts", 2.0, "auto", arc=0.6),
    row(E, "fix.b-to-cell", "t", "b-item", "b-cell-back", "one-to-one", "parts", 2.0, "auto", arc=0.6),
    row(E, "volume.box-to-cell", "a", "volume-box", "volume-cell", "many-to-one", "gather", 1.0, "fan-in", arc=0.4, lag=0.02, limit=3.0),
    row(E, "total.ht-to-bar", "a", "ht-cell", "ht-bar", "one-to-one", "parts", 1.3, "auto"),
    row(E, "total.hv-to-bar", "a", "hv-cell", "hv-bar", "one-to-one", "parts", 1.3, "auto"),
    row(E, "load.pieces-to-arrows", "c", "pieces", "load-arrows", "many-to-many", "scatter", 1.2, arc=0.3, lag=0.08, limit=4.0),
    row(E, "annual.plan-to-floor-tile", "a", "plan-floor", "floor-tile", "one-to-one", "parts", 1.6, "auto"),
    row(E, "annual.sum-to-days-tile", "a", "degree-days-sum", "days-tile", "one-to-one", "parts", 1.6, "auto", arc=0.5),
    row(E, "annual.ht-to-tile", "a", "ht-total", "h-tile", "one-to-one", "parts", 1.6, "auto", arc=0.5),
    row(E, "energy.dots-to-counts", "a", "tower-dots", "count-dots", "many-to-many", "scatter", 1.0, arc=0.5, lag=0.03, limit=2.6),
    row(E, "compare.capsules-to-marks", "a", "capsule-copies", "bar-marks", "many-to-many", "scatter", 1.3, "capsule-items", arc=0.3, lag=0.15, limit=4.0, dip=0.15),
    row(E, "shape.faces-to-net", "a", "capsule-faces", "net-spots", "many-to-many", "scatter", 1.2, lag=0.1, limit=3.0),
    row(E, "whatif.seeds-to-dots", "c", "seed-dots", "window-dots", "many-to-many", "scatter", 1.0, arc=0.3, lag=0.012, limit=2.4),
    row(E, "recap.symbols-to-legend", "a", "symbol-starts", "legend-symbols", "many-to-many", "scatter", 1.1, arc=0.3, lag=0.06, limit=2.0),
    row(E, "recap.dots-to-goals", "a", "window-dots", "tower-goals", "many-to-many", "scatter", 1.0, arc=0.3, lag=0.012, limit=2.4),
]

P = "excel.program"
program = [
    row(P, "type.plan-to-spots", "a", "plan-shapes", "spots", "many-to-many", "scatter", 1.0, arc=0.5, **swarm(1.0, 1.8)),
    row(P, "sheet.names-to-shapes", "c", "name-cells", "fitting-shapes", "many-to-many", "scatter", 0.9, arc=-0.4, **swarm(0.9, 1.6)),
    row(P, "area.shapes-to-cells", "a", "fitting-copies", "area-cells", "many-to-many", "scatter", 0.9, arc=0.4, **swarm(0.9, 1.6)),
    row(P, "net.labels-to-cell", "a", "dimension-labels", "net-cells", "many-to-many", "scatter", 1.0, arc=0.4, **swarm(1.0, 1.2)),
    row(P, "net.cell-to-floor", "c", "net-cell", "floor-fill", "one-to-one", "parts", 1.2, "auto", arc=-0.5),
    row(P, "free.pieces-to-cell", "a", "fitting-pieces", "free-cell", "many-to-one", "gather", 0.8, "fan-in", arc=0.4, **swarm(0.8, 1.6)),
    row(P, "free.cell-to-tint", "c", "free-cell", "floor-tint", "one-to-one", "parts", 1.2, "auto", arc=-0.5),
    row(P, "volume.box-to-cell", "a", "volume-box", "volume-cell", "one-to-one", "parts", 1.2, "auto", arc=0.5),
    row(P, "table.capsules-to-dots", "a", "table-capsules", "table-dots", "many-to-many", "scatter", 1.0, "capsule-items", arc=-0.7, lag=0.12, limit=2.4, dip=0.15),
    row(P, "table.rest-to-rows", "a", "rest-capsules", "row-count", "many-to-one", "gather", 0.7, "fan-in", arc=0.4, lag=0.012, limit=1.6),
    row(P, "row.pill-to-capsule", "c", "row-pill", "row-capsule", "one-to-one", "parts", 1.3, "item-capsule", arc=-0.6, dip=0.15),
    row(P, "countif.capsules-to-count", "a", "type-capsules", "type-count", "many-to-one", "gather", 0.8, "fan-in", arc=0.4, **swarm(0.8, 1.8)),
    row(P, "rest.capsules-to-total", "a", "other-capsules", "total-count", "many-to-one", "gather", 0.7, "fan-in", arc=0.4, **swarm(0.7, 2.2)),
    row(P, "countifs.seeds-to-capsules", "c", "seed-pills", "tower-capsules", "many-to-many", "scatter", 0.9, "item-capsules", arc=-0.3, dip=0.15, **swarm(0.9, 1.8)),
    row(P, "pull.values-to-cells", "a", "mini-values", "free-cells", "many-to-many", "scatter", 1.0, arc=0.5, **swarm(1.0, 1.8)),
    row(P, "xlookup.value-to-floor", "c", "found-cell", "floor-landing", "one-to-one", "parts", 1.2, "auto", arc=-0.5),
    row(P, "sumproduct.counts-to-tiles", "a", "count-copies", "product-tiles", "many-to-many", "scatter", 0.8, arc=0.3, **swarm(0.8, 2.0)),
    row(P, "chart.tiles-to-bases", "a", "tile-strips", "chart-bases", "many-to-one", "gather", 0.8, "fan-in", arc=0.4, **swarm(0.8, 1.8)),
    row(P, "towers.capsules-to-sums", "a", "tower-capsules", "tower-sums", "many-to-one", "gather", 0.7, "fan-in", arc=0.4, **swarm(0.7, 2.0)),
    row(P, "towers.total-to-glow", "a", "total-pill", "tower-glow", "one-to-one", "parts", 1.2, "auto", arc=0.4),
    row(P, "swap.capsule-lift", "k", "capsule-out", "capsule-lifted", "one-to-one", "move", 0.7),
    row(P, "swap.capsule-settle", "k", "capsule-lifted", "capsule-back", "one-to-one", "move", 0.8),
    row(P, "update.delta-flight", "c", "delta-starts", "delta-chips", "many-to-many", "scatter", 1.1, arc=-0.4, **swarm(1.1, 1.4)),
    row(P, "check.sums-to-ok", "a", "tower-sums", "check-ok", "many-to-one", "gather", 0.9, "fan-in", arc=0.4, **swarm(0.9, 1.3)),
    row(P, "typo.dot-to-ring", "c", "typo-dot", "typo-ring", "one-to-one", "parts", 1.2, "auto", arc=-0.8),
    row(P, "fix.ring-to-ok", "t", "typo-ring", "fix-ok", "many-to-one", "gather", 0.9, "fan-in", arc=0.4, **swarm(0.9, 0.9)),
    row(P, "recap.capsules-to-count", "a", "all-capsules", "recap-count", "many-to-one", "gather", 0.6, "fan-in", arc=0.4, **swarm(0.6, 2.2)),
]

B = "bim.information"
bim = [row(B, "context.raw-to-dimension", "c", "raw-value", "dimension-label", "one-to-one", "replace", 5.0)]

if __name__ == "__main__":
    write(EXCEL / "🦴️anatomy", anatomy)
    write(EXCEL / "⚡️energy", energy)
    write(EXCEL / "📋️program", program)
    folder = ARCH / "🏗️bim" / "🎬️clip" / "📊️information"
    target = folder / "🔀️transitions" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(spec.dumps({"$schema": "../../../../🔀️morph/🧬️schema/🔣️.schema.json", "version": 1, "transitions": bim}), encoding="utf-8")
    print(target.relative_to(ROOT), len(bim))
