import sys,re,os,time,subprocess
TAG="[DEBUG]"
def lex_ts(t):
    n=len(t);code=bytearray(n);i=0;stack=[]
    def skip_str(i,q):
        i+=1
        while i<n:
            c=t[i]
            if c=="\\":i+=2;continue
            if c==q:return i+1
            i+=1
        return i
    def scan_tpl(i):
        while i<n:
            c=t[i]
            if c=="\\":i+=2;continue
            if c=="`":return i+1
            if c=="$" and i+1<n and t[i+1]=="{":stack.append(0);return i+2
            i+=1
        return i
    while i<n:
        c=t[i]
        if c=="/" and i+1<n and t[i+1]=="/":
            while i<n and t[i]!="\n":i+=1
            continue
        if c=="/" and i+1<n and t[i+1]=="*":
            j=t.find("*/",i+2);i=n if j<0 else j+2;continue
        if c in "'\"":i=skip_str(i,c);continue
        if c=="`":i=scan_tpl(i+1);continue
        if c=="{":
            if stack:stack[-1]+=1
            code[i]=1;i+=1;continue
        if c=="}":
            if stack:
                if stack[-1]==0:
                    stack.pop();i=scan_tpl(i+1);continue
                stack[-1]-=1
            code[i]=1;i+=1;continue
        code[i]=1;i+=1
    return code
def match_paren(t,i,rust):
    n=len(t);depth=0;j=i
    while j<n:
        c=t[j]
        if c in "'\"" or (c=="`" and not rust):
            if rust and c=="'":
                m=re.match(r"'(?:\\.|[^\\'])'",t[j:j+6])
                if m:j+=m.end();continue
                j+=1;continue
            q=c;j+=1
            while j<n and t[j]!=q:
                if t[j]=="\\":j+=1
                elif q=="`" and t[j]=="$" and t[j+1]=="{":
                    k=match_brace(t,j+1);j=k
                j+=1
            j+=1;continue
        if c=="(":depth+=1
        elif c==")":
            depth-=1
            if depth==0:return j
        j+=1
    return -1
def match_brace(t,i):
    n=len(t);d=0;j=i
    while j<n:
        c=t[j]
        if c in "'\"":
            q=c;j+=1
            while j<n and t[j]!=q:
                if t[j]=="\\":j+=1
                j+=1
        elif c=="`":
            j+=1
            while j<n and t[j]!="`":
                if t[j]=="\\":j+=1
                elif t[j]=="$" and t[j+1]=="{":j=match_brace(t,j+1)
                j+=1
        elif c=="{":d+=1
        elif c=="}":
            d-=1
            if d==0:return j
        j+=1
    return n
CALL_TS=re.compile(r"\b(?:console\s*\.\s*(?:log|debug|error|warn|info)|process\s*\.\s*(?:stderr|stdout)\s*\.\s*write)\s*\(")
CALL_RS=re.compile(r"\be?println!\s*\(")
def process(path,apply):
    t=open(path,encoding="utf8").read();rust=path.endswith(".rs")
    code=None if rust else lex_ts(t)
    rx=CALL_RS if rust else CALL_TS
    cuts=[];manual=[]
    for m in rx.finditer(t):
        s=m.start()
        masked=code is not None and not code[s]
        o=m.end()-1;e=match_paren(t,o,rust)
        if e<0:continue
        arg=t[o+1:e].lstrip()
        if not (arg[:1] in "\"'`" and arg[1:1+len(TAG)]==TAG):continue
        if masked:
            ls=t.rfind(chr(10),0,s)+1;le=t.find(chr(10),e+1);le=len(t) if le<0 else le
            tail=t[e+1:le].strip()
            if t[ls:s].strip()!="" or tail not in ("",";"):
                manual.append((t.count(chr(10),0,s)+1,'masked',t[s:s+120]));continue
            pk=ls-1
            while pk>=0 and t[pk] in " \t\r\n":pk-=1
            if pk>=0 and t[pk] in ")>e:,(":
                manual.append((t.count(chr(10),0,s)+1,'masked-prev',t[s:s+120]));continue
        k=s-1
        while k>=0 and t[k] in " \t\r\n":k-=1
        prev=t[k] if k>=0 else ";"
        end=e+1
        while end<len(t) and t[end] in " \t":end+=1
        semi=end<len(t) and t[end]==";"
        if semi:end+=1
        nxt=t[end] if end<len(t) else "\n"
        line=t.count("\n",0,s)+1
        if prev not in ";{}\n" and not (prev==")" and False):
            manual.append((line,"prev="+repr(prev),t[max(0,s-60):e+1][:200]));continue
        if prev==")" :
            manual.append((line,"prev=)",t[max(0,s-60):e+1][:200]));continue
        if not semi and nxt not in "}\n;" :
            manual.append((line,"noterm",t[s:e+1][:200]));continue
        key=arg[1+len(TAG)+1:1+len(TAG)+200].split("${")[0].split('"')[0].split("`")[0].split("'")[0]
        self_pos=(path,arg_off(t,o))
        if any(((len(a)>=12 and len(b)>=12 and (b.startswith(a) or a.startswith(b))) or (a==b and len(a)>=12)) and (f,q)!=self_pos for f,q,a in OCC if (b:=key)):
            manual.append((line,"protocol-key",t[s:e+1][:160]));continue
        cuts.append((s,end,line,t[s:e+1][:140]))
    out=t
    for s,end,line,txt in reversed(cuts):
        ls=out.rfind("\n",0,s)+1;le=out.find("\n",end)
        le=len(out) if le<0 else le
        if out[ls:s].strip()=="" and out[end:le].strip()=="":
            out=out[:ls]+out[le+1:]
        else:
            a=s
            if out[a-1:a] in " \t" and out[end:end+1] in " \t}":a-=0
            out=out[:s]+out[end:]
    return t,out,cuts,manual
CORPUS=""
OCC=[]
def arg_off(t,o):
    k=o+1
    while t[k] in " \t\r\n":k+=1
    return k+1
def build_occ(files):
    for f in files:
        t=open(f,encoding="utf8").read();i=0
        while True:
            i=t.find(TAG,i)
            if i<0:break
            lit=t[i+len(TAG)+1:i+len(TAG)+200]
            for sep in ('"',"'","`","${","\\n"):lit=lit.split(sep)[0]
            pre=t[max(0,i-40):i-1]
            if not re.search(r'(?:console\s*\.\s*\w+|e?println!|\.write)\s*\(\s*$',pre):OCC.append((f,i,lit))
            i+=1
if __name__=="__main__":
    apply=sys.argv[1]=="apply";files=[l.strip() for l in open(sys.argv[2]) if l.strip()]
    now=time.time()
    build_occ([x.strip() for x in open(sys.argv[3]) if x.strip()])
    for f in files:
        if now-os.stat(f).st_mtime<1800:print("LIVE-SKIP",f);continue
        t,out,cuts,manual=process(f,apply)
        short="/".join(f.split("/")[-4:])
        print(f"## {short}: cut {len(cuts)} manual {len(manual)}")
        for s,e,l,x in sorted(cuts,key=lambda r:r[2]):print(f"  cut {l}: {x[:110]}")
        for l,why,x in manual:print(f"  MANUAL {l} [{why}]: {x[:200]}")
        if apply and cuts:open(f,"w",encoding="utf8").write(out)
