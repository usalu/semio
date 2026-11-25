import re,subprocess,sys,os,tempfile
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
G="--include=🦀️.rs --exclude-dir=.cache --exclude-dir=.🧬semio --exclude-dir=🎫️tickets --exclude-dir=node_modules"
sh=lambda c:subprocess.run(c,shell=True,capture_output=True,text=True).stdout.split("\n")
have=set()
for f in sh(f"grep -rIl artifact_persisted_binary_operation {G} ✏️s 🧰️framework"):
    if f:
        for m in re.finditer(r"artifact_persisted_binary_operation!\(\s*([\w:]+)",open(f,encoding="utf8").read()): have.add(m.group(1).split("::")[-1])
need=set()
for f in sh(f"grep -rIl 'type Mutation = ' {G} ✏️s"):
    if not f: continue
    t=open(f,encoding="utf8").read()
    if "ArtifactEditor for" in t:
        for m in re.finditer(r"type Mutation = ([\w:]+);",t): need.add(m.group(1).split("::")[-1])
miss=need-have
pat=re.compile(r"^(\s*)((?:\w+::)+)(original_\w+_operation_receiving)!\(\s*([\w:]+)")
done=set()
for f in sh(f"grep -rIlE 'operation_receiving!' {G} ✏️s 🧰️framework"):
    if not f: continue
    lines=open(f,encoding="utf8").read().split("\n"); out=[]; ch=False
    for i,l in enumerate(lines):
        out.append(l)
        m=pat.match(l)
        if m and m.group(4).split("::")[-1] in miss and l.rstrip().endswith(");"):
            n=m.group(4).split("::")[-1]
            if i>0 and lines[i-1].lstrip().startswith("#["): out.insert(len(out)-1,lines[i-1]) if False else None
            out.append(f"{m.group(1)}{m.group(2)}artifact_persisted_binary_operation!({m.group(4)});"); ch=True; done.add(n); print("ADD",n,f[-70:])
    if ch and apply:
        fd,p=tempfile.mkstemp(dir=os.path.dirname(f)); os.write(fd,"\n".join(out).encode()); os.close(fd); os.chmod(p,os.stat(f).st_mode); os.replace(p,f)
print("UNHANDLED",sorted(miss-done))
