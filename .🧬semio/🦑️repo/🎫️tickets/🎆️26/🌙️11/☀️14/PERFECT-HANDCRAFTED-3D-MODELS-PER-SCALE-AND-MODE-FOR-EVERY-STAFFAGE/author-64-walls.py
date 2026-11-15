"""🧱 Author 64: hand-written decisions of the brick wall family (etics-brick-wall-12, etics-brick-wall-30, solid-brick-wall-1960 at 1:100 and 1:500 body). Atomic, skipped when the source was touched in the last 20 minutes (unless --force-age)."""
import json, os, sys, time
T = os.path.dirname(os.path.abspath(__file__))
D = {}
D["components/etics-brick-wall-12"] = {
 "50": [
  ("Body: the 12 cm ETICS wall as built, 520 mm from room face to render: 15 mm lime plaster, the 365 mm brick leaf cut into 13 courses (the lowest 70 mm, eleven of 65 mm, the top one 95 mm so that the 1 m sample closes) separated by 12 mortar bed joints of 10 mm in a lighter tone, as the joint lines of the drawing, the 120 mm mineral wool board and the 20 mm mineral render, each a full 1 m by 1 m slab.",
   "Körper: die Ziegelwand mit 12 cm WDVS wie gebaut, 520 mm von der Raumseite bis zum Putz: 15 mm Kalkputz, die 365-mm-Ziegelschale in 13 Schichten (die unterste 70 mm, elf zu 65 mm, die oberste 95 mm, damit die 1-m-Probe aufgeht), getrennt durch 12 Mörtellagerfugen von 10 mm in hellerem Ton wie die Fugenlinien der Zeichnung, die 120-mm-Mineralwolleplatte und der 20-mm-Mineralputz, jeweils eine volle Platte von 1 m mal 1 m."),
  ("Thermal: the single region wall-field (1 m2) is four layer shells, innermost first (plaster, brick, insulation, render) with the resistances 0.021, 0.487, 3.429 and 0.023 m2K/W, stacked as slabs of their real thicknesses; plaster, insulation and render are references to the body slabs, only the brick shell is a slab of its own because the bed joints lie inside that layer instead of splitting it.",
   "Thermik: die einzige Region wall-field (1 m2) besteht aus vier Schichtschalen, innen zuerst (Putz, Ziegel, Dämmung, Außenputz) mit den Widerständen 0,021, 0,487, 3,429 und 0,023 m2K/W, gestapelt als Platten ihrer wirklichen Dicken; Putz, Dämmung und Außenputz verweisen auf die Körperplatten, nur die Ziegelschale ist eine eigene Platte, weil die Lagerfugen innerhalb dieser Schicht liegen und sie nicht teilen."),
  ("Structure: only the brick leaf carries load, so its 13 courses are the 13 member solids (each 365 mm deep over the metre); plaster, wool and render are cladding and absent, and one 7.5 mm bearing block under the leaf is the fixed support at the base anchor of the 719 kg strip.",
   "Tragwerk: nur die Ziegelschale trägt, daher sind ihre 13 Schichten die 13 Stabkörper (je 365 mm tief auf den Meter); Putz, Wolle und Außenputz sind Verkleidung und fehlen, und ein 7,5-mm-Lagerblock unter der Schale ist das feste Auflager am Fußanker des 719 kg schweren Streifens."),
 ],
 "100": [
  ("Body: plaster, courses and bed joints merge into one 380 mm masonry slab because the 1:100 drawing hatches the leaf as a single field; the 120 mm insulation and the 20 mm render stay separate slabs, the render being just above the 15 mm feature.",
   "Körper: Putz, Schichten und Lagerfugen gehen in einer 380 mm dicken Mauerwerksplatte auf, weil die Zeichnung 1:100 die Schale als ein Feld schraffiert; die 120-mm-Dämmung und der 20-mm-Außenputz bleiben getrennte Platten, der Putz knapp über der Einzelheit 15 mm."),
  ("Thermal and structure: three stacked slabs (masonry with the plaster merged in, insulation, render) keep the layer tags of the drawing; the room-side face of the masonry slab is a section face (-u) so the region counts the one exposed wall face, as in the 1:50 model. The leaf is one 380 mm member on a 15 mm base block; one member is right for a wall whose other layers are cladding.",
   "Thermik und Tragwerk: drei gestapelte Platten (Mauerwerk mit eingearbeitetem Putz, Dämmung, Außenputz) behalten die Schichtmarken der Zeichnung; die raumseitige Fläche der Mauerwerksplatte ist eine Schnittfläche (-u), damit die Region die eine freie Wandfläche zählt wie im 1:50-Modell. Die Schale ist ein 380-mm-Stab auf einem 15-mm-Fußblock; ein Stab ist richtig für eine Wand, deren übrige Schichten Verkleidung sind."),
 ],
 "200": [
  ("Body: the 20 mm render is under the 30 mm feature and is merged into the insulation, giving a 140 mm cladding slab next to the 380 mm masonry leaf, which is what the 1:200 drawing shows (two fields).",
   "Körper: der 20-mm-Außenputz liegt unter der Einzelheit 30 mm und geht in der Dämmung auf, es entsteht eine 140-mm-Verkleidungsplatte neben der 380-mm-Mauerschale, wie es die Zeichnung 1:200 zeigt (zwei Felder)."),
  ("Thermal and structure: masonry and cladding are the two slabs that give the region its coverage, the room-side face of the masonry stays a section face, and the leaf member sits on a 30 mm base block (one member, one support).",
   "Thermik und Tragwerk: Mauerwerk und Verkleidung sind die zwei Platten, die der Region ihre Abdeckung geben, die raumseitige Fläche des Mauerwerks bleibt Schnittfläche, und der Schalenstab steht auf einem 30-mm-Fußblock (ein Stab, ein Auflager)."),
  ("Minimal: two field slabs, one member and one support are the least that keeps the field boundary the 1:200 drawing still draws; merging the masonry with the insulation would erase it.",
   "Minimal: zwei Feldplatten, ein Stab und ein Auflager sind das Wenigste, das die Feldgrenze behält, die die Zeichnung 1:200 noch zeichnet; Mauerwerk und Dämmung zusammenzulegen würde sie löschen."),
 ],
 "500": [
  ("Body: the whole 520 mm wall is one block because the 1:500 drawing is a single rectangle; the 120 mm wool is thicker than the 75 mm feature but is not drawn apart.",
   "Körper: die ganze 520 mm dicke Wand ist ein Block, weil die Zeichnung 1:500 ein einziges Rechteck ist; die 120 mm Wolle sind dicker als die Einzelheit 75 mm, werden aber nicht getrennt gezeichnet."),
  ("Thermal and structure: one wall solid carries the region wall-field; the 380 mm masonry leaf is the member inside the block on a 75 mm base block, and the room-side face is a section face as at the finer scales.",
   "Thermik und Tragwerk: ein Wandkörper trägt die Region wall-field; die 380-mm-Mauerschale ist der Stab im Block auf einem 75-mm-Fußblock, und die raumseitige Fläche ist Schnittfläche wie in den feineren Maßstäben."),
  ("Minimal: one 520 mm block with one thermal solid, one member and one support is the least that still holds the four-layer region, the load path and the base.",
   "Minimal: ein 520-mm-Block mit einem Thermalkörper, einem Stab und einem Auflager ist das Wenigste, das die vierschichtige Region, den Lastpfad und den Fuß noch hält."),
 ],
}
D["components/etics-brick-wall-30"] = {
 "50": [
  ("Body: the 30 cm ETICS wall as built, 700 mm in total and dominated by its insulation: 15 mm lime plaster, the 365 mm brick leaf in 13 courses (70 mm at the foot, eleven of 65 mm, 95 mm at the top to close the 1 m sample) with twelve 10 mm bed joints in a lighter tone, a 300 mm mineral wool board that is almost as thick as the leaf, and the 20 mm mineral render, all full 1 m by 1 m slabs.",
   "Körper: die Ziegelwand mit 30 cm WDVS wie gebaut, insgesamt 700 mm und von ihrer Dämmung geprägt: 15 mm Kalkputz, die 365-mm-Ziegelschale in 13 Schichten (70 mm am Fuß, elf zu 65 mm, 95 mm oben, damit die 1-m-Probe aufgeht) mit zwölf 10-mm-Lagerfugen in hellerem Ton, eine 300-mm-Mineralwolleplatte, die fast so dick ist wie die Schale, und der 20-mm-Mineralputz, alles volle Platten von 1 m mal 1 m."),
  ("Thermal: the region wall-field (1 m2) has the same four shells as every layered wall, but here the 300 mm wool shell with its resistance of 8.571 m2K/W is the thickest and by far the most resistant; plaster, insulation and render are references to the body slabs, the brick shell is one slab of 365 mm that contains its bed joints.",
   "Thermik: die Region wall-field (1 m2) hat dieselben vier Schalen wie jede geschichtete Wand, doch hier ist die 300-mm-Wolleschale mit dem Widerstand 8,571 m2K/W die dickste und bei weitem die dämmendste; Putz, Dämmung und Außenputz verweisen auf die Körperplatten, die Ziegelschale ist eine 365-mm-Platte, die ihre Lagerfugen einschließt."),
  ("Structure: the thirteen brick courses are the member solids and the only load path of the 730 kg strip, the 300 mm of wool and the render carry nothing and are absent; a 7.5 mm fixed bearing block under the courses stands at the base anchor.",
   "Tragwerk: die dreizehn Ziegelschichten sind die Stabkörper und der einzige Lastpfad des 730 kg schweren Streifens, die 300 mm Wolle und der Außenputz tragen nichts und fehlen; ein festes 7,5-mm-Lagerblock unter den Schichten steht am Fußanker."),
 ],
 "100": [
  ("Body: the plaster and the 13 courses with their joints fold into one 380 mm masonry slab (the 1:100 drawing hatches one leaf); the 300 mm insulation slab is the thickest part of the wall and stays, and so does the 20 mm render slab above the 15 mm feature.",
   "Körper: Putz und die 13 Schichten mit ihren Fugen falten sich zu einer 380-mm-Mauerwerksplatte (die Zeichnung 1:100 schraffiert eine Schale); die 300-mm-Dämmplatte ist der dickste Teil der Wand und bleibt, ebenso die 20-mm-Putzplatte oberhalb der Einzelheit 15 mm."),
  ("Thermal and structure: masonry, 300 mm insulation and render are the three stacked slabs with the drawing's layer tags; the section face (-u) on the room side of the masonry keeps the region at its one exposed face. The 380 mm leaf is the single member, resting on a 15 mm base block.",
   "Thermik und Tragwerk: Mauerwerk, 300-mm-Dämmung und Außenputz sind die drei gestapelten Platten mit den Schichtmarken der Zeichnung; die Schnittfläche (-u) an der Raumseite des Mauerwerks hält die Region bei ihrer einen freien Fläche. Die 380-mm-Schale ist der einzige Stab und ruht auf einem 15-mm-Fußblock."),
 ],
 "200": [
  ("Body: render and insulation become one 320 mm cladding slab (the 20 mm render is under the 30 mm feature), which is nearly as thick as the 380 mm masonry leaf beside it; the drawing at 1:200 shows these two fields only.",
   "Körper: Außenputz und Dämmung werden eine 320-mm-Verkleidungsplatte (der 20-mm-Putz liegt unter der Einzelheit 30 mm), die fast so dick ist wie die 380-mm-Mauerschale daneben; die Zeichnung 1:200 zeigt nur diese zwei Felder."),
  ("Thermal and structure: the masonry slab and the 320 mm cladding slab cover the region, the section face stays on the room side of the masonry, and the single leaf member stands on a 30 mm base block.",
   "Thermik und Tragwerk: die Mauerwerksplatte und die 320-mm-Verkleidungsplatte decken die Region ab, die Schnittfläche bleibt an der Raumseite des Mauerwerks, und der einzelne Schalenstab steht auf einem 30-mm-Fußblock."),
  ("Minimal: a masonry field, a thick cladding field, one member and one support; the thick wool must stay a field of its own because it is the larger part of the drawn wall width.",
   "Minimal: ein Mauerwerksfeld, ein dickes Verkleidungsfeld, ein Stab und ein Auflager; die dicke Wolle muss ein eigenes Feld bleiben, weil sie den größeren Teil der gezeichneten Wandbreite ausmacht."),
 ],
 "500": [
  ("Body: the 700 mm wall is drawn as one rectangle at 1:500 and is therefore one block; the 300 mm wool that makes up over 40 % of the width is not drawn apart from the masonry any more.",
   "Körper: die 700 mm dicke Wand ist bei 1:500 ein Rechteck und daher ein Block; die 300 mm Wolle, die über 40 % der Breite ausmachen, sind nicht mehr vom Mauerwerk getrennt gezeichnet."),
  ("Thermal and structure: one solid of 700 mm carries the region wall-field, the 380 mm leaf member lies inside it on a 75 mm base block, and the section face on the room side is kept.",
   "Thermik und Tragwerk: ein 700-mm-Körper trägt die Region wall-field, der 380-mm-Schalenstab liegt darin auf einem 75-mm-Fußblock, und die Schnittfläche an der Raumseite bleibt erhalten."),
  ("Minimal: one 700 mm block, one thermal solid, one member and one support; no coarser model keeps both a heat region and a load path.",
   "Minimal: ein 700-mm-Block, ein Thermalkörper, ein Stab und ein Auflager; kein gröberes Modell behält zugleich eine Wärmeregion und einen Lastpfad."),
 ],
}
SB = "components/solid-brick-wall-1960"
SOLID = {
 "100": (0, ("Body: the 1960 wall has no insulation, so at 1:100 it is just the 380 mm masonry slab (plaster, courses and joints merged, the drawing hatches one leaf) and a separate 20 mm render skin that stays above the 15 mm feature.",
             "Körper: die Wand von 1960 hat keine Dämmung, daher besteht sie bei 1:100 nur aus der 380-mm-Mauerwerksplatte (Putz, Schichten und Fugen zusammengefasst, die Zeichnung schraffiert eine Schale) und einer getrennten 20-mm-Putzhaut oberhalb der Einzelheit 15 mm.")),
 "500": (0, ("Body: the 400 mm uninsulated wall is a single block at 1:500, as the drawing is one rectangle; plaster, leaf and render are not distinguishable at the 75 mm feature.",
             "Körper: die 400 mm dicke ungedämmte Wand ist bei 1:500 ein einziger Block, wie die Zeichnung ein Rechteck ist; Putz, Schale und Außenputz sind bei der Einzelheit 75 mm nicht unterscheidbar.")),
}
force = "--force-age" in sys.argv
for key, scales in list(D.items()) + [(SB, None)]:
    path = f"{T}/volumes/{key}.json"
    if (time.time() - os.path.getmtime(path)) / 60 < 20 and not force:
        print("SKIP", key); continue
    raw = open(path).read()
    src = json.loads(raw)
    if scales:
        for s, items in scales.items():
            src["volumes"][s]["decisions"] = [{"en": e, "de": d} for e, d in items]
    else:
        for s, (i, (e, d)) in SOLID.items():
            src["volumes"][s]["decisions"][i] = {"en": e, "de": d}
    out = json.dumps(src, ensure_ascii=False, separators=(",", ":")) + ("\n" if raw.endswith("\n") else "")
    if out == raw:
        print("UNCHANGED", key); continue
    assert open(path).read() == raw
    open(path, "w").write(out)
    print("WROTE", key)
