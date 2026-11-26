import re,subprocess,sys,pathlib,os
root=pathlib.Path(".").resolve(); py="/Users/ueli/Documents/semio/.cache/b/d/m/python/environments/workspace-3b97251b6599d245/bin/python"
fx=root/"🧫️fixtures"; out=[]
for f in sorted(root.glob("*/🐍️.py")):
    src=f.read_text(); m=re.search(r"python 🐍️\.py (check|write) <(?:path to )?(?:\S*?)(🧫️fixtures[^>]*)>",src) or re.search(r"<S>/(🧫️fixtures[^`]*)`",src)
    if not m: out.append((f.parent.name,"NOARGS",src[:0])); continue
    mode="check"; g=m.groups(); p=g[-1].strip()
    path=root/p
    if not path.exists(): out.append((f.parent.name,"NOPATH",str(path))); continue
    r=subprocess.run([py,str(f),mode,str(path)],capture_output=True,text=True,timeout=900,env={**os.environ,"PYTHONIOENCODING":"utf-8"})
    out.append((f.parent.name,"PASS" if r.returncode==0 else "FAIL",(r.stdout+r.stderr)[-600:].replace("\n"," | ")))
for o in out: print(*o,sep="\t")
