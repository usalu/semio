import json,subprocess,sys,os,shutil
R="/Users/ueli/Documents/semio"
here=os.path.dirname(os.path.abspath(__file__))
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad"
label=sys.argv[1]
units=json.load(open(here+"/units.json"))
only=set(map(int,sys.argv[2].split(","))) if len(sys.argv)>2 else None
for i,u in enumerate(units):
    if only is not None and i not in only: continue
    p=subprocess.run(["bun",here+"/closure.ts",R+"/"+u["entry"],R+"/"+u["pkg"],"list","test"],capture_output=True,text=True,cwd=R)
    if p.returncode!=0:
        print(i,u["name"],"CLOSURE-ERROR",p.stderr.strip().splitlines()[-1] if p.stderr.strip() else "",flush=True); continue
    files=[l.split("\t")[0] for l in p.stdout.splitlines() if l and not l.startswith("TOTAL")]
    dst=f"{S}/sb/{label}/{i:02d}"
    shutil.rmtree(dst,ignore_errors=True)
    for f in files:
        d=os.path.join(dst,f)
        os.makedirs(os.path.dirname(d),exist_ok=True)
        shutil.copyfile(os.path.join(R,f),d)
    json.dump(dict(unit=u,files=files),open(dst+"/_unit.json","w"),ensure_ascii=False)
    print(i,u["name"],len(files),flush=True)
