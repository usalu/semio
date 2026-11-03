"""🫖 kp: bind the 2 kW element to the water of the electric kettle (1 L fill) - run once: kettle_patch.py <heat_water_w> <boil_level>"""
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
PATH = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json"
data = json.loads(PATH.read_text(encoding="utf8"))
a = data["asset"]
water_w, boil = float(sys.argv[1]), float(sys.argv[2])

a["provenance"]["notes"]["en"] = (
    "A 1.7 L jug kettle with a 2 kW concealed element in a 0.18 m power base, a stainless body of 0.15 m by 0.19 m tapering upwards, a spout at the left, a C-shaped polypropylene handle at the right, a domed lid with a button and a vertical water gauge strip. "
    "Filled with 1 L of water the kettle weighs 2.22 kg (base 0.55, body 0.45, handle 0.12, lid 0.1, water 1.0). Heating 1 kg by 85 K from about 15 degC to 100 degC needs 356 kJ, which takes about 3 minutes at 2 kW; the element heats the water region directly (almost all of the 2 kW), so the boil clip follows the energy balance in real time. "
    "Once the water boils, the heat only evaporates it: the steam emitters show the vapour. The origin is the centre of the base on the surface."
)
a["provenance"]["notes"]["de"] = (
    "Ein 1,7-l-Wasserkocher mit einem verdeckten 2-kW-Heizelement in einem 0,18 m breiten Sockel, einem Edelstahlkörper von 0,15 m mal 0,19 m, der sich nach oben verjüngt, einem Ausguss links, einem C-förmigen Polypropylengriff rechts, einem gewölbten Deckel mit Knopf und einem senkrechten Wasserstandsstreifen. "
    "Mit 1 l Wasser wiegt der Kocher 2,22 kg (Sockel 0,55, Körper 0,45, Griff 0,12, Deckel 0,1, Wasser 1,0). Das Erwärmen von 1 kg um 85 K von etwa 15 °C auf 100 °C braucht 356 kJ, bei 2 kW sind das etwa 3 Minuten; das Heizelement heizt die Wasserregion direkt (fast die vollen 2 kW), der Aufkoch-Clip folgt also in Echtzeit der Energiebilanz. "
    "Sobald das Wasser kocht, verdampft die Wärme es nur noch: die Dampfemitter zeigen den Dampf. Der Ursprung ist die Mitte des Sockels auf der Unterlage."
)
a["dimensions"]["massKg"] = 2.22
for bone in a["rig"]["bones"]:
    if bone["id"] == "water":
        bone["mass"]["kg"] = 1.0
        bone["mass"]["comY"] = 0.04

th = a["thermal"]
heat = {"plate": 40.0, "shell": 15.0, "lid": 5.0, "water": water_w}
for r in th["regions"]:
    if r["id"] in heat:
        r["operating"]["heatW"] = heat[r["id"]]
    if r["id"] == "water":
        r["heatCapacityJM2K"] = round(1.0 * 4186 / r["areaM2"], -2)
        r["title"] = {"en": "Water fill and gauge", "de": "Wasserfüllung und Wasserstand"}
        r["layers"][0]["title"] = r["title"]
        r["source"] = "authored-estimate"
        r["layers"][0]["source"] = "authored-estimate"
        r["layers"][0]["heatCapacityJM2K"] = r["heatCapacityJM2K"]

th["assumptions"] = [{
    "en": f"The 2 kW element heats the water region directly ({water_w:.0f} W; plate 40, shell 15, lid 5): 1 kg of water (1 L, 4186 J/(kg K)) rises by 85 K in about 3 minutes (356 kJ). The region heat capacity is the water's per gauge area. At the boil the element level drops to the evaporation: the heat leaves as latent heat with the steam emitters.",
    "de": f"Das 2-kW-Heizelement heizt die Wasserregion direkt ({water_w:.0f} W; Platte 40, Hülle 15, Deckel 5): 1 kg Wasser (1 l, 4186 J/(kg K)) steigt in etwa 3 Minuten um 85 K (356 kJ). Die Wärmekapazität der Region ist die des Wassers je Wasserstandsfläche. Beim Sieden sinkt der Elementpegel auf die Verdunstung: die Wärme verlässt den Kocher als latente Wärme über die Dampfemitter.",
}]
a["structure"]["assumptions"] = [{
    "en": "The 2.2 kg kettle stands on its base: a unilateral contact of 0.17 m diameter carries the weight.",
    "de": "Der 2,2 kg schwere Wasserkocher steht auf seinem Sockel: ein einseitiger Kontakt von 0,17 m Durchmesser trägt das Gewicht.",
}]
for lv in th["operating"][0]["levels"]:
    if lv["v"] == 1:
        lv["title"] = {"en": "Heating at 2 kW", "de": "Heizt mit 2 kW"}
    elif lv["v"] == 0.6:
        lv["v"] = boil
        lv["title"] = {"en": "Boiling (evaporation)", "de": "Sieden (Verdunstung)"}
th["operating"][0]["levels"].sort(key=lambda lv: lv["v"])
PATH.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
print("ok")
