#!/usr/bin/env python3
import os,sys,json,subprocess
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
ST=C+"/law-drift-tools/staged"
def load():
    p=ST+"/patches.json"
    return json.load(open(p,encoding='utf-8')) if os.path.exists(p) else []
def save(rows): json.dump(rows,open(ST+"/patches.json","w",encoding='utf-8'),ensure_ascii=False,indent=1)
def apply_text(s,pairs):
    for pr in pairs:
        a,b=pr[0],pr[1]
        if len(pr)>2 and pr[2]=="all":
            if a in s: s=s.replace(a,b)
            continue
        if b in s and a!=b: continue
        if s.count(a)!=1:
            if b in s and s.count(b)>=1: continue
            raise SystemExit(f"anchor count {s.count(a)}: {a[:90]}")
        s=s.replace(a,b)
    return s
if __name__=="__main__":
    args=[a for a in sys.argv[1:] if not a.startswith("--")]
    opts=[a for a in sys.argv[1:] if a.startswith("--")]
    pid,file=args[0],args[1]
    testrel=next((a[7:] for a in opts if a.startswith("--test=")),file)
    pairs=json.load(sys.stdin)
    s=open(ROOT+"/"+file,encoding='utf-8').read()
    out=apply_text(s,pairs)
    rows=[r for r in load() if r['id']!=pid]
    rows.append({"id":pid,"file":file,"pairs":pairs})
    save(rows)
    outp=ST+"/out/"+pid+".ts"
    open(outp,'w',encoding='utf-8').write(out)
    if "--no-run" in opts: sys.exit(0)
    ov=[file+"="+outp]+[x for x in args[2:]]
    ov+= [r2['file']+"="+ST+"/out/"+r2['id']+".ts" for r2 in rows if r2['id']!=pid and any(o=="--with="+r2['id'] for o in opts)]
    r=subprocess.run([sys.executable,C+"/law-drift-tools/shadow.py",testrel]+ov,capture_output=True,text=True)
    txt=r.stdout+r.stderr
    import re
    print("\n".join(l[:230] for l in txt.split("\n") if re.match(r'(error:|\(fail\)|\s*\d+ (pass|fail)|Expected|Received)',l.strip()) or l.startswith('error'))[:3000])
