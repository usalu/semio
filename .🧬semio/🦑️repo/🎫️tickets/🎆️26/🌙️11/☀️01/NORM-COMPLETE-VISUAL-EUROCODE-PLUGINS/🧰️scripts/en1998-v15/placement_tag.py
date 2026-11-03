"""Rewrites the six placement mutation fixtures to the internally tagged wire form (`"mutation": "<camelCase>"`) of the aggregate, and adds the `mutation` const property to their leaf schemas. python -I placement_tag.py"""
import glob, json
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from validate import ANY, FX
def dump(d): return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
for d in glob.glob(FX + "🧬️mutations/📍️*/✅apply"):
    p = d + "/🔣️.mutation.json"; m = json.load(open(p, encoding="utf-8"))
    if "mutation" in m: continue
    (tag, body), = m.items(); open(p, "w", encoding="utf-8").write(dump({"mutation": tag[0].lower() + tag[1:], **body}))
for f in glob.glob(ANY + "🧬️schema/🧬️mutations/📍️*/🧬️schema/🔣️.json"):
    s = json.load(open(f, encoding="utf-8"))
    if "mutation" in s["properties"]: continue
    tag = s["title"][0].lower() + s["title"][1:]
    s["required"] = ["mutation"] + s["required"]; s["properties"] = {"mutation": {"const": tag}, **s["properties"]}
    open(f, "w", encoding="utf-8").write(dump(s))
    print("schema", f.split("mutations/")[1])
