"""Scratch helper for batch E2b: write a clip's canonical transition document from compact entries (canonical text comes from `spec.dumps`)."""

import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")

DEMAND = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip"
SCHEMA = "../../../../../🔀️morph/🧬️schema/🔣️.schema.json"


def entry(id, intent, source, target, duration, *, relation="one-to-one", recipe="replace", correspondence="order", easing="smooth", copy=False, lag=0.0, limit=None, **final):
    out = {
        "id": id,
        "intent": intent,
        "source": None if source is None else {"entity": source},
        "target": None if target is None else {"entity": target},
        "relation": relation,
        "recipe": recipe,
        "timing": {"duration": duration, "easing": easing, **({"lag": lag} if lag else {}), **({"limit": limit} if limit else {})},
    }
    if correspondence is not None:
        out["correspondence"] = {"kind": correspondence}
    if copy:
        out["copy"] = True
    if final:
        out["final"] = final
    return out


def write(clip, entries):
    folder = DEMAND / clip / "🔀️transitions"
    folder.mkdir(exist_ok=True)
    text = spec.dumps({"$schema": SCHEMA, "version": 1, "transitions": entries})
    (folder / "🔣️.json").write_text(text, encoding="utf-8", newline="\n")
    print(clip.encode("ascii", "backslashreplace").decode(), len(entries), "transitions")
