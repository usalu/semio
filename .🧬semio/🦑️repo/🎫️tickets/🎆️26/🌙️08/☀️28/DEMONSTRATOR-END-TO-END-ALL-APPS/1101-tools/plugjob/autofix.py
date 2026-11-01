#!/usr/bin/env python3
"""usage: autofix.py <log> <path-substring> [--apply]  -- inserts `.result` for the RetainedWorkOutcome migration where rustc says `?`/map/map_err hit an outcome."""
import re,sys,os
log,sub=sys.argv[1],sys.argv[2];apply='--apply' in sys.argv
ROOT='/Users/ueli/Documents/semio'
s=open(log,errors='replace').read()
edits={}
pat=re.compile(r'error(?:\[E\d+\])?: ([^\n]*)\n\s*--> (?:/Users/ueli/Documents/semio/)?(🧰️framework/[^\n]*?):(\d+):(\d+)\n((?:[^\n]*\n){1,8})')
for m in pat.finditer(s):
    msg,path,line,col,rest=m.group(1),m.group(2),int(m.group(3)),int(m.group(4)),m.group(5)
    if sub not in path:continue
    f=os.path.normpath(os.path.join(ROOT,path))
    if not os.path.isfile(f):continue
    kind=None
    if 'operator can only be applied to values that implement `Try`' in msg and 'RetainedWorkOutcome' in rest:kind='try'
    elif 'is not an iterator' in msg and 'RetainedWorkOutcome' in msg:kind='dot'
    elif 'no method named' in msg and 'RetainedWorkOutcome' in msg and ('map_err' in msg or 'unwrap_or_else' in msg or 'expect' in msg or 'unwrap' in msg or 'ok_or_else' in msg or 'map' in msg or 'is_ok' in msg or 'is_err' in msg):kind='dot'
    if not kind:continue
    edits.setdefault(f,set()).add((line,col,kind))
tot=0
for f,es in edits.items():
    lines=open(f).read().split('\n')
    for line,col,kind in sorted(es,key=lambda e:(-e[0],-e[1])):
        t=lines[line-1];i=col-1
        if kind=='dot':
            # caret under method name: insert before the preceding dot
            j=i
            while j>0 and t[j-1]!='.':j-=1
            if j==0:print('skip',f[-50:],line,col);continue
            pos=j-1
        else:
            d=0;j=i;pos=None
            while j<len(t):
                c=t[j]
                if c in '([{':d+=1
                elif c in ')]}':
                    d-=1
                    if d<0:break
                elif c=='?' and d==0:pos=j;break
                j+=1
            if pos is None:print('skip?',f[-50:],line,col);continue
        if t[pos:pos+7]=='.result':continue
        lines[line-1]=t[:pos]+'.result'+t[pos:];tot+=1
    if apply:open(f,'w').write('\n'.join(lines))
print(('applied' if apply else 'dry'),tot,'edits in',len(edits),'files')
