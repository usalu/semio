#!/usr/bin/env python3
"""Fail-closed patch: store codec export/import fn pointers return RetainedWorkOutcome<IoOutcome<T>,IoError>.
usage: apply.py dry|write [--allow-manual]
Aborts (exit 2, writes nothing) when any anchor does not match exactly as expected or when unmatched uses remain."""
import re,subprocess,sys,json,os
ROOT=os.getcwd()
MODE=sys.argv[1] if len(sys.argv)>1 else "dry"
ALLOW="--allow-manual" in sys.argv
P="🧰️framework/🛍️products/💻️os/🔨️modules/"
STORE_IO=P+"🏪️store/🚪️io/🦀️.rs"
REGISTRY=P+"🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs"
OPERATIONS=P+"🏪️store/📦️codec/🪶️snapshot-capability/🧰️codec-operations/🦀️.rs"
IOMECH=P+"🚪️io/🧭️io-mechanism/🦀️.rs"
GUEST=P+"🌉️mcp/🏠️workspace/🗂️guest-document-codec/🦀️.rs"
SAMPLES=0
RWO="semio_framework_value::RetainedWorkOutcome"
files={};touched={};manual=[]
def read(p):
    if p not in files: files[p]=open(os.path.join(ROOT,p),encoding="utf8").read()
    return files[p]
def put(p,s,why):
    files[p]=s;touched.setdefault(p,[]).append(why)
def die(m): print("ABORT:",m);sys.exit(2)
def close(s,i,o,c):
    d=0;j=i
    while True:
        ch=s[j]
        if ch=='"':
            j+=1
            while s[j]!='"':
                if s[j]=='\\': j+=1
                j+=1
        elif ch==o: d+=1
        elif ch==c:
            d-=1
            if d==0: return j
        j+=1
def newret(ret):
    m=re.fullmatch(r"\s*((?:[\w:]*::)?)IoResult<(.+)>\s*",ret,re.S)
    if not m: die(f"return type is not IoResult<..>: {ret[:80]}")
    return f"{RWO}<{m.group(1)}IoOutcome<{m.group(2)}>,{m.group(1)}IoError>"
def replace_once(p,old,new,why):
    s=read(p)
    if s.count(old)!=1: die(f"{p[-60:]}: anchor count {s.count(old)} != 1 for {old[:70]!r}")
    put(p,s.replace(old,new),why)
def retype_fn(p,name,why):
    """changes the return type of `fn name` (IoResult<X> -> outcome) in place"""
    s=read(p);ms=list(re.finditer(r"\bfn "+re.escape(name)+r"\b",s))
    if len(ms)!=1: die(f"{p[-60:]}: fn {name} occurs {len(ms)}x")
    m=ms[0];par=s.index("(",m.end());pe=close(s,par,"(",")");arrow=s.index("->",pe);brace=s.index("{",arrow)
    ret=s[arrow+2:brace];put(p,s[:arrow+2]+newret(ret)+s[brace:],why)
def wrap_fn(p,name,why):
    """keeps the old body under `<name>_result` and adds a RetainedWorkOutcome wrapper (one performed action) under the old name"""
    s=read(p);ms=list(re.finditer(r"((?:pub(?:\(crate\))? )?)fn "+re.escape(name)+r"\b",s))
    if len(ms)!=1: die(f"{p[-60:]}: fn {name} occurs {len(ms)}x")
    m=ms[0];gen_start=m.end();par=s.index("(",gen_start);generics=s[gen_start:par]
    pe=close(s,par,"(",")");params=s[par+1:pe];arrow=s.index("->",pe);brace=s.index("{",arrow);ret=s[arrow+2:brace]
    args=[];d=0;cur=""
    for ch in params+",":
        if ch in "(<[": d+=1
        elif ch in ")>]": d-=1
        if ch=="," and d==0:
            if cur.strip(): args.append(cur.strip().split(":",1)[0].strip().removeprefix("mut ").strip())
            cur=""
        else: cur+=ch
    turbofish=""
    if generics.strip().startswith("<"):
        names=[g.split(":",1)[0].strip() for g in re.split(r",(?![^<]*>)",generics.strip()[1:-1]) if g.strip() and not g.strip().startswith("'")]
        turbofish="::<"+",".join(names)+">" if names else ""
    vis=m.group(1)
    wrapper=f"{vis}fn {name}{generics}({params})->{newret(ret)}{{{RWO}{{result:{name}_result{turbofish}({','.join(args)}),work:1}}}}\n"
    new=s[:m.start()]+wrapper+s[m.start():m.start()+len(m.group(1))]+f"fn {name}_result"+s[gen_start:]
    new=new.replace(f"fn {name}_result"+s[gen_start:gen_start+0],f"fn {name}_result",1)
    put(p,new,why)
# A. type definitions
s=read(STORE_IO)
for line_head in ("pub type ArtifactSqliteSnapshotExportFile=fn(","pub type ArtifactSqliteSnapshotImportFile=fn("):
    i=s.find(line_head)
    if i<0 or s.count(line_head)!=1: die("store io type anchor "+line_head)
    j=s.index(";",i)
    seg=s[i:j];k=seg.rindex("->");put(STORE_IO,s[:i]+seg[:k+2]+newret(seg[k+2:])+s[j:],"fn-pointer type");s=read(STORE_IO)
for field in ("    pub export: fn(","        fn(&str, &semio_framework_artifact_reference::ArtifactDialect, &mut Option<"):
    i=s.find(field)
    if i<0 or s.count(field)!=1: die("store io field anchor "+field[:40])
    j=s.index(",\n",i);seg=s[i:j];k=seg.rindex("->");put(STORE_IO,s[:i]+seg[:k+2]+newret(seg[k+2:])+s[j:],"codec field type");s=read(STORE_IO)
# B. fillers
retype_fn(OPERATIONS,"export_snapshot_impl","codec-operations filler")
retype_fn(OPERATIONS,"import_snapshot_impl","codec-operations filler")
s=read(REGISTRY)
old_flat="fn flatten<T>(turn:RetainedWorkOutcome<IoResult<T>,NativeSnapshotFailure>)->IoResult<T>{turn.result.map_err(io_failure).and_then(|inner|inner)}"
new_flat="fn flatten<T>(turn:RetainedWorkOutcome<IoResult<T>,NativeSnapshotFailure>)->RetainedWorkOutcome<IoOutcome<T>,IoError>{RetainedWorkOutcome{result:turn.result.map_err(io_failure).and_then(|inner|inner),work:turn.work}}"
replace_once(REGISTRY,old_flat,new_flat,"flatten keeps the nested turn work")
retype_fn(REGISTRY,"export","registry export returns the nested turn work")
retype_fn(REGISTRY,"import","registry import returns the nested turn work")
wrap_fn(IOMECH,"export_snapshot_file_read","io-mechanism file filler")
wrap_fn(IOMECH,"import_snapshot_file_into","io-mechanism file filler")
wrap_fn(GUEST,"guest_sqlite_export","mcp guest filler")
wrap_fn(GUEST,"guest_sqlite_import","mcp guest filler")
REFUSAL=P+"🔌️plugin/🧪️tests/🔬️app-declarations-fixture/⚠️refusal/🦀️.rs"
wrap_fn(REFUSAL,"sqlite_guest_refusal_export","test filler")
wrap_fn(REFUSAL,"sqlite_guest_refusal_import","test filler")
# C. non-test callers inside io-mechanism: take the result, work stays with the enclosing outer API
replace_once(IOMECH,"callback(dialect,read,encoding,control,native)})();","callback(dialect,read,encoding,control,native).result})();","file export caller")
replace_once(IOMECH,"callback(dialect,bytes,destination,control,native)})();","callback(dialect,bytes,destination,control,native).result})();","file import caller")
replace_once(IOMECH,"(provider.export)(&codec.schema,&hop.from,payload,control,native)","(provider.export)(&codec.schema,&hop.from,payload,control,native).result","hop export caller")
replace_once(IOMECH,"(provider.import)(&codec.schema,&hop.into,&mut frame.snapshot.database,encoding,control,native)","(provider.import)(&codec.schema,&hop.into,&mut frame.snapshot.database,encoding,control,native).result","hop import caller")
# D. tests, fixtures and every other mention: append `.result` to calls through the four pointers
all_files=subprocess.run(["git","ls-files","-z","--","*.rs"],capture_output=True,cwd=ROOT).stdout.decode().split("\0")
pointer=re.compile(r"\(\s*[\w:.&*]+?\.(?:export|import|export_file|import_file)\s*\)\s*\(|\b(?:export_file|import_file)\.(?:unwrap|expect)\([^)]*\)\s*\(")
cands=[f for f in all_files if f and not f.startswith(".🧬") and os.path.exists(os.path.join(ROOT,f)) and f not in (STORE_IO,)]
hits=subprocess.run(["git","grep","-lE",r"\.(export|import|export_file|import_file)\)|(export_file|import_file)\.(unwrap|expect)","--","*.rs"],capture_output=True,cwd=ROOT).stdout.decode().split("\n")
for f in [h for h in hits if h and not h.startswith(".🧬") and h not in (STORE_IO,IOMECH)]:
    s=read(f);out=[];i=0;n=0
    while True:
        m=pointer.search(s,i)
        if not m: out.append(s[i:]);break
        j=close(s,m.end()-1,"(",")")+1;out.append(s[i:j])
        if not s.startswith(".result",j): out.append(".result");n+=1
        i=j
    if n:
        put(f,"".join(out),f"{n} pointer call(s) take .result")
        if "--sample" in sys.argv and SAMPLES<4:
            SAMPLES+=1;k=files[f].index(".result");print("##",f[-50:],files[f][max(0,k-200):k+40].replace(chr(10)," "))
    left=[x for x in re.findall(r".{0,60}\.(?:export_file|import_file)\b.{0,40}",s)]
# E. anything that still builds the codec struct or names the pointer types outside the known fillers
for f in hits+[x for x in all_files if x]:
    pass
res=subprocess.run(["git","grep","-nE",r"ArtifactSqliteSnapshotCodec *\{|ArtifactSqliteSnapshotExportFile|ArtifactSqliteSnapshotImportFile","--","*.rs"],capture_output=True,cwd=ROOT).stdout.decode().split("\n")
for l in res:
    if not l or l.startswith(".🧬") or any(l.startswith(x) for x in (STORE_IO,OPERATIONS,GUEST,IOMECH,REFUSAL)): continue
    body=l.split(":",2)[2] if l.count(":")>=2 else l
    literal=[m for m in re.finditer(r"([\w:]*)ArtifactSqliteSnapshotCodec *\{",body) if not re.search(r"->\s*$",body[:m.start()])]
    named=re.search(r"ArtifactSqliteSnapshot(?:Export|Import)File",body)
    if literal or named: manual.append(l[:200])
report={"mode":MODE,"touched":{k:v for k,v in sorted(touched.items())},"manual":manual}
rp=os.path.join(os.path.dirname(os.path.abspath(__file__)),"dry-run-report.json")
json.dump(report,open(rp,"w",encoding="utf8"),ensure_ascii=False,indent=1)
print("touched files:",len(touched),"manual leftovers:",len(manual))
if "--show" in sys.argv:
    for f in (OPERATIONS,REGISTRY,IOMECH,GUEST,STORE_IO):
        t=files[f]
        for m in re.finditer(r"(fn (?:export_snapshot_impl|import_snapshot_impl|flatten|export|import|export_snapshot_file_read|import_snapshot_file_into|guest_sqlite_export|guest_sqlite_import)(?:_result)?\b|pub type ArtifactSqliteSnapshot(?:Export|Import)File|    pub export:|        fn\(&str)",t):
            print("##",f[-40:],t[m.start():m.start()+int(os.environ.get("SHOWN","520"))].replace(chr(10)," ")[:520]);
if manual and not ALLOW and MODE=="write": die("manual leftovers present (see dry-run-report.json); rerun with --allow-manual after hand-fixing")
if MODE=="write":
    for p in touched: open(os.path.join(ROOT,p),"w",encoding="utf8").write(files[p])
    print("written")
