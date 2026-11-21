"""🧑‍🏫 Round 5 chalkboard: 1:100 running gap. The sliding leaf runs 4 mm in front of the fixed board, open at its free edge, so the room air reaches the enamel behind it (1:50 shows this). At 1:100 the flood cell is 3.75 mm and the 4 mm gap between leaf frame and board frame closed the air space behind the leaf: the left enamel face counted nothing (board-face 2.14 m2 against 4.5). The leaf of 1:100 is drawn 3.5 mm further out, so the gap is 7.5 mm, the narrowest gap the scale resolves (half its 15 mm feature); the leaf stays inside the header hood and the depth of 0.08 m.

Usage: python3 r5-chalkboard-model.py   (rewrites the volumes source from the catalog volumes)
See ./r5-brief.md, ./r3-objects-chalkboard.md, ./design.md C31.
"""
import r5_o02_lib as L

SHIFT = -0.0035
GAP_OLD = "The leaf reaches from the tray to the header: its running gaps of 8 mm are below the 15 mm feature and closed, the trolleys hidden in the hood are left out."
GAP_NEW = "The leaf reaches from the tray to the header: its running gaps of 8 mm at top and bottom are below the 15 mm feature and closed, the 4 mm gap between leaf and board is drawn at 7.5 mm, the narrowest gap this scale resolves, so that the room air reaches the board behind the leaf as it does in reality; the trolleys hidden in the hood are left out."
GAP_OLD_DE = None


def shift_leaf(v):
    """↔️ Moves the 1:100 leaf solids of every list 3.5 mm towards the room."""
    for key in ("body", "thermal", "structure"):
        for s in v[key]:
            if s.get("bone") != "leaf" or "of" in s:
                continue
            if s["kind"] == "extrude" and s["plane"] == "xy":
                s["from"], s["to"] = round(s["from"] + SHIFT, 5), round(s["to"] + SHIFT, 5)
            else:
                t = s.setdefault("transform", {}).setdefault("translation", [0.0, 0.0, 0.0])
                t[2] = round(t[2] + SHIFT, 5)
    return v


def volumes():
    vols = L.catalog("chalkboard")["asset"]["volumes"]
    v = vols["100"]
    shift_leaf(v)
    d = v["decisions"][0]
    if GAP_OLD not in d["en"]:
        raise SystemExit("1:100 decision text not matched")
    d["en"] = d["en"].replace(GAP_OLD, GAP_NEW)
    d["de"] = d["de"].rstrip() + " Der 4-mm-Spalt zwischen Schiebetafel und fester Tafel ist mit 7,5 mm gezeichnet, dem engsten Spalt, den dieser Maßstab auflöst, damit die Raumluft wie in Wirklichkeit die Tafel hinter der Schiebetafel erreicht."
    return vols


if __name__ == "__main__":
    print(L.write_source("chalkboard", volumes()))
