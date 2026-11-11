"""🧪 Verify capture writer preserves exact data even if optional metadata cannot encode."""
import importlib,json,pickle,sys,tempfile
from fractions import Fraction
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
from bim_projection_io import write
cache=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
folder=Path(__file__).parent/"🗑️generated"/"projection-writer-test"
folder.mkdir(parents=True,exist_ok=True)
facets=[{"rings":[[(Fraction(1,3),1.25,-2.0),(Fraction(7,11),0.0,4.0),(Fraction(2,3),2.0,0.0)]],"material":{"id":"x"}}]
matrix=((1.0,Fraction(1,7)),(-0.5,1.0),(0.0,2.0))
expected={"facets":facets,"matrix":matrix}
def native(x):return x
write(folder,facets,matrix,{"tag":"A"},cache,native)
with (folder/"bim-projection-current.pkl").open("rb") as stream:assert pickle.load(stream)==expected
record=json.loads((folder/"bim-projection-failure.json").read_text(encoding="utf8"))
assert cache.decode(record["facets"])==facets and cache.decode(record["matrix"])==matrix
(folder/"bim-projection-failure.json").unlink()
write(folder,facets,matrix,{"item":{"unsupported":frozenset({1,2})}},cache,native)
with (folder/"bim-projection-current.pkl").open("rb") as stream:assert pickle.load(stream)==expected
assert not (folder/"bim-projection-failure.json").exists()
print("[DEBUG] exact capture writer JSON and metadata-failure fallback passed",flush=True)
