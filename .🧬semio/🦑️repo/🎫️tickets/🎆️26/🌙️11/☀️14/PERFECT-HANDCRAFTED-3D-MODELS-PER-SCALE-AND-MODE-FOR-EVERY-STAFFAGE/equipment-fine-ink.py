"""🖊️ Author individual paper marks at the operating limits of the printed bands."""
from pathlib import Path
import json
import os

def apply(asset):
    edits={
        "plate-heat-exchanger":{"50":{"damper-hinge"}},
        "curtain-wall-bay":{"200":{"glass"}},
        "sliding-door":{"100":{"leaf-glass"},"200":{"leaf-glass"}},
        "solar-thermal-collector":{"200":{"front-top-rail","front-side-left","front-side-right","plan-glass"}},
    }[asset["id"]]
    for band,ids in edits.items():
        for shapes in asset["representations"][band]["geometry"].values():
            for shape in shapes:
                if shape["id"] in ids:shape["penMm"]=.20
    if asset["id"]=="sliding-door":
        for shape in asset["representations"]["50"]["geometry"]["plan"]:
            if shape["id"] in ("handle-base-cut","handle-grip-cut"):shape["x"]={"handle-base-cut":1.883,"handle-grip-cut":1.890}[shape["id"]]

if __name__=="__main__":
    t=Path(__file__).resolve().parent;c=next((t.parents[6]/"🎓️teaching").glob("*architecture/*staffage/*catalog"))
    for name in ("curtain-wall-bay","plate-heat-exchanger","sliding-door","solar-thermal-collector"):
        path=next(c.glob("*/"+name+"/*.json"));raw=path.read_bytes();doc=json.loads(raw);apply(doc["asset"])
        indent=next((i for i in (1,2,4) if raw.startswith(b'{\n'+b' '*i+b'"')),None)
        tmp=path.with_suffix(".partial");tmp.write_text(json.dumps(doc,ensure_ascii=False,indent=indent)+('\n' if raw.endswith(b'\n') else ''));assert path.read_bytes()==raw;os.replace(tmp,path)
