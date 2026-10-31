"""🧪 Validate an exact constant-section proof before enabling independent audit fast paths."""
import importlib.util,json,sys,time
from pathlib import Path
from jsonschema import Draft202012Validator
from shapely.geometry import Polygon
from scipy.integrate import quad
sys.dont_write_bytecode=True
ticket=Path(__file__).parent;root=Path.cwd();sys.path.insert(0,str(root))
spec=importlib.util.spec_from_file_location("expanded_interference_r95",ticket/"whole-expanded-interference-r92.py");module=importlib.util.module_from_spec(spec);sys.modules[spec.name]=module;spec.loader.exec_module(module)
fixture=json.loads((ticket/"whole-expanded-prism-cases-r95.json").read_text(encoding="utf8"));schema=json.loads((ticket/"whole-expanded-prism-cases-r95.schema.json").read_text(encoding="utf8"));Draft202012Validator(schema).validate(fixture)
start=time.monotonic();results=[]
for case in fixture["cases"]:
    mesh=case["mesh"];proof=module.constant_prism_proof(mesh["vertices"],mesh["faces"]);assert proof==case["accepted"],(case["id"],proof)
    try:
        solid=module.rawsolid({"outline":case["outline"],"z0":min(v[2]for v in mesh["vertices"]),"z1":max(v[2]for v in mesh["vertices"]),"mesh":mesh})
    except ValueError as failure:
        assert not case["accepted"],(case["id"],str(failure));results.append({"id":case["id"],"accepted":proof,"unsupported":str(failure)});continue
    assert (solid.poly is not None)==case["accepted"],case["id"]
    if case["accepted"]:
        expected=Polygon(case["outline"],case.get("holes",[]));assert solid.poly.symmetric_difference(expected).area==0
        original=module.constant_prism_proof
        try:
            module.constant_prism_proof=lambda *args:False
            reference=module.rawsolid({"outline":case["outline"],"z0":solid.lo,"z1":solid.hi,"mesh":mesh})
        finally:module.constant_prism_proof=original
        volume,error=quad(lambda z:reference.section(z).area,solid.lo,solid.hi,epsabs=1e-13,epsrel=1e-13)
        assert abs(volume-expected.area*(solid.hi-solid.lo))<1e-12,(case["id"],volume)
    results.append({"id":case["id"],"accepted":proof,"constant_section":solid.poly is not None})
out={"pass":True,"cases":results,"seconds":time.monotonic()-start};(ticket/"🗑️generated/whole-expanded-prism-contract-r95.json").write_text(json.dumps(out,indent=2),encoding="utf8");print(out,flush=True)
