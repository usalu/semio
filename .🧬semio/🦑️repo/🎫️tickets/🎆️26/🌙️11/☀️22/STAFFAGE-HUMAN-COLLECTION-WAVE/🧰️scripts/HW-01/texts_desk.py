"""📝️ Titles, summary, provenance, assumptions and per-scale decisions of `standing-desk-worker` (English first, German second)."""
import re

from hw import t

REFS = [
    {"id": "en-527-1-2011", "citation": "EN 527-1:2011 Office furniture, work tables and desks, part 1: dimensions (height-adjustable desks up to 1.20 m for standing work)."},
    {"id": "iso-9241-5-1998", "citation": "ISO 9241-5:1998 Ergonomic requirements for office work with visual display terminals, part 5: workstation layout and postural requirements (keyboard at or slightly below elbow height)."},
    {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (standing office and computer work 1.5 to 1.8 MET)."},
    {"id": "iec-62623-2012", "citation": "IEC 62623:2012 Desktop and notebook computers, measurement of energy consumption (typical notebook power in idle and active states, 65 W adapters as the common nameplate)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products, hygrothermal properties, tabulated design values (steel density, conductivity, specific heat)."},
    {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate (standing light hand and arm work)."},
]

SCALE_SIL = {
    "50": ("The three views are the true orthographic views of the 1:50 body and desk in the typing pose, {mm} mm tall: head with jaw, nose and chin under the hair cap, ear and eye, neck, hooded sweatshirt with collar rib, hem rib and the hood on the upper back, sleeves, pronated forearms and rib cuffs, hands with thumb and curled fingers on the home row, jeans seat, thighs with kneecaps, shanks, socks and sneakers. The sit-stand desk is drawn as it is built: laminated top with its edge band, per side a T-foot, a lower and an upper column tube, the drive housing at the column head and the side rail, the crossbar, the control box and the handset under the front edge; the laptop is base, open lid, keyboard field, touchpad and display outline. Front view (the figure's left is on the right of the sheet): the desk stands between the viewer and the person, the lid shows its back. Plan: the desk top covers the legs and feet, the hands and forearms lie on it over the laptop.",
           "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:50-Körpers mit Tisch in der Tipp-Pose, {mm} mm hoch: Kopf mit Kiefer, Nase und Kinn unter der Haarkappe, Ohr und Auge, Hals, Kapuzensweatshirt mit Kragenbund, Saumbund und der Kapuze auf dem oberen Rücken, Ärmel, einwärts gedrehte Unterarme und Bündchen, Hände mit Daumen und gekrümmten Fingern auf der Grundreihe, Jeansgesäß, Oberschenkel mit Kniescheiben, Unterschenkel, Socken und Sneaker. Der Sitz-Steh-Tisch ist gezeichnet, wie er gebaut ist: beschichtete Platte mit Umleimer, je Seite ein T-Fuß, ein unteres und ein oberes Säulenrohr, das Antriebsgehäuse am Säulenkopf und die Seitenzarge, die Traverse, die Steuerbox und das Bedienteil unter der Vorderkante; der Laptop ist Unterteil, offener Deckel, Tastaturfeld, Touchpad und Displayumriss. Vorderansicht (die linke Körperseite liegt rechts im Blatt): der Tisch steht zwischen Betrachter und Person, der Deckel zeigt seine Rückseite. Grundriss: die Tischplatte verdeckt Beine und Füße, Hände und Unterarme liegen darüber auf dem Laptop."),
    "100": ("The three views are the true orthographic views of the 1:100 body and desk, {mm} mm tall: head with nose under the hair cap, ear, neck, sweatshirt with the hood on the upper back, sleeves, forearms, hands on the keys, jeans seat, thighs, shanks and shoes. The desk is top, T-feet, one column per side with its drive housing, side rails and crossbar; the laptop is base and lid with the keyboard field and the display outline in plan.",
            "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:100-Körpers mit Tisch, {mm} mm hoch: Kopf mit Nase unter der Haarkappe, Ohr, Hals, Sweatshirt mit der Kapuze auf dem oberen Rücken, Ärmel, Unterarme, Hände auf den Tasten, Jeansgesäß, Oberschenkel, Unterschenkel und Schuhe. Der Tisch ist Platte, T-Füße, eine Säule je Seite mit ihrem Antriebsgehäuse, Seitenzargen und Traverse; der Laptop ist Unterteil und Deckel mit Tastaturfeld und Displayumriss im Grundriss."),
    "200": ("The three views are the true orthographic views of the 1:200 body and desk, {mm} mm tall, one mass per part: head, hair cap, neck, sweatshirt, hood, jeans seat, sleeves, forearms, hands, thighs, shanks and shoes; the desk is top, two columns, two T-feet and the crossbar, the laptop base and lid as two plates.",
            "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:200-Körpers mit Tisch, {mm} mm hoch, eine Masse je Teil: Kopf, Haarkappe, Hals, Sweatshirt, Kapuze, Jeansgesäß, Ärmel, Unterarme, Hände, Oberschenkel, Unterschenkel und Schuhe; der Tisch ist Platte, zwei Säulen, zwei T-Füße und die Traverse, der Laptop Unterteil und Deckel als zwei Platten."),
    "500": ("A person of {h} m ({mm} mm) at a desk, the true orthographic views of the 1:500 body: head with hair as one mass, neck, sweatshirt with the hood merged in, jeans seat, and per side sleeve, forearm, hand, thigh, shank and shoe; the desk is the top, two columns and two T-feet, the laptop two plates.",
            "Eine Person von {h} m ({mm} mm) am Tisch, die wahren orthogonalen Ansichten des 1:500-Körpers: Kopf mit Haar als eine Masse, Hals, Sweatshirt mit eingeschlossener Kapuze, Jeansgesäß und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel und Schuh; der Tisch ist Platte, zwei Säulen und zwei T-Füße, der Laptop zwei Platten."),
}

RETAINED = {
    "50": t("Everything of the 1:100 drawing plus the column tubes, drive housings, control box, handset, edge band, touchpad and the finger articulation that are still wider than 0.1 mm at this scale.", "Alles aus der Zeichnung 1:100 und dazu Säulenrohre, Antriebsgehäuse, Steuerbox, Bedienteil, Umleimer, Touchpad und die Fingergliederung, die bei diesem Maßstab noch breiter als 0,1 mm sind."),
    "100": t("The typing posture with elbows at the trunk and hands on the keys, the open lid, the desk frame with drives and rails.", "Die Tipphaltung mit Ellbogen am Rumpf und Händen auf den Tasten, der offene Deckel, das Tischgestell mit Antrieben und Zargen."),
    "200": t("Elbows and hands as separate masses so the typing posture reads, the open lid, the two columns that tell a sit-stand desk from a table.", "Ellbogen und Hände als eigene Massen, damit die Tipphaltung lesbar bleibt, der offene Deckel, die beiden Säulen, die einen Sitz-Steh-Tisch von einem Tisch unterscheiden."),
    "500": t("The upright stance at the desk, the forearms over the top, the lid and the exact stature, desk height and extent.", "Die aufrechte Haltung am Tisch, die Unterarme über der Platte, der Deckel und die genaue Körpergröße, Tischhöhe und Ausdehnung."),
}

PROP_MERGED = {
    "50": t("The telescoping column is two tubes, the drive and the column head one housing.", "Die Teleskopsäule sind zwei Rohre, Antrieb und Säulenkopf ein Gehäuse."),
    "100": t("Column tubes into one column per side; control box and handset into the rail.", "Säulenrohre zu einer Säule je Seite; Steuerbox und Bedienteil in die Zarge."),
    "200": t("Drive housings and rails into the columns and top; keyboard into the laptop base.", "Antriebsgehäuse und Zargen in Säulen und Platte; Tastatur in das Laptop-Unterteil."),
    "500": t("Crossbar, rails and drives into the top and columns; the laptop into two plates.", "Traverse, Zargen und Antriebe in Platte und Säulen; der Laptop in zwei Platten."),
}

PROP_OMITTED = {
    "50": t("Cable tray, glides under the feet, single keys and the hinge barrels.", "Kabelwanne, Gleiter unter den Füßen, einzelne Tasten und die Scharnierrollen."),
    "100": t("Control box, handset, touchpad and edge band.", "Steuerbox, Bedienteil, Touchpad und Umleimer."),
    "200": t("Keyboard field, display outline, drive housings and rails.", "Tastaturfeld, Displayumriss, Antriebsgehäuse und Zargen."),
    "500": t("Crossbar and every inner line of desk and laptop.", "Traverse und jede Innenlinie von Tisch und Laptop."),
}

THIN = {
    "50": t("The 25 mm top and the 18 mm laptop base are drawn at their true thickness; the 6 mm lid is a filled plate of true thickness.", "Die 25 mm starke Platte und das 18 mm hohe Laptop-Unterteil sind in wahrer Dicke gezeichnet; der 6 mm starke Deckel ist eine gefüllte Platte wahrer Dicke."),
    "100": t("Top, laptop base and lid keep their true thickness; the outline pen carries them.", "Platte, Laptop-Unterteil und Deckel behalten ihre wahre Dicke; der Umrissstift trägt sie."),
    "200": t("Top 0.13 mm, lid 0.03 mm on paper: both are filled plates that the body pen makes printable, never thickened.", "Platte 0,13 mm, Deckel 0,03 mm auf dem Papier: beide sind gefüllte Platten, die der Körperstift druckbar macht, nie verdickt."),
    "500": t("Top 0.05 mm and lid 0.01 mm on paper print as the 0.18 mm body pen line; nothing is thickened.", "Platte 0,05 mm und Deckel 0,01 mm auf dem Papier drucken als 0,18-mm-Linie des Körperstifts; nichts ist verdickt."),
}

STYLE_ADD = t("Desk top light, steel frame mid-grey, drive housings dark, laptop light aluminium.", "Tischplatte hell, Stahlgestell mittelgrau, Antriebsgehäuse dunkel, Laptop helles Aluminium.")
OVERLAY_ADD = t("Desk top, frame and drives carry their own passive regions (the drives hold the 200 W nameplate of the lifting columns), the laptop its operating region that warms with the load.", "Tischplatte, Gestell und Antriebe tragen ihre eigenen passiven Regionen (die Antriebe das 200-W-Typenschild der Hubsäulen), der Laptop seine Betriebsregion, die sich mit der Last erwärmt.")

VOL_ADD = {
    "body": t("The sit-stand desk is boxes as built (top, T-feet, column tubes, drive housings, rails, crossbar, control box), the laptop a base and a lid plate opened 108 degrees; the fingertips of the typing hands rest in the key field.", "Der Sitz-Steh-Tisch besteht aus Quadern wie gebaut (Platte, T-Füße, Säulenrohre, Antriebsgehäuse, Zargen, Traverse, Steuerbox), der Laptop aus Unterteil und einer um 108 Grad geöffneten Deckelplatte; die Fingerspitzen der tippenden Hände liegen im Tastenfeld."),
    "thermal": t("Desk top, steel frame and drives are three passive surfaces, the laptop one operating surface with its fan outlet at the hinge.", "Tischplatte, Stahlgestell und Antriebe sind drei passive Oberflächen, der Laptop eine Betriebsfläche mit dem Lüfterauslass am Scharnier."),
    "structure": t("The desk carries itself and the laptop through its feet into the floor (support solid under the feet, own contact); its mass and the laptop's are point masses.", "Der Tisch trägt sich und den Laptop über seine Füße in den Boden (Auflagerkörper unter den Füßen, eigener Kontakt); seine Masse und die des Laptops sind Punktmassen."),
}


def fix(text, h):
    out = {}
    for lang in ("en", "de"):
        s = text[lang]
        s = re.sub(r"1[.,]69 m", "1.76 m" if lang == "en" else "1,76 m", s)
        s = s.replace("58 kg", "66 kg").replace("relaxed pose", "typing pose").replace("entspannten Pose", "Tipp-Pose")
        out[lang] = s
    return out


def TEXTS(a, stature):
    h = round(stature + 0.02 * 1.04 + 0.01, 2)
    a["title"] = t("Standing Desk Worker", "Person am Stehpult")
    a["summary"] = t(
        f"A tall, slim woman of {stature:.2f} m (66 kg) in a hooded sweatshirt, jeans and sneakers types on a 15.6-inch laptop at a dual-motor sit-stand desk raised to 1.01 m, the keyboard just below her elbows. Poses for typing, scrolling on the touchpad and reading with crossed arms; loops for typing and idle breathing. 1.5 met at the keys, a 65 W laptop and the 200 W nameplate of the desk drives for internal-gain studies.",
        f"Eine große, schlanke Frau von {stature:.2f} m (66 kg) in Kapuzensweatshirt, Jeans und Sneakern tippt auf einem 15,6-Zoll-Laptop an einem auf 1,01 m hochgefahrenen Sitz-Steh-Tisch mit zwei Motoren, die Tastatur knapp unter den Ellbogen. Posen für Tippen, Scrollen auf dem Touchpad und Lesen mit verschränkten Armen; Schleifen für Tippen und ruhiges Atmen. 1,5 met an der Tastatur, ein 65-W-Laptop und das 200-W-Typenschild der Tischantriebe für Studien zu inneren Wärmegewinnen.")
    a["tags"] = ["human", "adult", "woman", "standing", "office", "standing-desk", "sit-stand-desk", "laptop", "typing", "internal-gains", "workplace"]
    pv = a["provenance"]
    pv["author"] = "Semio staffage human worker HW-01"
    pv["method"] = "handcrafted"
    pv["created"] = "2026-11-22"
    have = {x["id"] for x in pv["references"]}
    pv["references"] += [x for x in REFS if x["id"] not in have]
    pv["notes"] = t(
        f"Derived by hand from the archive's standing-adult (rig, outlines and the per-scale volumes of the body), scaled uniformly by 1.04 to a stature of {stature:.2f} m barefoot and 66 kg with the de Leva (1996) female segment table; the desk, the laptop, every pose, clip, thermal binding and the structure are authored for this asset by worker HW-01 (2026-11-22). Desk height 1.01 m follows ISO 9241-5 (home row at or a little below the standing elbow height of 1.12 m) within the EN 527-1 range of sit-stand desks; the arms were placed by inverse kinematics on the rig and checked in all three views. Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-01/build_desk.py.",
        f"Von Hand aus dem Archivobjekt standing-adult abgeleitet (Skelett, Umrisse und die maßstabsweisen Volumen des Körpers), gleichmäßig um 1,04 auf eine Körpergröße von {stature:.2f} m barfuß und 66 kg skaliert, mit der Frauentabelle von de Leva (1996); Tisch, Laptop, alle Posen, Clips, thermischen Bindungen und die Tragstruktur sind für dieses Objekt von HW-01 erstellt (22.11.2026). Die Tischhöhe von 1,01 m folgt ISO 9241-5 (Grundreihe auf oder knapp unter der Ellbogenhöhe im Stehen von 1,12 m) im Bereich der Sitz-Steh-Tische nach EN 527-1; die Arme wurden per inverser Kinematik am Skelett gesetzt und in allen drei Ansichten geprüft. Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-01/build_desk.py.")
    th = a["thermal"]
    th["assumptions"] = [
        t(f"Skin area {th['body']['skinAreaM2']:.3f} m2 after DuBois and DuBois (1916) for 66 kg and {stature:.2f} m; the regions keep the skin fractions of the base figure (0.07 face and neck, 0.03 scalp, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet).", f"Hautfläche {th['body']['skinAreaM2']:.3f} m2 nach DuBois und DuBois (1916) für 66 kg und {stature:.2f} m; die Regionen behalten die Hautanteile der Ausgangsfigur (0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße)."),
        th["assumptions"][1],
        t("Typing standing is 1.5 met, scrolling 1.4 met and reading with crossed arms 1.3 met (Ainsworth et al. 2011: standing computer work 1.5 to 1.8 MET; ISO 8996: standing light hand work), authored within those ranges.", "Tippen im Stehen sind 1,5 met, Scrollen 1,4 met und Lesen mit verschränkten Armen 1,3 met (Ainsworth et al. 2011: Computerarbeit im Stehen 1,5 bis 1,8 MET; ISO 8996: leichte Handarbeit im Stehen), innerhalb dieser Bereiche geschätzt."),
        t("The laptop states its 65 W adapter as nameplate (IEC 62623 practice) and releases 25 W while typing, 9 W idling; the desk drives state 200 W electrical input of the two lifting columns, but stand still and release nothing in every pose.", "Der Laptop nennt sein 65-W-Netzteil als Typenschild (Praxis nach IEC 62623) und gibt beim Tippen 25 W ab, im Leerlauf 9 W; die Tischantriebe nennen 200 W elektrische Aufnahme der beiden Hubsäulen, stehen aber in jeder Pose still und geben nichts ab."),
        th["assumptions"][3],
    ]
    a["structure"]["assumptions"] = [
        t("Segments are point masses at the de Leva centres (female table).", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle)."),
        t("Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose.", "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach in Hüftgelenkbreite."),
        t("The desk (37.5 kg) and the laptop (1.9 kg) stand on the floor through the desk's own contact and do not load the person; the fingers only touch the keys.", "Der Tisch (37,5 kg) und der Laptop (1,9 kg) stehen über den eigenen Kontakt des Tischs am Boden und belasten die Person nicht; die Finger berühren die Tasten nur."),
    ]
    for sc, rep in a["representations"].items():
        mm = round(h * 1000 / int(sc), 1)
        dec = rep["decisions"]
        en, de = SCALE_SIL[sc]
        dec["silhouette"] = t(en.format(mm=mm, h=f"{h:.2f}"), de.format(mm=str(mm).replace(".", ","), h=f"{h:.2f}".replace(".", ",")))
        dec["retained"] = RETAINED[sc]
        dec["merged"] = {k: f"{dec['merged'][k]} {PROP_MERGED[sc][k]}" for k in ("en", "de")}
        dec["omitted"] = {k: f"{dec['omitted'][k]} {PROP_OMITTED[sc][k]}" for k in ("en", "de")}
        dec["thinElements"] = {k: f"{dec['thinElements'][k]} {THIN[sc][k]}" for k in ("en", "de")}
        dec["styles"] = {k: f"{dec['styles'][k].replace('0.13 mm', '0.18 mm').replace('0,13 mm', '0,18 mm') if sc == '500' else dec['styles'][k]} {STYLE_ADD[k]}" for k in ("en", "de")}
        dec["overlays"] = {k: f"{dec['overlays'][k]} {OVERLAY_ADD[k]}" for k in ("en", "de")}
    for sc, vol in a["volumes"].items():
        ds = vol["decisions"]
        kinds = ["body", "thermal", "structure"] if len(ds) == 3 else ["body", "structure"]
        vol["decisions"] = [{k: f"{fix(d, h)[k]} {VOL_ADD[kind][k]}" for k in ("en", "de")} for d, kind in zip(ds, kinds)]
