#!/usr/bin/env python3
"""usage: wrapclosures.py <log> <crate-compiling-marker> [--apply]
Wraps the last closure argument of admit_original_construction/close_original_retained_step/close_original_retained_turn calls that contain a rustc error position into a RetainedWorkOutcome{result,work:1} (work-accounting port)."""
import re,sys,os
log,marker=sys.argv[1],sys.argv[2];apply='--apply' in sys.argv
ROOT='/Users/ueli/Documents/semio/'
s=open(log,errors='replace').read();s=s[s.rfind(marker):]
pat=re.compile(r'error(?:\[E\d+\])?: ([^\n]*)\n\s*--> (?:/Users/ueli/Documents/semio/)?(🧰️framework/[^\n]*?):(\d+):(\d+)')
errs={}
for m in pat.finditer(s):
    f=os.path.normpath(ROOT+m.group(2))
    if os.path.isfile(f):errs.setdefault(f,set()).add((int(m.group(3)),int(m.group(4))))
CALLS=('admit_original_construction(','close_original_retained_step(','close_original_retained_turn(')
W='semio_framework_value::RetainedWorkOutcome'
def match_paren(t,i):
    d=0;j=i;n=len(t)
    while j<n:
        c=t[j]
        if c=='"':
            j+=1
            while t[j]!='"':
                if t[j]=='\\':j+=1
                j+=1
        elif c=="'" and j+2<n and (t[j+2]=="'" or (t[j+1]=='\\' and t[j+3]=="'")):
            j+=3 if t[j+2]=="'" else 4;continue
        elif c in '([{':d+=1
        elif c in ')]}':
            d-=1
            if d==0:return j
        j+=1
    return -1
total=0
for f,es in errs.items():
    t=open(f).read();lines=t.split('\n');starts=[0]
    for l in lines:starts.append(starts[-1]+len(l)+1)
    offs=[starts[l-1]+c-1 for l,c in es if l-1<len(lines)]
    edits=[]
    for call in CALLS:
        for m in re.finditer(re.escape(call),t):
            op=m.end()-1;cl=match_paren(t,op)
            if cl<0:continue
            d=0;j=op;last=None;n=cl
            k=op+1
            while k<cl:
                c=t[k]
                if c=='"':
                    k+=1
                    while t[k]!='"':
                        if t[k]=='\\':k+=1
                        k+=1
                elif c=="'" and k+2<cl and (t[k+2]=="'" or (t[k+1]=='\\' and t[k+3]=="'")):
                    k+=3 if t[k+2]=="'" else 4;continue
                elif c in '([{':d+=1
                elif c in ')]}':d-=1
                elif c==',' and d==0:last=k
                k+=1
            if last is None:continue
            seg=t[last+1:cl];lead=len(seg)-len(seg.lstrip());cs=last+1+lead
            if t[cs]!='|':continue
            p2=t.index('|',cs+1);body_s=p2+1;body_e=cl
            body=t[body_s:body_e]
            if body.startswith('{let result=(||') or 'RetainedWorkOutcome' in body[:60]:continue
            if not any(body_s<=o<body_e for o in offs):continue
            edits.append((body_s,body_e,'{let result=(||%s)();%s{result,work:1}}'%(body,W)))
    edits.sort(reverse=True)
    last_s=None
    for a,b,new in edits:
        if last_s is not None and b>last_s:continue
        t=t[:a]+new+t[b:];last_s=a;total+=1
    if apply and edits:open(f,'w').write(t)
    print(f.split('instance/')[-1][:80],len(edits))
print('applied' if apply else 'dry',total)
