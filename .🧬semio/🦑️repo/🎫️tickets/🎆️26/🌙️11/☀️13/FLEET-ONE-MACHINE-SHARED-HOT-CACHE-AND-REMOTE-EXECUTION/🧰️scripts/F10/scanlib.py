import os,re,sys,json
FUNCS=("spawn","spawnSync","execFile","execFileSync","exec","execSync","fork")
POSIX=set("ps sysctl mkfifo hdiutil plutil vm_stat xcrun codesign sample lsof launchctl systemctl journalctl kill sh mount df du cp strace sandbox-exec sudo getent diskutil sw_vers uname chmod chown ln rm mv id stat tar nice renice ionice taskpolicy security open osascript pmset dscl hostname which".split())
SKIP_DIRS={"node_modules",".git","target","dist","🧪️tests","🗑️generated"}
def imports(t):
    names={};spaces=set();wild=False
    for m in re.finditer(r'import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*["\'](?:node:)?child_process["\']',t):
        for part in m.group(1).split(','):
            part=part.strip()
            if not part or part.startswith('type '):continue
            a=re.split(r'\s+as\s+',part);names[a[-1].strip()]=a[0].strip()
    for m in re.finditer(r'import\s+\*\s+as\s+(\w+)\s+from\s*["\'](?:node:)?child_process["\']',t):spaces.add(m.group(1))
    for m in re.finditer(r'import\s+(\w+)\s+from\s*["\'](?:node:)?child_process["\']',t):spaces.add(m.group(1))
    for m in re.finditer(r'(?:const|let|var)\s*\{([^}]*)\}\s*=\s*(?:await\s+import|require)\(\s*["\'](?:node:)?child_process["\']\s*\)',t):
        for part in m.group(1).split(','):
            part=part.strip()
            if not part:continue
            a=re.split(r'\s*:\s*',part);names[a[-1].strip()]=a[0].strip()
    for m in re.finditer(r'(?:const|let|var)\s+(\w+)\s*=\s*(?:await\s+import|require)\(\s*["\'](?:node:)?child_process["\']\s*\)',t):spaces.add(m.group(1))
    return names,spaces
def match_paren(t,i):
    d=0;q=None;j=i;n=len(t)
    while j<n:
        c=t[j]
        if q:
            if c=='\\':j+=1
            elif q=='`' and c=='$' and t[j+1:j+2]=='{':
                k=match_brace(t,j+1);j=k
            elif c==q:q=None
        elif c in '"\'`':q=c
        elif c=='/' and t[j+1:j+2]=='/':
            j=t.find('\n',j)
            if j<0:return -1
        elif c=='/' and t[j+1:j+2]=='*':
            j=t.find('*/',j)+1
        elif c in '([{':d+=1
        elif c in ')]}':
            d-=1
            if d==0:return j
        j+=1
    return -1
def match_brace(t,i):return match_paren(t,i)
def split_args(t,start,end):
    args=[];d=0;q=None;j=start+1;last=j
    while j<end:
        c=t[j]
        if q:
            if c=='\\':j+=1
            elif q=='`' and c=='$' and t[j+1:j+2]=='{':j=match_brace(t,j+1)
            elif c==q:q=None
        elif c in '"\'`':q=c
        elif c in '([{':d+=1
        elif c in ')]}':d-=1
        elif c==',' and d==0:
            args.append((last,j));last=j+1
        j+=1
    if t[last:end].strip():args.append((last,end))
    return args
def calls(t):
    names,spaces=imports(t)
    out=[]
    pats=[]
    if names:pats.append(r'(?<![\w.$])('+'|'.join(map(re.escape,names))+r')\s*(?:<[^()]*>)?\s*\(')
    if spaces:pats.append(r'(?<![\w$])(?:'+'|'.join(map(re.escape,spaces))+r')\.(?:'+'|'.join(FUNCS)+r')\s*\(')
    pats.append(r'(?<![\w$])Bun\.(?:spawn|spawnSync)\s*\(')
    pats.append(r'import\(\s*["\'](?:node:)?child_process["\']\s*\)\s*\)?\.(?:'+'|'.join(FUNCS)+r')\s*\(')
    seen=set()
    for p in pats:
        for m in re.finditer(p,t):
            k=m.end()-1
            if k in seen:continue
            seen.add(k)
            ls=t.rfind('\n',0,m.start())+1
            if t[ls:m.start()].lstrip().startswith(('//','*','/*')):continue
            e=match_paren(t,k)
            if e<0:continue
            fa=split_args(t,k,e)
            if fa and re.match(r'\s*\w+\??\s*:\s*[A-Za-z]',t[fa[0][0]:fa[0][1]]):continue
            out.append((m.start(),k,e))
    return sorted(out)
def first_literal(t,k,e):
    a=split_args(t,k,e)
    if not a:return None
    s=t[a[0][0]:a[0][1]].strip()
    m=re.fullmatch(r'"([^"]*)"|\'([^\']*)\'',s)
    if m:return m.group(1) or m.group(2)
    return None
def posix_only(lit):
    if lit is None:return False
    return lit.startswith('/') or lit in POSIX
def violations(t):
    res=[]
    for s,k,e in calls(t):
        if 'windowsHide' in t[k:e+1]:continue
        lit=first_literal(t,k,e)
        if posix_only(lit):continue
        res.append((t.count('\n',0,s)+1,s,k,e))
    return res
def walk(roots):
    for root in roots:
        for d,ds,fs in os.walk(root):
            ds[:]=[x for x in sorted(ds) if x not in SKIP_DIRS]
            for f in sorted(fs):
                if f.endswith(('.ts','.mjs','.cjs')):yield os.path.join(d,f)
if __name__=="__main__":
    base=sys.argv[1];out=[]
    for p in walk(sys.argv[2:]):
        t=open(p,encoding='utf8').read()
        for line,*_ in violations(t):out.append(os.path.relpath(p,base)+":"+str(line))
    print(json.dumps(out,ensure_ascii=False,indent=0))
