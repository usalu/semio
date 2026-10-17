import sys,re,json,time,os
# usage: moveitem.py <src> <dst> <name>... : moves top-level items (fn/struct/enum/const/static/type/trait + their impl blocks) with docs/attributes; undo json in ~/.cache/r8a
src,dst=sys.argv[1],sys.argv[2]; names=sys.argv[3:]
s=open(src,encoding='utf8').read(); d=open(dst,encoding='utf8').read(); undo={src:s,dst:d}
sys.path.insert(0,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r8a-tools')
import rsbrace
def extent(t,start):
    return rsbrace.item_end(t,start)
def head(t,i):
    ls=t.rfind('\n',0,i)+1
    while True:
        pl=t.rfind('\n',0,ls-1)+1
        prev=t[pl:ls-1] if ls>0 else ''
        if prev.startswith('///') or prev.startswith('#[') or prev.startswith('//!')==False and prev.strip().startswith('#[') : 
            ls=pl
            if ls==0: break
        else: break
    return ls
moved=[]
for name in names:
    pat=re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:fn|struct|enum|const|static|type|trait)\s+'+re.escape(name)+r'\b',re.M)
    ms=list(pat.finditer(s)); assert len(ms)==1,(name,len(ms))
    i=ms[0].start(); a=head(s,i); b=extent(s,i)
    moved.append(s[a:b]); s=s[:a]+s[b:]
    # impl blocks of the type
    for im in list(re.finditer(r'^impl(?:<[^>]*>)?\s+(?:[\w:<>]+\s+for\s+)?'+re.escape(name)+r'\b[^{]*\{',s,re.M))[::-1]:
        a=head(s,im.start()); b=extent(s,im.start()); moved.append(s[a:b]); s=s[:a]+s[b:]
d=d.rstrip('\n')+'\n\n'+'\n'.join(moved)
open(src,'w',encoding='utf8').write(s); open(dst,'w',encoding='utf8').write(d)
json.dump(undo,open(f'/Users/ueli/.cache/r8a/undo-move-{int(time.time())}.json','w'))
print('moved',len(moved),'blocks')
