#!/usr/bin/env python3
"""P1 landing: three-way merges payload.tar (mirror = P1 version, base = live file P1 started from) onto the live tree under T/landing.lock.
usage: land.py --dry-run | --land    (needs N/LAND-OK for --land; never removes a lock it does not own; git merge-file works on three plain files, it touches no repository)"""
import os,sys,tarfile,tempfile,subprocess,hashlib,time,shutil,datetime
HERE=os.path.dirname(os.path.abspath(__file__))
def root():
    for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"]:
        if os.path.exists(c+"/AGENTS.md") and os.path.exists(c+"/.cache/f/machine.json"):return c
    sys.exit("no worktree root found")
R=root();L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
IDENTICAL=("🏗️build/📥️source/🔏️closure/🔁️dependencies/🧪️tests/🟦️.ts","🏗️build/📥️source/🔏️closure/🔁️dependencies/🧪️tests/🔣️.json","🏗️build/📥️source/🔏️closure/🔁️dependencies/🧪️tests/📥️process/🟦️.ts")
TESTS=["⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🧪️tests/🟦️.ts","⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🧪️tests/🌱️probe/🟦️.ts","🏗️build/📥️source/🔏️closure/🔁️dependencies/🧪️tests/🛂️admission/🟦️.ts","🏗️build/📥️source/🔏️closure/🔁️dependencies/🧪️tests/🟦️.ts","🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️waves/🟦️.ts","⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts"]
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
GUIDE=R+"/🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
MARK="Projection edits while a command plans"
EN="- **Projection edits while a command plans**: the Original loads the private Nx projection (plugin root, private leaves, contract schemas) when it starts and checks the files again just before it hands the Nx process its identity. A code file that moved in between (a worker plans for tens of minutes while the replica brings peer edits) is adopted: a fresh process evaluates the current sources, the Original takes the identity and revisions it reports, and the command goes on without a restart or a second plan. A moved contract file (`*.json`), a move after the identity was handed out, or a fresh process that fails still restarts the outermost command (at most 6 times) with a message that names the files in English and German.\n"
DE="- **Änderungen der Projektion während ein Befehl plant**: der Original lädt die private Nx-Projektion (Plugin-Wurzel, private Blätter, Vertragsschemas) beim Start und prüft die Dateien erneut, bevor er dem Nx-Prozess die Identität übergibt. Eine Codedatei, die sich dazwischen änderte (ein Worker plant zig Minuten, während das Replikat Änderungen anderer bringt), wird übernommen: ein frischer Prozess wertet die aktuellen Quellen aus, der Original übernimmt Identität und Revisionen, die er meldet, und der Befehl läuft ohne Neustart und ohne zweite Planung weiter. Eine geänderte Vertragsdatei (`*.json`), eine Änderung nach der Übergabe der Identität oder ein fehlschlagender frischer Prozess startet den äußersten Befehl weiterhin neu (höchstens 6 Mal) mit einer Meldung, die die Dateien auf Englisch und Deutsch nennt.\n"
def guide(land):
    if not os.path.exists(GUIDE):return "guide missing"
    text=open(GUIDE,encoding="utf8").read()
    if MARK in text:return "guide already has the entry"
    lines=text.splitlines(keepends=True);out=[];done=0
    for line in lines:
        out.append(line)
        if line.startswith("- **Live**: a run finishes"):out.append(EN);done+=1
        elif line.startswith("- **Live**: ein Lauf endet"):out.append(DE);done+=1
    if done!=2:return "guide anchors not found (%d)"%done
    if land:open(GUIDE+".p1tmp","w",encoding="utf8").write("".join(out));os.replace(GUIDE+".p1tmp",GUIDE)
    return "guide entry in EN and DE"
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
    mode=sys.argv[1] if len(sys.argv)>1 else "--dry-run"
    tmp=tempfile.mkdtemp(prefix="p1-land-");assert os.path.realpath(tmp).startswith(os.path.realpath(tempfile.gettempdir()))
    with tarfile.open(HERE+"/payload.tar") as t:t.extractall(tmp)
    plan=[]
    for d,_,fs in os.walk(tmp+"/mirror"):
        for f in fs:
            rel=os.path.relpath(os.path.join(d,f),tmp+"/mirror")
            if rel in IDENTICAL:continue
            plan.append(rel)
    out={}
    for rel in sorted(plan):
        mine=open(tmp+"/mirror/"+rel,"rb").read();live_path=L+"/"+rel;base_path=tmp+"/base/"+rel
        if not os.path.exists(base_path):
            if os.path.exists(live_path) and open(live_path,"rb").read()!=mine:sys.exit("new file exists with other content: "+rel)
            out[rel]=("new",mine);continue
        live=open(live_path,"rb").read();base=open(base_path,"rb").read()
        if live==mine:out[rel]=("same",mine);continue
        if live==base:out[rel]=("copy",mine);continue
        lp=tmp+"/live.tmp";open(lp,"wb").write(live)
        r=subprocess.run(["git","merge-file","-p",lp,base_path,tmp+"/mirror/"+rel],capture_output=True)
        if r.returncode!=0:sys.exit("conflict (%s) in %s: rebase the payload by hand"%(r.returncode,rel))
        out[rel]=("merge",r.stdout)
    for rel,(how,data) in out.items():print(how.ljust(5),rel,sha(data)[:12])
    print(guide(False))
    if mode!="--land":print("dry run: nothing written");return
    if not os.path.exists(N+"/LAND-OK"):sys.exit("LAND-OK does not exist")
    lock=T+"/landing.lock";owner="P1-%d-%d"%(os.getpid(),int(time.time()))
    try:os.mkdir(lock)
    except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip()+": wait, never remove it")
    open(lock+"/owner","w").write(owner+"\n")
    try:
        before={}
        for rel,(how,data) in out.items():
            p=L+"/"+rel
            if how in("same",):continue
            before[rel]=sha(open(p,"rb").read()) if os.path.exists(p) else "-"
            if how in("merge","copy"):
                live=open(p,"rb").read()
                if how=="merge" and False:pass
            os.makedirs(os.path.dirname(p),exist_ok=True);open(p+".p1tmp","wb").write(data);os.replace(p+".p1tmp",p)
        print(guide(True));res=[]
        for rel in TESTS:
            cwd=os.path.dirname(L+"/"+rel);env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
            os.makedirs(T+"/🗑️generated",exist_ok=True);env["SEMIO_TEST_ARTIFACT_DIR"]=T+"/🗑️generated"
            r=subprocess.run([BUN,"test","./"+os.path.basename(rel),"--timeout","180000"],cwd=cwd,env=env,capture_output=True,text=True)
            tail=(r.stdout+r.stderr).strip().splitlines()[-4:];res.append((rel,r.returncode,tail));print(rel,r.returncode,*tail,sep="\n  ")
        stamp=datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
        with open(T+"/landings.md","a",encoding="utf8") as f:
            f.write("\n## Package P1 landing (%s): restart narrowing of the Nx projection admission\n"%stamp)
            for rel,(how,data) in out.items():f.write("- %s `LIB/%s` before %s after %s\n"%(how,rel,before.get(rel,"-")[:12],sha(data)[:12]))
            for rel,code,tail in res:f.write("- test `LIB/%s` exit %d: %s\n"%(rel,code,tail[-1] if tail else ""))
    finally:
        if open(lock+"/owner").read().strip()==owner:
            os.remove(lock+"/owner");os.rmdir(lock)
main()
