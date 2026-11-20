"""🏗️ Round 3 volumes of components/etics-brick-wall-12, stated by hand per scale and list from the real build-up in the kit; writes `volumes/components/etics-brick-wall-12.json`."""
import json
import os
import importlib.util

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-etics-brick-wall-12-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

FULL_V = (0.0, K.HEIGHT)
FULL_W = K.WALL_W
TONE = {"plaster": 0.9, "brick": 0.5, "mortar": 0.78, "wool": 0.88, "render": 0.8}


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


def wool_boards50():
    """🟨 The 800 x 625 mm mineral wool boards in stretcher bond: lower row joint at w 0.15, upper row joint at w -0.25, row joint at v 0.40."""
    rows = [("low", (0.0, K.BOARD_JOINT_V), K.BOARD_JOINT_LOW_W), ("high", (K.BOARD_JOINT_V, K.HEIGHT), K.BOARD_JOINT_HIGH_W)]
    out = []
    for name, v, j in rows:
        out.append(slab(f"wool-board-{name}-left", K.WOOL, "mineral-wool", TONE["wool"], v=v, w=(K.WALL_W[0], j)))
        out.append(slab(f"wool-board-{name}-right", K.WOOL, "mineral-wool", TONE["wool"], v=v, w=(j, K.WALL_W[1])))
    return out


def dowel_revolve(id_, w, v, plate):
    """📌 A plate anchor along u: shank from 50 mm inside the brick to the wool face, with its 60 mm plate flush under the render when `plate`."""
    tip, face = K.DOWEL_TIP_U, K.WOOL[1]
    if plate:
        profile = [[tip, 0.0], [tip, K.DOWEL_SHANK_R], [face - K.DOWEL_PLATE_T, K.DOWEL_SHANK_R], [face - K.DOWEL_PLATE_T, K.DOWEL_PLATE_R], [face, K.DOWEL_PLATE_R], [face, 0.0]]
    else:
        profile = [[tip, 0.0], [tip, K.DOWEL_SHANK_R], [face, K.DOWEL_SHANK_R], [face, 0.0]]
    out = {"id": id_, "kind": "revolve", "bone": "root", "material": "etics-anchor", "role": "detail", "tone": 0.3, "profile": [[K.r5(a), K.r5(r)] for a, r in profile], "axis": [1, 0, 0], "origin": [0.0, K.r5(v), K.r5(w)], "member": "root", "part": "joint"}
    if not plate:
        out["slender"] = True
    return out


def leaf(id_):
    return K.box(id_, K.BRICK, FULL_V, FULL_W, "solid-brick", 0.5, member="root", part="member")


def support(thick):
    return K.box("support-base-fixed", K.BRICK, (-thick, 0.0), FULL_W, "masonry-mortar", 0.2, member="root", part="support", anchor="base")


def mass(r):
    return {"id": "mass-wall-strip", "kind": "ellipsoid", "bone": "root", "material": "solid-brick", "role": "body", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [K.r5(K.COM_U), K.COM_V, 0.0]}, "member": "root", "part": "mass"}


def ref(id_, of, layer, faces):
    return {"id": id_, "of": of, "region": "wall-field", "layer": layer, "sectionFaces": faces}


CUT = ["-v", "+v", "-w", "+w"]


def scale50():
    body = [slab("plaster", K.PLASTER, "lime-plaster", TONE["plaster"])] + masonry50() + wool_boards50() + [slab("render", K.RENDER, "mineral-render", TONE["render"])]
    thermal = [
        ref("th-plaster", "plaster", "plaster", ["-u"] + CUT),
        slab("th-brick", K.BRICK, "solid-brick", 0.5, region="wall-field", layer="brick"),
        slab("th-insulation", K.WOOL, "mineral-wool", TONE["wool"], region="wall-field", layer="insulation"),
        ref("th-render", "render", "render", ["-u"] + CUT),
    ]
    structure = [leaf("member-brick-leaf"), support(0.0075), mass(0.04)] + [dowel_revolve(f"joint-anchor-{i + 1}", w, v, True) for i, (w, v) in enumerate(K.DOWELS)]
    decisions = [
        t("Body: the wall as built in a 1 m x 1 m sample, 520 mm thick. 15 mm lime plaster; the 365 mm solid brick leaf of 1960 in the NF block bond (240 x 115 x 71 mm bricks, twelve courses of 1/12 m with 12.3 mm bed joints and 10 mm head joints): one course has the stretchers on the room side and the headers outside, the next the reverse, so the 10 mm longitudinal joint changes side from course to course and the head joints of neighbouring courses are offset by a quarter brick; the bricks at both ends of the sample are cut. The first course stands on its mortar bed. Then four 120 mm mineral wool boards of 800 x 625 mm in stretcher bond (row joint at 0.40 m, vertical joints 400 mm apart) and the 20 mm mineral render. Cut faces are flush, as a saw cut leaves them: the joints show by their lighter mortar tone.",
          "Körper: die Wand wie gebaut als Probe von 1 m x 1 m, 520 mm dick. 15 mm Kalkputz; die 365-mm-Vollziegelschale von 1960 im NF-Blockverband (Steine 240 x 115 x 71 mm, zwölf Schichten zu 1/12 m mit 12,3-mm-Lagerfugen und 10-mm-Stoßfugen): eine Schicht hat die Läufer raumseitig und die Binder außen, die nächste umgekehrt, so dass die 10-mm-Längsfuge von Schicht zu Schicht die Seite wechselt und die Stoßfugen benachbarter Schichten um einen Viertelstein versetzt sind; die Steine an beiden Enden der Probe sind geschnitten. Die erste Schicht steht auf ihrem Mörtelbett. Dann vier 120-mm-Mineralwolleplatten von 800 x 625 mm im Läuferverband (Reihenfuge bei 0,40 m, senkrechte Fugen 400 mm versetzt) und der 20-mm-Mineralputz. Die Schnittflächen sind eben, wie ein Sägeschnitt sie hinterlässt: die Fugen zeigen sich durch den helleren Mörtelton."),
        t("Thermal: the region wall-field is four nested shells at their real thickness, innermost first: plaster (reference to the body slab, its room face marked as the conditioned interior face, C25 c), brick (one homogeneous 365 mm shell: the declared layer has no joints), the 120 mm wool (one shell over the four boards, which are butt-jointed without a gap) and the render (reference to the body slab). The top, bottom and end faces are cuts through the continuing wall.",
          "Thermik: die Region wall-field besteht aus vier verschachtelten Schalen in wirklicher Dicke, innen zuerst: Putz (Verweis auf die Körperplatte, ihre Raumseite als konditionierte Innenfläche markiert, C25 c), Ziegel (eine homogene 365-mm-Schale: die erklärte Schicht hat keine Fugen), die 120-mm-Wolle (eine Schale über den vier stumpf gestoßenen Platten) und der Außenputz (Verweis auf die Körperplatte). Ober-, Unter- und Stirnflächen sind Schnitte durch die weiterlaufende Wand."),
        t("Structure: the 365 mm brick leaf with its mortar is the only member and carries the 719 kg of the strip down to the fixed bearing, a 7.5 mm bed under the leaf at the base anchor; the mass sits at the real centre of mass 209 mm from the room face (inside the leaf, because the wool weighs 7 of the 719 kg). The ETICS carries nothing but hangs on the leaf: its fixing is shown by the four plate anchors of the T scheme that fall into this sample (60 mm plate flush with the wool face, 8 mm shank reaching 50 mm into the brick), the joints of the cladding to the member.",
          "Tragwerk: die 365-mm-Ziegelschale mit ihrem Mörtel ist der einzige Stab und trägt die 719 kg des Streifens zum eingespannten Auflager, einem 7,5-mm-Bett unter der Schale am Fußanker; die Masse sitzt im wirklichen Schwerpunkt 209 mm von der Raumseite (in der Schale, denn die Wolle wiegt 7 der 719 kg). Das WDVS trägt nichts, sondern hängt an der Schale: seine Befestigung zeigen die vier Tellerdübel des T-Schemas, die in diese Probe fallen (60-mm-Teller bündig mit der Wollfläche, 8-mm-Schaft 50 mm tief im Ziegel), die Gelenke der Bekleidung zum Stab."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    masonry = (K.PLASTER[0], K.BRICK[1])
    body = [slab("masonry", masonry, "solid-brick", TONE["brick"]), slab("insulation", K.WOOL, "mineral-wool", TONE["wool"]), slab("render", K.RENDER, "mineral-render", TONE["render"])]
    thermal = [ref("th-masonry", "masonry", "brick", ["-u"] + CUT), ref("th-insulation", "insulation", "insulation", CUT), ref("th-render", "render", "render", CUT)]
    structure = [leaf("member-brick-leaf"), support(0.015), mass(0.05)] + [dowel_revolve(f"joint-anchor-{i + 1}", w, v, False) for i, (w, v) in enumerate(K.DOWELS)]
    decisions = [
        t("Body: three slabs. The 15 mm plaster joins the brick into one 380 mm masonry slab, as the 1:100 section draws it under the room-face pen; the courses and joints (10 to 12 mm) and the board joints are below the 15 mm smallest feature and the leaf is one slab; the 120 mm wool and the 20 mm render stay apart.",
          "Körper: drei Platten. Der 15-mm-Innenputz geht mit dem Ziegel in einer 380-mm-Mauerwerksplatte auf, wie der Schnitt 1:100 ihn unter dem Stift der Raumseite zeichnet; Schichten und Fugen (10 bis 12 mm) und die Plattenfugen liegen unter der kleinsten Einzelheit von 15 mm, die Schale ist eine Platte; die 120-mm-Wolle und der 20-mm-Putz bleiben getrennt."),
        t("Thermal: three shells as references to the body slabs, masonry (carrying the merged plaster, room face marked as the conditioned interior face), wool and render; cut faces top, bottom and at both ends.",
          "Thermik: drei Schalen als Verweise auf die Körperplatten, Mauerwerk (mit dem eingeschlossenen Putz, Raumseite als konditionierte Innenfläche markiert), Wolle und Putz; Schnittflächen oben, unten und an beiden Enden."),
        t("Structure: the 365 mm leaf member on a 15 mm fixed bed at the base anchor, the mass at the centre of mass 209 mm from the room face, and the four anchors as 8 mm shanks (slender, at true size; the 60 mm plates are thinner than the 15 mm feature).",
          "Tragwerk: der 365-mm-Schalenstab auf einem 15-mm-Festbett am Fußanker, die Masse im Schwerpunkt 209 mm von der Raumseite und die vier Dübel als 8-mm-Schäfte (schlank, in wirklicher Größe; die 60-mm-Teller sind dünner als die Einzelheit 15 mm)."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    masonry = (K.PLASTER[0], K.BRICK[1])
    cladding = (K.WOOL[0], K.RENDER[1])
    body = [slab("masonry", masonry, "solid-brick", TONE["brick"]), slab("etics", cladding, "mineral-wool", TONE["wool"])]
    thermal = [ref("th-masonry", "masonry", "brick", ["-u"] + CUT), ref("th-etics", "etics", "insulation", CUT)]
    structure = [leaf("member-brick-leaf"), support(0.03), mass(0.06)]
    decisions = [
        t("Body: two slabs, the 380 mm masonry with its plaster and the 140 mm ETICS of wool and render; the render (20 mm) and the plaster (15 mm) are under the 30 mm feature and join their neighbours, as the 1:200 section draws two fields.",
          "Körper: zwei Platten, das 380-mm-Mauerwerk mit seinem Putz und das 140-mm-WDVS aus Wolle und Putz; Außenputz (20 mm) und Innenputz (15 mm) liegen unter der Einzelheit 30 mm und gehen in ihren Nachbarn auf, wie der Schnitt 1:200 zwei Felder zeichnet."),
        t("Thermal: the two body slabs as shells by reference, masonry (room face marked as the conditioned interior face) and ETICS; the region keeps all four layers in its data.",
          "Thermik: die zwei Körperplatten als Schalen per Verweis, Mauerwerk (Raumseite als konditionierte Innenfläche markiert) und WDVS; die Region behält in ihren Daten alle vier Schichten."),
        t("Structure: the 365 mm leaf member on a 30 mm fixed bed at the base anchor and the mass at the centre of mass; the anchors (8 mm) are dropped below the 30 mm feature, the weight of the ETICS is in the mass.",
          "Tragwerk: der 365-mm-Schalenstab auf einem 30-mm-Festbett am Fußanker und die Masse im Schwerpunkt; die Dübel (8 mm) entfallen unter der Einzelheit 30 mm, das Gewicht des WDVS steckt in der Masse."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale500():
    wall = (K.PLASTER[0], K.RENDER[1])
    masonry = (K.PLASTER[0], K.BRICK[1])
    cladding = (K.WOOL[0], K.RENDER[1])
    body = [slab("wall", wall, "solid-brick", 0.6)]
    thermal = [slab("th-masonry", masonry, "solid-brick", TONE["brick"], extra_faces=["-u"], region="wall-field", layer="brick"), slab("th-etics", cladding, "mineral-wool", TONE["wool"], region="wall-field", layer="insulation")]
    structure = [leaf("member-brick-leaf"), support(0.075), mass(0.075)]
    decisions = [
        t("Body: the 520 mm wall is one block, as the 1:500 section draws it as one poché field (1.04 mm on paper).",
          "Körper: die 520 mm dicke Wand ist ein Block, wie der Schnitt 1:500 sie als ein gefülltes Feld zeichnet (1,04 mm auf dem Papier)."),
        t("Thermal: two shells inside the block, the 380 mm masonry with its plaster (room face marked as the conditioned interior face) and the 140 mm ETICS: the 120 mm wool is more than one and a half times the 75 mm feature and is the layer that makes this wall, so the thermal model keeps it although the body does not draw it apart.",
          "Thermik: zwei Schalen im Block, das 380-mm-Mauerwerk mit seinem Putz (Raumseite als konditionierte Innenfläche markiert) und das 140-mm-WDVS: die 120-mm-Wolle ist mehr als das Anderthalbfache der Einzelheit 75 mm und ist die Schicht, die diese Wand ausmacht, daher behält das Wärmemodell sie, obwohl der Körper sie nicht abtrennt."),
        t("Structure: the 365 mm leaf member, its 75 mm fixed bed at the base anchor and the mass at the centre of mass.",
          "Tragwerk: der 365-mm-Schalenstab, sein 75-mm-Festbett am Fußanker und die Masse im Schwerpunkt."),
        t("Minimal: one block, two thermal shells, one member, one support and one mass; the thermal and structural lists cannot lose a part without losing the insulation or the load path.",
          "Minimal: ein Block, zwei Wärmeschalen, ein Stab, ein Auflager und eine Masse; Wärme- und Tragwerksliste können keinen Teil verlieren, ohne die Dämmung oder den Lastpfad zu verlieren."),
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
