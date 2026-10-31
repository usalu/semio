"""🔬 Compare first-party analytic solid measures with independent GEOS/SciPy true-curve intervals."""
import importlib,importlib.util,json,math,sys,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));B=Path(__file__).parent;start=time.monotonic()
M=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🐍️")
spec=importlib.util.spec_from_file_location("independent_csg_oracle",B/"analytic-difference-oracle-r95.py");O=importlib.util.module_from_spec(spec);spec.loader.exec_module(O)
case=json.loads((Path(M.__file__).parent/"🧪️tests/🔣️.json").read_text(encoding="utf-8"));rows=[]
for row in case["cases"]:
    own=M.measure(row["part"]);other,error=O.volume(row["part"]["csg"]);delta=own["volume"]-other
    positive=not row["expected"].get("owned_volume_positive")or(own["volume"]>0 and other>0)
    equal=abs(delta)<=max(1e-11,8*(own["absolute_error"]+error))
    witnesses=[O.intersection_volume(row["part"]["csg"],cut["operand"])[0]for cut in row["part"]["csg"]["cutters"]]
    result={"id":row["id"],"first_party":own,"independent_volume":other,"independent_error_estimate":error,"delta":delta,"both_retain_required_positive_material":positive,"agree":equal,"independent_owned_cutter_intersections":witnesses};rows.append(result);print("[DEBUG] "+json.dumps(result),flush=True)
report={"success":all(r["agree"]and r["both_retain_required_positive_material"]and not any(r["independent_owned_cutter_intersections"])for r in rows),"rows":rows,"seconds":time.monotonic()-start,"scope":"Independent analytic interval/GEOS/SciPy validation of neutral semantic solids; numerical errors are estimates, not certified bounds"};(B/"🗑️generated/native-r92/analytic-difference-crosscheck-r95.json").write_text(json.dumps(report,indent=2),encoding="utf-8");raise SystemExit(not report["success"])
