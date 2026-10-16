import json,os,re,sys
from targets import *
here=os.path.dirname(os.path.abspath(__file__))
units=json.load(open(here+"/units.json"))
NEW_PKG="🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/📦️packages/🦀️rust"
KEY="semio-framework-trace-memory-testing"
dry="--apply" not in sys.argv
mount_re=re.compile(r'#\[path\s*=\s*"[^"\n]*🧪️testing/📥️requests/🦀️\.rs"\]\n((?:pub\(crate\) )?)mod (\w+);')
files=sorted({m for u in units for m in u["mounts"]})
for f in files:
    p=os.path.join(ROOT,f)
    s=open(p,encoding="utf8").read()
    ms=mount_re.findall(s)
    assert len(ms)==1 and s.count("📥️requests/🦀️.rs")==1,(f,ms)
    t=mount_re.sub(lambda m:f"{m.group(1)}use semio_framework_trace_memory_testing as {m.group(2)};",s)
    print("MOUNT",f.split("/")[-2],ms[0][1],ms[0][0] or "private")
    if not dry: open(p,"w",encoding="utf8").write(t)
mans={}
for u in units: mans[u["name"]]=u["pkg"]
for name,pkg in sorted(mans.items()):
    mp=os.path.join(ROOT,pkg,"Cargo.toml")
    s=open(mp,encoding="utf8").read()
    assert KEY+" =" not in s,name
    rel=os.path.relpath(os.path.join(ROOT,NEW_PKG),os.path.join(ROOT,pkg))
    line=f'{KEY} = {{ path = "{rel}" }}'
    m=re.search(r'(?m)^\[dev-dependencies\]\n',s)
    assert m,name
    sec_end=re.search(r'(?m)^\[',s[m.end():])
    sec=s[m.end():m.end()+(sec_end.start() if sec_end else len(s))]
    tm=re.search(r'(?m)^semio-framework-trace(?: =|\.workspace)[^\n]*\n',sec)
    if tm: pos=m.end()+tm.end()
    else: pos=m.end()
    t=s[:pos]+line+"\n"+s[pos:]
    print("MANIFEST",name,"after trace" if tm else "first",rel if len(rel)<90 else rel[:90]+"…")
    if not dry: open(mp,"w",encoding="utf8").write(t)
