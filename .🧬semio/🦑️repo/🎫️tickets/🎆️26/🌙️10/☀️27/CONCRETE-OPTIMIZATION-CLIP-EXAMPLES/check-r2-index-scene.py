"""✅ Check that every equation the index scene prints holds exactly as printed, for all six variants and both weight sets (run with the teaching Python)."""

import importlib
import sys
from decimal import ROUND_HALF_UP, Decimal
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
scene = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.⚖️index.🐍️")
room = scene._room
model = scene._model
bad = []


def r(value, digits):
    return float(round(value, digits))


def check(ok, label):
    print(("OK      " if ok else "MISMATCH"), label)
    if not ok:
        bad.append(label)


for item in model.variants(True):
    found, name = room.result(item["design"]), item["id"]
    digits = scene.energy_digits(found)
    shown = room.apportion([found[k] for k, _ in scene.ENERGY], digits)
    total = r(sum(shown), digits)
    check(r(total / scene.FLOOR_AREA, 2) == r(found["intensity"], 2), f"{name} total {total} ÷ 48 = intensity {found['intensity']:.3f}")
    check(abs(total - r(found["electricity"], digits)) < 1e-9, f"{name} parts add up to the printed total at {digits} digits")
    for k, factor in enumerate((room.systems()["cop"], room.systems()["eer"])):
        key = scene.ENERGY[k][0]
        precision, sign = scene.reproduce(lambda v: v / factor, [found[key + "_demand"]], shown[k], digits)
        check(sign == "=", f"{name} {key}: demand at {precision} digits ÷ {factor} = {shown[k]}")
    power = room.constant("lighting", "power_density") * scene.FLOOR_AREA
    precision, sign = scene.reproduce(lambda v: power * room.constant("lighting", "hours") / 1000 * v / 100, [100 * (1 - found["autonomy"])], shown[2], digits)
    check(sign == "=", f"{name} lighting share printed at {precision} digits gives {shown[2]}")
    price = lambda n: room.constant("prices", n)
    raws = [price("fixed"), found["wall_area"] * price("wall"), found["wall_area"] * item["design"]["insulation"] * 100 * price("insulation_per_cm"), found["window_area"] * price("window"), 8 * item["design"]["overhang"] * price("overhang")]
    check(sum(room.apportion(raws, 0)) == r(found["cost"], 0), f"{name} cost parts {room.apportion(raws, 0)} add up to {found['cost']:.1f}")
    factor = lambda n: room.constant("carbon", n)
    raws = [found["wall_area"] * factor("wall"), found["wall_area"] * item["design"]["insulation"] * 100 * factor("insulation_per_cm"), found["window_area"] * factor("window"), 8 * item["design"]["overhang"] * factor("overhang")]
    check(sum(room.apportion(raws, 0)) == r(found["carbon"], 0), f"{name} carbon parts {room.apportion(raws, 0)} add up to {found['carbon']:.1f}")
    const = lambda n: room.constant("daylight", n)
    depth = item["design"]["overhang"]
    theta = const("sky_angle") * (1 - const("sky_angle_loss") * depth ** 2 / (depth ** 2 + const("overhang_half_depth") ** 2))
    area = r(found["window_area"], 1)
    precision, sign = scene.reproduce(lambda a: const("transmittance") * area * a / (const("surface_area") * (1 - const("reflectance") ** 2)), [theta], found["daylight"], 2)
    check(sign == "=", f"{name} DF from theta printed at {precision} digits = {found['daylight']:.3f}")

study = model.load_study()["weights"]
for label, weights in (("balanced", tuple(study["balanced"][c] for c in scene.ORDER)), ("cost-heavy", tuple(model.reweigh(study["balanced"], "cost", .6)[c] for c in scene.ORDER))):
    for identifier in scene.ids():
        printed_w, norm, parts, digits = scene.Beat4_NormalizedIndex.index_terms(None, identifier, weights)
        row = scene.table(tuple(round(w, 6) for w in weights))[identifier]
        check(r(sum(parts), 2) == r(row["score"], 2), f"{label} {identifier}: parts {parts} add up to score {row['score']:.4f}")
        check(all(abs(p - r(w * n, 2)) < 0.0101 for p, w, n in zip(parts, printed_w, norm)), f"{label} {identifier}: each product w × n = part within 0.01 ({printed_w} {norm} {parts})")
        check(r(sum(w * n for w, n in zip(printed_w, norm)), 2) == r(row["score"], 2), f"{label} {identifier}: printed w × n sum reproduces the score")

five = scene.ids()
cap = scene.BUDGET_CHF
check({i for i in five if scene.raw(i, "cost") > cap} == {"glass_shaded", "glass_bare"}, "budget 21,000 CHF removes exactly the shaded and the bare glass wall")
allowed = tuple(i for i in five if scene.raw(i, "cost") <= cap)
rows = scene.table(tuple(study["balanced"][c] for c in scene.ORDER), pool=allowed)
check(min(rows.values(), key=lambda row: row["score"])["id"] == "ribbon", "the allowed three are ranked Ribbon first")
print("failures:", len(bad))
sys.exit(1 if bad else 0)
