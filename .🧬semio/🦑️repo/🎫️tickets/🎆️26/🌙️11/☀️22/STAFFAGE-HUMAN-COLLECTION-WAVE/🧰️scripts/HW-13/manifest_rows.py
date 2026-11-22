"""📥️ Writes the manifest rows of the HW-13 humans to T/📥️manifest-rows/<id>.json in the shape of the catalog's `entries`, every value read from the evaluated asset (power from `totalPowerW`)."""
import json
import os

from lib import HUMANS, T, bun, t

SLUGS = ["person-sweeping", "person-hanging-laundry", "person-carrying-shopping-bags"]
DISTINCTION = {
    "person-sweeping": t("A 1.69 m adult in fleece sweatshirt, jeans and sneakers sweeping a floor with a 400 mm push broom at 3.3 met: stroke start, push, stroke end, the lifted return and a rest with the broom upright; the bristles stand on the floor as a contact and the broom head swivels flat.",
                         "Eine erwachsene Person von 1,69 m in Fleece-Sweatshirt, Jeans und Sneakern, die bei 3,3 met mit einem 400 mm breiten Schiebebesen einen Boden kehrt: Strichbeginn, Schieben, Strichende, das angehobene Zurückführen und eine Pause mit aufrechtem Besen; die Borsten stehen als Kontakt auf dem Boden, der Besenkopf liegt im Gelenk flach auf."),
    "person-hanging-laundry": t("A 1.75 m man in softshell jacket and cap hanging a wet T-shirt on a clothesline at 1.80 m with two wooden pegs, the laundry basket at his feet: lifting from the basket, shaking out and pegging, 2.3 to 2.8 met, in spring sun and breeze.",
                                "Ein Mann von 1,75 m in Softshelljacke und Mütze, der ein nasses T-Shirt mit zwei Holzklammern an eine Wäscheleine in 1,80 m Höhe hängt, der Wäschekorb zu seinen Füßen: Herausheben aus dem Korb, Ausschütteln und Anklammern, 2,3 bis 2,8 met, in Frühlingssonne und Wind."),
    "person-carrying-shopping-bags": t("A 1.78 m man in overcoat and scarf walking with a full 4.5 kg paper shopping bag in each fist (baguette and leek showing): straight loaded arms with little swing, bags clear of the legs with a slight pendulum lag, 3.5 met at 4 km/h.",
                                       "Ein Mann von 1,78 m in Mantel und Schal, der mit je einer vollen Papiertüte von 4,5 kg in jeder Faust geht (Baguette und Lauch schauen heraus): gestreckte belastete Arme mit wenig Schwung, Tüten frei neben den Beinen mit leichtem Pendelnachlauf, 3,5 met bei 4 km/h."),
}


def main():
    power = json.loads(bun("power.ts", *SLUGS))
    os.makedirs(f"{T}/📥️manifest-rows", exist_ok=True)
    for slug in SLUGS:
        a = json.load(open(f"{HUMANS}/{slug}/🔣️.json", encoding="utf-8"))["asset"]
        row = {"id": a["id"], "category": a["category"], "title": a["title"], "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"], "powerW": round(power[slug], 1),
               "poses": [p["id"] for p in a["poses"] if not p.get("key")], "clips": [c["id"] for c in a["clips"]], "representations": {k: a["representations"][k]["status"] for k in ("50", "100", "200", "500")},
               "symbols": [s["id"] for s in a.get("symbols", [])], "analytics": ["thermal", "transport", "structural"], "exports": a["exports"], "distinction": DISTINCTION[slug], "completion": a["completion"]}
        with open(f"{T}/📥️manifest-rows/{slug}.json", "w", encoding="utf-8") as f:
            json.dump(row, f, ensure_ascii=False, indent=1)
            f.write("\n")
        print(slug, row["powerW"], row["heightM"], row["widthM"])


if __name__ == "__main__":
    main()
