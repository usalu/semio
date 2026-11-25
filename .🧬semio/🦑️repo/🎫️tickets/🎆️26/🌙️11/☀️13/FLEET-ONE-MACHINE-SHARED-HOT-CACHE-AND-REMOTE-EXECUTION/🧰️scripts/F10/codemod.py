import sys,re
from scanlib import *
def plan(t):
    edits=[];manual=[]
    for line,s,k,e in violations(t):
        a=split_args(t,k,e)
        objs=[x for x in a if t[x[0]:x[1]].lstrip().startswith('{')]
        isbun=t[s:k].startswith('Bun.')
        if isbun:
            cand=[x for x in objs if x is not a[0]]
        else:
            cand=objs
        if len(a)==1 and not isbun: pass
        if cand:
            x=cand[-1];pos=t.index('{',x[0])
            inner=t[pos+1:x[1]].strip()
            ins='windowsHide:true,' if inner and not inner.startswith('}') and inner!='' else 'windowsHide:true'
            if inner=='' : ins='windowsHide:true'
            edits.append((pos+1,ins,line));continue
        last=a[-1];txt=t[last[0]:last[1]].strip()
        isfn='=>' in txt or txt.startswith('function')
        if isfn and len(a)>=2:
            pos=last[0];edits.append((pos,' {windowsHide:true},',line));continue
        if len(a)>=2 and re.fullmatch(r'\[.*\]',t[a[1][0]:a[1][1]].strip(),re.S) or (len(a)==2 and isbun is False and t[a[0][0]:a[0][1]].strip()!=''):
            if not isfn and not any(re.search(r'opt|spec|init|config',t[x[0]:x[1]]) for x in a[2:]) and len(a)<=2:
                edits.append((a[-1][1],',{windowsHide:true}',line));continue
        manual.append(line)
    return edits,manual
if __name__=="__main__":
    mode=sys.argv[1]
    for p in sys.argv[2:]:
        t=open(p,encoding='utf8').read()
        edits,manual=plan(t)
        print(p.split('🔨️modules/')[-1][-70:],len(edits),'edits; manual',manual)
        if mode=='apply' and edits:
            for pos,ins,line in sorted(edits,reverse=True):t=t[:pos]+ins+t[pos:]
            open(p,'w',encoding='utf8').write(t)
