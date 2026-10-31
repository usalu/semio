"""🤝 Publish one proven material interface without changing any physical source dimensions or peer objects."""
import copy,hashlib,json,sys
from pathlib import Path
from jsonschema import Draft202012Validator
sys.dont_write_bytecode=True;root=Path.cwd();ticket=Path(__file__).parent
path=root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json";schema_path=path.parent/"🧬️schema/🔣️.schema.json";raw=path.read_bytes();before=json.loads(raw);model=copy.deepcopy(before)
obj=next(row for row in model["elements"]if row["tag"]=="FIN-ROOF-GRAVEL");original=next(row for row in before["elements"]if row["tag"]==obj["tag"])
refs=obj["geometry"].setdefault("interface_owners",[])
if "CHIMNEY-SALON-ROOF"not in refs:refs.append("CHIMNEY-SALON-ROOF")
obj["psets"].setdefault("Pset_MaterialReceivingInterfaces",{})["MasonryPriority"]={"value":"The declared roof finish yields only to actual salon chimney masonry; original Reference geometry and chimney dimensions remain unchanged. The unmeasured closure is unresolved. / Der deklarierte Dachaufbau weicht nur dem tatsächlichen Mauerwerk des Salon-Schornsteins; ursprüngliche Referenzgeometrie und Schornsteinmaße bleiben unverändert. Der nicht gemessene Anschluss bleibt ungeklärt.","ifc_type":"IfcText","basis":"inferred","source":"SAB2024-p117-119; FLC-H1-13-306"}
assert obj["guid"]==original["guid"]and obj["geometry"]["parts"]==original["geometry"]["parts"]
assert [row for row in before["elements"]if row["tag"]!=obj["tag"]]==[row for row in model["elements"]if row["tag"]!=obj["tag"]]
assert {key:value for key,value in before.items()if key!="elements"}=={key:value for key,value in model.items()if key!="elements"}
schema_raw=schema_path.read_bytes();Draft202012Validator(json.loads(schema_raw)).validate(model)
encoded=(json.dumps(model,ensure_ascii=False,indent=2)+"\n").encode("utf8")
if path.read_bytes()!=raw or schema_path.read_bytes()!=schema_raw:raise RuntimeError("Source or schema moved during guarded declaration validation")
path.write_bytes(encoded)
result={"scope":"One explicit material interface and qualified source interpretation only.","before":hashlib.sha256(raw).hexdigest(),"after":hashlib.sha256(encoded).hexdigest(),"tags":[obj["tag"]],"interfaces":refs,"source_parts_unchanged":True,"other_objects_unchanged":True,"schema_errors":0}
(ticket/"🗑️generated/roof-masonry-interface-source-r95.json").write_text(json.dumps(result,indent=2),encoding="utf8")
print("[DEBUG]",json.dumps(result),flush=True)
