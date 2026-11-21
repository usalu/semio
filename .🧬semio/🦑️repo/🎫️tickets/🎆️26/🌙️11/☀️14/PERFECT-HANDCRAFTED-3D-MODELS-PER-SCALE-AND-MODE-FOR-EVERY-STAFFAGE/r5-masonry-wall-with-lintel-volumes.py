"""🧱 Round 5 volumes of components/masonry-wall-with-lintel: the round 3 construction (body and thermal of every scale from `r3-masonry-wall-with-lintel-kit.py`) with the load path closed: the 115 mm facing leaf, which carries its own weight and the exposed outer lintel down to the foundation (DIN EN 1996-2/NA: self-supporting, non-load-bearing leaf), is a member of the wall at every scale; at 1:500 the lintel member spans the whole wall thickness as the drawn lintel band does. Writes `volumes/components/masonry-wall-with-lintel.json`.

Usage: python3 r5-masonry-wall-with-lintel-volumes.py
See: r5-brief.md, r3-components-masonry-wall-with-lintel.md
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    """📦 A ticket script loaded as a module."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name + ".py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


K = module("r3-masonry-wall-with-lintel-kit")
R3 = module("r3-masonry-wall-with-lintel-volumes")
t = K.t


def facing_member():
    """🧱 The facing leaf as a member: 115 mm brick with the opening, the lintel zone and the sill zone cut out, standing on the foundation pads; the outer lintel bears on its piers."""
    return K.st(K.slab("member-wall-facing", K.WALL, [K.FIELD_HOLE], (K.EXT, K.FILL), "brick"), "wall", "member")


def structure500():
    """🏗️ 1:500 structure: inner leaf and facing leaf, the lintel band through the whole thickness as one member, the sill, contacts and masses."""
    f = 0.075
    return [
        K.st(K.slab("member-wall-leaf", K.WALL, [K.LEAF_HOLE_COARSE], (K.LEAF, K.PLASTER), "block"), "wall", "member"),
        facing_member(),
        K.st(K.box("member-lintel", K.LINTEL_X, (K.LINTEL_Y, K.LINTEL_TOP), (K.EXT, K.PLASTER), "concrete", "lintel"), "lintel", "member"),
        K.st(K.box("member-sill", K.SILL_X, (K.SILL_Y, K.PARAPET), (K.EXT, K.TONGUE), "concrete", "sill"), "sill", "member"),
    ] + K.supports(f) + K.masses(0.075, K.body50())


def with_facing(structure):
    """➕ A structure list with the facing member placed right after the inner leaf."""
    return structure[:1] + [facing_member()] + structure[1:]


STRUCTURE = {
    "50": t("Structure: both leaves are wall members: the 175 mm calcium silicate leaf with its opening carries the floors and the beam, the 115 mm facing leaf with its opening carries its own weight and the exposed outer lintel down to the foundation (a self-supporting leaf, DIN EN 1996-2/NA); both precast lintels are lintel members on their four bearings (thin bed under the inner lintel on the calcium silicate piers, the 12.3 mm bed under the outer lintel on the facing piers), the sill sits on its bed on the facing; a bearing pad on the wall head carries the beam; the 4 mm stainless wall ties in ten bed joints 0.25 m apart (8 per m2, slender) hold the facing against wind; three contact pads under both leaves at the foundation anchors; mass spheres at the centres of mass of wall, lintel and sill.",
            "Tragwerk: beide Schalen sind Wandstäbe: die 175-mm-Kalksandsteinschale mit ihrer Öffnung trägt Decken und Träger, die 115-mm-Vormauerschale mit ihrer Öffnung trägt ihr Eigengewicht und den sichtbaren Außensturz bis zum Fundament (eine selbsttragende Schale, DIN EN 1996-2/NA); beide Fertigteilstürze sind Sturzstäbe auf ihren vier Auflagern (Dünnbett unter dem Innensturz auf den Kalksandsteinpfeilern, das 12,3-mm-Mörtelbett unter dem Außensturz auf den Pfeilern der Vormauerung), die Fensterbank liegt auf ihrem Bett auf der Vormauerung; ein Lagerkissen auf dem Wandkopf trägt den Träger; die 4-mm-Edelstahl-Luftschichtanker in zehn Lagerfugen im Abstand von 0,25 m (8 je m2, schlank) halten die Vormauerung gegen Wind; drei Kontaktplatten unter beiden Schalen an den Fundamentankern; Massekugeln in den Schwerpunkten von Wand, Sturz und Fensterbank."),
    "100": t("Thermal: three shells per region, innermost first: inner masonry with the plaster merged, wool, outer layer; wool and outer layer reference the body, the outer layer counts its exterior face only (`sectionFaces`). Structure: both leaves as wall members (the facing carries its own weight and the outer lintel to the foundation), both lintels on two merged bearings, the sill, the beam bearing pad, the wall ties kept as slender wires, three contact pads of 15 mm and the mass spheres.",
             "Thermisch: drei Schalen je Region, von innen: Innenmauerwerk mit eingeschlossenem Putz, Dämmung, äußere Schicht; Dämmung und äußere Schicht verweisen auf den Körper, die äußere Schicht zählt nur ihre Außenfläche (`sectionFaces`). Tragwerk: beide Schalen als Wandstäbe (die Vormauerung trägt ihr Eigengewicht und den Außensturz bis zum Fundament), beide Stürze auf zwei zusammengefassten Auflagern, die Fensterbank, das Trägerlagerkissen, die Luftschichtanker als schlanke Drähte, drei Kontaktplatten zu 15 mm und die Massekugeln."),
    "200": t("Thermal: per region the inner masonry (plaster merged), the insulation and the outer layer as shells of their own, because the body no longer separates them; the outer layer counts its exterior face only. Structure: the 175 mm inner leaf and the 115 mm facing leaf as wall members, the inner lintel over the opening of the inner leaf and the outer lintel over that of the facing, the sill, three contact pads of 30 mm and the mass spheres; ties, bearings and the beam pad are below the 30 mm feature and are left out.",
             "Thermisch: je Region das Innenmauerwerk (mit Putz), die Dämmung und die äußere Schicht als eigene Schalen, weil der Körper sie nicht mehr trennt; die äußere Schicht zählt nur ihre Außenfläche. Tragwerk: die 175-mm-Innenschale und die 115-mm-Vormauerschale als Wandstäbe, der Innensturz über der Öffnung der Innenschale und der Außensturz über der der Vormauerung, die Fensterbank, drei Kontaktplatten zu 30 mm und die Massekugeln; Anker, Auflager und Trägerkissen liegen unter der Einzelheit von 30 mm und entfallen."),
    "500": t("Thermal: the field keeps inner masonry, the 140 mm wool and the facing as three shells (the insulation is wider than the 75 mm feature and is what the wall is about), drawn over field and sill zone together as the 1:500 elevation merges them; the lintel zone and the sill band are one solid each that counts its exterior face. Structure: the 175 mm inner leaf and the 115 mm facing leaf, one lintel member through the whole thickness (both lintels merged, as the drawn lintel band), the precast sill in the wall plane, three contact pads of 75 mm and the mass spheres.",
             "Thermisch: das Feld behält Innenmauerwerk, die 140-mm-Dämmung und die Vormauerung als drei Schalen (die Dämmung ist breiter als die Einzelheit von 75 mm und ist das, worum es bei der Wand geht), über Feld und Fensterbankzone zusammen gezogen, wie die Ansicht 1:500 sie zusammenfasst; Sturzzone und Fensterbankband sind je ein Körper, der seine Außenfläche zählt. Tragwerk: die 175-mm-Innenschale und die 115-mm-Vormauerschale, ein Sturzstab über die ganze Wanddicke (beide Stürze zusammengefasst, wie das gezeichnete Sturzband), die Fertigteil-Fensterbank in der Wandebene, drei Kontaktplatten zu 75 mm und die Massekugeln."),
}


def volumes():
    """🧊 The four authored scales: round 3 body and thermal, round 5 structure and its decisions."""
    lists = {
        "50": (K.body50(), K.thermal50(), with_facing(K.structure50())),
        "100": (K.body100(), K.thermal100(), with_facing(K.structure100())),
        "200": (K.body200(), K.thermal200(), with_facing(K.structure200())),
        "500": (K.body500(), K.thermal500(), structure500()),
    }
    out = {}
    for s, (b, th, st) in lists.items():
        decisions = [dict(d) for d in R3.DECISIONS[s]]
        decisions[-1] = STRUCTURE[s]
        out[s] = {"status": "authored", "denominator": int(s), "body": b, "thermal": th, "structure": st, "decisions": decisions}
    return out


def main():
    """🚀 Write the volumes source."""
    out = {"volumes": volumes()}
    with open(K.SOURCE, "w", encoding="utf-8") as f:
        f.write(json.dumps(out, indent=1, ensure_ascii=False) + "\n")
    for s, v in out["volumes"].items():
        print(f"1:{s} body {len(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])}")
    print(K.SOURCE)


if __name__ == "__main__":
    main()
