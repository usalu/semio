import re,sys,os
log=sys.argv[1]
s=open(log,errors='replace').read()
edits={}
for m in re.finditer(r'error\[E0308\]: mismatched types\n\s*--> ([^\n]*?):(\d+):(\d+)\n((?:[^\n]*\n){1,14})',s):
    if 'RetainedWorkOutcome' not in m.group(4):continue
    f=os.path.normpath(m.group(1))
    if not os.path.isfile(f) or '🛍️products' not in f:continue
    edits.setdefault(f,set()).add((int(m.group(2)),int(m.group(3))))
tot=0
for f,es in edits.items():
    text=open(f).read();lines=text.split('\n')
    offs=[0]
    for l in lines:offs.append(offs[-1]+len(l)+1)
    ins=[]
    for l,c in es:
        pos=offs[l-1]+len(lines[l-1][:c-1].encode().decode())  # char col
        pos=offs[l-1]+(c-1)
        if not text[pos:pos+3]=='Ok(':continue
        j=pos-1
        while j>0 and text[j].isspace():j-=1
        if text[j]!='{':continue
        if text[max(0,j-7):j]=='.result':continue
        ins.append(j)
    for j in sorted(set(ins),reverse=True):
        text=text[:j]+'.result '+text[j:];tot+=1
    open(f,'w').write(text)
print('arm edits',tot)
# E0609 'no field result on Result' — remove over-applied .result
edits2={}
for m in re.finditer(r'error\[E0609\]: no field `result` on type `Result<[^\n]*\n\s*--> ([^\n]*?):(\d+):(\d+)\n',s):
    f=os.path.normpath(m.group(1))
    if os.path.isfile(f):edits2.setdefault(f,set()).add((int(m.group(2)),int(m.group(3))))
k=0
for f,es in edits2.items():
    L=open(f).read().split('\n')
    for l,c in sorted(es,reverse=True):
        t=L[l-1];i=c-1
        if t[i:i+6]=='result' and t[i-1]=='.':L[l-1]=t[:i-1]+t[i+6:];k+=1
    open(f,'w').write('\n'.join(L))
print('unres',k)
