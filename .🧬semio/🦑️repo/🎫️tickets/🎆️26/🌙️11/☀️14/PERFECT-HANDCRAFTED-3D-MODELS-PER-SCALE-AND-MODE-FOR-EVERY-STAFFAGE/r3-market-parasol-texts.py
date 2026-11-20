"""📝 Bilingual texts of the market parasol's root fixes (summary, notes, structure assumptions, reference, 2D decisions per scale), English first, German second.

See ./r3-objects-market-parasol.md
"""


def t(en, de):
    return {"en": en, "de": de}


SUMMARY = t(
    "A tilting market parasol with a concrete ballast block, a clevis foot, a two-part aluminium mast with coupling collar and crank winch, and an octagonal canopy of eight flat polyester gores on eight ribs at 45 degrees (11 degrees plan offset, so all eight ribs show in elevation); a runner with eight stretchers folds the ribs down along the mast.",
    "Ein neigbarer Marktschirm mit Betonballastblock, Gabelfuß, zweiteiligem Aluminiummast mit Kupplungsmuffe und Kurbelwinde und einem achteckigen Schirm aus acht ebenen Polyesterbahnen auf acht Speichen im 45-Grad-Abstand (11 Grad Planversatz, damit alle acht Speichen im Aufriss sichtbar sind); ein Läufer mit acht Spreizen klappt die Speichen am Mast nach unten.",
)

PROVENANCE_NOTES = t(
    "Side elevation. The base is a 0.4 x 0.3 x 0.2 m concrete block (about 58 kg) with a 15 mm aluminium cover plate and a clevis bracket; the mast tilts on a horizontal pin 60 mm above the plate (the analysis treats the foot as a fixed socket, so tipping and sliding of the base are not checked; ballast data unknown). Mast: lower tube 48 mm, upper tube 40 mm, joined by a 68 mm coupling collar, crank winch at 1.38 m. Canopy: eight 16 x 10 mm aluminium ribs, 1.368 m from the mast axis to the tip cap, pinned at the crown 2.35 m above ground and propped by eight 0.55 m stretchers from the runner, 15.3 degrees below horizontal; eight flat gores, hem at 1.99 m (headroom; reference: 2.04 m for an eight-rib 2.7 m parasol). Wind: q = 0.5 rho v^2 at v = 8 m/s (about 40 Pa), drag 1.2 and uplift 0.5 on 5.3 m2 projected canopy, authored estimates on the basis of EN 1991-1-4. Each rib carries the mass of its gore and stretcher. Tilt is 8 degrees towards the sun on the right.",
    "Seitenansicht. Der Fuß ist ein Betonblock 0,4 x 0,3 x 0,2 m (etwa 58 kg) mit 15 mm Aluminium-Deckplatte und Gabelbock; der Mast neigt sich auf einem waagrechten Bolzen 60 mm über der Platte (die Berechnung behandelt den Fuß als feste Hülse, Kippen und Gleiten des Fußes werden nicht nachgewiesen; Ballastdaten unbekannt). Mast: Unterrohr 48 mm, Oberrohr 40 mm, verbunden durch eine 68-mm-Kupplungsmuffe, Kurbelwinde auf 1,38 m. Schirm: acht Aluminiumspeichen 16 x 10 mm, 1,368 m von der Mastachse bis zur Endkappe, an der Krone 2,35 m über Grund gelagert und von acht 0,55 m langen Spreizen vom Läufer abgestützt, 15,3 Grad unter der Waagrechten; acht ebene Stoffbahnen, Saum auf 1,99 m (Durchgangshöhe; Referenz: 2,04 m bei einem achtspeichigen 2,7-m-Schirm). Wind: q = 0,5 rho v^2 bei v = 8 m/s (etwa 40 Pa), Widerstand 1,2 und Auftrieb 0,5 auf 5,3 m² projizierter Schirmfläche, Schätzungen auf Basis EN 1991-1-4. Jede Speiche trägt die Masse ihrer Bahn und Spreize. Neigung 8 Grad zur Sonne rechts.",
)

STRUCTURE_ASSUMPTIONS = [
    t("Base is a fixed socket; the canopy wind loads act at the crown hub as a quasi-static design state.", "Der Fuß ist eine feste Hülse; die Windlasten am Schirm wirken an der Krone als quasistatischer Bemessungszustand."),
    t("Sliding and tipping of a free base are not checked.", "Gleiten und Kippen eines freien Fußes werden nicht nachgewiesen."),
    t("The eight ribs are pinned at the crown at 45 degree spacing and each is propped by its own stretcher from the runner, pinned 0.50 m along the rib; the linkage closes in every pose (open, tilted, flutter, furled).", "Die acht Speichen sind an der Krone im 45-Grad-Abstand gelagert und jede wird von ihrer eigenen Spreize vom Läufer abgestützt, 0,50 m entlang der Speiche gelenkig angeschlossen; das Gestänge schließt in jeder Pose (offen, geneigt, Flattern, geschlossen)."),
]

REFERENCE = {"id": "maanta-lybra", "citation": "Maanta, LYBRA octagonal parasol, product data (aluminium pole 38 mm, eight ribs, headroom 2.04 m at 2.7 m, 1.98 m at 3.0 m canopy), archiexpo.com product 4631045-2565737, accessed 2026-11-20."}


def references(old):
    return [r for r in old if r["id"] != REFERENCE["id"]] + [REFERENCE]


COMMON_OVERLAYS = t(
    "Gores = canopy-top (sun-lit upper fabric); mast, collar, crank, clevis, plate, ribs, tips, runner, stretchers and crown = frame-alu; block = base-concrete; the shaded underside (canopy-underside) is hidden in every view and read numerically.",
    "Stoffbahnen = canopy-top (besonnter Oberstoff); Mast, Muffe, Kurbel, Gabelbock, Platte, Speichen, Endkappen, Läufer, Spreizen und Krone = frame-alu; Block = base-concrete; die beschattete Unterseite (canopy-underside) ist in jeder Ansicht verdeckt und wird numerisch gelesen.",
)

DECISIONS_2D = {
    "50": {
        "silhouette": t(
            "Side elevation 52 x 49 mm, the true view of the 1:50 model: concrete block with seam, cover plate with bolts, clevis cheek with pin, two-part mast with coupling collar, clamp knob, winch and crank, runner, crown hub with top cap and finial, and the octagonal canopy of eight flat gores, each an extruded solid on its rib, so the side view shows the shallow pyramid with the rib ridges as gore edges and every tip cap at the hem. Front: the same parts seen from ahead with the clevis cheeks and pin across; plan: the octagon of the eight gores with the eight tip caps, top cap and finial.",
            "Seitenansicht 52 x 49 mm, die wahre Ansicht des 1:50-Modells: Betonblock mit Fuge, Deckplatte mit Schrauben, Gabelwange mit Bolzen, zweiteiliger Mast mit Kupplungsmuffe, Klemmknopf, Winde und Kurbel, Läufer, Krone mit Kappe und Knauf, und der achteckige Schirm aus acht ebenen Bahnen, jede ein extrudierter Körper auf ihrer Speiche, sodass die Seitenansicht die flache Pyramide mit den Speichengraten als Bahnkanten und jede Endkappe am Saum zeigt. Vorderansicht: dieselben Teile von vorn mit den Gabelwangen und dem Bolzen quer; Draufsicht: das Achteck der acht Bahnen mit den acht Endkappen, Kappe und Knauf.",
        ),
        "retained": t(
            "All eight gores, ribs with tip caps and stretchers (hidden under the fabric except where they show below the hem), runner, crown hub with top cap and finial, mast tubes, coupling collar with clamp knob, winch, crank arm and handle, clevis cheeks and pin, cover plate with four bolts, block with seam and grit.",
            "Alle acht Bahnen, Speichen mit Endkappen und Spreizen (unter dem Stoff verdeckt, außer wo sie unter dem Saum erscheinen), Läufer, Krone mit Kappe und Knauf, Mastrohre, Kupplungsmuffe mit Klemmknopf, Winde, Kurbelarm und Griff, Gabelwangen und Bolzen, Deckplatte mit vier Schrauben, Block mit Fuge und Körnung.",
        ),
        "merged": t("Nothing merged.", "Nichts verschmolzen."),
        "omitted": t(
            "The canopy underside (hidden from every view), the winch cord inside the mast, the rib pivot wire inside the hub.",
            "Die Unterseite der Bespannung (in jeder Ansicht verdeckt), die Windenschnur im Mast, der Speichenlagerdraht in der Krone.",
        ),
        "thinElements": t(
            "Ribs 16 x 10 mm and stretchers 11 mm are members of their true section (0.2 to 0.32 mm on paper); gores 8 mm thick, the feature of the scale (cloth 0.5 mm); crank arm 8 mm, tip caps 18 mm.",
            "Speichen 16 x 10 mm und Spreizen 11 mm sind Stäbe mit ihrem wahren Querschnitt (0,2 bis 0,32 mm auf dem Papier); Bahnen 8 mm dick, das Mindestmaß des Maßstabs (Stoff 0,5 mm); Kurbelarm 8 mm, Endkappen 18 mm.",
        ),
        "hatching": t("None; base grit and mast seam are single 0.13 mm strokes.", "Keine; Betonkörnung und Mastnaht sind einzelne 0,13-mm-Striche."),
        "styles": t("Fabric very light (0.82), frame mid (0.2-0.45), block 0.62; body pen 0.5, gore edges 0.35, details 0.25, texture 0.13.", "Stoff sehr hell (0,82), Gestell mittel (0,2-0,45), Block 0,62; Körperstift 0,5, Bahnkanten 0,35, Details 0,25, Textur 0,13."),
        "overlays": COMMON_OVERLAYS,
    },
    "100": {
        "silhouette": t(
            "Side elevation 26 x 24.6 mm: block with cover plate and clevis, one mast strip with coupling collar and the winch with its crank merged into one small outline, runner, crown hub merged with its top cap and finial, and the eight gores as the shallow octagonal pyramid with the rib ridges as gore edges. Front: block, plate, clevis block, mast, collar, runner, hub and the eight gores; plan: the octagon of the gores and the top cap.",
            "Seitenansicht 26 x 24,6 mm: Block mit Deckplatte und Gabelbock, ein Maststreifen mit Kupplungsmuffe und die Winde mit Kurbel zu einer kleinen Kontur verschmolzen, Läufer, Krone mit Kappe und Knauf verschmolzen, und die acht Bahnen als flache achteckige Pyramide mit den Speichengraten als Bahnkanten. Vorderansicht: Block, Platte, Gabelblock, Mast, Muffe, Läufer, Krone und die acht Bahnen; Draufsicht: das Achteck der Bahnen und die Kappe.",
        ),
        "retained": t("Eight gores, eight ribs and stretchers, runner, hub, collar, winch with crank, clevis, plate, block.", "Acht Bahnen, acht Speichen und Spreizen, Läufer, Krone, Muffe, Winde mit Kurbel, Gabelbock, Platte, Block."),
        "merged": t(
            "The two mast diameters (48 and 40 mm, a 4 mm step) into one 44 mm strip; winch, crank arm and handle into one outline; hub, top cap and finial into one outline; clevis cheeks, pin and flange into the clevis outline.",
            "Die zwei Mastdurchmesser (48 und 40 mm, 4 mm Absatz) zu einem 44-mm-Streifen; Winde, Kurbelarm und Griff zu einer Kontur; Krone, Kappe und Knauf zu einer Kontur; Gabelwangen, Bolzen und Flansch zur Gabelkontur.",
        ),
        "omitted": t("Rib tip caps, bolts, clamp knob, block seam and grit (below 15 mm).", "Endkappen, Schrauben, Klemmknopf, Blockfuge und Körnung (unter 15 mm)."),
        "thinElements": t("Ribs and stretchers as members of their true section; gores 15 mm thick.", "Speichen und Spreizen als Stäbe mit wahrem Querschnitt; Bahnen 15 mm dick."),
        "hatching": t("None.", "Keine."),
        "styles": t("Fabric 0.82, frame 0.25-0.45, block 0.62; body pen 0.35, gore edges 0.3.", "Stoff 0,82, Gestell 0,25-0,45, Block 0,62; Körperstift 0,35, Bahnkanten 0,3."),
        "overlays": COMMON_OVERLAYS,
    },
    "200": {
        "silhouette": t(
            "Side elevation 13 x 12.3 mm: block, the clevis merged with the plate into one foot block, the mast strip, the runner, the hub with cap and finial as one tapering outline and the eight gores (30 mm) as the shallow octagonal pyramid. Front: the same; plan: the octagon of the eight gores with the hub.",
            "Seitenansicht 13 x 12,3 mm: Block, Gabelbock und Platte zu einem Fußblock verschmolzen, der Maststreifen, der Läufer, die Krone mit Kappe und Knauf als eine sich verjüngende Kontur und die acht Bahnen (30 mm) als flache achteckige Pyramide. Vorderansicht: dasselbe; Draufsicht: das Achteck der acht Bahnen mit der Krone.",
        ),
        "retained": t("Octagonal canopy of eight gores, runner, hub, mast, foot block, base block.", "Achteckiger Schirm aus acht Bahnen, Läufer, Krone, Mast, Fußblock, Betonblock."),
        "merged": t("Plate, clevis and pin into one foot block; ribs into the gore edges (they lie under the fabric); collar into the mast strip.", "Platte, Gabelbock und Bolzen zu einem Fußblock; Speichen in die Bahnkanten (sie liegen unter dem Stoff); Muffe im Maststreifen."),
        "omitted": t("Stretchers (hidden under the canopy), crank and winch, tip caps (below 30 mm).", "Spreizen (unter dem Schirm verdeckt), Kurbel und Winde, Endkappen (unter 30 mm)."),
        "thinElements": t("The 44 mm mast is a 0.22 mm strip; gores 30 mm thick.", "Der 44-mm-Mast ist ein 0,22-mm-Streifen; Bahnen 30 mm dick."),
        "hatching": t("None.", "Keine."),
        "styles": t("Fabric 0.82, frame 0.3-0.45, block 0.62; body pen 0.25.", "Stoff 0,82, Gestell 0,3-0,45, Block 0,62; Körperstift 0,25."),
        "overlays": COMMON_OVERLAYS,
    },
    "500": {
        "silhouette": t(
            "Parasol sign 5.2 x 4.9 mm: the base block, the mast as one stroke and the eight gores (75 mm) as the flat octagonal pyramid of the canopy; front the same seen from ahead, plan the octagon.",
            "Schirmzeichen 5,2 x 4,9 mm: der Betonblock, der Mast als ein Strich und die acht Bahnen (75 mm) als flache achteckige Pyramide des Schirms; Vorderansicht dasselbe von vorn, Draufsicht das Achteck.",
        ),
        "retained": t("Overall width 2.59 m and height 2.46 m, the octagonal canopy as the dominant shape, the block as the only mass at the foot.", "Gesamtbreite 2,59 m und Höhe 2,46 m, der achteckige Schirm als dominante Form, der Block als einzige Masse am Fuß."),
        "merged": t("Ribs, hub, top cap and runner into the gores and the mast stroke.", "Speichen, Krone, Kappe und Läufer in die Bahnen und den Maststrich."),
        "omitted": t("Stretchers, crank, collar, clevis and plate (below 75 mm).", "Spreizen, Kurbel, Muffe, Gabelbock und Platte (unter 75 mm)."),
        "thinElements": t("The 44 mm mast is a single 0.18 mm stroke (90 mm real); the gores are 75 mm (0.15 mm) plates.", "Der 44-mm-Mast ist ein einzelner 0,18-mm-Strich (90 mm real); die Bahnen sind 75 mm (0,15 mm) dicke Platten."),
        "hatching": t("None.", "Keine."),
        "styles": t("Fabric 0.82, block 0.62, mast 0.45; one 0.18 mm outline.", "Stoff 0,82, Block 0,62, Mast 0,45; eine 0,18-mm-Kontur."),
        "overlays": COMMON_OVERLAYS,
    },
}
