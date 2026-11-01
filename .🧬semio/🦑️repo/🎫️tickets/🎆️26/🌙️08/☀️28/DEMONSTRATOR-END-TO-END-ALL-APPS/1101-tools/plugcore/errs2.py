import re,sys,os
s=open(sys.argv[1],errors='replace').read()
flt=sys.argv[2]
seen=set()
for b in re.split(r'\n(?=(?:error|warning)[\[:])',s):
    if not b.startswith('error'):continue
    m=re.search(r'--> ([^\n]*?):(\d+):(\d+)\n',b)
    if not m or flt not in m.group(1):continue
    f=os.path.normpath(m.group(1));l=int(m.group(2));c=int(m.group(3))
    k=(f,l,c)
    if k in seen:continue
    seen.add(k)
    try:t=open(f).read().split('\n')[l-1][max(c-1,0):c+int(sys.argv[3]) if len(sys.argv)>3 else c+160]
    except Exception:t=''
    notes=[x.strip() for x in b.split('\n') if re.match(r'\s*= note: (expected|found)|\s*(expected|found) |\s*= help',x)]
    print(f"{f.split('/')[-3]}/{os.path.basename(os.path.dirname(f))} {l}:{c} {b.split(chr(10))[0][:110]}\n   SRC: {t}\n   "+" || ".join(notes[:3])[:300])
