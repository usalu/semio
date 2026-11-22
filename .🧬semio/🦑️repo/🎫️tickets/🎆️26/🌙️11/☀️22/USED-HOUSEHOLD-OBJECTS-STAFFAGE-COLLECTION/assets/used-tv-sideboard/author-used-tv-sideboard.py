"""📺 Writes the asset document `🔣️.json` of the used TV sideboard (chipboard lowboard 1.80 x 0.40 x 0.45 m with two drop-down flap doors, a drawer, an open niche with cable hole and scuffed white foil) from the hand-chosen numbers of `author-geometry.py`.

Run: `python3 author-used-tv-sideboard.py` in this folder.

@see ../../brief.md — the author brief of the used household objects wave
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — the typed contract
"""
import importlib.util
import json
import math
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location("geometry", HERE / "author-geometry.py")
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)
exec(compile((HERE / "author-texts.py").read_text(encoding="utf-8"), str(HERE / "author-texts.py"), "exec"))
exec(compile((HERE / "author-model.py").read_text(encoding="utf-8"), str(HERE / "author-model.py"), "exec"))

doc = build()
(HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(f"[DEBUG] wrote {HERE / '🔣️.json'}: mass {doc['asset']['dimensions']['massKg']} kg")
