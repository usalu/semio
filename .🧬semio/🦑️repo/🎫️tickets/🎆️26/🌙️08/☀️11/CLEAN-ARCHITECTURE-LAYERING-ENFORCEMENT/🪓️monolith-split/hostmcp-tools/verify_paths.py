import sys,os,re,subprocess
from rlex import lex
files=sys.argv[1:]
bad=0;total=0
for f in files:
    t=open(f,encoding='utf-8').read()
    d=os.path.dirname(f)
    for m in re.finditer(r'(#\[path\s*=\s*"([^"]+)"\])|(include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\))',t):
        lit=m.group(2) or m.group(4)
        total+=1
        p=os.path.normpath(os.path.join(d,lit))
        if not os.path.exists(p):
            bad+=1;print("MISSING",f[-70:],lit)
        if len(p.encode())>240:print("LONG",len(p.encode()),p)
    if len(f.encode())>240:print("LONGFILE",len(f.encode()),f)
    L=[l+'\n' for l in t.split('\n')]
    I=lex(L)
    if I[-1]['end_depth']!=0 and t:
        print("BRACE-IMBALANCE",f)
        bad+=1
    reg=0
    for l in t.split('\n'):
        s=l.strip()
        if re.match(r'//\s*#region\b',s):reg+=1
        if re.match(r'//\s*#endregion\b',s):reg-=1
    if reg!=0:print("REGION-IMBALANCE",reg,f[-80:]);
    r=subprocess.run(["rustfmt","--edition","2021","--emit","stdout"],input=t,capture_output=True,text=True)
    if r.returncode!=0:
        bad+=1;print("RUSTFMT-FAIL",f[-60:],r.stderr[:300])
    act=sum(1 for l in t.split('\n') if l.strip() and not l.strip().startswith('//'))
    print(f"{act:5d} active {len(t.encode()):7d} bytes {t.count(chr(10)):5d} lines  {os.path.relpath(f,os.path.commonpath(files))}")
print("checked refs",total,"bad",bad)
