"""📝️ Titles, summary, provenance, assumptions and per-scale decisions of `presenter-at-flipchart` (English first, German second)."""
from hw import t
from texts_desk import REFS
from scrub import scrub

DROP = (r"flag|pointer|pole|guide|sun|wind", r"\bFahne\b|Fahnenstange|\bStange|Zeigestab|Führ|Sonne|\bWind")

SIL = {
    "50": t("Every filled shape is the true orthographic outline of the 1:50 model in the presenting pose, per bone: head with nose, lips, ears and eyes under the cap with brim and button, short hair, neck in the stand collar, the softshell jacket with zip, pockets and hem band, the badge on its lanyard, sleeves, the bare right wrist, the right hand closed round the marker and the open left hand, chinos and shoes. The tripod flipchart is drawn as built: two front legs in the board plane with rubber glides, the melamine board, the paper pad with three written lines, the clamp bar at the head, the pen tray and the rear leg; the marker has barrel and cap.", "Jede gefüllte Form ist der wahre orthogonale Umriss des 1:50-Modells in der Vortragspose, je Knochen: Kopf mit Nase, Lippen, Ohren und Augen unter der Kappe mit Schirm und Knopf, kurzes Haar, Hals im Stehkragen, die Softshelljacke mit Reißverschluss, Taschen und Saumbund, der Ausweis am Band, Ärmel, das freie rechte Handgelenk, die rechte Hand um den Marker geschlossen und die offene linke Hand, Chinohose und Schuhe. Das Dreibein-Flipchart ist gezeichnet, wie es gebaut ist: zwei Vorderbeine in der Tafelebene mit Gummigleitern, die Melamintafel, der Papierblock mit drei beschriebenen Zeilen, die Klemmleiste am Kopf, die Stiftablage und das Hinterbein; der Marker hat Schaft und Kappe."),
    "100": t("Every filled shape is the true outline of the 1:100 model in the presenting pose: head with nose and ears, cap with brim, neck and collar, jacket with hem band, badge, sleeves, forearms, the bare right wrist, both hands, seat, trouser legs and shoes; the flipchart as two front legs, board, paper pad with three lines, clamp bar, tray and rear leg; the marker as one barrel.", "Jede gefüllte Form ist der wahre Umriss des 1:100-Modells in der Vortragspose: Kopf mit Nase und Ohren, Kappe mit Schirm, Hals und Kragen, Jacke mit Saumbund, Ausweis, Ärmel, Unterarme, das freie rechte Handgelenk, beide Hände, Gesäß, Hosenbeine und Schuhe; das Flipchart als zwei Vorderbeine, Tafel, Papierblock mit drei Zeilen, Klemmleiste, Ablage und Hinterbein; der Marker als ein Schaft."),
    "200": t("Every filled shape is the true outline of the 1:200 model in the presenting pose, one mass per part: head, cap with brim, neck, jacket, badge, sleeves, forearms, hands, seat, trouser legs and shoes; the flipchart as legs, board, paper pad, tray and clamp bar, the marker as a strip.", "Jede gefüllte Form ist der wahre Umriss des 1:200-Modells in der Vortragspose, eine Masse je Teil: Kopf, Kappe mit Schirm, Hals, Jacke, Ausweis, Ärmel, Unterarme, Hände, Gesäß, Hosenbeine und Schuhe; das Flipchart als Beine, Tafel, Papierblock, Ablage und Klemmleiste, der Marker als Streifen."),
    "500": t("Every filled shape is the true outline of the 1:500 model in the presenting pose: head, cap, neck, the jacket as one form, sleeves, forearms, hands, seat, trouser legs and shoes; the flipchart as three legs, the board and the paper on it; the marker merges into the raised hand.", "Jede gefüllte Form ist der wahre Umriss des 1:500-Modells in der Vortragspose: Kopf, Kappe, Hals, die Jacke als eine Form, Ärmel, Unterarme, Hände, Gesäß, Hosenbeine und Schuhe; das Flipchart als drei Beine, die Tafel und das Papier darauf; der Marker geht in der erhobenen Hand auf."),
}
RET = {
    "50": t("Glides, clamp bar, tray, the written lines and the marker cap, so the board reads as a working flipchart.", "Gleiter, Klemmleiste, Ablage, die beschriebenen Zeilen und die Markerkappe, damit die Tafel als benutztes Flipchart lesbar ist."),
    "100": t("The written lines on the sheet, the raised marker and the open gesturing hand.", "Die beschriebenen Zeilen auf dem Blatt, der erhobene Marker und die offene, gestikulierende Hand."),
    "200": t("The badge, the raised marker arm against the paper and the tripod with its rear leg.", "Der Ausweis, der erhobene Markerarm am Papier und das Dreibein mit seinem Hinterbein."),
    "500": t("The raised arm at the board and the three legs of the easel, so a presenter at a flipchart reads at 3.7 mm height.", "Der erhobene Arm an der Tafel und die drei Beine des Gestells, damit eine vortragende Person am Flipchart bei 3,7 mm Höhe lesbar ist."),
}
MER = {
    "50": t("Cap, brim and button are one shape, hand and fingers one shape per hand; the hinge of the rear leg merges into the clamp bar.", "Kappe, Schirm und Knopf sind eine Form, Hand und Finger je Hand eine Form; das Gelenk des Hinterbeins geht in der Klemmleiste auf."),
    "100": t("Marker cap into the barrel, glides into the legs.", "Markerkappe in den Schaft, Gleiter in die Beine."),
    "200": t("Lanyard into the badge, the written lines into the paper.", "Band in den Ausweis, die beschriebenen Zeilen in das Papier."),
    "500": t("Paper pad, clamp bar and tray into the board; the marker into the hand.", "Papierblock, Klemmleiste und Ablage in die Tafel; der Marker in die Hand."),
}
OMI = {
    "50": t("Fabric folds, seams, laces, single letters of the writing.", "Stofffalten, Nähte, Schnürsenkel, einzelne Buchstaben der Schrift."),
    "100": t("Glides, marker cap, zip and pockets.", "Gleiter, Markerkappe, Reißverschluss und Taschen."),
    "200": t("The written lines, the lanyard, zip, pockets and face details.", "Die beschriebenen Zeilen, das Band, Reißverschluss, Taschen und Gesichtsdetails."),
    "500": t("Badge, tray, clamp bar, writing and the marker as a shape of its own.", "Ausweis, Ablage, Klemmleiste, Schrift und der Marker als eigene Form."),
}
THIN = {
    "50": t("Legs (28 × 30 mm), board (18 mm) and marker (18 mm) at their true outline; the 6 mm paper pad is a filled sheet; the written lines are open detail strokes.", "Beine (28 × 30 mm), Tafel (18 mm) und Marker (18 mm) in ihrem wahren Umriss; der 6 mm starke Papierblock ist eine gefüllte Lage; die beschriebenen Zeilen sind offene Detailstriche."),
    "100": t("Legs, board and marker at their true outline, the paper pad a filled sheet.", "Beine, Tafel und Marker in ihrem wahren Umriss, der Papierblock eine gefüllte Lage."),
    "200": t("The 28 mm legs print as 0.14 mm strips plus the body pen; nothing is thickened.", "Die 28 mm starken Beine drucken als 0,14-mm-Streifen plus Körperstift; nichts ist verdickt."),
    "500": t("Legs and board are thinner than the pen and print as the 0.18 mm body line; the 1:500 body pen is 0.18 mm so that they do.", "Beine und Tafel sind dünner als der Stift und drucken als 0,18-mm-Körperlinie; der Körperstift von 1:500 ist dafür 0,18 mm."),
}
HATCH = t("None on the flipchart; the person keeps the hatching of the base figure.", "Keine am Flipchart; die Person behält die Schraffur der Ausgangsfigur.")
STYLE = t("Jacket dark, chinos mid, skin light; board and paper very light so the person reads in front of them, aluminium mid, marker dark.", "Jacke dunkel, Chino mittel, Haut hell; Tafel und Papier sehr hell, damit die Person davor lesbar ist, Aluminium mittel, Marker dunkel.")
OVER = t("Each shape carries the thermal region of the garment, body part or flipchart part it shows (board, paper, easel frame); the marker is too small for a region of its own and adds no heat.", "Jede Form trägt die thermische Region des Kleidungsstücks, Körperteils oder Flipchartteils, das sie zeigt (Tafel, Papier, Gestell); der Marker ist für eine eigene Region zu klein und gibt keine Wärme ab.")
VOL = {
    "body": t("The flipchart is boxes as built: two front legs, board, paper pad, tray and clamp bar along the 8 degree board plane, the rear leg to the head; the marker barrel and cap sit in the right hand and touch the paper with the tip.", "Das Flipchart besteht aus Quadern wie gebaut: zwei Vorderbeine, Tafel, Papierblock, Ablage und Klemmleiste entlang der um 8 Grad geneigten Tafelebene, das Hinterbein zum Kopf; Markerschaft und Kappe liegen in der rechten Hand und berühren mit der Spitze das Papier."),
    "thermal": t("Board, paper and easel frame are three passive surfaces by reference to their body solids.", "Tafel, Papier und Gestell sind drei passive Oberflächen als Verweis auf ihre Körper."),
    "structure": t("The easel carries itself through its feet into the floor (own contact, a floor bar between the feet); the marker is a member of the hand.", "Das Gestell trägt sich über seine Füße in den Boden (eigener Kontakt, ein Bodenstab zwischen den Füßen); der Marker ist ein Glied der Hand."),
}


def TEXTS(a, stature):
    a["title"] = t("Presenter at Flipchart", "Vortragende Person am Flipchart")
    a["summary"] = t(
        f"A workshop presenter of {stature:.2f} m (82 kg) in a softshell jacket over a shirt, chinos, cap and lanyard badge stands beside a tripod flipchart (board 0.70 × 1.00 m from 1.00 to 2.00 m), the marker raised to a word on the sheet and the free hand open to the group. Poses for presenting, writing and listening; loops for explaining, writing a line and idle breathing; 1.3 to 1.6 met with speech breathing, indoors at 21 °C.",
        f"Eine Person von {stature:.2f} m (82 kg) leitet einen Workshop: in Softshelljacke über dem Hemd, Chinohose, Kappe und mit Ausweis am Band steht sie neben einem Dreibein-Flipchart (Tafel 0,70 × 1,00 m von 1,00 bis 2,00 m), den Marker an ein Wort auf dem Blatt erhoben, die freie Hand offen zur Gruppe. Posen für Vortragen, Schreiben und Zuhören; Schleifen für Erklären, eine Zeile schreiben und ruhiges Atmen; 1,3 bis 1,6 met mit Sprechatmung, innen bei 21 °C.")
    a["tags"] = ["human", "adult", "standing", "office", "workshop", "presenter", "flipchart", "marker", "gesture", "speech", "workplace"]
    pv = a["provenance"]
    pv["author"] = "Semio staffage human worker HW-01"
    pv["method"] = "handcrafted"
    pv["created"] = "2026-11-22"
    have = {x["id"] for x in pv["references"]}
    pv["references"] += [x for x in REFS if x["id"] not in have]
    pv["notes"] = t(
        f"Derived by hand from the archive's tour-guide (rig, outlines and per-scale volumes of the body), scaled uniformly by 1.03 to a stature of {stature:.2f} m and 82 kg (de Leva 1996 male segment fractions); flag, pole and pointer removed, the cap, jacket and badge kept. Flipchart (tripod easel after common 0.70 × 1.00 m office products), marker, poses (the marker arm placed by inverse kinematics on the rig so that the tip touches the paper), clips, the indoor environment, thermal bindings and structure are authored by worker HW-01 (2026-11-22). Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-01/build_presenter.py.",
        f"Von Hand aus dem Archivobjekt tour-guide abgeleitet (Skelett, Umrisse und maßstabsweise Volumen des Körpers), gleichmäßig um 1,03 auf {stature:.2f} m und 82 kg skaliert (männliche Segmentanteile nach de Leva 1996); Fahne, Stange und Zeigestab entfernt, Kappe, Jacke und Ausweis behalten. Flipchart (Dreibein nach üblichen Büroprodukten 0,70 × 1,00 m), Marker, Posen (der Markerarm per inverser Kinematik am Skelett so gesetzt, dass die Spitze das Papier berührt), Clips, die Innenraumumgebung, thermische Bindungen und Tragstruktur sind von HW-01 erstellt (22.11.2026). Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-01/build_presenter.py.")
    for m in a["materials"]:
        if m["id"] == "pointer-aluminium":
            m["title"] = t("Anodised aluminium of the badge clip", "Eloxiertes Aluminium der Ausweisklammer")
    th = a["thermal"]
    th["environment"] = {"airTempC": 21, "meanRadiantTempC": 21, "skyTempC": 21, "groundTempC": 21, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
    th["assumptions"] = [
        t(f"DuBois skin area {th['body']['skinAreaM2']:.3f} m2 for {stature:.2f} m and 82 kg; layered clothing of long-sleeve shirt (0.25 clo), trapped air and a softshell jacket (0.3 clo) on the trunk and arms, cotton chinos (0.24 clo), after ISO 9920 with 1 clo = 0.155 m2K/W.", f"Hautfläche nach DuBois {th['body']['skinAreaM2']:.3f} m2 für {stature:.2f} m und 82 kg; Kleidung in Schichten aus Langarmhemd (0,25 clo), eingeschlossener Luft und Softshelljacke (0,3 clo) an Rumpf und Armen, Baumwoll-Chino (0,24 clo), nach ISO 9920 mit 1 clo = 0,155 m2K/W."),
        t("Presenting with an active arm is 1.5 met, writing on the chart 1.6 met, listening 1.3 met (ISO 8996 standing with light arm work; Ainsworth et al. 2011), in a meeting room at 21 °C, 50 % humidity and still air.", "Vortragen mit aktivem Arm sind 1,5 met, Schreiben am Flipchart 1,6 met, Zuhören 1,3 met (ISO 8996 Stehen mit leichter Armarbeit; Ainsworth et al. 2011), in einem Besprechungsraum bei 21 °C, 50 % Feuchte und ruhender Luft."),
        th["assumptions"][2],
        t("Board, paper and the aluminium easel are passive surfaces without internal heat; the marker is part of the hand's thermal surface. Total power is the metabolic heat alone (the easel states 0 W passive).", "Tafel, Papier und das Aluminiumgestell sind passive Oberflächen ohne innere Wärme; der Marker gehört zur thermischen Oberfläche der Hand. Die Gesamtleistung ist allein die Stoffwechselwärme (das Gestell nennt 0 W passiv)."),
    ]
    a["structure"]["assumptions"] = [
        t("Segments are point masses at the de Leva centres; the raised marker arm loads the right shoulder with the weight of arm, hand and marker (15 g) times their lever arms.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; der erhobene Markerarm belastet die rechte Schulter mit dem Gewicht von Arm, Hand und Marker (15 g) mal ihren Hebelarmen."),
        t("The easel (5.5 kg) stands on the floor through its own contact and does not load the person; the marker tip only touches the paper.", "Das Gestell (5,5 kg) steht über den eigenen Kontakt am Boden und belastet die Person nicht; die Markerspitze berührt das Papier nur."),
        t("Gestures are quasi-static key frames.", "Gesten sind quasistatische Schlüsselbilder."),
    ]
    for sc, rep in a["representations"].items():
        rep["decisions"] = {"silhouette": SIL[sc], "retained": RET[sc], "merged": MER[sc], "omitted": OMI[sc], "thinElements": THIN[sc], "hatching": HATCH, "styles": STYLE, "overlays": OVER}
    for sc, vol in a["volumes"].items():
        kinds = ["body", "thermal", "structure"]
        vol["decisions"] = [{k: f"{scrub(d, DROP)[k]} {VOL[kind][k]}".strip() for k in ("en", "de")} for d, kind in zip(vol["decisions"], kinds)]
