"""🏗️ Validate current structural domains with durable results and semantic endpoint identity."""
import ast
from datetime import datetime,timezone
import hashlib
import json
import os
from pathlib import Path
import runpy
import sys
import time
import traceback
import unittest

sys.dont_write_bytecode=True
sys.stdout.reconfigure(encoding="utf8",errors="backslashreplace",line_buffering=True)
sys.stderr.reconfigure(encoding="utf8",errors="backslashreplace",line_buffering=True)
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/"🧰️framework").is_dir())
MODEL=ROOT/"🎓️teaching/🏛️architecture/🏢️nakagin"
OUTPUT=TICKET/"🗑️generated/structural-final"
OUTPUT.mkdir(parents=True,exist_ok=True)
for key in ("TMP","TEMP","TMPDIR","SEMIO_TEACHING_OUTPUT_ROOT"):
    os.environ[key]=str(OUTPUT)
DOMAINS=("📏️dimensions","🗼️core","🌉️bridge","🏬️podium","🏭️roof")
INPUTS=[Path(__file__),MODEL/"🐍️.py",MODEL/"🔣️.json",MODEL/"🧬️schema/🔣️.schema.json",MODEL/"🧪️tests/🐍️.py",MODEL/"🧪️cases/🔣️.json"]
for domain in DOMAINS:
    INPUTS.extend((MODEL/"🧪️tests"/domain/"🐍️.py",MODEL/"🧪️cases"/domain/"🔣️.json"))

def fingerprints():
    """🔐 Hash Python semantics and canonical portable JSON, independent of line endings."""
    result={}
    for path in INPUTS:
        source=path.read_text(encoding="utf8")
        content=ast.dump(ast.parse(source),include_attributes=False) if path.suffix==".py" else json.dumps(json.loads(source),ensure_ascii=False,sort_keys=True,separators=(",",":"))
        result[path.relative_to(ROOT).as_posix()]=hashlib.sha256(content.encode("utf8")).hexdigest()
    return result

report={"started_at":datetime.now(timezone.utc).isoformat(),"source_fingerprints":fingerprints(),"domains":[]}
original=unittest.TextTestRunner.run
captured=[]
def record(self,suite):
    """🧾 Retain the real unittest result without changing assertions or discovery."""
    result=original(self,suite)
    captured.append(result)
    return result
unittest.TextTestRunner.run=record
for domain in DOMAINS:
    target=MODEL/"🧪️tests"/domain/"🐍️.py"
    print("[DEBUG] structural domain "+domain,flush=True)
    start=time.perf_counter()
    count=len(captured)
    error=None
    try:
        sys.argv=[str(target)]
        runpy.run_path(str(target),run_name="__main__")
    except SystemExit as stop:
        if stop.code not in (None,0):
            error="exit "+str(stop.code)
    except BaseException as exc:
        error=type(exc).__name__+": "+str(exc)
        traceback.print_exc()
    results=captured[count:]
    row={"domain":domain,"tests":sum(r.testsRun for r in results),"failures":sum(len(r.failures) for r in results),"errors":sum(len(r.errors) for r in results),"elapsed_seconds":time.perf_counter()-start,"error":error,"success":bool(results) and error is None and all(r.wasSuccessful() for r in results)}
    report["domains"].append(row)
    (OUTPUT/"progress.json").write_text(json.dumps(report,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
report["source_fingerprints_end"]=fingerprints()
report["source_unchanged"]=report["source_fingerprints_end"]==report["source_fingerprints"]
report["success"]=report["source_unchanged"] and all(row["success"] for row in report["domains"])
(OUTPUT/"result.json").write_text(json.dumps(report,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] structural final "+str(report["success"]),flush=True)
raise SystemExit(0 if report["success"] else 1)
