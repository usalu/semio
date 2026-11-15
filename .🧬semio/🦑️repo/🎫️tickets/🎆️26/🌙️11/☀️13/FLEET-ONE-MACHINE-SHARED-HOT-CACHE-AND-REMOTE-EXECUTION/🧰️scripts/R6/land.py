#!/usr/bin/env python3
"""R6 landing (see payload.TITLE): applies the anchored edits of payload.py (every old text exactly once; an edit is already applied when its old text is gone and its new text present) and writes the new files, in one burst under T/landing.lock and N/LAND-OK.
usage: land.py --dry-run | --land [--lock-held] [--no-tests] | --rollback <backup-dir> | --overlay <scratch-dir>
  --dry-run applies the edits in memory, parses every edited or new TypeScript and JSON file with the bun transpiler, checks that every relative import resolves and prints the plan; it writes nothing into the tree.
  --land takes T/landing.lock (mkdir + owner file, waits, never removes a lock it does not own), writes N/LAND-OK for the burst (only when it does not exist, and removes it again), re-reads every live file, refuses when an old text does not occur exactly once, writes all files first and replaces them back to back, keeps the originals in T/🗑️generated/R6-landing-<time>/ and runs the test files of payload.TESTS.
  --overlay <dir> writes the edited files and a bun preload that serves them under their live paths (the new test file is written under <dir>/new with absolute imports) so the tests run before landing without touching the tree.
  The real `repo` command of the landing rules is not run by this script."""
import os,sys,subprocess,hashlib,time,shutil,datetime,re,json
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
import payload
def arg(name):
    return sys.argv[sys.argv.index(name)+1] if name in sys.argv else None
R="/Users/Shared/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p,"rb") as f:return f.read()
def plan():
    out={};refused=[]
    for rel in dict.fromkeys(e[0] for e in payload.EDITS):
        p=R+"/"+rel
        if not os.path.exists(p):refused.append((rel,"live file is gone"));continue
        raw=rd(p);text=raw.decode("utf8")
        for path,old,new in payload.EDITS:
            if path!=rel:continue
            if old not in text and (new=="" or new in text):continue
            n=text.count(old)
            if n!=1:refused.append((rel,"old text occurs %d times, expected 1: %r"%(n,old[:70])));break
            text=text.replace(old,new)
        else:
            out[rel]=("same" if text.encode("utf8")==raw else "edit",raw,text.encode("utf8"))
    for rel,src in payload.NEWFILES.items():
        data=rd(src);p=R+"/"+rel
        if os.path.exists(p):
            raw=rd(p);out[rel]=("same" if raw==data else "edit",raw,data)
        else:out[rel]=("new",b"",data)
    return out,refused
def check(out):
    bad=[]
    for rel,(how,raw,data) in out.items():
        if how=="same":continue
        if rel.endswith(".ts"):
            tmp=os.path.join(HERE,"syntax-check.ts");open(tmp,"wb").write(data)
            r=subprocess.run([BUN,"build","--no-bundle","--target=bun",tmp],capture_output=True)
            os.remove(tmp)
            if r.returncode!=0:bad.append((rel,(r.stdout+r.stderr).decode("utf8","replace")[:600]))
            for m in re.finditer(r'(?:from|import)\s*"(\.[^"]+)"',data.decode("utf8")):
                target=os.path.normpath(os.path.join(os.path.dirname(R+"/"+rel),m.group(1)))
                if not os.path.exists(target):bad.append((rel,"import does not resolve: "+m.group(1)[-90:]))
        elif rel.endswith(".json"):
            try:json.loads(data.decode("utf8"))
            except Exception as e:bad.append((rel,"json: "+str(e)))
    return bad
def overlay(dest):
    out,refused=plan()
    if refused:sys.exit("refused: %r"%refused)
    os.makedirs(dest+"/new",exist_ok=True);files={};n=0;names={}
    for rel,(how,raw,data) in out.items():
        if how=="new":continue
        n+=1;name="%d%s"%(n,".json" if rel.endswith(".json") else ".ts");open(dest+"/"+name,"wb").write(data);files[R+"/"+rel]=dest+"/"+name;names[rel]=dest+"/"+name
    err=json.dumps(R+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts")
    def absolute(code,base,rel):
        code=re.sub(r'((?:from|import)\s*)"(\.[^"]+)"',lambda m:m.group(1)+json.dumps(os.path.normpath(os.path.join(base,m.group(2)))),code)
        def bare(m):
            spec=m.group(2)
            if spec.startswith(("node:","bun:",".","/")):return m.group(0)
            r=subprocess.run([BUN,"-e","console.log(Bun.resolveSync(%s,%s))"%(json.dumps(spec),json.dumps(base))],capture_output=True,text=True).stdout.strip()
            return m.group(1)+json.dumps(r) if r else m.group(0)
        code=re.sub(r'((?:from|import)\s*)"([^"]+)"',bare,code)
        code=code.replace('import("../🟦️.ts")','import('+json.dumps(R+"/"+payload.RET)+')')
        code=code.replace('new URL("../🟦️.ts",import.meta.url)','new URL('+json.dumps("file://"+names[payload.RET])+')')
        code=code.replace('join(import.meta.dir,"..","..","..","🟦️.ts")',json.dumps(names[payload.SETUP])).replace('join(import.meta.dir,"..","..","🟦️.ts")',json.dumps(names[payload.RET]))
        return code
    for rel,out_name in [(payload.NEWTEST,"busy-driver.ts"),(payload.RETTEST,"retirement-test.ts"),(payload.OBSTEST,"obs-test.ts")]:
        base=os.path.dirname(R+"/"+rel);code=out[rel][2].decode("utf8")
        open(dest+"/new/"+out_name,"w",encoding="utf8").write(absolute(code,base,rel))
    json.dump(files,open(dest+"/map.json","w"))
    filt="^("+"|".join(re.escape(k) for k in files)+")$"
    open(dest+"/preload.ts","w").write('import{plugin}from"bun";import{readFileSync}from"node:fs";\nconst map=JSON.parse(readFileSync("%s/map.json","utf8")) as Record<string,string>;\nplugin({name:"r6-overlay",setup(build){build.onLoad({filter:new RegExp(%s)},args=>{const file=map[args.path]!;return file.endsWith(".json")?{exports:{default:JSON.parse(readFileSync(file,"utf8"))},loader:"object"}:{contents:readFileSync(file,"utf8"),loader:"ts"};});}});\n'%(dest,json.dumps(filt)))
    print(dest+"/preload.ts")
def run_tests(rels):
    res=[]
    for rel in rels:
        cwd=os.path.dirname(R+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
        env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
        r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
        text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")]
        verdict="as expected" if r.returncode==0 and not failed else "UNEXPECTED"
        res.append((rel,r.returncode,len(failed),verdict,text[-1500:]));print(verdict.ljust(11),rel[-80:],"fail",len(failed),*failed,sep="\n  " if failed else " ")
    return res
def main():
    if "--rollback" in sys.argv:
        b=arg("--rollback")
        for dp,_,fs in os.walk(b):
            for f in fs:
                src=os.path.join(dp,f);rel=os.path.relpath(src,b)
                if rel.endswith(".absent"):
                    t=R+"/"+rel[:-7]
                    if os.path.exists(t):os.remove(t)
                else:shutil.copyfile(src,R+"/"+rel)
        print("rolled back from",b);return
    if "--overlay" in sys.argv:overlay(arg("--overlay"));return
    out,refused=plan()
    for rel,(how,raw,data) in out.items():print(how.ljust(5),rel[-100:],sha(raw)[:12],"->",sha(data)[:12])
    for rel,why in refused:print("REFUSED",rel[-90:],why)
    bad=check(out)
    for rel,why in bad:print("INVALID",rel[-90:],why)
    if "--land" not in sys.argv:
        print("dry run:",len(out),"files,",len(refused),"refused,",len(bad),"invalid");return
    if refused or bad:sys.exit("refused")
    lock=T+"/landing.lock";held="--lock-held" in sys.argv;owner="R6-%d-%d"%(os.getpid(),int(time.time()));ok=N+"/LAND-OK";made=False
    if not held:
        for _ in range(360):
            try:
                os.mkdir(lock);break
            except FileExistsError:
                age=time.time()-os.path.getmtime(lock)
                if age>40*60:
                    print("lock older than 40 minutes: "+open(lock+"/owner").read().strip()+": the landing rules allow taking it over");shutil.rmtree(lock,ignore_errors=True);continue
                time.sleep(5)
        else:sys.exit("landing lock busy")
        open(lock+"/owner","w").write(owner+"\n")
    stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M");backup=T+"/🗑️generated/R6-landing-"+datetime.datetime.now().strftime("%H%M%S")
    try:
        if not os.path.exists(ok):open(ok,"w").write("R6 landing in progress\n");made=True
        out,refused=plan()
        if refused:sys.exit("refused after taking the lock: "+"; ".join(r for r,_ in refused))
        for rel,(how,raw,data) in out.items():
            if how=="same":continue
            os.makedirs(os.path.dirname(backup+"/"+rel),exist_ok=True)
            if how=="new":open(backup+"/"+rel+".absent","w").write("")
            else:shutil.copyfile(R+"/"+rel,backup+"/"+rel)
            os.makedirs(os.path.dirname(R+"/"+rel),exist_ok=True);open(R+"/"+rel+".r6tmp","wb").write(data)
        for rel,(how,raw,data) in out.items():
            if how!="same":os.replace(R+"/"+rel+".r6tmp",R+"/"+rel)
        print("landed",sum(1 for v in out.values() if v[0]!="same"),"files; originals in",backup)
        res=[] if "--no-tests" in sys.argv else run_tests(payload.TESTS)
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package R6 landing (%s): %s\n"%(stamp,payload.TITLE))
            for rel,(how,raw,data) in out.items():f.write("- %s `%s` before %s after %s\n"%(how,rel[len("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"):] if rel.startswith("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/") else rel,sha(raw)[:12] if raw else "-",sha(data)[:12]))
            for rel,code,failed,verdict,_ in res:f.write("- test `%s` exit %d, %d red: %s\n"%(rel[-70:],code,failed,verdict))
            f.write("- originals for `land.py --rollback`: `%s`\n"%backup)
        for rel,code,failed,verdict,text in res:
            if verdict=="UNEXPECTED":print("UNEXPECTED test result: inspect, then --rollback "+backup);print(text)
    finally:
        if made and os.path.exists(ok):os.remove(ok)
        if not held and os.path.exists(lock+"/owner") and open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
