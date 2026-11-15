#!/usr/bin/env python3
"""R15 landing: the pre-install import closure of the front door loads no bare package specifier.
usage: land.py --dry-run | --land      (--land needs N/LAND-OK and takes T/landing.lock)
Edits are literal and idempotent: a file that already holds the new text is reported as same."""
import os,sys,subprocess,hashlib,datetime
R="/Users/Shared/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
M="🧰️framework/🔨️modules/"
Z="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🧪️tests/🟦️.ts"
EDITS=[
 (M+"🧬️schema/📄️source/🔗️closure/🟦️.ts",'"@semio-tech/pack-json-decode-ts"','"../../../🎒️pack/🔤️json/📥️decode/📦️packages/🟦️typescript/🟦️.ts"'),
 (M+"🧬️schema/📄️source/🔗️closure/🟦️.ts",'"@semio-tech/framework-pack-json-ts/members"','"../../../🎒️pack/🔤️json/📦️packages/🟦️typescript/🧩️members/🟦️.ts"'),
 (M+"🎒️pack/🔤️json/📥️decode/🟦️.ts",'"@semio-tech/framework-pack-json-ts/members"','"../📦️packages/🟦️typescript/🧩️members/🟦️.ts"'),
 (M+"🖱️ui/🎨️styling/📽️projection/🟦️.ts",'"@semio-tech/assets/resolver/delivery"','"../../../🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🌐️delivery/🟦️.ts"'),
 (M+"🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts",'"@semio-tech/framework/identity/grapheme"','"../../../../../📦️packages/🟦️typescript/🪪️identity/🧩️grapheme/🟦️.ts"'),
]
TESTS=[Z,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🚦️startup/🟦️.ts",M+"🧬️schema/📄️source/🔗️closure/🧪️tests"]
def sha(p):return hashlib.sha256(open(p,"rb").read()).hexdigest()
def state():
    rows=[]
    for rel,old,new in EDITS:
        t=open(R+"/"+rel,encoding="utf8").read()
        rows.append((rel,"same" if old not in t and new in t else "apply" if t.count(old)==1 else "REFUSED"))
    marker="copy of their relative closure that has no node_modules"
    rows.append((Z,"same" if marker in open(R+"/"+Z,encoding="utf8").read() else "REFUSED (test block missing)"))
    return rows
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run";rows=state()
    for r in rows:print(*r[::-1])
    if any(s.startswith("REFUSED") for _,s in rows):sys.exit("refused")
    if mode=="--dry-run":print("dry-run clean");return
    if not os.path.exists(N+"/N/LAND-OK"):sys.exit("N/LAND-OK missing")
    lock=T+"/landing.lock"
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock held by "+open(lock+"/owner").read().strip())
    open(lock+"/owner","w").write("R15-%d\n"%os.getpid())
    try:
        for rel,old,new in EDITS:
            p=R+"/"+rel;t=open(p,encoding="utf8").read()
            if old in t:open(p+".r15tmp","w",encoding="utf8").write(t.replace(old,new));os.replace(p+".r15tmp",p)
        env=dict(os.environ,BUN_RUNTIME_TRANSPILER_CACHE_PATH="0")
        for k in list(env):
            if k.startswith("SEMIO_FLEET") or k.startswith("SEMIO_REMOTE_CACHE"):del env[k]
        out=[]
        for tf in TESTS[:2]:
            d=os.path.dirname(R+"/"+tf);r=subprocess.run([BUN,"test","./"+os.path.basename(tf),"--timeout","180000"],cwd=d,env=env,capture_output=True,text=True)
            out.append((tf,r.returncode,[l for l in (r.stdout+r.stderr).splitlines() if l.startswith("(fail)")]))
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## R15 %s: no bare package specifier in the pre-install import closure\n"%datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%MZ"))
            for rel,old,new in EDITS:f.write("- `%s` sha256 %s\n"%(rel,sha(R+"/"+rel)[:16]))
            f.write("- `%s` sha256 %s (runtime copy law added)\n"%(Z,sha(R+"/"+Z)[:16]))
            for tf,rc,fails in out:f.write("- test `%s` exit %d, %d red: %s\n"%(tf,rc,len(fails),"; ".join(x[:90] for x in fails)))
        print(out)
    finally:
        if open(lock+"/owner").read().startswith("R15-"):os.remove(lock+"/owner");os.rmdir(lock)
main()
