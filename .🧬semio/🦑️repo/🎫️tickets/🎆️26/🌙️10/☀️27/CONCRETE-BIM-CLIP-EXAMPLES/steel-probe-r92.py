"""🔩 Inspect actual open-section prism rings before native clipping."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
from shapely.geometry import Polygon
core=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
source=core.load_model();spec=core._find(source,"TY-CURTAIN-METAL")["filling"]
spec["frame_width"]["value"]=spec["frame_depth"]["value"]=.035
spec["member_sections"]={kind:{"shape":shape,"thickness":{"value":.004}} for kind,shape in [("perimeter","angle"),("interior","tee")]}
core._subtract_prisms=lambda parts,cutters:parts
geometry=core.filling_geometry("CW-1",source)
for index,p in enumerate(geometry["parts"]):
    ring=Polygon(p["outline"])
    try:
        importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")._convex_parts(p["outline"])
    except Exception as error:
        print("[DEBUG] "+json.dumps({"index":index,"part":p,"area":ring.area,"valid":ring.is_valid,"error":str(error)}),flush=True)
