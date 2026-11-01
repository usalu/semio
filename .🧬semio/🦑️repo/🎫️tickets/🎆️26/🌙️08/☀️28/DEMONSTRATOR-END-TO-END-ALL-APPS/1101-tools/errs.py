import re,sys,os
s=open(sys.argv[1],errors='replace').read()
seen=set()
for m in re.finditer(r'(error(?:\[E\d+\])?: [^\n]*)\n\s*--> ([^\n]*?):(\d+):(\d+)\n',s):
    if 'could not' in m.group(1):continue
    f=os.path.normpath(m.group(2));l=int(m.group(3));c=int(m.group(4))
    k=(f,l,c,m.group(1))
    if k in seen:continue
    seen.add(k)
    try:t=open(f).read().split('\n')[l-1][max(c-1,0):c+80]
    except Exception:t=''
    print(f.split('plugin/')[-1][-58:],l,c,m.group(1)[:90],'|',t)
