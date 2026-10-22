import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rsparse import match_brace
n=0
files=[l for l in open(sys.argv[1][1:],encoding='utf-8').read().split('\n') if l]
for f in files:
    s=open(f,encoding='utf-8').read();out=[];pos=0;ch=False
    for m in re.finditer(r'^impl(?:<[^{\n]*>)?\s+(?:[\w]+::)*ColdMutation<[^{\n]*>\s+for[^{\n]*\{',s,re.M):
        ob=m.end()-1;eb=match_brace(s,ob);b=s[ob:eb+1]
        if 'Self::Diff' in b:
            out.append(s[pos:ob]);out.append(b.replace('Self::Diff','Self::ReferenceDiff'));pos=eb+1;ch=True
    out.append(s[pos:])
    if ch: open(f,'w',encoding='utf-8').write(''.join(out));n+=1
print(n)
