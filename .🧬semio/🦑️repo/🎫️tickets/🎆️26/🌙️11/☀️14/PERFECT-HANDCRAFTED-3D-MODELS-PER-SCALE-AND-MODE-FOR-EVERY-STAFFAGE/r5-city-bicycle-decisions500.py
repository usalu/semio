"""🚲 Round 5 vehicles/city-bicycle 1:500: states the bilingual decisions of the 3D-only scale (its 2D representation is unavailable below the printable feature) for body, thermal and structure in the volumes source `volumes/vehicles/city-bicycle.json`; the solids are left as they are.

Usage: python3 r5-city-bicycle-decisions500.py"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(T, "volumes", "vehicles", "city-bicycle.json")
DECISIONS = [
    {"en": "1:500 is a 3D-only scale: the 2D representation is unavailable because the tubes fall below the printable feature, so the body keeps the real tube widths and is a clean massing of the 1:200 parts. Each wheel is one ring of tyre and rim with a hub drum and four spoke directions that tie the hub to the ring; the frame is seat tube, down tube, the swept loop tube, head tube and one bar per stay; the carrier is one hoop on one leg per side; the saddle one pad; the chain case a closed slab; cranks one bar each with a pedal block; mudguards folded sheets over both tyres; the fork a crown and two blades; quill and stem one bar; each handlebar half one swept tube to its grip; the head lamp one housing and the tail lamp one block; the stand two legs.", "de": "1:500 ist ein reiner 3D-Maßstab: die 2D-Darstellung ist nicht verfügbar, weil die Rohre unter dem druckbaren Merkmal liegen; der Körper behält darum die wirklichen Rohrbreiten und ist eine klare Massenform der 1:200-Teile. Jedes Rad ist ein Ring aus Reifen und Felge mit Nabentrommel und vier Speichenrichtungen, die Nabe und Ring verbinden; der Rahmen besteht aus Sattelrohr, Unterrohr, dem geschwungenen Bügelrohr, Steuerrohr und je einem Stab je Strebe; der Gepäckträger ist ein Bügel auf einem Bein je Seite; der Sattel ein Polster; der Kettenkasten eine geschlossene Platte; jede Kurbel ein Stab mit Pedalblock; die Schutzbleche gekantete Bleche über beiden Reifen; die Gabel Krone und zwei Scheiden; Schaft und Vorbau ein Stab; jede Lenkerhälfte ein geschwungenes Rohr bis zum Griff; der Scheinwerfer ein Gehäuse und das Rücklicht ein Block; der Ständer zwei Beine."},
    {"en": "Thermal: every region by reference to its body solids; the rim is its own ring inside the tyre ring and the crank axle belongs to the frame tubes. Only the sum of the areas is compared at this scale.", "de": "Thermik: jede Region per Verweis auf ihre Körper; die Felge ist ein eigener Ring im Reifenring, die Kurbelachse gehört zu den Rahmenrohren. In diesem Maßstab wird nur die Flächensumme verglichen."},
    {"en": "Structure: the frame tubes and stays, carrier hoop and legs, steerer with its bearing, crown, blades, stem and the two swept handlebar bars, wheel ring and four spoke directions per wheel with the hub bearing, crank axle with its bearing, crank bars and pedal blocks, stand legs with the hinge; the front tyre patch is the support; masses of frame, stays, steer assembly and wheels.", "de": "Tragwerk: die Rahmenrohre und Streben, Gepäckträgerbügel und -beine, Gabelschaft mit Lager, Krone, Scheiden, Vorbau und die beiden geschwungenen Lenkerstäbe, Radring und vier Speichenrichtungen je Rad mit dem Nabenlager, Kurbelachse mit Lager, Kurbelstäbe und Pedalblöcke, Ständerbeine mit dem Gelenk; die vordere Reifenaufstandsfläche ist das Auflager; Massen von Rahmen, Streben, Lenkeinheit und Rädern."},
]

if __name__ == "__main__":
    source = json.load(open(SRC, encoding="utf8"))
    source["volumes"]["500"]["decisions"] = DECISIONS
    with open(SRC, "w", encoding="utf8") as fh:
        json.dump(source, fh, ensure_ascii=False)
        fh.write("\n")
    print("1:500 decisions", len(DECISIONS))
