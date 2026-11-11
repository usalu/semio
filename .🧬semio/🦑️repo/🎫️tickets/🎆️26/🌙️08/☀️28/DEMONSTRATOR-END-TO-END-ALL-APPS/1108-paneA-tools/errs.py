import sys,re
txt=open(sys.argv[1]).read().split('\n')
flt=sys.argv[2] if len(sys.argv)>2 else ''
i=0
while i<len(txt):
    l=txt[i]
    if l.startswith('error') and not l.startswith('error: could not'):
        msg=l;j=i+1
        while j<len(txt) and not txt[j].lstrip().startswith('--> '):j+=1
        if j>=len(txt):break
        loc=txt[j].strip()[4:]
        m=re.search(r'(.*):(\d+):(\d+)$',loc)
        if flt in loc and m:
            f,ln,col=m.group(1),int(m.group(2)),int(m.group(3))
            print('##',msg[:200]);print('  ',f.split('/')[-2],ln,col)
            k=j+1;extra=[]
            while k<len(txt) and not txt[k].startswith('error') and not txt[k].startswith('warning'):
                t=txt[k].strip()
                if t.startswith('= ') or t.startswith('| ') and '^' in t and len(t)<300: extra.append(t[:300])
                k+=1
            for e in extra[:6]:print('    ',e)
        i=j
    i+=1
