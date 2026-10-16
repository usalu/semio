"""🔬️ Scoped patched resolution differential: HEAD sandbox versus working-tree sandbox, output to /dev/null only. Usage: oracle.py <sandbox-dir> <lib-root-rel>..."""
import json,os,re,subprocess,sys,shutil,collections
ROOT="/Users/ueli/Documents/semio"
os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
reach=json.load(open(F+"/🗑️generated/dac-reach.json"))
SB=os.path.realpath(sys.argv[1])
assert SB.startswith("/private/tmp/claude-501/") or SB.startswith("/tmp/claude-501/"),SB
roots=sys.argv[2:]
TOOL="+nightly-2026-07-20"
GLOB=re.compile(r"^([ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+(?:::)?(?:semio_[A-Za-z0-9_]*|ui_[A-Za-z0-9_]*)(?:::[A-Za-z0-9_]+)*::\*[ \t]*;)",re.M)
EXTERN=re.compile(r"^[ \t]*extern[ \t]+crate[ \t]+[A-Za-z0-9_]+[^;\n]*;[ \t]*$",re.M)
def head(p):
    r=subprocess.run(["git","-c","core.fsmonitor=false","show","HEAD:"+p],capture_output=True)
    return r.stdout.decode("utf8") if r.returncode==0 else None
def patch(t):
    t=GLOB.sub("const _: () = ();",t)
    t=EXTERN.sub("",t)
    return t
def inside(root):
    libs={o["lib"] for o in reach[root]}
    return [f for f,v in reach.items() if any(o["lib"] in libs for o in v)]
def build(tag,files,use_head):
    base=os.path.join(SB,tag)
    shutil.rmtree(base,ignore_errors=True)
    for f in files:
        t=head(f) if use_head else (open(f,encoding="utf8").read() if os.path.exists(f) else None)
        if t is None: continue
        dst=os.path.join(base,f)
        assert os.path.realpath(os.path.dirname(dst)).startswith(SB) or True
        os.makedirs(os.path.dirname(dst),exist_ok=True)
        assert not os.path.islink(dst)
        open(dst,"w",encoding="utf8").write(patch(t))
    return base
ERR=re.compile(r"^(?:.*?):\d+:\d+: (error(?:\[E\d+\])?: .*)$")
def run(base,root,test,crate):
    cmd=["rustc",TOOL,"--edition","2021","--crate-type","lib","--crate-name",crate,"--emit=metadata","-o","/dev/null","--error-format=short","--cap-lints","allow"]
    if test: cmd.append("--test")
    cmd.append(os.path.join(base,root))
    r=subprocess.run(cmd,capture_output=True,text=True,cwd=base)
    out=collections.Counter()
    for l in r.stderr.split("\n"):
        m=ERR.match(l)
        if m: out[re.sub(r"/private/tmp/claude-501/[^`\s]*?/dac-sb/(?:base|cur)/","",m.group(1))]+=1
    return out
total_new=0
for root in roots:
    files=inside(root)
    crate=[o["lib"] for o in reach[root]][0]
    b0=build("base",files,True); b1=build("cur",files,False)
    for test in (False,True):
        a=run(b0,root,test,crate); c=run(b1,root,test,crate)
        noext=lambda k: "semio_framework_" in k or "semio_s_" in k or "ui_" in k.split("`")[1:2].__str__()
        newonly={k:c[k]-a.get(k,0) for k in c if c[k]>a.get(k,0) and not noext(k)}
        gone={k:a[k]-c.get(k,0) for k in a if a[k]>c.get(k,0) and not noext(k)}
        ext_new=sum(v-a.get(k,0) for k,v in c.items() if noext(k) and v>a.get(k,0))
        print(f"{crate} test={test} base_errors={sum(a.values())} cur_errors={sum(c.values())} NEW_NONEXT={len(newonly)} GONE_NONEXT={len(gone)} extcrate_delta={ext_new}")
        for k,v in list(newonly.items())[:8]: print("   NEW",v,k[:200])
        total_new+=len(newonly)
print("TOTAL_NEW_NONEXT",total_new)
