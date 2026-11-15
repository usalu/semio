import json, pathlib
p = pathlib.Path(r"C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🧪️tests/🔣️.cases.json")
d = json.loads(p.read_text(encoding="utf-8"))
print(json.dumps(d, ensure_ascii=False)[:3000])
