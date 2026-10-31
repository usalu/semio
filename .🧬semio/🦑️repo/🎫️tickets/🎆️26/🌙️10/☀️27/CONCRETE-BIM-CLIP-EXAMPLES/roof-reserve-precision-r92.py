"""📏 Compare exact roof adjacent-cell witness with independent GEOS."""
import sys,importlib,json
from pathlib import Path
from shapely.geometry import Polygon
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");c=json.loads(Path(__file__).with_suffix(".json").read_text());print("[DEBUG] "+json.dumps({"geos":Polygon(c["a"]).intersection(Polygon(c["b"])).area,"native9":a._area(a._analysis._clip(c["a"],c["b"])),"native12":a._area(a._analysis._clip(c["a"],c["b"],digits=12))}),flush=True)
