"""🏗️ Second-round structure rework of vehicles/delivery-van (source `volumes/vehicles/delivery-van.json`): wing rails that tie the strut towers back to the hinge pillars at 1:100, MacPherson struts under strut towers at 1:500, and the mass markers of body and axles at the coarse scales. Reference: `rework/assets/vehicles--delivery-van.md`."""
import json

PATH = "volumes/vehicles/delivery-van.json"
document = json.load(open(PATH, encoding="utf8"))
V = document["volumes"]
BIW = {"material": "paint", "role": "detail", "tone": 0.5, "member": "chassis", "part": "member"}
IRON = {"material": "cast-iron", "role": "detail", "tone": 0.5}


def put(scale, solid):
    rows = V[scale]["structure"]
    V[scale]["structure"] = [row for row in rows if row["id"] != solid["id"]] + [solid]


def decide(scale, en, de):
    rows = [row for row in V[scale]["decisions"] if not row["en"].startswith(en[:40])]
    V[scale]["decisions"] = rows + [{"en": en, "de": de}]


for side, sign in (("l", 1), ("r", -1)):
    put("100", {"id": f"upper-rail-{side}", "kind": "capsule", "bone": "chassis", **BIW, "p0": [1.25, 1.05, sign * 0.92], "p1": [1.66, 1.0, sign * 0.7], "r0": 0.03, "r1": 0.03})
    put("500", {"id": f"strut-tower-{side}", "kind": "capsule", "bone": "chassis", **BIW, "p0": [1.66, 0.46, sign * 0.5], "p1": [1.64, 0.98, sign * 0.64], "r0": 0.055, "r1": 0.055})
    put("500", {"id": f"axle-f-strut-{side}", "kind": "capsule", "bone": "axle-f", **IRON, "member": "axle-f", "part": "member", "p0": [0, 0, sign * 0.72], "p1": [-0.06, 0.6, sign * 0.64], "r0": 0.0375, "r1": 0.0375})
for scale in ("200", "500"):
    put(scale, {"id": "axle-f-mass", "kind": "ellipsoid", "bone": "axle-f", **IRON, "radii": [0.07, 0.07, 0.07], "transform": {"translation": [0, 0, 0]}, "member": "axle-f", "part": "mass"})
    put(scale, {"id": "axle-r-mass", "kind": "ellipsoid", "bone": "axle-r", **IRON, "radii": [0.07, 0.07, 0.07], "transform": {"translation": [0, -0.01, 0]}, "member": "axle-r", "part": "mass"})
put("500", {"id": "com-chassis", "kind": "ellipsoid", "bone": "chassis", **IRON, "radii": [0.15, 0.15, 0.15], "transform": {"translation": [0.48, 0.85, 0]}, "member": "chassis", "part": "mass"})

decide("100", "Structure, second round: a wing rail on each side ties the head of the strut tower back to the hinge pillar, so the spring force of the front wheel enters the body shell at two points as in the real unibody.", "Tragwerk, zweite Runde: Je ein Kotflügelträger bindet den Kopf des Federbeindoms an die Scharniersäule, sodass die Federkraft des Vorderrads wie in der echten selbsttragenden Karosserie an zwei Punkten in die Schale geht.")
decide("200", "Structure, second round: the masses of front and rear axle (210 kg and 190 kg) are marked at their axle centres beside the body mass.", "Tragwerk, zweite Runde: Die Massen von Vorder- und Hinterachse (210 kg und 190 kg) sind neben der Aufbaumasse in ihren Achsmitten markiert.")
decide("500", "Structure, second round: the front wheels are sprung by MacPherson struts of 75 mm that bear on strut towers standing on the chassis rails, and the masses of body (1366 kg) and both axles are marked, so the load path from body to tyre is complete at this scale too.", "Tragwerk, zweite Runde: Die Vorderräder sind über 75 mm starke MacPherson-Federbeine gefedert, die sich an Federbeindomen auf den Längsträgern abstützen, und die Massen von Aufbau (1366 kg) und beiden Achsen sind markiert, sodass der Lastpfad vom Aufbau zum Reifen auch in diesem Maßstab vollständig ist.")
json.dump(document, open(PATH, "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("written", {scale: len(V[scale]["structure"]) for scale in V})
