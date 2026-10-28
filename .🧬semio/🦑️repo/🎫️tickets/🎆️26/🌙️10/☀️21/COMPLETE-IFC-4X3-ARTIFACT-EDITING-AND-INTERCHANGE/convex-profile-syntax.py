"""🦀️ Parse the paired scalar math source without implying compiler or native funding proof."""
from pathlib import Path
import subprocess,json

ticket=Path(__file__).parent
source=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🔲️convex/🦀️.rs"
result=subprocess.run(["rustfmt","--edition","2024","--emit","stdout",str(source)],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,text=True)
value={"code":result.returncode,"stderr":result.stderr,"source":str(source),"nativeTypeChecked":False,"nativeExecuted":False,"scope":"Paired fixed-cost scalar boundary source only; not a mounted original-funded IFC producer."}
(ticket/"🗑️generated/convex-profile-syntax.json").write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
print("[DEBUG] "+json.dumps({"syntax":result.returncode,"nativeExecuted":False,"stderr":result.stderr}))
raise SystemExit(result.returncode)
