import sys,os,re,json,time
# usage: repoint.py <tag> <dir> <oldPath> <newPath> name... : repoints `oldPath::NAME` imports and qualified uses to `newPath::NAME` under <dir>
tag,d,old,new=sys.argv[1:5]; names=set(sys.argv[5:])
esc=re.escape(old); alts='|'.join(re.escape(n) for n in names)
undo={}; n=0
def split_entries(body):
    entries=[];depth=0;cur=''
    for ch in body:
        if ch=='{':depth+=1
        if ch=='}':depth-=1
        if ch==',' and depth==0: entries.append(cur.strip());cur=''
        else: cur+=ch
    if cur.strip(): entries.append(cur.strip())
    return entries
for dp,dn,fn in os.walk(d):
    dn[:]=[x for x in dn if x not in('node_modules','target','🤖️generated')]
    for f in fn:
        if f!='🦀️.rs': continue
        p=os.path.join(dp,f); t=open(p,encoding='utf8').read(); o=t
        def brace(m):
            pre=m.group(1); body=m.group(2); post=m.group(3)
            keep=[];move=[]
            for e in split_entries(body):
                mm=re.match(r'^([A-Za-z_]\w*)(\s+as\s+\w+)?$',e)
                if mm and mm.group(1) in names: move.append(e)
                else: keep.append(e)
            if not move: return m.group(0)
            out=[]
            if keep: out.append('%suse %s::{%s};'%(pre,old,', '.join(keep)))
            out.append('%suse %s::{%s};'%(pre if not keep else '',new,', '.join(move)))
            return '\n'.join(out)
        t=re.sub(r'((?:^|\n)[ \t]*(?:#\[[^\n]*\]\s*)?)use\s+'+esc+r'::\{([^;]*?)\};()',lambda m:brace(m) if False else m.group(0),t) if False else t
        # brace imports
        def brace2(m):
            body=m.group(1); keep=[];move=[]
            for e in split_entries(body):
                mm=re.match(r'^([A-Za-z_]\w*)(\s+as\s+\w+)?$',e)
                if mm and mm.group(1) in names: move.append(e)
                else: keep.append(e)
            if not move: return m.group(0)
            parts=[]
            if keep: parts.append('use %s::{%s};'%(old,', '.join(keep)))
            parts.append('use %s::{%s};'%(new,', '.join(move)))
            return '\n'.join(parts)
        t=re.sub(r'use\s+'+esc+r'::\{([^;]*?)\};',brace2,t)
        t=re.sub(r'use\s+'+esc+r'::('+alts+r')((?:\s+as\s+\w+)?);',lambda m:'use %s::%s%s;'%(new,m.group(1),m.group(2)),t)
        t=re.sub(r'(?<![\w:])'+esc+r'::('+alts+r')\b',lambda m:'%s::%s'%(new,m.group(1)),t)
        if t!=o: undo[p]=o; open(p,'w',encoding='utf8').write(t); n+=1
json.dump(undo,open(f'/Users/ueli/.cache/r8a/undo-{tag}-{int(time.time())}.json','w'))
print('repointed files',n)
