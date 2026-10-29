"""📤 Generate native exchange test samples into this ticket's disposable output folder."""
import importlib
import json
from pathlib import Path
import sys

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
core=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
CASES=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/🧪️tests/🪟️fillings.json"
fixture=json.loads(CASES.read_text(encoding="utf-8"))
samples=[]
for case in fixture["cases"]:
    for language in fixture["languages"]:
        result=core.step_excerpt(case["tag"],language=language)
        physical=core.filling_geometry(case["tag"])
        vertices=[[*p,z] for part in physical["parts"] for p in part["outline"] for z in (part["z0"],part["z1"])]
        bounds=[[fn(p[k] for p in vertices) for k in range(3)] for fn in (min,max)]
        samples.append({"bounds":bounds,"tag":case["tag"],"name":core.element(case["tag"])["name"][language],"language":language,"entity":case["entity"],"type":case["type"],"ids":result["ids"],"text":"\n".join(result["lines"]),"quantities":core.quantities(case["tag"])})
output=Path(__file__).parent/"🗑️generated/native-step-r18.json"
output.parent.mkdir(exist_ok=True)
output.write_text(json.dumps(samples,ensure_ascii=False),encoding="utf-8")
                                                                                                                                                                                                                                            