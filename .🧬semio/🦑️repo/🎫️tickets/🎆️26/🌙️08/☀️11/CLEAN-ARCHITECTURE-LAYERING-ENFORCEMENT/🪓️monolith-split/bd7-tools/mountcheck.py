import re,sys,os
def dec(t): return re.sub(r'\\u\{([0-9a-fA-F]+)\}',lambda m:chr(int(m.group(1),16)),t)
STR=re.compile(r'"(?:\\.|[^"\\])*"')
def check(root):
    missing=[]
    seen=set()
    def walk(path):
        if path in seen: return
        seen.add(path)
        try: txt=open(path,encoding="utf-8").read()
        except Exception:
            missing.append((path,"unreadable")); return
        d0=os.path.dirname(path)
        depth=0
        stack=[]
        pend=None
        incl=False
        for ln in txt.split("\n"):
            code=ln
            if code.lstrip().startswith("//"): continue
            for m2 in re.finditer(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"',code):
                base=stack[-1][1] if False else d0
                p=os.path.normpath(os.path.join(d0,dec(m2.group(1))))
                if not os.path.exists(p): missing.append((path,p))
            m=re.match(r'\s*#\[path\s*=\s*"([^"]*)"\]\s*',code)
            if m:
                pend=dec(m.group(1)); code=code[m.end():]
                if not code.strip(): 
                    continue
            mm=re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(;|\{)',code)
            body=STR.sub('""',code)
            body=body.split("//")[0]
            if mm:
                name,kind=mm.group(1),mm.group(2)
                base=stack[-1][1] if stack else d0
                if kind==";":
                    if pend is not None: p=os.path.normpath(os.path.join(base,pend))
                    else:
                        p1=os.path.join(base,name+".rs"); p2=os.path.join(base,name,"mod.rs")
                        p=p1 if os.path.exists(p1) else p2
                    if not os.path.exists(p): missing.append((path,p))
                    else: walk(p)
                    pend=None
                else:
                    nd=os.path.normpath(os.path.join(base,pend)) if pend is not None else os.path.join(base,name)
                    stack.append((depth,nd)); pend=None
                    depth+=body.count("{")-body.count("}")
                    while stack and depth<=stack[-1][0]: stack.pop()
                continue
            if pend is not None and not code.strip().startswith("#["): pend=None
            depth+=body.count("{")-body.count("}")
            while stack and depth<=stack[-1][0]: stack.pop()
    walk(root)
    return missing
if __name__=="__main__":
    for m in check(sys.argv[1]): print(m)
