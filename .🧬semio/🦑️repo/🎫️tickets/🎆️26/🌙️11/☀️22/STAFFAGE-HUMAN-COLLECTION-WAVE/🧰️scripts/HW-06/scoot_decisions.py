"""🛵️ Representation and volume decisions of `mobility-scooter-user` per scale (English first, German second)."""
import hw06lib as L

t = L.t

RETAINED = t("Ids, tones, regions and materials are one per part in all three views and follow the body solids of this scale; the scooter parts are the orthographic outlines of their solids.",
             "Kennungen, Tonwerte, Regionen und Materialien sind je Teil in allen drei Ansichten gleich und folgen den Körpern dieses Maßstabs; die Teile des Elektromobils sind die orthografischen Umrisse ihrer Körper.")
OVERLAYS = t("One region per shape: rider regions as for the figure he is derived from (at 1:500 the forearm also carries gloves and fingertips), the scooter in body panels, upholstery, tyres, battery box and drive motor; the drive motor sits behind the near rear wheel in the side view and shows between the wheels in the front view; paint order follows the depth of each part at rest.",
             "Eine Region je Form: Regionen des Fahrers wie bei der Figur, von der er abgeleitet ist (in 1:500 trägt der Unterarm auch Handschuhe und Fingerspitzen), das Elektromobil in Verkleidung, Polsterung, Reifen, Akkukasten und Antriebsmotor; der Antriebsmotor liegt in der Seitenansicht hinter dem nahen Hinterrad und zeigt sich in der Vorderansicht zwischen den Rädern; die Zeichenfolge folgt der Tiefe jedes Teils in Ruhe.")
DECISIONS = {
    "50": dict(
        silhouette=t("The rider as redrawn from the 1:50 body (cap with cuff, face with eyes, brows and mouth, hood and collar, quilted jacket with hem and pocket, sleeves with cuffs, fingerless gloves with finger blocks and thumb, corduroy legs, leather shoes with midsoles) on a scooter drawn part by part: four wheels with tyre and silver hub, rear shroud, battery box, transaxle motor, footboard with heel rest, nose with fenders, bumper and headlamp, seat post, seat base and cushion, 12-degree backrest, armrests on posts, tiller with cover, console and handlebar with rubber grips, wire basket.",
                     "Der Fahrer, neu gezeichnet nach dem 1:50-Körper (Mütze mit Umschlag, Gesicht mit Augen, Brauen und Mund, Kapuze und Kragen, Steppjacke mit Bund und Tasche, Ärmel mit Bündchen, fingerlose Handschuhe mit Fingerblöcken und Daumen, Cordbeine, Lederschuhe mit Zwischensohlen) auf einem Teil für Teil gezeichneten Elektromobil: vier Räder mit Reifen und silberner Nabe, Heckhaube, Akkukasten, Achsmotor, Trittbrett mit Fersenstütze, Bugnase mit Kotflügeln, Stoßfänger und Scheinwerfer, Sitzsäule, Sitzunterteil und Polster, um 12 Grad geneigte Rückenlehne, Armlehnen auf Stützen, Lenksäule mit Verkleidung, Bedienpult und Lenker mit Gummigriffen, Drahtkorb."),
        merged=t("Wire basket as one closed box outline; tyre and hub as two discs per wheel; console as one block.", "Drahtkorb als ein geschlossener Kastenumriss; Reifen und Nabe als zwei Scheiben je Rad; Bedienpult als ein Block."),
        omitted=t("Basket wires, throttle lever, wheel bolts, tread, mirrors, cable runs, drawstring ends.", "Korbdrähte, Gashebel, Radschrauben, Profil, Spiegel, Kabel, Kordelenden."),
        thinElements=t("Tiller 50 mm, armrest posts 24 mm and handlebar 28 mm at their true sections.", "Lenksäule 50 mm, Armlehnstützen 24 mm und Lenker 28 mm in ihren wahren Querschnitten."),
        hatching=t("None.", "Keine."), styles=t("Body 0.25 mm, detail 0.13 mm.", "Körper 0,25 mm, Detail 0,13 mm.")),
    "100": dict(
        silhouette=t("The rider as redrawn from the 1:100 body (mitten gloves with thumb, one shoe form with sole) on the scooter with wheels as tyre and hub, shroud, battery box, motor, footboard with heel rest, nose, fenders, bumper, seat post, base, cushion, backrest, armrests, tiller with cover, console, handlebar and basket.",
                     "Der Fahrer, neu gezeichnet nach dem 1:100-Körper (Fäustlingshandschuhe mit Daumen, eine Schuhform mit Sohle), auf dem Elektromobil mit Rädern aus Reifen und Nabe, Haube, Akkukasten, Motor, Trittbrett mit Fersenstütze, Bugnase, Kotflügeln, Stoßfänger, Sitzsäule, Unterteil, Polster, Lehne, Armlehnen, Lenksäule mit Verkleidung, Pult, Lenker und Korb."),
        merged=t("Grips into the handlebar, headlamp into the nose.", "Griffe in den Lenker, Scheinwerfer in die Bugnase."),
        omitted=t("Eyes, brows, lips, drawstrings, basket wires, bolts.", "Augen, Brauen, Lippen, Kordeln, Korbdrähte, Schrauben."),
        thinElements=t("Armrest posts and handlebar thickened to 32 mm (0.32 mm on paper).", "Armlehnstützen und Lenker auf 32 mm verstärkt (0,32 mm auf Papier)."),
        hatching=t("None.", "Keine."), styles=t("Body 0.20 mm, detail 0.10 mm.", "Körper 0,20 mm, Detail 0,10 mm.")),
    "200": dict(
        silhouette=t("One shape per body segment of the rider; the scooter as wheels (one disc each), shroud, battery box, motor, footboard, nose, fenders, bumper, seat post, base, cushion, backrest, armrests, tiller, cover, handlebar, console and basket.",
                     "Eine Form je Körpersegment des Fahrers; das Elektromobil als Räder (je eine Scheibe), Haube, Akkukasten, Motor, Trittbrett, Bugnase, Kotflügel, Stoßfänger, Sitzsäule, Unterteil, Polster, Lehne, Armlehnen, Lenksäule, Verkleidung, Lenker, Pult und Korb."),
        merged=t("Tyre and hub into one wheel disc; face details into the head.", "Reifen und Nabe zu einer Radscheibe; Gesichtsdetails in den Kopf."),
        omitted=t("Heel rest, headlamp, grips, hood details.", "Fersenstütze, Scheinwerfer, Griffe, Kapuzendetails."),
        thinElements=t("None below 30 mm.", "Keine unter 30 mm."),
        hatching=t("None.", "Keine."), styles=t("Body 0.18 mm.", "Körper 0,18 mm.")),
    "500": dict(
        silhouette=t("The generalised rider (cap, head, jacket, arms, hands, thighs, shanks, shoes) on a scooter of 75 mm parts: four wheel discs, rear shroud, battery box, motor, footboard, nose with fender block and bumper, one seat block with cushion, backrest, tiller, handlebar block and basket.",
                     "Der verallgemeinerte Fahrer (Mütze, Kopf, Jacke, Arme, Hände, Oberschenkel, Unterschenkel, Schuhe) auf einem Elektromobil aus 75-mm-Teilen: vier Radscheiben, Heckhaube, Akkukasten, Motor, Trittbrett, Bugnase mit Kotflügelblock und Stoßfänger, ein Sitzblock mit Polster, Lehne, Lenksäule, Lenkerblock und Korb."),
        merged=t("Seat post and base into one block; handlebar and console into one block; fenders into a 75 mm block.", "Sitzsäule und Unterteil zu einem Block; Lenker und Pult zu einem Block; Kotflügel zu einem 75-mm-Block."),
        omitted=t("Armrests, grips, headlamp, heel rest.", "Armlehnen, Griffe, Scheinwerfer, Fersenstütze."),
        thinElements=t("Tiller thickened from 50 to 75 mm; the backrest is 120 mm thick and 360 mm wide (within the trunk in the front view) so that it prints at 0.24 mm.",
                       "Lenksäule von 50 auf 75 mm verstärkt; die Lehne ist 120 mm dick und 360 mm breit (in der Vorderansicht innerhalb des Rumpfs), damit sie mit 0,24 mm druckt."),
        hatching=t("None.", "Keine."), styles=t("Body 0.13 mm.", "Körper 0,13 mm.")),
}
VOLUMES = {
    "50": [t("Rider: the wheelchair user's hand-stated 1:50 body scaled to a man of 1.72 m (0.983), with his cap, quilted jacket, fingerless gloves, corduroy trousers and leather shoes; both arms solved onto the handlebar grips in the driving pose.",
             "Fahrer: der handgearbeitete 1:50-Körper der rollstuhlnutzenden Person, auf einen Mann von 1,72 m skaliert (0,983), mit Mütze, Steppjacke, fingerlosen Handschuhen, Cordhose und Lederschuhen; beide Arme in der Fahrhaltung auf die Lenkergriffe gelöst."),
           t("Scooter: rounded boxes for shroud (500 x 90 x 600 mm), battery box (380 x 170 x 340), footboard, heel rest, nose, fenders, console, seat base, cushion (520 x 71 x 480), backrest (80 x 560 x 460, 12 degrees) and armrests; capsules for the transaxle motor (130 mm), front axle, bumper, seat post, armrest posts, tiller (50 mm) with its cover and the handlebar with grips; an ellipsoid headlamp; each wheel a 250 x 80 mm tyre ring around a silver hub disc that stands 6 mm proud; the wire basket as a closed 240 x 260 x 360 mm box.",
             "Elektromobil: abgerundete Quader für Haube (500 x 90 x 600 mm), Akkukasten (380 x 170 x 340), Trittbrett, Fersenstütze, Bugnase, Kotflügel, Pult, Sitzunterteil, Polster (520 x 71 x 480), Rückenlehne (80 x 560 x 460, 12 Grad) und Armlehnen; Kapseln für Achsmotor (130 mm), Vorderachse, Stoßfänger, Sitzsäule, Armlehnstützen, Lenksäule (50 mm) mit Verkleidung und den Lenker mit Griffen; ein Ellipsoid als Scheinwerfer; jedes Rad ein Reifenring von 250 x 80 mm um eine silberne Nabenscheibe, die 6 mm vorsteht; der Drahtkorb als geschlossener Kasten von 240 x 260 x 360 mm."),
           t("Thermal: rider regions as in the source figure; every scooter part is a surface of its region by reference (body panels, upholstery, tyres, battery box, drive motor). Structure: the rider's skeleton bone by bone; the chassis members are footboard, shroud, battery box, heel rest, nose, fender, seat post, base, cushion, backrest, tiller, motor and front axle, the wheel pairs their tyres and hubs, and a seat contact pad carries the ischial tuberosities into the cushion.",
             "Thermik: Regionen des Fahrers wie bei der Ausgangsfigur; jedes Teil des Elektromobils ist per Verweis eine Oberfläche seiner Region (Verkleidung, Polsterung, Reifen, Akkukasten, Antriebsmotor). Tragwerk: das Skelett des Fahrers Knochen für Knochen; die Glieder des Fahrgestells sind Trittbrett, Haube, Akkukasten, Fersenstütze, Bugnase, Kotflügel, Sitzsäule, Unterteil, Polster, Lehne, Lenksäule, Motor und Vorderachse, die Radpaare ihre Reifen und Naben, und ein Sitzkontaktpolster trägt die Sitzbeinhöcker in das Polster.")],
    "100": [t("1:100 (15 mm): the rider as the source figure's 1:100 body scaled (cross-sections at the feature keep it); the scooter as at 1:50 without headlamp and grips, edges less rounded, armrest posts and handlebar at 32 mm.",
              "1:100 (15 mm): der Fahrer als skalierter 1:100-Körper der Ausgangsfigur (Querschnitte auf dem Merkmal behalten es); das Elektromobil wie in 1:50 ohne Scheinwerfer und Griffe, Kanten weniger gerundet, Armlehnstützen und Lenker mit 32 mm."),
            t("Thermal: one surface per solid. Structure: the merged skeleton of the source figure, the chassis members and wheel members as at 1:50, the seat contact pad.",
              "Thermik: eine Oberfläche je Körper. Tragwerk: das zusammengefasste Skelett der Ausgangsfigur, Fahrgestell- und Radglieder wie in 1:50, das Sitzkontaktpolster.")],
    "200": [t("1:200 (30 mm): one form per rider segment; wheels as single 250 x 80 mm discs, sharp-edged boxes, no heel rest, headlamp or grips; posts and handlebar at 32 mm.",
              "1:200 (30 mm): eine Form je Segment des Fahrers; Räder als einzelne Scheiben von 250 x 80 mm, scharfkantige Quader, ohne Fersenstütze, Scheinwerfer und Griffe; Stützen und Lenker mit 32 mm."),
            t("Thermal: totals only. Structure: the source figure's 1:200 skeleton, chassis and wheel members, seat contact pad.", "Thermik: nur Summen. Tragwerk: das 1:200-Skelett der Ausgangsfigur, Fahrgestell- und Radglieder, Sitzkontaktpolster.")],
    "500": [t("1:500 (75 mm): generalised rider; the scooter in 75 mm parts: four wheel discs, shroud, battery box, motor, 75 mm footboard, nose, fender block, bumper, one seat block, cushion, a 120 x 560 x 360 mm backrest set 30 mm into the back, a 75 mm tiller, a handlebar block and the basket; no armrests.",
              "1:500 (75 mm): verallgemeinerter Fahrer; das Elektromobil in 75-mm-Teilen: vier Radscheiben, Haube, Akkukasten, Motor, 75-mm-Trittbrett, Bugnase, Kotflügelblock, Stoßfänger, ein Sitzblock, Polster, eine Lehne von 120 x 560 x 360 mm, 30 mm in den Rücken gesetzt, eine 75-mm-Lenksäule, ein Lenkerblock und der Korb; ohne Armlehnen."),
            t("Thermal: totals only. Structure: spine, rib cage, pelvic and shoulder bars, skull, one bone per limb segment; chassis and wheel members by reference, seat contact pad.",
              "Thermik: nur Summen. Tragwerk: Wirbelsäule, Brustkorb, Becken- und Schulterstab, Schädel, ein Knochen je Gliedmaßensegment; Fahrgestell- und Radglieder per Verweis, Sitzkontaktpolster.")],
}


def apply(asset):
    for key, d in DECISIONS.items():
        rep = asset["representations"][key]
        rep["decisions"] = {"silhouette": d["silhouette"], "retained": RETAINED, "merged": d["merged"], "omitted": d["omitted"], "thinElements": d["thinElements"],
                            "hatching": d["hatching"], "styles": d["styles"], "overlays": OVERLAYS}
        asset["volumes"][key]["decisions"] = VOLUMES[key]
    sym = asset["symbols"][0]
    sym["dimensionalNote"] = t("The dot marks the true position of the rear axle of the scooter; its 0.9 mm diameter and the arrow are paper sizes and carry no dimension. Conventional symbol: it is not drawn to scale and carries no body dimension in the side, front or top view; the top view repeats the side view because a position mark looks the same from above, the front view shows the heading as a ring around the dot (the arrow points at the viewer).",
                               "Der Punkt markiert die wahre Position der Hinterachse des Elektromobils; sein Durchmesser von 0,9 mm und der Pfeil sind Papiergrößen ohne Maßbedeutung. Konventionelles Symbol: Es ist nicht maßstäblich gezeichnet und trägt weder in der Seiten-, Vorder- noch Draufsicht ein Körpermaß; die Draufsicht wiederholt die Seitenansicht, da eine Positionsmarke von oben gleich aussieht, die Vorderansicht zeigt die Richtung als Ring um den Punkt (der Pfeil zeigt auf die Betrachtenden).")
