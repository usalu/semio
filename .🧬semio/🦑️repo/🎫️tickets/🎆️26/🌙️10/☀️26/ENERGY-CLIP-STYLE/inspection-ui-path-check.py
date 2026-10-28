"""🔎 Isolate production browser dependency paths without rendering media."""
import faulthandler
import importlib
import json
import os
from pathlib import Path
import sys
import time
TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]=str(TICKET/"🗑️generated/i")
faulthandler.enable()
START=time.monotonic()
def mark(message):
    print(f"[DEBUG] {time.monotonic()-START:.3f}s {message}",flush=True)
PACKAGE="🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
browser=importlib.import_module(PACKAGE+".🌐️browser.🐍️")
hierarchy=importlib.import_module(PACKAGE+".🥞️hierarchy.🐍️")
mark("imported")
paths=browser.source_files()
mark(f"source_files returned {len(paths)} paths")
(TICKET/"🗑️generated/i/ui-paths.json").write_text(json.dumps([str(p) for p in paths],ensure_ascii=False,indent=2),encoding="utf8")
for index,path in enumerate(paths):
    path.resolve()
    if index%100==0: mark(f"resolved {index} {path}")
mark("all paths resolved")
identity=browser._handoff_identity(dict(catalog=hierarchy.hierarchy_contract(),language="de",viewport=dict(width=480,height=270)))
mark(f"identity {identity}")
