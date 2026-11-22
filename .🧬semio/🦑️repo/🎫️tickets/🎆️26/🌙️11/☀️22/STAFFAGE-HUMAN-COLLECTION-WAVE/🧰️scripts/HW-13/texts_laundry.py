"""📝️ Texts, references, materials and thermal regions of `humans/person-hanging-laundry` (English first, German second)."""
from lib import t

TITLE = t("Person Hanging Laundry", "Person beim Wäscheaufhängen")
SUMMARY = t(
    "A man of 1.75 m in a softshell jacket, cap and chinos hangs a wet T-shirt on a clothesline in the garden: he takes it from the plastic "
    "basket at his feet, shakes it out, raises both arms and pegs it by the shoulders at 1.80 m with two wooden pegs; sun and a light breeze "
    "on the wet cotton, about 2.3 to 2.8 met, a 7 s loop and a pause holding the shirt at the line.",
    "Ein Mann von 1,75 m in Softshelljacke, Mütze und Chinohose hängt im Garten ein nasses T-Shirt auf die Wäscheleine: Er nimmt es aus dem "
    "Kunststoffkorb zu seinen Füßen, schüttelt es aus, hebt beide Arme und klammert es in 1,80 m Höhe mit zwei Holzklammern an den Schultern "
    "fest; Sonne und leichter Wind auf der nassen Baumwolle, etwa 2,3 bis 2,8 met, eine Schleife von 7 s und eine Pause mit dem Hemd an der Leine.")

DIMENSIONS = {"heightM": 1.89, "widthM": 0.857, "depthM": 1.2}

BONES = {
    "shirt": t("Wet T-shirt with its two pegs, held at the right shoulder seam", "Nasses T-Shirt mit seinen zwei Klammern, an der rechten Schulternaht gehalten"),
    "clothesline": t("Stretch of clothesline (fixed in the garden)", "Stück Wäscheleine (im Garten fest)"),
    "basket": t("Laundry basket on the ground (fixed in front of the feet)", "Wäschekorb auf dem Boden (fest vor den Füßen)"),
}
ANCHORS = {"basket-base": t("Basket bottom on the ground", "Korbboden auf dem Boden")}

MATERIALS = [
    {"id": "wet-cotton", "title": t("Wet cotton jersey", "Nasser Baumwolljersey"), "emissivity": 0.96, "solarAbsorptivity": 0.45, "densityKgM3": 1100, "source": "authored-estimate"},
    {"id": "beech-peg", "title": t("Beech clothes peg", "Wäscheklammer aus Buche"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 720, "source": "authored-estimate"},
    {"id": "steel-spring", "title": t("Steel peg spring", "Stahlfeder der Klammer"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "source": "authored-estimate"},
    {"id": "pp-line", "title": t("Plastic-coated clothesline", "Kunststoffummantelte Wäscheleine"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "source": "authored-estimate"},
    {"id": "pp-basket", "title": t("Polypropylene laundry basket", "Wäschekorb aus Polypropylen"), "emissivity": 0.92, "solarAbsorptivity": 0.4, "densityKgM3": 905, "conductivityWmK": 0.22, "source": "authored-estimate"},
]

REFERENCES = [
    {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (home activities, laundry: hanging washing, folding, light effort)."},
    {"id": "en-17121-2019", "citation": "EN 1729 and garden-equipment practice summarised as authored estimates: clothesline height 1.7 to 1.9 m, peg length 72 mm; no normative value is claimed."},
]

NOTES = t(
    "Derived by hand from the archive's tour-guide (rig, anthropometry of a 1.75 m, 78.6 kg man after de Leva 1996, softshell clothing, body volumes "
    "and outlines unchanged); pointer, flag, pole, lanyard and badge are removed. New: a wet cotton T-shirt with two beech pegs carried by the right "
    "fist, a 1.2 m stretch of clothesline at 1.80 m and a polypropylene basket of 560 x 380 x 270 mm with 3.1 kg of wet laundry standing in front of "
    "the feet; line and basket are fixed parts placed under the feet's frames, so they stay where they are while the man bends and stretches. Every "
    "new part is handcrafted per scale as one prism or profile of revolution that is both the 3D solid and, with its solid placement, the drawing of "
    "every view; three poses solved for the grips by inverse kinematics on the library's own rig solve; 2.3 met pegging, 2.5 met shaking out and "
    "2.8 met lifting from the basket (authored estimates after Ainsworth et al. 2011, laundry, light effort, with the arms overhead).",
    "Von Hand aus dem Archivobjekt tour-guide abgeleitet (Skelett, Anthropometrie eines Mannes von 1,75 m und 78,6 kg nach de Leva 1996, "
    "Softshellkleidung, Körpervolumen und Umrisse unverändert); Zeigestab, Fahne, Stange, Umhängeband und Ausweis sind entfernt. Neu: ein nasses "
    "Baumwoll-T-Shirt mit zwei Buchenklammern an der rechten Faust, ein 1,2 m langes Stück Wäscheleine in 1,80 m Höhe und ein Polypropylenkorb von "
    "560 x 380 x 270 mm mit 3,1 kg nasser Wäsche vor den Füßen; Leine und Korb sind feste Teile unter den Rahmen der Füße, sodass sie stehen bleiben, "
    "während sich der Mann bückt und streckt. Jedes neue Teil ist je Maßstab von Hand als Prisma oder Drehprofil gebaut, das zugleich 3D-Körper und – "
    "mit seiner Körperplatzierung – die Zeichnung jeder Ansicht ist; drei Posen, deren Griffe per inverser Kinematik auf der Skelettlösung der "
    "Bibliothek gelöst sind; 2,3 met beim Anklammern, 2,5 met beim Ausschütteln und 2,8 met beim Herausheben aus dem Korb (Schätzwerte nach "
    "Ainsworth et al. 2011, Wäsche, leichte Anstrengung, mit Armen über Kopf).")

REGIONS = [
    {"id": "wet-shirt", "title": t("Wet T-shirt", "Nasses T-Shirt"), "kind": "surface", "model": "passive", "material": "wet-cotton", "areaM2": 0.82, "characteristicLengthM": 0.5, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 4500, "source": "authored-estimate"},
    {"id": "line-and-pegs", "title": t("Clothesline and pegs", "Wäscheleine und Klammern"), "kind": "surface", "model": "passive", "material": "pp-line", "areaM2": 0.03, "characteristicLengthM": 0.005, "geometry": "cylinder", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 1500, "source": "authored-estimate"},
    {"id": "basket", "title": t("Laundry basket", "Wäschekorb"), "kind": "surface", "model": "passive", "material": "pp-basket", "areaM2": 0.95, "characteristicLengthM": 0.4, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 3000, "source": "authored-estimate"},
    {"id": "basket-laundry", "title": t("Wet laundry in the basket", "Nasse Wäsche im Korb"), "kind": "surface", "model": "passive", "material": "wet-cotton", "areaM2": 0.2, "characteristicLengthM": 0.4, "geometry": "horizontal-plate-up", "skyViewFactor": 0.8, "layers": [], "heatCapacityJM2K": 9000, "source": "authored-estimate"},
]

PEG_BREATH = {"id": "pegging", "title": t("Light work with raised arms", "Leichte Arbeit mit gehobenen Armen"), "rateBpm": 16, "tidalVolumeL": 0.75, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.07, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"}

THERMAL_ASSUMPTIONS = [
    t("DuBois skin area for 1.75 m and 78.59 kg; layered clothing of long-sleeve shirt (0.25 clo), trapped air and a windproof softshell (0.3 clo) on the trunk and arms, cotton trousers (0.24 clo), after ISO 9920 with 1 clo = 0.155 m2K/W.",
      "DuBois-Hautfläche für 1,75 m und 78,59 kg; Lagenkleidung aus Langarmhemd (0,25 clo), eingeschlossener Luft und winddichter Softshell (0,3 clo) an Rumpf und Armen, Baumwollhose (0,24 clo), nach ISO 9920 mit 1 clo = 0,155 m2K/W."),
    t("Garden in spring at 16 degrees Celsius, 55 % humidity, 1.5 m/s breeze and 250 W/m2 of sun; hanging laundry is light work at 2.3 to 2.8 met (authored estimates after Ainsworth et al. 2011 and ISO 8996 class 1 with raised arms); breathing through the nose at 16 per minute with 0.75 L.",
      "Garten im Frühling bei 16 Grad Celsius, 55 % Feuchte, 1,5 m/s Wind und 250 W/m2 Sonne; Wäscheaufhängen ist leichte Arbeit mit 2,3 bis 2,8 met (Schätzwerte nach Ainsworth et al. 2011 und ISO 8996 Klasse 1 mit gehobenen Armen); Nasenatmung mit 16 Zügen pro Minute und 0,75 L."),
    t("Shirt, laundry, line, pegs and basket are passive surfaces without internal heat; the wet cotton holds water, so its heat capacity is high and it stays cooler than dry cloth in the sun (evaporation is not modelled for passive surfaces).",
      "Hemd, Wäsche, Leine, Klammern und Korb sind passive Flächen ohne innere Wärme; die nasse Baumwolle hält Wasser, daher ist ihre Wärmekapazität hoch und sie bleibt in der Sonne kühler als trockener Stoff (Verdunstung wird für passive Flächen nicht modelliert)."),
]

STRUCTURE_ASSUMPTIONS = [
    t("Segments are point masses at the de Leva centres (male table); the wet shirt with its pegs (0.36 kg) hangs from the right fist and is steadied by the left, so both raised arms carry it and the shoulders carry the moments of the outstretched arms.",
      "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Männertabelle); das nasse Hemd mit seinen Klammern (0,36 kg) hängt an der rechten Faust und wird von der linken gehalten, sodass beide gehobenen Arme es tragen und die Schultern die Momente der ausgestreckten Arme aufnehmen."),
    t("Supports are the heel and ball of each foot and the basket bottom on the ground; the basket (3.9 kg with its laundry) carries itself, the line is a fixed garden element without load on the person.",
      "Auflager sind Ferse und Ballen jedes Fußes und der Korbboden auf dem Boden; der Korb (3,9 kg mit Wäsche) trägt sich selbst, die Leine ist ein festes Gartenelement ohne Last auf der Person."),
]

POSES = {
    "pegging-shirt": (t("Pegging the shirt", "Hemd anklammern"), t("Standing upright before the line, both arms raised forward, each fist pressing a peg over a shoulder of the shirt onto the line at 1.80 m; the head tilted back to watch the pegs.", "Aufrecht vor der Leine, beide Arme nach vorn gehoben, jede Faust drückt eine Klammer über eine Schulter des Hemds auf die Leine in 1,80 m Höhe; der Kopf zurückgeneigt, der Blick auf die Klammern.")),
    "shaking-out": (t("Shaking out the shirt", "Hemd ausschütteln"), t("Upright, the shirt held by both shoulders at chest height in front of the body, ready to snap it straight.", "Aufrecht, das Hemd an beiden Schultern auf Brusthöhe vor dem Körper gehalten, bereit, es glatt zu schlagen.")),
    "lifting-from-basket": (t("Lifting a shirt from the basket", "Hemd aus dem Korb heben"), t("Knees bent and trunk bowed over the basket, both fists lifting a shirt by its shoulders out of the wet laundry.", "Knie gebeugt und Rumpf über den Korb geneigt, beide Fäuste heben ein Hemd an den Schultern aus der nassen Wäsche.")),
}

CLIPS = {
    "hang-shirt": (t("Hanging a shirt", "Ein Hemd aufhängen"), t("A 7 s loop: lift a shirt from the basket, straighten up and shake it out with two snaps, raise it to the line and press the two pegs on, hold, then bend back down to the basket for the next one; about 2.3 to 2.8 met.", "Eine Schleife von 7 s: ein Hemd aus dem Korb heben, aufrichten und mit zwei Schlägen ausschütteln, zur Leine heben und die zwei Klammern aufdrücken, halten, dann wieder zum Korb bücken für das nächste; etwa 2,3 bis 2,8 met.")),
    "idle-breathing": (t("Idle breathing at the line", "Ruhiges Atmen an der Leine"), t("A five-second loop holding the pegged shirt at the line: quiet breathing through the nose, the trunk and head sway slightly; feet and basket stay on the ground.", "Eine Schleife von fünf Sekunden, das angeklammerte Hemd an der Leine haltend: ruhiges Atmen durch die Nase, Rumpf und Kopf wiegen sich leicht; Füße und Korb bleiben am Boden.")),
}


def _d(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles, "overlays": overlays}


DECISIONS = {
    "50": _d(
        t("Every filled shape is the true orthographic outline of the 1:50 model pegging the shirt, per bone: head with nose, lips, ears and eyes under the cap with brim and button, short hair, neck in the stand collar, the softshell jacket with zip, pockets and hem band, raised sleeves with the bare right wrist, both fists at the line, chinos and walking shoes; the T-shirt with sleeves and neckline hanging from the line, two pegs with their springs, the clothesline, and on the ground the basket with its grips and the heap of wet laundry. Front view (the figure's left is on the right of the sheet): the shirt full face before the jacket; side view: the shirt and the line edge-on. Plan: the basket before the shoes, the line across the sheet above it.",
          "Jede gefüllte Form ist der wahre orthogonale Umriss des 1:50-Modells beim Anklammern, je Knochen: Kopf mit Nase, Lippen, Ohren und Augen unter der Mütze mit Schirm und Knopf, kurzes Haar, Hals im Stehkragen, die Softshelljacke mit Reißverschluss, Taschen und Saumbund, gehobene Ärmel mit freiem rechtem Handgelenk, beide Fäuste an der Leine, Chinohose und Wanderschuhe; das T-Shirt mit Ärmeln und Halsausschnitt an der Leine, zwei Klammern mit ihren Federn, die Wäscheleine und auf dem Boden der Korb mit seinen Griffen und dem Haufen nasser Wäsche. Vorderansicht (die linke Körperseite liegt rechts im Blatt): das Hemd in voller Fläche vor der Jacke; Seitenansicht: Hemd und Leine von der Kante. Grundriss: der Korb vor den Schuhen, die Leine quer darüber."),
        t("Front view strokes of the jacket (zip and pocket zips, mouth), the shirt sleeves and neckline, the peg springs, the basket grips and the domed top of the laundry.", "Striche der Jacke in der Vorderansicht (Reißverschluss, Taschenreißverschlüsse, Mund), Ärmel und Halsausschnitt des Hemds, die Klammerfedern, die Korbgriffe und die gewölbte Oberseite der Wäsche."),
        t("Cap, brim and button into one shape, fingers into one fist per hand, sole and lacing into the shoe; the wet laundry into one heap.", "Mütze, Schirm und Knopf in eine Form, Finger in eine Faust je Hand, Sohle und Schnürung in den Schuh; die nasse Wäsche in einen Haufen."),
        t("Fabric folds and creases of the wet shirt, the woven holes of the basket, the line posts outside the sheet.", "Stofffalten und Knitter des nassen Hemds, die Flechtlöcher des Korbs, die Leinenpfosten außerhalb des Blatts."),
        t("The 5 mm line is drawn at its true 0.1 mm width inside the 0.25 mm pen; the 6 mm shirt is a thin plate seen edge-on in the side view.", "Die 5 mm starke Leine ist in wahrer Breite von 0,1 mm im 0,25-mm-Stift gezeichnet; das 6 mm dünne Hemd ist in der Seitenansicht eine schmale Platte von der Kante."),
        t("None.", "Keine."),
        t("0.25 mm outlines, 0.13 mm detail strokes; jacket dark, chinos mid, skin light, the wet shirt and the laundry very light, pegs mid, line dark, basket mid-light.", "0,25-mm-Umrisse, 0,13-mm-Detailstriche; Jacke dunkel, Chinos mittel, Haut hell, nasses Hemd und Wäsche sehr hell, Klammern mittel, Leine dunkel, Korb mittelhell."),
        t("Each shape carries its thermal region in all three views: jacket, sleeves, wrist, hands, face, hair and cap, chinos, shoes, the wet shirt, line and pegs, basket and the laundry in it.", "Jede Form trägt in allen drei Ansichten ihre thermische Region: Jacke, Ärmel, Handgelenk, Hände, Gesicht, Haar und Mütze, Chinos, Schuhe, das nasse Hemd, Leine und Klammern, Korb und die Wäsche darin.")),
    "100": _d(
        t("Every filled shape is the true outline of the 1:100 model pegging the shirt: head with nose and ears, cap with brim, hair at the nape, neck and collar, jacket and skirt with hem band, raised sleeves, forearms, the bare right wrist, both fists, seat, trouser legs, shoes; the T-shirt with its sleeves and neckline, two pegs, the line; the basket with grips and the laundry heap.",
          "Jede gefüllte Form ist der wahre Umriss des 1:100-Modells beim Anklammern: Kopf mit Nase und Ohren, Mütze mit Schirm, Haar im Nacken, Hals und Kragen, Jacke und Schoß mit Saumbund, gehobene Ärmel, Unterarme, das freie rechte Handgelenk, beide Fäuste, Gesäß, Hosenbeine, Schuhe; das T-Shirt mit Ärmeln und Halsausschnitt, zwei Klammern, die Leine; der Korb mit Griffen und dem Wäschehaufen."),
        t("The pegs and the shirt sleeves, so the action reads; the basket grips.", "Die Klammern und die Hemdärmel, damit die Handlung lesbar ist; die Korbgriffe."),
        t("Fingers into one band per hand, sole into the shoe, cuff into the forearm; peg springs into the pegs.", "Finger in ein Band je Hand, Sohle in den Schuh, Bündchen in den Unterarm; Klammerfedern in die Klammern."),
        t("Face details, zips, pockets, lacing, the dome of the laundry.", "Gesichtsdetails, Reißverschlüsse, Taschen, Schnürung, die Wölbung der Wäsche."),
        t("Line at its true 5 mm (0.05 mm) inside the pen; pegs 12 mm wide (0.12 mm).", "Leine in wahren 5 mm (0,05 mm) im Stift; Klammern 12 mm breit (0,12 mm)."),
        t("None.", "Keine."),
        t("0.25 mm outlines, 0.13 mm strokes; tones as at 1:50.", "0,25-mm-Umrisse, 0,13-mm-Striche; Töne wie bei 1:50."),
        t("Each shape carries its thermal region in all three views.", "Jede Form trägt in allen drei Ansichten ihre thermische Region.")),
    "200": _d(
        t("Every filled shape is the true outline of the 1:200 model pegging the shirt: head with nose, cap with brim, hair, neck and collar, jacket and skirt, raised sleeves, forearms, wrist, fists, seat, trouser legs, shoes; the T-shirt as one outline with short sleeves and neckline notch, the pegs, the line as an 8 mm rod; the basket and the laundry heap.",
          "Jede gefüllte Form ist der wahre Umriss des 1:200-Modells beim Anklammern: Kopf mit Nase, Mütze mit Schirm, Haar, Hals und Kragen, Jacke und Schoß, gehobene Ärmel, Unterarme, Handgelenk, Fäuste, Gesäß, Hosenbeine, Schuhe; das T-Shirt als ein Umriss mit kurzen Ärmeln und Halskerbe, die Klammern, die Leine als 8 mm starker Stab; der Korb und der Wäschehaufen."),
        t("The raised arms and the shirt at the line, the basket at the feet.", "Die gehobenen Arme und das Hemd an der Leine, der Korb zu den Füßen."),
        t("Hands into single fists, sole into the shoe, hem band into the jacket; basket grips into the basket.", "Hände in einzelne Fäuste, Sohle in den Schuh, Saumbund in die Jacke; Korbgriffe in den Korb."),
        t("Ears, zips, pockets, the shirt's sleeve hems.", "Ohren, Reißverschlüsse, Taschen, die Ärmelsäume des Hemds."),
        t("Line drawn as an 8 mm rod (0.04 mm) inside its 0.2 mm outline, pegs 16 mm wide.", "Leine als 8 mm starker Stab (0,04 mm) in ihrem 0,2-mm-Umriss, Klammern 16 mm breit."),
        t("None.", "Keine."),
        t("0.2 mm outlines; tones as at 1:50.", "0,2-mm-Umrisse; Töne wie bei 1:50."),
        t("Each shape carries its thermal region in all three views.", "Jede Form trägt in allen drei Ansichten ihre thermische Region.")),
    "500": _d(
        t("Every filled shape is the true outline of the 1:500 model pegging the shirt: head, cap with brim, neck, the jacket as one form, raised sleeves and forearms, wrist, fists, seat, trouser legs, shoes; the shirt as a T-shaped plate, the line as a 25 mm rod, the basket as one block with the laundry heap.",
          "Jede gefüllte Form ist der wahre Umriss des 1:500-Modells beim Anklammern: Kopf, Mütze mit Schirm, Hals, die Jacke als eine Form, gehobene Ärmel und Unterarme, Handgelenk, Fäuste, Gesäß, Hosenbeine, Schuhe; das Hemd als T-förmige Platte, die Leine als 25 mm starker Stab, der Korb als ein Block mit dem Wäschehaufen."),
        t("The raised arms, the shirt hanging from the line and the basket, so the chore reads at 3.6 mm height.", "Die gehobenen Arme, das an der Leine hängende Hemd und der Korb, damit die Tätigkeit bei 3,6 mm Höhe lesbar ist."),
        t("Jacket, collar and skirt into one form; hair under the cap; the pegs into the shirt shoulders.", "Jacke, Kragen und Schoß in eine Form; Haar unter der Mütze; die Klammern in die Hemdschultern."),
        t("Face, ears, the shirt neckline, pegs and basket grips.", "Gesicht, Ohren, der Halsausschnitt des Hemds, Klammern und Korbgriffe."),
        t("Line and shirt are drawn and modelled 25 mm thick (the line as a rod, the shirt as a plate) so that both print where they are seen edge-on.", "Leine und Hemd werden 25 mm dick gezeichnet und modelliert (die Leine als Stab, das Hemd als Platte), damit beide drucken, wo sie von der Kante gesehen werden."),
        t("None.", "Keine."),
        t("0.15 mm outlines; tones as at 1:50.", "0,15-mm-Umrisse; Töne wie bei 1:50."),
        t("Each shape carries its thermal region in all three views; the pegs share the shirt shape, their region is drawn by the line.", "Jede Form trägt in allen drei Ansichten ihre thermische Region; die Klammern teilen die Hemdform, ihre Region zeichnet die Leine.")),
}

_EDITS = [
    ("; a lanyard round the collar down to a badge with its clip", ""), ("; ein Umhängeband um den Kragen bis zu einem Ausweis mit Clip", ""),
    ("the left fist round the flag pole with four fingers and the thumb, the right hand holding the pointer like a baton, index finger stretched along it, thumb against it, three fingers curled",
     "the left fist closed round a shoulder of the shirt with four fingers and the thumb, the right hand pinching the other shoulder and its peg, index finger stretched along the peg, thumb against it, three fingers curled"),
    ("die linke Faust mit vier Fingern und Daumen um die Fahnenstange, die rechte Hand hält den Zeigestab wie einen Taktstock, der Zeigefinger daran gestreckt, der Daumen dagegen, drei Finger eingerollt",
     "die linke Faust mit vier Fingern und Daumen um eine Schulter des Hemds geschlossen, die rechte Hand hält die andere Schulter und ihre Klammer, der Zeigefinger an der Klammer gestreckt, der Daumen dagegen, drei Finger eingerollt"),
    ("; the telescopic pointer with grip, three sections and ball tip; the 24 mm flag pole with rubber ferrule and ball finial, the flag cloth rippled in the wind on its hoist sleeve", ""),
    ("; der Teleskop-Zeigestab mit Griff, drei Auszügen und Kugelspitze; die 24-mm-Fahnenstange mit Gummikappe und Kugelknauf, das Fahnentuch im Wind gewellt an seinem Hohlsaum", ""),
    ("pointer, pole and flag are their real parts", "shirt, pegs, line, basket and laundry are their real parts"), ("Zeigestab, Stange und Fahne sind ihre wirklichen Teile", "Hemd, Klammern, Leine, Korb und Wäsche sind ihre wirklichen Teile"),
    ("; the pointer is carried by its grip and sections, the flag by the pole and the cloth on its hoist", "; the shirt hangs from the right fist, line and basket stand on their own"),
    ("; der Zeigestab trägt über Griff und Auszüge, die Fahne über die Stange und das Tuch an seinem Hohlsaum", "; das Hemd hängt an der rechten Faust, Leine und Korb stehen für sich"),
    (", the badge on its true-size lanyard", ""), (", der Ausweis am Umhängeband in wahrer Größe", ""),
    ("one finger band round the pole", "one finger band round the shirt shoulder"), ("ein Fingerband um die Stange", "ein Fingerband um die Hemdschulter"),
    ("index finger along the pointer", "index finger along the peg"), ("Zeigefinger am Zeigestab", "Zeigefinger an der Klammer"),
    ("; the pointer as grip and one true-size rod, the pole with finial and hoist sleeve, the flag 15 mm thick", ""), ("; der Zeigestab als Griff und ein Stab in wahrer Größe, die Stange mit Knauf und Hohlsaum, die Fahne 15 mm dick", ""),
    ("; the pointer by its grip and rod, the pole and the flag cloth by reference", "; shirt, line and basket by reference"), ("; der Zeigestab über Griff und Stab, Stange und Fahnentuch per Verweis", "; Hemd, Leine und Korb per Verweis"),
    (", the badge (the lanyard and the hem band are below the feature)", " (the hem band is below the feature)"), (", der Ausweis (Umhängeband und Saumbund liegen unter dem Merkmal)", " (der Saumbund liegt unter dem Merkmal)"),
    ("and the thumb round the pole", "and the thumb round the shirt shoulder"), ("und dem Daumen um die Stange", "und dem Daumen um die Hemdschulter"),
    ("; the pointer as one true-size rod, the pole with its finial, the flag 30 mm thick (the hoist sleeve merges into the pole)", ""), ("; der Zeigestab als ein Stab in wahrer Größe, die Stange mit ihrem Knauf, die Fahne 30 mm dick (der Hohlsaum geht in der Stange auf)", ""),
    ("masses of trunk, pelvis, head, thighs, pointer and pole; pointer, pole and flag by reference", "masses of trunk, pelvis, head, thighs, shirt and basket; shirt, line and basket by reference"),
    ("Massen von Rumpf, Becken, Kopf, Oberschenkeln, Zeigestab und Stange; Zeigestab, Stange und Fahne per Verweis", "Massen von Rumpf, Becken, Kopf, Oberschenkeln, Hemd und Korb; Hemd, Leine und Korb per Verweis"),
    ("; pointer and pole as true-size rods, the flag 75 mm thick", ""), ("; Zeigestab und Stange als Stäbe in wahrer Größe, die Fahne 75 mm dick", ""),
    ("trunk, neck and the flag hoist", "trunk and neck"), ("Rumpf, Hals und am Fahnensaum", "Rumpf und Hals"),
    ("; pointer, pole and flag by reference", "; shirt, line and basket by reference"), ("; Zeigestab, Stange und Fahne per Verweis", "; Hemd, Leine und Korb per Verweis"),
]

_PARTS = {
    "50": t("Laundry at 1:50: the wet T-shirt as one 6 mm plate of its outline with short sleeves and neckline, two beech pegs of 75 x 12 x 24 mm with their steel springs clamping shirt and line, a 5 mm clothesline 1.2 m long, the basket as a hollow shell of 8 mm walls with rounded corners, its floor and two grips, the heap of wet laundry with a domed top. Line and basket hang on fixed bones under the feet, so they stand still while the man bends. Thermal: shirt, pegs and line, basket and laundry are their own regions. Structure: shirt, line and basket are members of their bones, masses of shirt and basket, the support under the basket.",
            "Wäsche bei 1:50: das nasse T-Shirt als eine 6 mm dünne Platte seines Umrisses mit kurzen Ärmeln und Halsausschnitt, zwei Buchenklammern von 75 x 12 x 24 mm mit ihren Stahlfedern, die Hemd und Leine klemmen, eine 5 mm starke Wäscheleine von 1,2 m, der Korb als hohle Schale mit 8 mm Wänden und gerundeten Ecken, sein Boden und zwei Griffe, der Haufen nasser Wäsche mit gewölbter Oberseite. Leine und Korb hängen an festen Knochen unter den Füßen und stehen still, während sich der Mann bückt. Thermisch: Hemd, Klammern und Leine, Korb und Wäsche sind eigene Regionen. Tragwerk: Hemd, Leine und Korb sind Stäbe ihrer Knochen, Massen von Hemd und Korb, das Auflager unter dem Korb."),
    "100": t("Laundry at 1:100: shirt plate, two plain pegs, the 5 mm line, the basket with 15 mm walls, floor and grips, the laundry heap. Thermal and structure as at 1:50.",
             "Wäsche bei 1:100: Hemdplatte, zwei einfache Klammern, die 5 mm Leine, der Korb mit 15 mm Wänden, Boden und Griffen, der Wäschehaufen. Thermisch und Tragwerk wie bei 1:50."),
    "200": t("Laundry at 1:200: shirt plate of a simpler outline, pegs 16 mm wide, the line as an 8 mm slender rod, the basket with 30 mm walls and floor, the laundry heap; the grips merge into the walls.",
             "Wäsche bei 1:200: Hemdplatte mit einfacherem Umriss, Klammern 16 mm breit, die Leine als 8 mm starker schlanker Stab, der Korb mit 30 mm Wänden und Boden, der Wäschehaufen; die Griffe gehen in den Wänden auf."),
    "500": t("Laundry at 1:500: the shirt as a T-shaped plate of 25 mm (declared sheet), the pegs merged into it, the line as a 25 mm slender rod, the basket as one block with the laundry heap on top of it.",
             "Wäsche bei 1:500: das Hemd als T-förmige Platte von 25 mm (erklärtes Blatt), die Klammern darin aufgegangen, die Leine als 25 mm starker schlanker Stab, der Korb als ein Block mit dem Wäschehaufen darauf."),
}


def volume_decisions(scale, source):
    out = []
    for d in source:
        en, de = d["en"], d["de"]
        for old, new in _EDITS:
            en = en.replace(old, new)
            de = de.replace(old, new)
        out.append({"en": en, "de": de})
    return out + [_PARTS[scale]]
