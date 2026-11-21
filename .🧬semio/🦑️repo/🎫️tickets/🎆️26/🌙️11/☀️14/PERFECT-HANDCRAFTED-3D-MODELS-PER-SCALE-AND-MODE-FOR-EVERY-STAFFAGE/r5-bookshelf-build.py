"""📚️ Round 5, furniture/bookshelf: the round 3 bookcase with the floor supports kept under the carcass, written as the candidate document (🗑️generated/r5-bookshelf/🔣️.json) and as the volumes source (volumes/furniture/bookshelf.json).

Change against round 3: the support solid under each side panel was the 18 x 300 mm floor edge grown by 6 / 12 / 20 / 30 mm all round, so at 1:200 and 1:500 it stood 20 to 30 mm outside the carcass in front, behind and beside it (seen in plan as green tabs). Each support now keeps the round 3 width (18 mm plus twice the growth) but is laid from the outer face of its side panel inward, and spans exactly the 300 mm depth of the floor edge: the contact strip stays under the bookcase at every scale.

Usage: python3 r5-bookshelf-build.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bookshelf/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r5-bookshelf")


def module(name):
    """📦 Loads one of the round 3 sibling scripts."""
    spec = importlib.util.spec_from_file_location("r3_bookshelf_" + name, os.path.join(HERE, f"r3-bookshelf-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def patch(model):
    """🟢 The contact strip under each side panel, from its outer face inward, 18 mm plus twice `grow` wide, over the 300 mm depth."""
    K = model.K

    def supports(grow, height):
        """🟢 One support pad per floor-edge anchor, inside the plan of the carcass."""
        out = []
        width = K.T + 2 * grow
        for anchor, x in model.ANCHORS:
            span = (0.0, width) if x < K.W / 2 else (K.W - width, K.W)
            pad = K.box("support-" + anchor, "frame", K.BOARD, span, (0, height), (K.ZF, K.ZB), tone=0.2, role="ground")
            out.append(model.own(pad, "frame", "support") | {"anchor": anchor})
        return out

    model.supports = supports


def candidate():
    """🧩 The catalog document with the round 3 roots and the round 5 volumes."""
    model = module("model")
    patch(model)
    document = json.load(open(CATALOG, encoding="utf-8"))
    module("roots").apply(document["asset"])
    document["asset"]["volumes"] = model.volumes()
    from runpy import run_path
    run_path(os.path.join(HERE, "digital-shelf.py"))["author"](document["asset"])
    return document


def main():
    document = candidate()
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    volumes = document["asset"]["volumes"]
    json.dump({"volumes": volumes}, open(os.path.join(HERE, "volumes", "furniture", "bookshelf.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("candidate", {k: [sum(s.get("repeat", {}).get("count", 1) for s in v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
