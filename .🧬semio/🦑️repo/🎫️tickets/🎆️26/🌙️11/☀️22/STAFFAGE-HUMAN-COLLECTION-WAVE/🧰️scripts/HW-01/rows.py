"""📥️ Writes the manifest rows of HW-01's three assets to T/📥️manifest-rows/<id>.json (values from the evaluated catalog assets)."""
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from hw import manifest_row, t  # noqa: E402

DISTINCTION = {
    "standing-desk-worker": t("A tall woman at a dual-motor sit-stand desk raised to standing height, typing on an open 15.6-inch laptop with pronated forearms and curled fingers on the home row; the desk stands on its own contact, the laptop (65 W) and the desk drives (200 W) carry nameplates.", "Eine große Frau an einem auf Stehhöhe hochgefahrenen Sitz-Steh-Tisch mit zwei Motoren, die mit einwärts gedrehten Unterarmen und gekrümmten Fingern auf der Grundreihe auf einem offenen 15,6-Zoll-Laptop tippt; der Tisch steht auf eigenem Kontakt, Laptop (65 W) und Tischantriebe (200 W) tragen Typenschilder."),
    "presenter-at-flipchart": t("A workshop presenter in softshell jacket, cap and lanyard badge beside a tripod flipchart, the marker raised to the paper with the arm abducted toward the board and the free hand open to the group; loops for explaining with speech breathing and for writing a line, the easel on its own contact.", "Eine Person leitet in Softshelljacke, Kappe und mit Ausweis am Band einen Workshop neben einem Dreibein-Flipchart, den Marker mit seitlich zur Tafel gehobenem Arm am Papier, die freie Hand offen zur Gruppe; Schleifen für Erklären mit Sprechatmung und für das Schreiben einer Zeile, das Gestell auf eigenem Kontakt."),
    "office-worker-carrying-laptop": t("An office worker in overcoat and scarf walking heel to toe with a closed laptop standing on the level near forearm against the hip, the far arm swinging; the laptop is a 1.9 kg point mass on the forearm, its 65 W adapter counts in the total power.", "Eine büroangestellte Person in Mantel und Schal im Fersen-Zehen-Gang mit einem zugeklappten Laptop, der auf dem waagerechten nahen Unterarm an der Hüfte steht, der ferne Arm schwingt; der Laptop ist eine Punktmasse von 1,9 kg am Unterarm, sein 65-W-Netzteil zählt zur Gesamtleistung."),
}

for slug, text in DISTINCTION.items():
    row = manifest_row(slug, text)
    print(slug, row["heightM"], row["widthM"], row["powerW"], row["poses"], row["clips"])
