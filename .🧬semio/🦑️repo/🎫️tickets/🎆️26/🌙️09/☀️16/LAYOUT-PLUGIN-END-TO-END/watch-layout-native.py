from pathlib import Path
import json,sys,base64
p=Path(sys.argv[1]);w=p.parent/"watch.json"
state=json.loads(w.read_text())if w.exists()else{"offset":0}
f=p.open("rb");f.seek(state["offset"]);data=f.read()
pending=base64.b64decode(state["pendingB64"])if"pendingB64"in state else state.get("pending","").encode()
parts=(pending+data).split(b"\n");pending=parts.pop()
lines=[x.decode(errors="replace")for x in parts]
hits=[x for x in lines if x.lstrip().startswith(("Compiling semio-s-artifact-layout-layout","error","Running ","test result:"))]
w.write_text(json.dumps({"offset":f.tell(),"pendingB64":base64.b64encode(pending).decode()})+"\n")
print(json.dumps({"newBytes":len(data),"boundaries":hits,"latestHeartbeat":[x for x in lines if"still running after"in x or"[repo] still working"in x][-1:]},ensure_ascii=False))
