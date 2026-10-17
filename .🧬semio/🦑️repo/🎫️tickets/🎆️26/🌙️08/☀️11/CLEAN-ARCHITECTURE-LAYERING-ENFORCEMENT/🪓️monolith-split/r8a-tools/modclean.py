import os,re,sys,json,time
# usage: modclean.py <tag> <dir>: drops `use crate::...::<module>;` lines whose module name is never used as a path root in the file
pat=re.compile(r'^use (crate::[\w:]+)::([a-z_][a-z0-9_]*)( as (\w+))?;[ \t]*\n',re.M)
undo={};n=0
for dp,dn,fn in os.walk(sys.argv[2]):
    if '🧪️tests' in dp: continue
    for f in fn:
        if not f.endswith('.rs'): continue
        p=os.path.join(dp,f);t=open(p,encoding='utf8').read()
        def fix(m):
            name=m.group(4) or m.group(2)
            body=t.replace(m.group(0),'')
            return m.group(0) if re.search(r'(?<![\w:])'+name+r'::',body) else ''
        u=pat.sub(fix,t)
        if u!=t: undo[p]=t;open(p,'w',encoding='utf8').write(u);n+=1
json.dump(undo,open(f'/Users/ueli/.cache/r8a/undo-{sys.argv[1]}-{int(time.time())}.json','w'));print('cleaned',n)
