"""😴 pa: OI-35 - sleeping is 0.7 met (ISO 8996, ASHRAE 55): poses, base metabolism, the turn-over track and the texts - run once."""
from jio import load, save


def walk(node, visit):
    if isinstance(node, dict):
        visit(node)
        for value in node.values():
            walk(value, visit)
    elif isinstance(node, list):
        for value in node:
            walk(value, visit)


path, data = load("sleeping-person")
a = data["asset"]
count = {"met": 0}


def fix(node):
    if node.get("activityMet") == 0.8:
        node["activityMet"] = 0.7
        count["met"] += 1
    if node.get("baseMet") == 0.8:
        node["baseMet"] = 0.7
        count["met"] += 1
    if node.get("target") == "activityMet":
        for key in node["keys"]:
            if key["v"] == 0.8:
                key["v"] = 0.7
                count["met"] += 1


walk(a, fix)

pairs = {
    "en": [
        ("at 0.8 met and made to lie", "at 0.7 met and made to lie"),
        ("Sleeping is 0.8 met (ISO 8996 gives 0.7 met for sleeping, 0.8 is used for the light sleep of the energy clips); the turning clip raises it to 0.95 met for a few seconds.", "Sleeping is 0.7 met (ISO 8996 and ASHRAE 55 for sleeping; 66 W basal metabolism over 1.742 m2 is 0.65 met); the turning clip raises it to 0.95 met for a few seconds."),
    ],
    "de": [
        ("bei 0,8 met und gemacht", "bei 0,7 met und gemacht"),
        ("Schlafen sind 0,8 met (ISO 8996 nennt 0,7 met für Schlafen, 0,8 gilt für den leichten Schlaf der Energieclips); der Dreh-Clip hebt den Wert für einige Sekunden auf 0,95 met.", "Schlafen sind 0,7 met (ISO 8996 und ASHRAE 55 für Schlafen; 66 W Grundumsatz über 1,742 m2 sind 0,65 met); der Dreh-Clip hebt den Wert für einige Sekunden auf 0,95 met."),
    ],
}


def patch(holder):
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in holder[lang]:
                holder[lang] = holder[lang].replace(old, new)


patch(a["summary"])
for item in a["thermal"]["assumptions"]:
    patch(item)
save(path, data)
print("ok", count)
