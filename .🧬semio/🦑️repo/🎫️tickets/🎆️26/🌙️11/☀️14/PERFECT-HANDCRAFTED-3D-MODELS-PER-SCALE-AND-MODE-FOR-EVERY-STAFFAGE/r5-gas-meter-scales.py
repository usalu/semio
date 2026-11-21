"""🧯 Round 5, appliances/gas-meter: the test-dial pointer leaves 1:100 and 1:200 (it cannot print there, see `r5-gas-meter-roots.py`), the pointer spindle becomes a member of the scale's own section, and 1:500 is authored anew as a crisp massing of the compact G4 meter (rounded case with the seam band, index head, two stubs). Rewrites all four scales of `volumes/appliances/gas-meter.json` from the round-3 definitions plus the overrides stated here.

Usage: python3 r5-gas-meter-scales.py
"""
import importlib.util
import json
import os
from pathlib import Path
from runpy import run_path

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("model", os.path.join(HERE, "r3-gas-meter-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)
K = M.K


def spindle(radius, length):
    """🧷 The pointer spindle as a member of the scale's own section, from the dial face into the index head."""
    zi = K.IZ_FRONT
    return K.st(K.rev("member-pointer-spindle", "pointer", M.ABS, "body", 0.3, [(0, 0), (0, radius), (length, radius), (length, 0)], (0, 0, zi - 0.002), K.ZAX), "pointer", "member")


def without_needle(solids):
    """✂️ The list without the pointer blade."""
    return [s for s in solids if s["id"] != "needle"]


def with_spindle(solids, radius, length):
    """🔁 The list with the spindle replaced."""
    return [spindle(radius, length) if s["id"] == "member-pointer-spindle" else s for s in solids]


def body500():
    """🔎 1:500 body: the case as one rounded prism across the depth, the seam band, the index head and the two Ø30 stubs."""
    bw, bd, _ = K.BEAD
    zi = K.IZ_FRONT
    case = K.ext("housing", "body", M.ST, "body", 0.74, "xy", K.rrect(-bw + 0.0045, K.YB, bw - 0.0045, K.YT, K.R, 2), -bd + 0.0045, bd - 0.0045)
    seam = K.ext("seam", "body", M.ST, "body", 0.6, "zx", K.rrect(-bd, -bw, bd, bw, K.R, 2), -K.BEAD[2], K.BEAD[2])
    return [
        case,
        seam,
        K.box("index", "body", M.ABS, "body", M.T_HEAD, (2 * K.IX, K.IY1 - K.IY0, 0.024), (0, (K.IY0 + K.IY1) / 2, zi + 0.012)),
        K.twice(K.rev("stubs", "body", M.PIPE, "body", M.T_STUB, [(K.YT - 0.002, 0), (K.YT - 0.002, K.STUB_R), (K.Y_TOP, K.STUB_R), (K.Y_TOP, 0)], (-K.XS, 0, 0), K.YAX)),
    ]


def thermal500():
    """🌡️ 1:500 thermal: the case (its seam band is merged into the shell region), index head, stubs with their cut tops, the hidden measuring unit as one core."""
    return [
        K.ref("th-housing", "housing", region="shell"),
        K.ref("th-index", "index", region="index"),
        K.ref("th-stubs", "stubs", region="pipes", sectionFaces=["+v"]),
        K.th(K.box("th-unit", "body", M.MU, "body", 0.35, (0.150, 0.110, 0.090), (0, -0.040, 0)), "measuring-unit"),
    ]


def structure500():
    """🏗️ 1:500 structure: the stubs as hangers, the pressed case as one sheet monocoque from crown to crown with its shoulders, the unit slab, the spindle, the two supports and the mass."""
    bw, bd, _ = K.BEAD
    return [
        K.st(K.twice(K.rev("member-stubs", "body", M.PIPE, "body", M.T_STUB, [(0.090, 0), (0.090, K.STUB_R), (K.Y_STUB_TOP, K.STUB_R), (K.Y_STUB_TOP, 0)], (-K.XS, 0, 0), K.YAX)), "body", "member"),
        K.st(K.loft("member-case", "body", M.ST, "body", 0.7, [((0, K.YB, 0), bw - K.R, bd - K.R, 4), ((0, K.YSB, 0), bw, bd, 4), ((0, K.YST, 0), bw, bd, 4), ((0, K.YT, 0), bw - K.R, bd - K.R, 4)]), "body", "member"),
        K.st(K.box("member-unit", "body", M.MU, "body", 0.5, (0.150, 0.094, 0.030), (0, -0.049, 0)), "body", "member"),
        spindle(0.01, 0.024),
    ] + M.supports(K.Y_STUB_TOP, False) + [
        K.st(K.ell("mass-body", "body", M.MU, "detail", 0.5, (0.02, 0.02, 0.02), (0, K.COM_Y, 0)), "body", "mass"),
    ]


def t(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def decisions(texts):
    """📝 Round-3 decisions with the pointer sentences corrected at 1:100 and 1:200, and the new 1:500 decisions."""
    d = {k: [dict(x) for x in v] for k, v in texts.items()}
    swaps = {
        "100": [("with the counter strip, the dial and the pointer as sheets on its face", "with the counter strip and the dial as sheets on its face (the 5 mm pointer blade cannot print at this scale and is left out)",
                 "mit Zählwerksstreifen, Prüfrad und Zeiger als Blätter auf seiner Front", "mit Zählwerksstreifen und Prüfrad als Blätter auf seiner Front (die Zeigerklinge von 5 mm ist in diesem Maßstab nicht druckbar und entfällt)"),
                ("the pointer spindle,", "the pointer spindle at the 15 mm feature,", "die Zeigerwelle,", "die Zeigerwelle auf dem 15-mm-Merkmal,")],
        "200": [("a 40 mm dial and a bolder pointer", "and a 40 mm dial (the pointer would be a hairline and is left out)", "einem 40-mm-Prüfrad und einem kräftigeren Zeiger", "und einem 40-mm-Prüfrad (der Zeiger wäre eine Haarlinie und entfällt)"),
                ("The pointer spindle,", "The pointer spindle at the 30 mm feature,", "Zeigerwelle,", "Zeigerwelle auf dem 30-mm-Merkmal,")],
    }
    for key, pairs in swaps.items():
        for en0, en1, de0, de1 in pairs:
            hit = [x for x in d[key] if en0 in x["en"]]
            assert len(hit) == 1 and de0 in hit[0]["de"], (key, en0)
            hit[0]["en"] = hit[0]["en"].replace(en0, en1)
            hit[0]["de"] = hit[0]["de"].replace(de0, de1)
    d["500"] = [
        t("Own 1:500 model, a crisp massing of the compact G4 meter: the case as one prism of the front outline (0.193 x 0.195 m, 18 mm corners) across its depth, the folded seam as a proud band round the middle, the index head as a 24 mm proud block on the front and the two connections as plain 30 mm stubs at 110 mm spacing; counter, dial, plate, nuts and tailpieces are merged away.",
          "Eigenes 1:500-Modell, eine klare Massenform des kompakten G4-Zählers: das Gehäuse als ein Prisma des Frontumrisses (0,193 x 0,195 m, 18-mm-Ecken) über seine Tiefe, die gefalzte Naht als vorstehendes Band um die Mitte, der Zählwerkskopf als 24 mm vorstehender Block auf der Front und die zwei Anschlüsse als glatte 30-mm-Stutzen im Abstand von 110 mm; Zählwerk, Prüfrad, Schild, Muttern und Einlegeteile gehen darin auf."),
        t("Thermal by reference to case, index and stubs (the seam band merges into the shell), the measuring unit as one hidden core of 0.15 x 0.11 x 0.09 m in the lower half. Structure: the stubs carry the meter from the two supports at the unions, the pressed case as one sheet monocoque from crown to crown with its chamfered shoulders, the unit slab, the pointer spindle and the mass.",
          "Thermik als Verweis auf Gehäuse, Zählwerkskopf und Stutzen (das Nahtband geht in der Schale auf), das Messwerk als ein verborgener Kern von 0,15 x 0,11 x 0,09 m in der unteren Hälfte. Tragwerk: die Stutzen tragen den Zähler von den zwei Auflagern an den Verschraubungen, das gepresste Gehäuse als eine Blechschale von Scheitel zu Scheitel mit ihren gefasten Schultern, die Messwerkscheibe, die Zeigerwelle und die Masse."),
    ]
    return d


def main():
    """💾 Write all four scales."""
    texts = json.load(open(M.TEXTS, encoding="utf-8"))
    d = decisions(texts)
    vols = {
        "50": {"status": "authored", "denominator": 50, "body": M.body50(), "thermal": M.thermal50(), "structure": M.structure50(), "decisions": d["50"]},
        "100": {"status": "authored", "denominator": 100, "body": without_needle(M.body100()), "thermal": M.thermal100(), "structure": with_spindle(M.structure100(), 0.0075, 0.024), "decisions": d["100"]},
        "200": {"status": "authored", "denominator": 200, "body": without_needle(M.body200()), "thermal": M.thermal200(), "structure": with_spindle(M.structure200(), 0.015, 0.03), "decisions": d["200"]},
        "500": {"status": "authored", "denominator": 500, "body": body500(), "thermal": thermal500(), "structure": structure500(), "decisions": d["500"]},
    }
    catalog = next(Path(HERE).parents[6].glob("🎓*/*/*staffage/🗂*/*/gas-meter/🔣️.json"))
    asset = json.loads(catalog.read_text(encoding="utf-8"))["asset"]
    asset["volumes"] = vols
    run_path(os.path.join(HERE,"digital-coarse-parts.py"))["author"](asset)
    json.dump({"volumes": vols}, open(M.SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print({s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items()})


main()
