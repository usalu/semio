import json,re,subprocess,sys,os
R='/Users/ueli/Documents/semio'
files=subprocess.run(['git','ls-files','-z','--','🧰️framework','🚀️launcher'],cwd=R,capture_output=True,text=True).stdout.split('\0')
files=[f for f in files if f.endswith(('.ts','.mjs','.json','.py'))]
def istest(f): return '🧪️tests' in f or '🧫️fixtures' in f or '/.tmp' in f
rows=[]
for b in sys.argv[1:]:
    m=json.load(open(b+'/manifest.json'))
    rows+=[k for k,e in m.items() if e.get('new')]
dirs={os.path.dirname(r) for r in rows}
# test files near changed files: any test file whose path starts with an ancestor module dir (up to 2 levels) of a changed row
def near(f):
    for d in dirs:
        p=d
        for _ in range(1):
            if f.startswith(p+'/') : return True
            p=os.path.dirname(p)
    return False
src=[]
for f in files:
    if istest(f): continue
    try: src.append(open(os.path.join(R,f),encoding='utf8',errors='ignore').read())
    except: pass
corpus='\n'.join(src)
print('corpus',len(corpus)//1000000,'MB',file=sys.stderr)
pat=re.compile(r'''\.(?:toThrow|toMatch|toContain|toThrowError)\(\s*(/(?:[^/\\\n]|\\.)+/[a-z]*|"(?:[^"\\\n]|\\.)*"|'(?:[^'\\\n]|\\.)*'|`(?:[^`\\]|\\.)*`)''')
n=0
for f in files:
    if not istest(f) or not f.endswith(('.ts','.mjs')) or not near(f): continue
    try: t=open(os.path.join(R,f),encoding='utf8').read()
    except: continue
    n+=1
    for m in pat.finditer(t):
        s=m.group(1)
        if s[0]=='/': body=re.sub(r'\\(.)',r'\1',s[1:s.rindex('/')])
        else: body=s[1:-1].replace('\\"','"')
        body=re.split(r'\$\{|\.\*|\\s|\[|\(|\|',body)[0]
        if len(body)<8 or not re.fullmatch(r"[A-Za-z ,:'-]+",body.strip()) or ' ' not in body.strip(): continue
        if body not in corpus: print(f[-75:],'|',repr(body))
print('tests scanned',n,file=sys.stderr)
