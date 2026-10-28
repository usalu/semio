from pathlib import Path
j = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🔣️.json")
t = j.read_text(encoding="utf-8")
a = t.index('    "note": {"en": "Sun path on the solstices'); b = t.index("\n", a)
t = t[:a] + '''    "note": {"en": "Sun path on the solstices at 47.4° latitude: noon altitudes, typical low summer sun in the morning and evening, and the compass bearings between which the sun is above 3° (180° is south). The overhang is judged over the hourly sun positions that shine on the façade (hours: noon ± 4 h), each weighted by the beam it delivers.", "de": "Sonnenbahn zu den Sonnenwenden auf 47,4° Breite: Mittagshöhen, typische tiefe Sommersonne morgens und abends und die Kompasswinkel, zwischen denen die Sonne über 3° steht (180° ist Süd). Das Vordach wird über die stündlichen Sonnenstände beurteilt, die auf die Fassade scheinen (Stunden: Mittag ± 4 h), jeweils gewichtet mit der Strahlung, die sie liefern."}''' + t[b:]
t = t.replace("Overhang depth, in window heights", "Overhang depth, in window heights")
j.write_text(t, encoding="utf-8", newline="\n")
