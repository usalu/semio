import json,os,re,sys
F=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
G=F+"/🗑️generated"; R="/Users/ueli/Documents/semio"
rows=json.load(open(G+"/hdr-near.json"))
out={}
for r in rows:
    if r['own']: continue
    mp=R+"/"+r['manifest']
    mt=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',open(mp).read())
    lp=os.path.normpath(os.path.join(os.path.dirname(mp),mt.group(1) if mt else 'src/lib.rs'))
    try: txt=open(lp).read()
    except Exception as e: out[r['name']]=('missing',lp); continue
    lines=[l for l in txt.split("\n")]
    mounts=0;other=0
    for l in lines:
        s=l.strip()
        if not s or s.startswith("//") or s.startswith("#![") or s.startswith("extern crate"): continue
        if re.match(r'#\[path\s*=',s) or re.match(r'(pub )?mod \w+;',s): mounts+= 1 if s.startswith(('pub mod','mod')) else 0; continue
        other+=1
    out[r['name']]=('family' if other==0 and mounts>=6 else 'mixed',mounts,other,len(lines),lp)
    print(f"{r['m']:.3f} {r['name'].replace('semio-s-artifact-','A:').replace('semio-framework-','F:')} {out[r['name']][0]} mounts={mounts} other={other} lines={len(lines)}")
json.dump(out,open(G+"/hdr-rootkind.json","w"),ensure_ascii=False)
