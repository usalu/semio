"""🛋️ Round 5, furniture/three-seat-sofa: the round 3 sofa with the 1:500 load path closed, written as the candidate document (🗑️generated/r5-three-seat-sofa/🔣️.json) and as the volumes source (volumes/furniture/three-seat-sofa.json).

Change against round 3: at 1:500 the seat bearing bands stood on nothing between the two base rails (the deck was missing). The ply deck is now a 75 mm member from rail to rail under the seat bands (0.155 to 0.23 m, between the arms), so the cushion loads reach the rails, the arm panels and the feet; the structure decision says so.

Usage: python3 r5-three-seat-sofa-build.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/three-seat-sofa/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r5-three-seat-sofa")

DECISION_500 = ("Thermal: the blocks are split back into their regions by own solids (plinth, seat cushions, back cushions, back frame), arms and feet are their surfaces. Structure: the feet, two 75 mm base rails with the 75 mm ply deck between them under the seat bands, a 75 mm back panel, a 75 mm panel per arm, 75 mm bearing bands of the cushions, glides of the foot size, masses.",
                "Thermik: die Blöcke sind durch eigene Körper wieder in ihre Regionen geteilt (Sockel, Sitzkissen, Rückenkissen, Rückenrahmen), Armlehnen und Füße sind ihre Oberflächen. Tragwerk: die Füße, zwei 75-mm-Sockelschienen mit dem 75-mm-Sperrholzdeck dazwischen unter den Sitzbändern, eine 75-mm-Rückenplatte, je Armlehne eine 75-mm-Platte, 75-mm-Auflagebänder der Kissen, Gleiter in Fußgröße, Massen.")


def module(name):
    """📦 Loads one of the round 3 sibling scripts."""
    spec = importlib.util.spec_from_file_location("r3_three_seat_sofa_" + name, os.path.join(HERE, f"r3-three-seat-sofa-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def structure(lists, kit, scale, body):
    """🏗 Round 3 structure; at 1:500 plus the deck between the base rails."""
    out = lists.structure(scale, body)
    if scale == "500":
        deck = kit.box("member-deck", "frame", kit.PLY, 0.35, kit.BASE_X, (kit.DECK_Y - 0.075, kit.DECK_Y), (kit.BASE_Z[0] + 0.075, kit.BASE_Z[1] - 0.075))
        at = next(k for k, s in enumerate(out) if s["id"] == "member-base-rail") + 1
        out.insert(at, lists.own(deck, "frame", "member"))
    return out


def decisions(lists, scale):
    """🗒 Round 3 decisions; the 1:500 structure sentence names the deck."""
    out = lists.decisions(scale)
    if scale == "500":
        out[1] = {"en": DECISION_500[0], "de": DECISION_500[1]}
    return out


def candidate():
    """🧩 The catalog document with the round 3 roots and the round 5 volumes."""
    kit, body, lists, roots, draw = (module(name) for name in ("kit", "body", "lists", "roots", "draw"))
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    roots.apply(asset)
    for scale in ("50", "100", "200", "500"):
        solids = body.body(scale)
        asset["volumes"][scale] = {"status": "authored", "denominator": int(scale), "body": solids, "thermal": lists.thermal(scale, solids), "structure": structure(lists, kit, scale, solids), "decisions": decisions(lists, scale)}
    draw.apply(asset)
    return document


def main():
    document = candidate()
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    volumes = document["asset"]["volumes"]
    json.dump({"volumes": volumes}, open(os.path.join(HERE, "volumes", "furniture", "three-seat-sofa.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("candidate", {k: [sum(s.get("repeat", {}).get("count", 1) for s in v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
