import sys,re
txt=open(sys.argv[1]).read().split('\n')
flt=sys.argv[2];w=int(sys.argv[3]) if len(sys.argv)>3 else 300
i=0
while i<len(txt):
    if txt[i].startswith('error') and not txt[i].startswith('error: could not'):
        j=i+1
        while j<len(txt) and not txt[j].lstrip().startswith('--> '):j+=1
        k=j+1
        while k<len(txt) and not txt[k].startswith('error') and not txt[k].startswith('warning'):k+=1
        if flt in txt[j]:
            for l in txt[i:k]:
                if l.strip()=='|' or not l.strip():continue
                m=re.match(r'^(\s*\d+ \|)(.*)',l)
                print(l[:w] if not m else m.group(1)+m.group(2)[:w])
            print()
        i=k;continue
    i+=1
