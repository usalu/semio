"""🗒️ Joins the list of provenance notes of an asset document into the one Text the schema asks for, rewriting only that span of the file: python3 notes-one-text.py <asset json>…"""
import json
import sys

sys.path.insert(0, __file__.rsplit("/", 2)[0] + "/🧰️three-views")
from tv2_doc import Doc

for path in sys.argv[1:]:
    doc = Doc(path)
    notes = doc.get("asset", "provenance", "notes")
    if not isinstance(notes, list):
        print(path, "already one Text")
        continue
    start, end = doc.spans[("asset", "provenance", "notes")]
    line = doc.text.rfind("\n", 0, start) + 1
    pad = " " * (len(doc.text[line:start]) - len(doc.text[line:start].lstrip(" ")))
    one = {language: " ".join(note[language] for note in notes) for language in ("en", "de")}
    doc.text = doc.text[:start] + "{\n" + ",\n".join(f"{pad} {json.dumps(k, ensure_ascii=False)}: {json.dumps(v, ensure_ascii=False)}" for k, v in one.items()) + "\n" + pad + "}" + doc.text[end:]
    doc._scan()
    doc.save()
    print(path, "joined", len(notes), "notes")
