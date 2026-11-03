"""🧪 bp: build a variants manifest: mkvar.py <out.json> id[:key=json,...] ... ; ids are manifest casts, `+nt` removes transport (named id-nt)."""
import json, sys
from pathlib import Path
root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
m = json.load(open(root / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json", encoding="utf8"))["casts"]
out = {}
for spec in sys.argv[2:]:
    name, _, rest = spec.partition(":")
    cast = dict(m[name])
    new = name
    for kv in filter(None, rest.split(",")):
        k, _, v = kv.partition("=")
        if k == "id": new = v
        elif k == "notransport": cast.pop("transport", None)
        else: cast[k] = json.loads(v)
    out[new] = cast
json.dump({"version": 1, "casts": out}, open(sys.argv[1], "w", encoding="utf8"), ensure_ascii=False, indent=1)
