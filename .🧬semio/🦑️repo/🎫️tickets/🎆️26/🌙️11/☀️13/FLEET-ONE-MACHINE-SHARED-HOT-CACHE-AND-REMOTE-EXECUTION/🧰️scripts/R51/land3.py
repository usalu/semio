#!/usr/bin/env python3
"""R51 landing 3: repairs the two new law tests of landing 2 (a derived-question refusal as the child refusal; escaped newlines of the fixture) and runs them; takes landing.lock."""
import os,sys,subprocess,datetime,hashlib,re
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
SI=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/"
HQ=SI+"🪆️heavy-questions/🧪️tests/🟦️.ts";PF=SI+"🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R51-%d\n"%os.getpid())
try:
    t=open(HQ,encoding="utf8").read()
    t=t.replace('recipe:{fn:"tool-closure",name:"no-such-tool",view:"scheduling"}','recipe:{fn:"no-such-recipe"}').replace('expect(reason.key).toBe("build.closure.tool-unknown");expect(reason.values).toEqual({name:"no-such-tool"});','expect(reason.key).toBe("build.closure.derived-question-unknown");expect(String(reason.values.question)).toContain("no-such-recipe");').replace('expect(english).toContain("no-such-tool");expect(german).toContain("no-such-tool");','expect(english).toContain("no-such-recipe");expect(german).toContain("no-such-recipe");')
    open(HQ+".r51tmp","w",encoding="utf8").write(t);os.replace(HQ+".r51tmp",HQ)
    p=open(PF,encoding="utf8").read()
    p=re.sub(r'"export const generated = 1;\n"','"export const generated = 1;\\\\n"',p)
    p=re.sub(r"'import \"\./gen\.ts\";\n'","'import \"./gen.ts\";\\\\n'",p)
    open(PF+".r51tmp","w",encoding="utf8").write(p);os.replace(PF+".r51tmp",PF)
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    res=[]
    for d in [os.path.dirname(HQ),os.path.dirname(PF)]:
        r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","600000"],cwd=d,env=env,capture_output=True,text=True)
        out=r.stdout+r.stderr;red=[l for l in out.splitlines() if l.startswith("(fail)")];res.append((d,r.returncode,red));print("ok  " if r.returncode==0 else "RED ",d[len(SI):],*red)
        if r.returncode:print(out[-3500:])
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R51 landing 3 (%s): repairs of the two new law tests of landing 2 (landing 2 first imported the error module through the package re-export, which lacks readRepoErrorDataV1; the live tree was red for the sources of that import for about 8 minutes, fixed under the lock)\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        for p in (HQ,PF):f.write("- edit `%s` after %s\n"%(p[len(R)+1:],sha(p)))
        for d,c,red in res:f.write("- test `%s` exit %d, %d red%s\n"%(d[len(R)+1:],c,len(red),": "+"; ".join(red) if red else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
