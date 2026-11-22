"""📝 Market parasol, second round pass 1: decisions and descriptions rewritten for the slab base with socket tube and the knuckle under the runner.

Usage: python3 rework-market-parasol-texts.py [--publish]   (without --publish it only lists the remaining stale words)
"""
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json"
SOURCE = os.path.join(T, "volumes/objects/market-parasol.json")

MODEL = {
    "50": {
        "en": "Own 1:50 model of the real market parasol in the open-still pose: concrete ballast slab 500 x 500 x 96 mm with 12 mm rounded edges, steel flange 200 x 200 x 10 mm with four bolts, welded socket tube 60 mm with rim ring, four 8 mm gussets and two clamp knobs; the 48 mm pole stands 0.29 m deep in the socket, coupling collar 68 mm with release knob at 1.07 m, 40 mm pole up to the tilt knuckle (56 mm barrel with pin) at 1.80 m, mast head 40 mm; crank winch with housing, bent arm and handle at 0.92 m; runner sleeve with the stretcher notch ring; crown hub with domed top cap and finial; eight 16 x 10 mm ribs 1.37 m long at 45 degrees with tip caps and stretcher brackets; eight 11 mm stretchers; eight gores of 0.5 mm cloth, so the canopy is the taut octagonal pyramid of a real market parasol with ridges over the ribs.",
        "de": "Eigenes 1:50-Modell des realen Marktschirms in der Pose open-still: Betonballastplatte 500 x 500 x 96 mm mit 12 mm gerundeten Kanten, Stahlflansch 200 x 200 x 10 mm mit vier Schrauben, angeschweißtes Standrohr 60 mm mit Randring, vier 8-mm-Knotenblechen und zwei Klemmknäufen; das 48-mm-Rohr steht 0,29 m tief im Standrohr, Kupplungsmuffe 68 mm mit Löseknauf auf 1,07 m, 40-mm-Rohr bis zum Knickgelenk (56-mm-Walze mit Bolzen) auf 1,80 m, Mastkopf 40 mm; Kurbelwinde mit Gehäuse, gekröpftem Arm und Griff auf 0,92 m; Läuferhülse mit Kerbring für die Spreizen; Krone mit gewölbter Kappe und Knauf; acht Speichen 16 x 10 mm, 1,37 m lang, im 45-Grad-Raster mit Endkappen und Spreizenlaschen; acht 11-mm-Spreizen; acht Bahnen aus 0,5-mm-Stoff, sodass das Dach die gespannte achteckige Pyramide eines realen Marktschirms mit Graten über den Speichen ist.",
    },
    "100": {
        "en": "Own 1:100 model: slab, 15 mm flange, the socket as one 60 mm cylinder (gussets, knobs and bolts are below the 15 mm feature and merged away), one 44 mm mast tube from the socket to the knuckle (the 4 mm step between 48 and 40 mm is below the feature), coupling collar, winch housing with crank as one bent part, knuckle barrel 56 mm, mast head, plain runner sleeve, crown hub merged with top cap and finial, eight ribs and eight stretchers as true-size slender bars under the canopy, eight gores of 0.5 mm cloth.",
        "de": "Eigenes 1:100-Modell: Platte, 15-mm-Flansch, das Standrohr als ein 60-mm-Zylinder (Knotenbleche, Knäufe und Schrauben liegen unter dem Mindestmaß 15 mm und entfallen), ein 44-mm-Mastrohr vom Standrohr bis zum Knickgelenk (der 4-mm-Absatz zwischen 48 und 40 mm liegt unter dem Mindestmaß), Kupplungsmuffe, Windengehäuse mit Kurbel als ein gekröpftes Teil, Gelenkwalze 56 mm, Mastkopf, glatte Läuferhülse, Krone mit Kappe und Knauf verschmolzen, acht Speichen und acht Spreizen als maßhaltige schlanke Stäbe unter dem Schirm, acht Bahnen aus 0,5-mm-Stoff.",
    },
    "200": {
        "en": "Own 1:200 model: slab, socket as one 70 mm cylinder standing on it, one 44 mm mast to the knuckle, knuckle barrel 60 mm, mast head, runner sleeve, crown merged with its cap, eight true-size ribs and stretchers as slender bars and eight gores; flange, collar, winch and crank are below the 30 mm feature and left out.",
        "de": "Eigenes 1:200-Modell: Platte, Standrohr als ein 70-mm-Zylinder darauf, ein 44-mm-Mast bis zum Knickgelenk, Gelenkwalze 60 mm, Mastkopf, Läuferhülse, Krone mit Kappe verschmolzen, acht maßhaltige Speichen und Spreizen als schlanke Stäbe und acht Bahnen; Flansch, Muffe, Winde und Kurbel liegen unter dem Mindestmaß 30 mm und entfallen.",
    },
    "500": {
        "en": "Own 1:500 model: the 96 mm slab, the mast as one true 44 mm slender bar from the slab to the knuckle, the short mast head under the canopy thickened to the 75 mm feature, crown with cap as one plug, eight true-size ribs and stretchers as slender bars and eight gores; socket, knuckle barrel, runner and all fittings are below the 75 mm feature and left out.",
        "de": "Eigenes 1:500-Modell: die 96-mm-Platte, der Mast als ein maßhaltiger schlanker 44-mm-Stab von der Platte bis zum Knickgelenk, der kurze Mastkopf unter dem Schirm auf das Mindestmaß 75 mm verdickt, Krone mit Kappe als ein Zapfen, acht maßhaltige Speichen und Spreizen als schlanke Stäbe und acht Bahnen; Standrohr, Gelenkwalze, Läufer und alle Beschläge liegen unter dem Mindestmaß 75 mm und entfallen.",
    },
}
STRUCTURE = {
    "50": {
        "en": "Structure: the load path of the real frame: the ballast slab as member with its mass and the fixed support under it, flange, socket tube and gussets as the welded foot, the clamp as the joint that holds the pole in the socket, the pole as hollow tubes (48 x 2 below, 40 x 1.5 above the coupling collar, which is the splice joint), the knuckle barrel with its pin as the joint to the mast head, crown hub and runner as joints, the eight ribs and eight stretchers as members with the rib brackets as their pin joints, and the masses of pole, mast head, runner and every rib at their centres. The cord that holds the runner up runs inside the pole to the winch and is not modelled.",
        "de": "Tragwerk: der Lastpfad des realen Gestells: die Ballastplatte als Stab mit ihrer Masse und dem festen Auflager darunter, Flansch, Standrohr und Knotenbleche als geschweißter Fuß, die Klemmung als Gelenk, das den Mast im Standrohr hält, der Mast als Hohlrohre (48 x 2 unten, 40 x 1,5 über der Kupplungsmuffe, dem Stoßgelenk), die Gelenkwalze mit Bolzen als Gelenk zum Mastkopf, Krone und Läufer als Gelenke, die acht Speichen und acht Spreizen als Stäbe mit den Speichenlaschen als ihren Bolzengelenken, und die Massen von Mast, Mastkopf, Läufer und jeder Speiche in ihren Schwerpunkten. Das Seil, das den Läufer hält, läuft im Mast zur Winde und ist nicht modelliert.",
    },
}
TILT = {
    "en": "The pole stands fixed in the base socket and carries winch and crank at 0.92 m; only the mast head with crown, runner and canopy tilts, up to 30 degrees, in the knuckle at 1.80 m just below the open runner, and the furled runner slides down over the straightened knuckle.",
    "de": "Der Mast steht fest im Standrohr des Fußes und trägt Winde und Kurbel auf 0,92 m; nur der Mastkopf mit Krone, Läufer und Schirmdach neigt sich, um bis zu 30 Grad, im Knickgelenk auf 1,80 m knapp unter dem offenen Läufer, und der geschlossene Läufer gleitet über das gestreckte Gelenk nach unten.",
}
REPLACE = [
    ("The base is a 0.4 x 0.3 x 0.2 m concrete block (about 58 kg) with a 15 mm aluminium cover plate and a clevis bracket; the mast tilts on a horizontal pin 60 mm above the plate (the analysis", "The base is a 0.5 x 0.5 x 0.096 m concrete slab (about 58 kg) with a 10 mm steel flange and a welded 60 mm socket tube 0.29 m high, in which the pole is clamped by two knobs; the canopy head tilts in a knuckle 1.80 m above ground, just below the open runner (the analysis"),
    ("crank winch at 1.38 m", "crank winch at 0.92 m"),
    ("Tilt is 8 degrees towards the sun on the right.", "Tilt is 20 degrees towards the sun on the right."),
    ("Der Fuß ist ein Betonblock 0,4 x 0,3 x 0,2 m (etwa 58 kg) mit 15 mm Aluminium-Deckplatte und Gabelbock; der Mast neigt sich auf einem waagrechten Bolzen 60 mm über der Platte (die Berechnung", "Der Fuß ist eine Betonplatte 0,5 x 0,5 x 0,096 m (etwa 58 kg) mit 10 mm Stahlflansch und angeschweißtem 60-mm-Standrohr von 0,29 m Höhe, in dem der Mast mit zwei Knäufen geklemmt ist; der Schirmkopf neigt sich in einem Knickgelenk 1,80 m über Grund, knapp unter dem offenen Läufer (die Berechnung"),
    ("Kurbelwinde auf 1,38 m", "Kurbelwinde auf 0,92 m"),
    ("Neigung 8 Grad zur Sonne rechts.", "Neigung 20 Grad zur Sonne rechts."),
    ("mit Betonballastblock, Gabelfuß, zweiteiligem Aluminiummast mit Kupplungsmuffe und Kurbelwinde", "mit Betonballastplatte und angeschweißtem Standrohr, zweiteiligem Aluminiummast mit Kupplungsmuffe, Kurbelwinde und Knickgelenk unter dem Läufer"),
    ("Betonblock mit Fuge, Deckplatte mit Schrauben, Gabelwange mit Bolzen, zweiteiliger Mast", "Betonplatte mit Fuge, Flansch mit Schrauben, Standrohr mit Knotenblechen und Klemmknäufen, zweiteiliger Mast mit Knickgelenk"),
    ("with the clevis cheeks and pin across", "with the knuckle barrel across the mast"),
    ("mit den Gabelwangen und dem Bolzen quer", "mit der Gelenkwalze quer zum Mast"),
    ("clevis cheeks and pin, cover plate with four bolts, block with seam and grit", "tilt knuckle, socket tube with gussets and clamp knobs, flange with four bolts, slab with seam and grit"),
    ("Gabelwangen und Bolzen, Deckplatte mit vier Schrauben, Block mit Fuge und Körnung", "Knickgelenk, Standrohr mit Knotenblechen und Klemmknäufen, Flansch mit vier Schrauben, Platte mit Fuge und Körnung"),
    ("the clevis merged with the plate into one foot block", "the socket tube as one upright cylinder"),
    ("Gabelbock und Platte zu einem Fußblock verschmolzen", "das Standrohr als ein stehender Zylinder"),
    ("Plate, clevis and pin into one foot block", "Flange, gussets and clamp knobs into the socket tube"),
    ("Platte, Gabelbock und Bolzen zu einem Fußblock", "Flansch, Knotenbleche und Klemmknäufe im Standrohr"),
    ("Fußblock, Betonblock", "Standrohr, Betonplatte"),
    ("Deckplatte", "Flansch"), ("Gabelbock und Platte", "Standrohr und Flansch"), ("Gabelbock, Platte", "Standrohr, Flansch"), ("Gabelbock", "Standrohr"),
    ("Betonblock", "Betonplatte"), ("Block = base-concrete", "Platte = base-concrete"), ("Block mit", "Platte mit"), ("Platte, Block.", "Flansch, Platte."),
    ("cover plate and clevis", "flange and socket tube"), ("clevis and plate", "socket and flange"), ("clevis, plate", "socket, flange"), ("clevis", "socket"), ("cover plate", "flange"),
    ("concrete block", "concrete slab"), ("block = base-concrete", "slab = base-concrete"), ("block with", "slab with"), ("plate, block.", "flange, slab."), ("foot block", "socket"),
    ("Structure: block with mass and support, pivot pin, one 44 x 2 mm mast tube with its mass, crown and runner joints", "Structure: slab with mass and support, socket, one 44 x 2 mm mast tube with its mass, the knuckle joint, mast head, crown and runner joints"),
    ("Tragwerk: Block mit Masse und Auflager, Gelenkbolzen, ein Mastrohr 44 x 2 mm mit seiner Masse, Kronen- und Läufergelenk", "Tragwerk: Platte mit Masse und Auflager, Standrohr, ein Mastrohr 44 x 2 mm mit seiner Masse, Knickgelenk, Mastkopf, Kronen- und Läufergelenk"),
    ("Structure: block with mass and support, mast, crown", "Structure: slab with mass and support, socket, mast, knuckle, mast head, crown"),
    ("Tragwerk: Block mit Masse und Auflager, Mast, Kronen", "Tragwerk: Platte mit Masse und Auflager, Standrohr, Mast, Knickgelenk, Mastkopf, Kronen"),
    ("a concrete ballast block, a clevis foot, a two-part aluminium mast with coupling collar and crank winch", "a concrete ballast slab with a welded socket tube, a two-part aluminium mast with coupling collar, crank winch and a tilt knuckle under the runner"),
    ("concrete block with seam, cover plate with bolts, clevis cheek with pin, two-part mast", "concrete slab with seam, flange with bolts, gusseted socket tube with clamp knobs, two-part mast with tilt knuckle"),
]
REPLACE += [
    ("Front: block, plate, clevis block, mast", "Front: slab, flange, socket tube, mast with knuckle"),
    ("Vorderansicht: Block, Platte, Gabelblock, Mast", "Vorderansicht: Platte, Flansch, Standrohr, Mast mit Knickgelenk"),
    ("clevis cheeks, pin and flange into the clevis outline", "gussets and clamp knobs into the socket outline"),
    ("Gabelwangen, Bolzen und Flansch zur Gabelkontur", "Knotenbleche und Klemmknäufe zur Standrohrkontur"),
]
REPLACE += [("ballast block", "ballast slab"), ("Ballastblock", "Ballastplatte"), ("Blockfuge", "Plattenfuge"), ("block", "slab"), ("Block", "Platte")]
REPLACE.sort(key=lambda pair: -len(pair[0]))
STALE = re.compile(r"clevis|Gabel|ballast block|Ballastblock|Betonblock|concrete block|10 degrees|10 Grad|pivot pin|Gelenkbolzen|8 degrees|8 Grad|block|Block|1\.38|1,38")


def walk(value, path=""):
    if isinstance(value, dict):
        for key, item in value.items():
            if isinstance(item, str):
                for old, new in REPLACE:
                    if old in item:
                        item = item.replace(old, new)
                value[key] = item
                if key in ("en", "de") and STALE.search(item):
                    print("STALE", path + "." + key, "|", STALE.search(item).group(0), "|", item[:160])
            else:
                walk(item, path + "." + key)
    elif isinstance(value, list):
        for index, item in enumerate(value):
            walk(item, path + "[" + str(index) + "]")


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    for scale, v in asset["volumes"].items():
        v["decisions"][0] = MODEL[scale]
        v["decisions"][-1] = TILT
        if scale in STRUCTURE:
            v["decisions"][2] = STRUCTURE[scale]
    walk({k: v for k, v in asset.items() if k not in ("rig",)})
    if "--publish" in sys.argv:
        scope = {}
        exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("\ndef member_end")[0], scope)
        json.dump({"volumes": asset["volumes"]}, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
        open(CATALOG, "w", encoding="utf-8").write(scope["dump"](document, 0) + "\n")
        print("written catalog + source")


main()
