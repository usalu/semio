import sys,re,os,json,time
# usage: repath.py <tag> <prefix> <dir> <name=newmodulepath>... : rewrites `<prefix>::NAME` (single use, grouped use, inline) to `<newmodulepath>::NAME`
tag,prefix,root=sys.argv[1:4]; maps=dict(a.split('=',1) for a in sys.argv[4:])
alts='|'.join(sorted(map(re.escape,maps),key=len,reverse=True)); pe=re.escape(prefix)
undo={};n=0
def grp(m):
    body=m.group(1); entries=[];d=0;cur=''
    for ch in body:
        if ch in '{(': d+=1
        if ch in '})': d-=1
        if ch==',' and d==0: entries.append(cur.strip());cur=''
        else: cur+=ch
    if cur.strip(): entries.append(cur.strip())
    stay=[];move={}
    for e in entries:
        mm=re.match(r'^(\w+)(\s+as\s+\w+)?$',e)
        if mm and mm.group(1) in maps: move.setdefault(maps[mm.group(1)],[]).append(e)
        else: stay.append(e)
    if not move: return m.group(0)
    out=[]
    if stay: out.append(f'use {prefix}::{{{", ".join(stay)}}};')
    for mod,es in move.items(): out.append(f'use {mod}::{{{", ".join(es)}}};')
    return '\n'.join(out)
for dp,dn,fn in os.walk(root):
    for f in fn:
        if not f.endswith('.rs'): continue
        p=os.path.join(dp,f); t=open(p,encoding='utf8').read(); o=t
        t=re.sub(r'use '+pe+r'::\{([^;]*?)\};',grp,t,flags=re.S)
        t=re.sub(r'use '+pe+r'::('+alts+r')((?:\s+as\s+\w+)?);',lambda m: f'use {maps[m.group(1)]}::{m.group(1)}{m.group(2)};',t)
        t=re.sub(r'(?<![\w:])'+pe+r'::('+alts+r')\b',lambda m: f'{maps[m.group(1)]}::{m.group(1)}',t)
        if t!=o: undo[p]=o; open(p,'w',encoding='utf8').write(t); n+=1
json.dump(undo,open(f'/Users/ueli/.cache/r8a/undo-{tag}-{int(time.time())}.json','w')); print('repathed',n)
