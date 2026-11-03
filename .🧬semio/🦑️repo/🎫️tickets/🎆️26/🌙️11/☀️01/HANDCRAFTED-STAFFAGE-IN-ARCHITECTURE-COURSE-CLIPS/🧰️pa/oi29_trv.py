"""🚰 pa: OI-29 - the valve body is heated once, by the 3 W of the standing water; the 55 degrees C pipe contact of the poses is removed (it was a second source and ignored the supply level) - run once."""
from jio import load, save

path, data = load("thermostatic-radiator-valve")
a = data["asset"]
for pose in a["poses"]:
    body = pose.get("thermal", {}).get("regions", {}).get("body")
    if body:
        body.pop("contact", None)

extra = {
    "en": "The body is heated once: by the 3 W of the standing water, which stand for the whole path from the pipe through the brass wall; the pipe stub is not a second thermal contact, so no pipe temperature is bound in the poses and a closed supply cools the body with the supply level.",
    "de": "Der Körper wird nur einmal beheizt: durch die 3 W des stehenden Wassers, die für den ganzen Weg vom Rohr durch die Messingwand stehen; der Rohranschluss ist kein zweiter Wärmekontakt, daher ist in den Posen keine Rohrtemperatur gebunden und eine geschlossene Zufuhr kühlt den Körper mit der Zufuhrstufe ab.",
}
a["thermal"]["assumptions"].insert(1, extra)
save(path, data)
print("ok")
