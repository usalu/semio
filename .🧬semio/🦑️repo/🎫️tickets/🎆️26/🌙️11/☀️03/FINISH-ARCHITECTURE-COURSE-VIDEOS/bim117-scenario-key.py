"""🗝️ Compare expected scene scenario slab identities without computing geometry."""
import ast,copy,importlib,sys
from pathlib import Path
from types import SimpleNamespace
import numpy as np
sys.path.insert(0,str(Path.cwd()))
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
c=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
source=Path("🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models/🐍️.py")
tree=ast.parse(source.read_text(encoding="utf8"))
owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_prepare")
statements=[n for n in method.body if 146<=n.lineno<=163]
model=a.load_model()
column=next(e for e in model["elements"] if e["tag"]==a.role("column_example"))
profile=next(t for t in model["types"] if t["tag"]==column["type"])["profile"]
target={"tag":column["tag"],"x":column["geometry"]["axis"][0],"y":column["geometry"]["axis"][1],"radius":profile["diameter"]["value"]/2}
scene=SimpleNamespace(model=model,geometry={"pilotis":[target]})
env={"self":scene,"copy":copy,"np":np,"_model":a}
exec(compile(ast.Module(body=statements,type_ignores=[]),str(source),"exec"),env)
root,sources,slots,eligible=c.canonical_context(a.geometry_producers())
for slab in scene.scenario["elements"]:
    if slab["tag"] in ("SL-1","SL-R"):
        print("[DEBUG]",slab["tag"],c.identity(scene.scenario,slab,root,sources,slots),flush=True)
