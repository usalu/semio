import json, sys
path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["errors"]["#SPILL!"] = {"en": "#SPILL!", "de": "#ÜBERLAUF!", "tip": {"en": "The result needs more room than the cells below allow.", "de": "Das Ergebnis braucht mehr Platz, als die Zellen darunter bieten."}}
open(path, "w", encoding="utf-8").write(json.dumps(data, ensure_ascii=False, indent=2) + "\n")
