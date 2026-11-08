import re,sys,os,collections
from lex import lex_pairs
ROOT='/Users/ueli/Documents/semio/🧰️framework'
log=sys.argv[1];dry=len(sys.argv)>2 and sys.argv[2]=='dry'
t=open(log,encoding='utf8',errors='replace').read()
blocks=re.split(r'\n(?=error)',t)
sites=collections.defaultdict(list)
for b in blocks:
    if not b.startswith('error'):continue
    m=re.search(r'-->\s*(\S+?):(\d+):(\d+)',b)
    if not m:continue
    path=os.path.normpath(os.path.join(ROOT,m.group(1)));ln=int(m.group(2));col=int(m.group(3))
    u=re.search(r'\n\s*\|\s*(\s*)(\^+)',b);ulen=len(u.group(2)) if u else 0
    if b.startswith('error[E0308]: mismatched') and 'expected `&mut RetainedAllocationEffect' in b and 'found `RetainedCloneGrant`' in b:
        sites[path].append(['arg',ln,col,ulen])
    elif b.startswith('error[E0061]: this method takes 2 arguments but 1 argument was supplied') and 'RetainedAllocationEffect' in b:
        sites[path].append(['missing',ln,col,ulen])
tot=0;skipped=[]
def chain_start(s,k,match):
    p=k
    while True:
        if p>=2 and s[p-1]==':' and s[p-2]==':':p-=2
        elif p>=1 and s[p-1]=='.':p-=1
        else:break
        while p>0 and s[p-1] in ')]':
            p=match.get(p-1,p-1)
        q=p
        while q>0 and (s[q-1].isalnum() or s[q-1]=='_'):q-=1
        if q==p:break
        p=q
    return p
for path,lst in sites.items():
    s=open(path,encoding='utf8').read()
    starts=[0]
    for i,ch in enumerate(s):
        if ch=='\n':starts.append(i+1)
    offs=[[kind,starts[ln-1]+col-1,ulen] for kind,ln,col,ulen in lst]
    offs.sort(key=lambda x:-x[1])
    while offs:
        kind,pos,ulen=offs.pop(0)
        match=lex_pairs(s)
        if kind=='arg':
            e=pos
            while e<len(s) and s[e] not in ',)':
                e=match[e]+1 if (s[e] in '([{' and e in match) else e+1
            ulen=e-pos
            argtxt=s[pos:pos+ulen];j=pos-1;o=None
            while j>=0:
                ch=s[j]
                if ch in ')]}':
                    if j in match:j=match[j]-1;continue
                elif ch in '([{':o=j;break
                j-=1
            if o is None or s[o]!='(' or o not in match:skipped.append((path[-60:],pos,'noparen'));continue
            c=match[o];g=argtxt;rel=pos-(o+1);inner=s[o+1:c];newinner=inner[:rel]+'e'+inner[rel+ulen:]
        else:
            o=pos+ulen
            if s[o]!='(' or o not in match:skipped.append((path[-60:],pos,'noopen'));continue
            c=match[o];inner=s[o+1:c];k=0
            while k<len(inner):
                ch=inner[k]
                if ch in '([{' and (o+1+k) in match:k=match[o+1+k]-(o+1)
                elif ch==',':break
                k+=1
            g=inner[:k].strip();newinner=inner+',e'
        k=o
        while k>0 and (s[k-1].isalnum() or s[k-1]=='_'):k-=1
        start=chain_start(s,k,match)
        new='crate::fx('+g+',|e|'+s[start:o+1]+newinner+')'+')'
        if dry:print(path[-40:],'\n  OLD:',s[start:c+1][:200],'\n  NEW:',new[:240]);tot+=1;continue
        pre=len('crate::fx('+g+',|e|')
        s=s[:start]+new+s[c+1:]
        for x in offs:
            if x[1]>=start:x[1]+=pre
        tot+=1
    if not dry:open(path,'w',encoding='utf8').write(s)
print('rewritten',tot,'skipped',len(skipped))
for x in skipped:print(x)
