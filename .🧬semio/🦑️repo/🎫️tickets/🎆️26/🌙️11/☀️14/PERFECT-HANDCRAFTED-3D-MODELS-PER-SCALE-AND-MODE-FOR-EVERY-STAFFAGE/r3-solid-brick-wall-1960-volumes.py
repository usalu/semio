"""🏗️ Round 3 volumes of components/solid-brick-wall-1960, stated by hand per scale and list from the real build-up in the kit; writes `volumes/components/solid-brick-wall-1960.json`."""
import json
import os
import importlib.util

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-solid-brick-wall-1960-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

FULL_V = (0.0, K.HEIGHT)
FULL_W = K.WALL_W
TONE = {"plaster": 0.9, "brick": 0.5, "mortar": 0.78, "render": 0.8}
CUT = ["-v", "+v", "-w", "+w"]


def t(en, de):
    return {"en": en, "de": de}


def slab(id_, u, material, tone, v=FULL_V, w=FULL_W, extra_faces=(), **extra):
    """🧱 A layer slab of the sample with its cut faces."""
    return K.box(id_, u, v, w, material, tone, sectionFaces=list(extra_faces) + K.cuts(u, v, w), **extra)


def masonry50():
    """🧱 The 36.5 cm leaf in the NF block bond: twelve courses, each its bricks, head joints and longitudinal joint, the two course kinds repeated six times; the bed joints repeated twelve times."""
    out = []
    step = {"count": 6, "translation": [0.0, K.r5(2 * K.COURSE), 0.0]}
    v0, v1 = K.bed_v(0)
    out.append(K.box("bed-joint", K.BRICK, (v0, v1), FULL_W, "masonry-mortar", TONE["mortar"], sectionFaces=["-v", "-w", "+w"], repeat={"count": 12, "translation": [0.0, K.r5(K.COURSE), 0.0]}))
    for k, kind in ((0, "a"), (1, "b")):
        lay = K.LAYERS[kind.upper()]
        v = K.brick_v(k)
        top = ["+v"] if kind == "b" else []
        out.append(K.box(f"longitudinal-joint-{kind}", lay["collar"], v, FULL_W, "masonry-mortar", TONE["mortar"], sectionFaces=top + ["-w", "+w"], repeat=step))
        for row, first, pitch in (("header", K.HEADER_JOINT0, K.HEADER_PITCH), ("stretcher", lay["stretcherJoint"], K.STRETCHER_PITCH)):
            centres = K.joints(first, pitch)
            for i, w in enumerate(K.pieces(centres)):
                out.append(K.box(f"{row}-{kind}-{i + 1:02d}", lay[row], v, w, "solid-brick", TONE["brick"], sectionFaces=top + [f for f in ("-w", "+w") if f in K.cuts(lay[row], (0.5, 0.5), w)], repeat=step))
            for i, c in enumerate(centres):
                out.append(K.box(f"{row}-joint-{kind}-{i + 1:02d}", lay[row], v, (c - K.HEAD / 2, c + K.HEAD / 2), "masonry-mortar", TONE["mortar"], sectionFaces=top, repeat=step))
    for b in out:
        if not b["sectionFaces"]:
            del b["sectionFaces"]
    return out


def bonded_leaf(masonry):
    """🧲 The load-bearing leaf as it is built: every brick and every joint of the bond is part of the one member."""
    return [{"id": "member-" + s["id"], "of": s["id"], "member": "root", "part": "member"} for s in masonry]


def leaf(id_):
    return K.box(id_, K.BRICK, FULL_V, FULL_W, "solid-brick", 0.5, member="root", part="member")


def support(thick):
    return K.box("support-base-fixed", K.BRICK, (-thick, 0.0), FULL_W, "masonry-mortar", 0.2, member="root", part="support", anchor="base")


def mass(r):
    return {"id": "mass-wall-strip", "kind": "ellipsoid", "bone": "root", "material": "solid-brick", "role": "body", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [K.r5(K.COM_U), K.COM_V, 0.0]}, "member": "root", "part": "mass"}


def ref(id_, of, layer, faces):
    return {"id": id_, "of": of, "region": "wall-field", "layer": layer, "sectionFaces": faces}


def scale50():
    masonry = masonry50()
    body = [slab("plaster", K.PLASTER, "lime-plaster", TONE["plaster"])] + masonry + [slab("render", K.RENDER, "mineral-render", TONE["render"])]
    thermal = [
        ref("th-plaster", "plaster", "plaster", ["-u"] + CUT),
        slab("th-brick", K.BRICK, "solid-brick", 0.5, region="wall-field", layer="brick"),
        ref("th-render", "render", "render", ["-u"] + CUT),
    ]
    structure = bonded_leaf(masonry) + [support(0.0075), mass(0.04)]
    decisions = [
        t("Body: the wall as built in a 1 m x 1 m sample, 400 mm thick. 15 mm lime-cement plaster on the room side; the 365 mm solid brick leaf of 1960 in the NF block bond (240 x 115 x 71 mm bricks, twelve courses of 1/12 m with 12.3 mm bed joints and 10 mm head joints): one course has the stretchers on the room side and the headers outside, the next the reverse, so the 10 mm longitudinal joint changes side from course to course and the head joints of neighbouring courses are offset by a quarter brick; the bricks at both ends of the sample are cut. The first course stands on its mortar bed. Outside the 20 mm mineral render. Cut faces are flush, as a saw cut leaves them: the joints show by their lighter mortar tone.",
          "Körper: die Wand wie gebaut als Probe von 1 m x 1 m, 400 mm dick. Raumseitig 15 mm Kalkzementputz; die 365-mm-Vollziegelschale von 1960 im NF-Blockverband (Steine 240 x 115 x 71 mm, zwölf Schichten zu 1/12 m mit 12,3-mm-Lagerfugen und 10-mm-Stoßfugen): eine Schicht hat die Läufer raumseitig und die Binder außen, die nächste umgekehrt, so dass die 10-mm-Längsfuge von Schicht zu Schicht die Seite wechselt und die Stoßfugen benachbarter Schichten um einen Viertelstein versetzt sind; die Steine an beiden Enden der Probe sind geschnitten. Die erste Schicht steht auf ihrem Mörtelbett. Außen der 20-mm-Mineralputz. Die Schnittflächen sind eben, wie ein Sägeschnitt sie hinterlässt: die Fugen zeigen sich durch den helleren Mörtelton."),
        t("Thermal: the region wall-field is three nested shells at their real thickness, innermost first: plaster (reference to the body slab, its room face marked as the conditioned interior face, C25 c), brick (one homogeneous 365 mm shell: the declared layer has no joints, its conductivity is that of the masonry) and the render (reference to the body slab), whose weather face is the 1 m² of the region. The top, bottom and end faces are cuts through the continuing wall.",
          "Thermik: die Region wall-field besteht aus drei verschachtelten Schalen in wirklicher Dicke, innen zuerst: Putz (Verweis auf die Körperplatte, ihre Raumseite als konditionierte Innenfläche markiert, C25 c), Ziegel (eine homogene 365-mm-Schale: die erklärte Schicht hat keine Fugen, ihre Leitfähigkeit ist die des Mauerwerks) und der Außenputz (Verweis auf die Körperplatte), dessen Wetterseite der eine Quadratmeter der Region ist. Ober-, Unter- und Stirnflächen sind Schnitte durch die weiterlaufende Wand."),
        t("Structure: the load path of masonry is compression through the bonded units, so the one member is the leaf exactly as it is laid: every brick, every bed, head and longitudinal joint of the block bond by reference to the body, without the plaster and the render, which carry nothing. The 712 kg of the strip go down to the fixed bearing, a 7.5 mm bed under the 365 mm leaf at the base anchor; the mass sits at the real centre of mass 201 mm from the room face and at half height.",
          "Tragwerk: der Lastpfad von Mauerwerk ist Druck durch die im Verband liegenden Steine, daher ist der eine Stab die Schale genau so, wie sie gemauert ist: jeder Stein, jede Lager-, Stoß- und Längsfuge des Blockverbands als Verweis auf den Körper, ohne Innen- und Außenputz, die nichts tragen. Die 712 kg des Streifens gehen zum eingespannten Auflager, einem 7,5-mm-Bett unter der 365-mm-Schale am Fußanker; die Masse sitzt im wirklichen Schwerpunkt 201 mm von der Raumseite und auf halber Höhe."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    masonry = (K.PLASTER[0], K.BRICK[1])
    body = [slab("masonry", masonry, "solid-brick", TONE["brick"]), slab("render", K.RENDER, "mineral-render", TONE["render"])]
    thermal = [ref("th-masonry", "masonry", "brick", ["-u"] + CUT), ref("th-render", "render", "render", ["-u"] + CUT)]
    structure = [leaf("member-brick-leaf"), support(0.015), mass(0.05)]
    decisions = [
        t("Body: two slabs. The 15 mm plaster joins the brick into one 380 mm masonry slab, as the 1:100 section draws it under the room-face pen; the courses and joints (10 to 12 mm) are below the 15 mm smallest feature, so the leaf is one slab; the 20 mm render stays a skin of its own.",
          "Körper: zwei Platten. Der 15-mm-Innenputz geht mit dem Ziegel in einer 380-mm-Mauerwerksplatte auf, wie der Schnitt 1:100 ihn unter dem Stift der Raumseite zeichnet; Schichten und Fugen (10 bis 12 mm) liegen unter der kleinsten Einzelheit von 15 mm, die Schale ist daher eine Platte; der 20-mm-Außenputz bleibt eine eigene Haut."),
        t("Thermal: two shells as references to the body slabs, masonry (carrying the merged plaster, room face marked as the conditioned interior face) and render; cut faces top, bottom and at both ends.",
          "Thermik: zwei Schalen als Verweise auf die Körperplatten, Mauerwerk (mit dem eingeschlossenen Putz, Raumseite als konditionierte Innenfläche markiert) und Außenputz; Schnittflächen oben, unten und an beiden Enden."),
        t("Structure: the 365 mm leaf as one member at its real place between plaster and render, on a 15 mm fixed bed under the leaf at the base anchor, and the mass at the centre of mass 201 mm from the room face.",
          "Tragwerk: die 365-mm-Schale als ein Stab an ihrem wirklichen Ort zwischen Innen- und Außenputz, auf einem 15-mm-Festbett unter der Schale am Fußanker, und die Masse im Schwerpunkt 201 mm von der Raumseite."),
        t("Minimal: a masonry slab, a render skin, one member, one bed and one mass are the whole of a plain rendered leaf at this scale; nothing can go without losing a drawn layer or the load path.",
          "Minimal: eine Mauerwerksplatte, eine Putzhaut, ein Stab, ein Bett und eine Masse sind in diesem Maßstab die ganze verputzte Schale; nichts kann entfallen, ohne eine gezeichnete Schicht oder den Lastpfad zu verlieren."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    wall = (K.PLASTER[0], K.RENDER[1])
    body = [slab("wall", wall, "solid-brick", 0.55)]
    thermal = [ref("th-wall", "wall", "brick", ["-u"] + CUT)]
    structure = [leaf("member-brick-leaf"), support(0.03), mass(0.06)]
    decisions = [
        t("Body: one 400 mm block. The 15 mm plaster and the 20 mm render are both below the 30 mm smallest feature and join the brick, as the 1:200 section draws one field; neither skin is thickened to make it visible.",
          "Körper: ein 400-mm-Block. Der 15-mm-Innenputz und der 20-mm-Außenputz liegen beide unter der kleinsten Einzelheit von 30 mm und gehen im Ziegel auf, wie der Schnitt 1:200 ein Feld zeichnet; keine der beiden Häute wird verdickt, um sichtbar zu werden."),
        t("Thermal: the block is the one shell of the region by reference (room face marked as the conditioned interior face, weather face 1 m²); the region keeps its three layers in its data.",
          "Thermik: der Block ist per Verweis die eine Schale der Region (Raumseite als konditionierte Innenfläche markiert, Wetterseite 1 m²); die Region behält in ihren Daten ihre drei Schichten."),
        t("Structure: the 365 mm leaf member inside the block (15 mm from the room face, 20 mm from the weather face), its 30 mm fixed bed at the base anchor and the mass at the centre of mass.",
          "Tragwerk: der 365-mm-Schalenstab im Block (15 mm von der Raumseite, 20 mm von der Wetterseite), sein 30-mm-Festbett am Fußanker und die Masse im Schwerpunkt."),
        t("Minimal: one block, one thermal shell, one member, one bed and one mass; a homogeneous wall has no more at this scale.",
          "Minimal: ein Block, eine Wärmeschale, ein Stab, ein Bett und eine Masse; mehr hat eine homogene Wand in diesem Maßstab nicht."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale500():
    wall = (K.PLASTER[0], K.RENDER[1])
    body = [slab("wall", wall, "solid-brick", 0.4)]
    thermal = [ref("th-wall", "wall", "brick", ["-u"] + CUT)]
    structure = [leaf("member-brick-leaf"), support(0.075), mass(0.075)]
    decisions = [
        t("Body: the 400 mm wall is one block, as the 1:500 section draws it as one dark poché field (0.8 mm on paper).",
          "Körper: die 400 mm dicke Wand ist ein Block, wie der Schnitt 1:500 sie als ein dunkel gefülltes Feld zeichnet (0,8 mm auf dem Papier)."),
        t("Thermal: the block is the one shell of the region by reference, room face marked as the conditioned interior face.",
          "Thermik: der Block ist per Verweis die eine Schale der Region, die Raumseite als konditionierte Innenfläche markiert."),
        t("Structure: the 365 mm leaf member inside the block, standing on its 75 mm fixed bed of the same width at the base anchor, with the mass at the centre of mass.",
          "Tragwerk: der 365-mm-Schalenstab im Block, auf seinem gleich breiten 75-mm-Festbett am Fußanker, mit der Masse im Schwerpunkt."),
        t("Minimal: one block, one thermal shell, one member, one bed and one mass.",
          "Minimal: ein Block, eine Wärmeschale, ein Stab, ein Bett und eine Masse."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
    with open(K.SOURCE, "w", encoding="utf-8") as f:
        f.write(json.dumps({"volumes": volumes}, indent=2, ensure_ascii=False) + "\n")
    for s, v in volumes.items():
        print(s, {k: len(v[k]) for k in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
