"""🧊 Round 3 volumes of components/masonry-wall-with-lintel: body, thermal and structure of every scale from the kit, with the decisions; writes `volumes/components/masonry-wall-with-lintel.json`.

Usage: python3 r3-masonry-wall-with-lintel-volumes.py
"""
import importlib.util
import json
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-masonry-wall-with-lintel-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
t = K.t

DECISIONS = {
    "50": [
        t("Body: the facing leaf brick by brick in stretcher bond (NF 240 x 71 mm, 33 courses of 1/12 m with 12.3 mm bed and 10 mm head joints, flush pointing, half bricks of 115 mm at every second reveal course, cut bricks against lintel and sill), open head joints as weep holes in the bottom course and above the lintel, the 140 mm cavity wool, the calcium silicate leaf block by block (248 mm courses, thin-bed joints shown as seams, blocks cut down to the parapet and notched at the reveals), 15 mm plaster with its returns into the reveals and the soffit, the inner precast lintel 175 x 250 mm, the exposed outer precast lintel 175 x 238 mm on its bed with a chamfered front edge, 80 mm wool between the two lintels, and the cast-stone sill with 50 mm projection, drip groove, 5 degree weathering and raised ends in the reveals.",
          "Körper: die Vormauerschale Stein für Stein im Läuferverband (NF 240 x 71 mm, 33 Schichten zu 1/12 m mit 12,3-mm-Lager- und 10-mm-Stoßfugen, Fugenglattstrich, halbe Steine zu 115 mm in jeder zweiten Laibungsschicht, Passsteine an Sturz und Fensterbank), offene Stoßfugen als Entwässerung in der untersten Schicht und über dem Sturz, die 140-mm-Kerndämmung, die Kalksandsteinschale Stein für Stein (248-mm-Schichten, Dünnbettfugen als Nähte, Steine unter der Öffnung auf die Brüstung gekürzt und an den Laibungen ausgeklinkt), 15 mm Putz mit seinen Umläufen in Laibungen und Sturzuntersicht, der innere Fertigteilsturz 175 x 250 mm, der sichtbare äußere Fertigteilsturz 175 x 238 mm auf seinem Mörtelbett mit gefaster Vorderkante, 80 mm Dämmung zwischen den beiden Stürzen und die Betonwerkstein-Fensterbank mit 50 mm Überstand, Tropfnase, 5 Grad Gefälle und hochgezogenen Enden in den Laibungen."),
        t("Thermal: the three regions of the EN ISO 6946 calculation (wall field, lintel zone, sill zone) as strips through the wall, every layer a shell at its real thickness, innermost first: plaster, inner leaf, wool, facing for the field (solids of their own, because the body is brick by brick); plaster, inner lintel, 80 mm wool, outer lintel for the lintel zone; plaster, calcium silicate, 80 mm wool, sill for the sill zone. The outer layer of each region counts its exterior face only: its other faces are cuts of the continuing wall, contacts with the neighbouring zone or the reveal against which the window is fitted (`sectionFaces`).",
          "Thermisch: die drei Regionen der Berechnung nach EN ISO 6946 (Wandfeld, Sturzzone, Fensterbankzone) als Streifen durch die Wand, jede Schicht eine Schale in ihrer wirklichen Dicke, von innen nach außen: Putz, Innenschale, Dämmung, Vormauerung für das Feld (eigene Körper, weil der Körper Stein für Stein gebaut ist); Putz, Innensturz, 80 mm Dämmung, Außensturz für die Sturzzone; Putz, Kalksandstein, 80 mm Dämmung, Fensterbank für die Fensterbankzone. Die äußere Schicht jeder Region zählt nur ihre Außenfläche: ihre übrigen Flächen sind Schnitte der weiterlaufenden Wand, Kontakte mit der Nachbarzone oder die Laibung, an die das Fenster anschließt (`sectionFaces`)."),
        t("Structure: the 175 mm calcium silicate leaf with its opening is the wall member (the facing leaf is self-supporting cladding and is left out); both precast lintels are lintel members on their four bearings (thin bed under the inner, the 12.3 mm bed under the outer lintel), the sill sits on its bed; a bearing pad on the wall head carries the beam; the 4 mm stainless wall ties in ten bed joints 0.25 m apart (8 per m2, slender) tie the facing to the leaf; three contact pads at the foundation anchors; mass spheres at the centres of mass of wall, lintel and sill.",
          "Tragwerk: die 175-mm-Kalksandsteinschale mit ihrer Öffnung ist der Wandstab (die Vormauerschale ist selbsttragende Bekleidung und fehlt); beide Fertigteilstürze sind Sturzstäbe auf ihren vier Auflagern (Dünnbett unter dem inneren, 12,3-mm-Mörtelbett unter dem äußeren Sturz), die Fensterbank liegt auf ihrem Bett; ein Lagerkissen auf dem Wandkopf trägt den Träger; die 4-mm-Edelstahl-Luftschichtanker in zehn Lagerfugen im Abstand von 0,25 m (8 je m2, schlank) binden die Vormauerung an die Innenschale; drei Kontaktplatten an den Fundamentankern; Massekugeln in den Schwerpunkten von Wand, Sturz und Fensterbank."),
    ],
    "100": [
        t("Body: the four leaves as slabs (115 mm facing, 140 mm wool, 175 mm calcium silicate, 15 mm plaster: the plaster is one smallest feature and stays), the two lintels as plain 175 mm bars with the 80 mm wool between them, the sill with its 50 mm projection and one straight weathering. Merged against 1:50: bricks, blocks and their 10 to 12 mm joints (below the 15 mm feature, drawn as a coursing hatch), the weep joints, the reveal and soffit plaster (into the opening outline), the bed under the outer lintel, the lintel chamfer, the drip groove and the raised sill ends.",
          "Körper: die vier Schalen als Scheiben (115 mm Vormauerung, 140 mm Dämmung, 175 mm Kalksandstein, 15 mm Putz: der Putz ist eine kleinste Einzelheit und bleibt), die zwei Stürze als einfache 175-mm-Balken mit den 80 mm Dämmung dazwischen, die Fensterbank mit 50 mm Überstand und einem geraden Gefälle. Gegenüber 1:50 zusammengefasst: Steine, Blöcke und ihre 10 bis 12 mm Fugen (unter der Einzelheit von 15 mm, als Schichtschraffur gezeichnet), die offenen Stoßfugen, der Laibungs- und Untersichtputz (in die Öffnungskontur), das Bett unter dem Außensturz, die Sturzfase, die Tropfnase und die hochgezogenen Fensterbankenden."),
        t("Thermal: three shells per region, innermost first: inner masonry with the plaster merged, wool, outer layer; wool and outer layer reference the body, the outer layer counts its exterior face only (`sectionFaces`). Structure: the leaf, both lintels on two merged bearings, the sill, the beam bearing pad, the wall ties kept as slender wires, three contact pads of 15 mm and the three mass spheres.",
          "Thermisch: drei Schalen je Region, von innen: Innenmauerwerk mit eingeschlossenem Putz, Dämmung, äußere Schicht; Dämmung und äußere Schicht verweisen auf den Körper, die äußere Schicht zählt nur ihre Außenfläche (`sectionFaces`). Tragwerk: die Innenschale, beide Stürze auf zwei zusammengefassten Auflagern, die Fensterbank, das Trägerlagerkissen, die Luftschichtanker als schlanke Drähte, drei Kontaktplatten zu 15 mm und die drei Massekugeln."),
    ],
    "200": [
        t("Body: the wall as one 445 mm mass with the opening and the lintel and sill zones cut out, the lintel band over the whole thickness (both lintels, the wool between them and the plaster merged), the precast sill with its 50 mm projection (more than one 30 mm feature) and the parapet cap behind it. The 1:200 drawing shows no layers; the leaves are merged into the mass.",
          "Körper: die Wand als eine 445-mm-Masse mit ausgeschnittener Öffnung, Sturz- und Fensterbankzone, das Sturzband über die ganze Dicke (beide Stürze, die Dämmung dazwischen und der Putz zusammengefasst), die Fertigteil-Fensterbank mit 50 mm Überstand (mehr als eine Einzelheit von 30 mm) und die Brüstungsabdeckung dahinter. Die Zeichnung 1:200 zeigt keine Schichten; die Schalen sind in der Masse aufgegangen."),
        t("Thermal: per region the inner masonry (plaster merged), the insulation and the outer layer as shells of their own, because the body no longer separates them; the outer layer counts its exterior face only. Structure: the 175 mm leaf, the inner lintel as the bearing member over the opening and the outer lintel, the sill, three contact pads of 30 mm and the mass spheres; ties, bearings and the beam pad are below the 30 mm feature and are left out.",
          "Thermisch: je Region das Innenmauerwerk (mit Putz), die Dämmung und die äußere Schicht als eigene Schalen, weil der Körper sie nicht mehr trennt; die äußere Schicht zählt nur ihre Außenfläche. Tragwerk: die 175-mm-Innenschale, der Innensturz als tragender Stab über der Öffnung und der Außensturz, die Fensterbank, drei Kontaktplatten zu 30 mm und die Massekugeln; Anker, Auflager und Trägerkissen liegen unter der Einzelheit von 30 mm und entfallen."),
    ],
    "500": [
        t("Minimal: the wall mass with the opening, the lintel band through the whole thickness and the sill band through the whole thickness with its nose 50 mm in front of the face (the true depth of the wall; the band itself is 83 mm high and 495 mm deep, above the 75 mm feature); layers, weathering and drip are merged, as in the 1:500 drawing.",
          "Minimal: die Wandmasse mit der Öffnung, das Sturzband über die ganze Dicke und das Fensterbankband über die ganze Dicke mit seiner Nase 50 mm vor der Wandfläche (die wirkliche Tiefe der Wand; das Band selbst ist 83 mm hoch und 495 mm tief, über der Einzelheit von 75 mm); Schichten, Gefälle und Tropfnase sind zusammengefasst, wie in der Zeichnung 1:500."),
        t("Thermal: the field keeps inner masonry, the 140 mm wool and the facing as three shells (the insulation is wider than the 75 mm feature and is what the wall is about), drawn over field and sill zone together as the 1:500 elevation merges them; the lintel zone and the sill band are one solid each that counts its exterior face. Structure: the 175 mm leaf, the inner lintel, the precast sill in the wall plane, three contact pads of 75 mm and the mass spheres.",
          "Thermisch: das Feld behält Innenmauerwerk, die 140-mm-Dämmung und die Vormauerung als drei Schalen (die Dämmung ist breiter als die Einzelheit von 75 mm und ist das, worum es bei der Wand geht), über Feld und Fensterbankzone zusammen gezogen, wie die Ansicht 1:500 sie zusammenfasst; Sturzzone und Fensterbankband sind je ein Körper, der seine Außenfläche zählt. Tragwerk: die 175-mm-Innenschale, der Innensturz, die Fertigteil-Fensterbank in der Wandebene, drei Kontaktplatten zu 75 mm und die Massekugeln."),
    ],
}


def volumes():
    """🧊 The four authored scales."""
    lists = {
        "50": (K.body50(), K.thermal50(), K.structure50()),
        "100": (K.body100(), K.thermal100(), K.structure100()),
        "200": (K.body200(), K.thermal200(), K.structure200()),
        "500": (K.body500(), K.thermal500(), K.structure500()),
    }
    return {s: {"status": "authored", "denominator": int(s), "body": b, "thermal": th, "structure": st, "decisions": DECISIONS[s]} for s, (b, th, st) in lists.items()}


def main():
    out = {"volumes": volumes()}
    with open(K.SOURCE, "w", encoding="utf-8") as f:
        f.write(json.dumps(out, indent=1, ensure_ascii=False) + "\n")
    for s, v in out["volumes"].items():
        print(f"[DEBUG] 1:{s} body {len(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])}")
    print(K.SOURCE)


if __name__ == "__main__":
    main()
