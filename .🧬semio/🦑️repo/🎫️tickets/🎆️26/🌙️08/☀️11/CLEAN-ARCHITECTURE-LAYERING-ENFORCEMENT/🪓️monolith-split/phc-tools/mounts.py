import re, os, sys, json
REPO="/Users/ueli/Documents/semio/"
H=REPO+"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
LIBROOT=H+"📦️packages/🦀️rust/🦀️.rs"
PATH_RE=re.compile(r'((?:\s*#\[[^\]]*\]\s*)*?)\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', re.M)
def mounts(path):
    lines=open(path,encoding="utf-8").read().split("\n")
    out=[]
    attrs=[]
    for ln in lines:
        t=ln.strip()
        if t.startswith("#["):
            attrs.append(t); continue
        if t.startswith("//") or t=="" :
            continue
        m=re.match(r'(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',t)
        if m:
            pm=None
            for a in attrs:
                x=re.match(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]',a)
                if x: pm=x.group(1)
            cfg=[a for a in attrs if a.startswith("#[cfg")]
            tgt=os.path.normpath(os.path.join(os.path.dirname(path),pm)) if pm else None
            out.append((m.group(1),tgt,cfg))
        attrs=[]
    return out
def walk(root):
    seen={}
    def go(p,parent,name,cfg,depth):
        if p in seen:
            seen[p]['parents'].append((parent,name,cfg)); return
        seen[p]={'parents':[(parent,name,cfg)],'children':[]}
        if not os.path.exists(p): return
        for (n,t,c) in mounts(p):
            if t is None: continue
            seen[p]['children'].append((n,t,c))
            go(t,p,n,c,depth+1)
    go(root,None,'<root>',[],0)
    return seen
if __name__=="__main__":
    seen=walk(LIBROOT)
    def rel(p): return p.replace(H,'H/')
    def show(p,ind=0,done=set()):
        s=seen[p]
        print('  '*ind+rel(p), os.path.getsize(p) if os.path.exists(p) else 'MISSING')
        for n,t,c in s['children']:
            print('  '*(ind+1)+f'mod {n} cfg={c}')
            if t in done: print('  '*(ind+2)+'(dup) '+rel(t)); continue
            done.add(t); show(t,ind+2,done)
    show(LIBROOT)
