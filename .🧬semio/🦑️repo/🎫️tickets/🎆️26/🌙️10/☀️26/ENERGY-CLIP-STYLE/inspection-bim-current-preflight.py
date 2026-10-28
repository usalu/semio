"""🔎 Read current BIM narration, literal provider closure, signatures and schema without rendering."""
from pathlib import Path
import ast
import hashlib
import importlib
import inspect
import json
import os
import shutil
import uuid
import sys

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
BIM=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
fixture=BIM/"🧪️tests/🎭️presentation/🔣️.cases.json"
cases=json.loads(fixture.read_text(encoding="utf-8"))
required={ROOT/"🎓️teaching"/p for p in cases["dependencies"]}
leaves={case["id"]:ROOT/"🎓️teaching"/case["script"] for case in cases["chapters"]}
inputs=required|set(leaves.values())|{fixture,BIM/"🧱️model/🔣️.json",BIM/"🧱️model/🧬️schema/🔣️.schema.json"}
before={p.as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}
runtime=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
visuals=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
methods=set(cases["renderer_calls"])|{"class_box","class_tree","property_table","step_panel","model_card","guid_chip"}
result={"scenes":[],"provider_count":len(required),"renderer_errors":[]}
for case in cases["chapters"]:
    path=leaves[case["id"]]
    tree=ast.parse(path.read_text(encoding="utf-8"))
    cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name==case["scene"])
    assignment=next(n for n in cls.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=="NARRATION" for t in n.targets))
    narration=ast.literal_eval(assignment.value)
    keys=[row[0] for row in narration]
    missing=sorted(p.as_posix() for p in required-set(runtime.source_dependencies(path)))
    result["scenes"].append({"id":case["id"],"count":len(keys),"keys":keys,"fixture_matches":keys==case["keys"],
        "localized":all(len(row)==3 and all(isinstance(text,str) and text.strip() for text in row) for row in narration),
        "handler":any(isinstance(n,ast.FunctionDef) and n.name=="present_"+case["topic"] for n in cls.body),
        "missing_providers":missing})
    for call in ast.walk(cls):
        if not isinstance(call,ast.Call) or not isinstance(call.func,ast.Attribute) or not isinstance(call.func.value,ast.Name) or call.func.value.id!="self" or call.func.attr not in methods:continue
        try:
            inspect.signature(getattr(visuals.ClipScene,call.func.attr)).bind_partial(None,
                *[None for arg in call.args if not isinstance(arg,ast.Starred)],**{kw.arg:None for kw in call.keywords if kw.arg is not None})
        except (AttributeError,TypeError) as error:
            result["renderer_errors"].append({"id":case["id"],"method":call.func.attr,"line":call.lineno,"error":str(error)})
output=TICKET/"🗑️generated/ip"
output.mkdir(parents=True,exist_ok=True)
session=runtime.ToolSession()
try:
    runtime.run_command([os.environ.get("SEMIO_REPO_EXE") or shutil.which("repo"),"tools","exec","node","--light","--",str(TICKET/"inspection-bim-schema-current.mjs")],
        session=session,log=output/("schema-"+uuid.uuid4().hex[:8]+".log"))
except Exception as error:
    result["schema_invocation_error"]=str(error)
finally:
    session.cancel()
result["schema"]=json.loads((output/"schema-validation.json").read_text(encoding="utf-8"))
after={p.as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}
result["changed_inputs"]=[p for p in before if before[p]!=after[p]]
result["input_sha256"]=after
result["total_clauses"]=sum(row["count"] for row in result["scenes"])
result["passed"]=not result["changed_inputs"] and result["schema"]["valid"] and not result["renderer_errors"] and all(row["fixture_matches"] and row["localized"] and row["handler"] and not row["missing_providers"] for row in result["scenes"])
(output/"result.json").write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding="utf-8")
print("[DEBUG] "+json.dumps({k:v for k,v in result.items() if k!="input_sha256"},ensure_ascii=False))
raise SystemExit(not result["passed"])
