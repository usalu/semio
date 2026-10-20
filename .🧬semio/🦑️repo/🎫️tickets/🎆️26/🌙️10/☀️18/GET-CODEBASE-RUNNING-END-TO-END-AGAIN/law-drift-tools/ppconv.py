import re,sys
MOD=re.compile(r'^\s*((?:(?:private|public|protected|readonly|override)\s+)+)(.*)$',re.S)
def split_params(s):
    out=[];d=0;cur="";i=0;q=None
    while i<len(s):
        c=s[i]
        if q:
            cur+=c
            if c=="\\": cur+=s[i+1]; i+=1
            elif c==q: q=None
        elif c in "\"'`": q=c;cur+=c
        elif c in "([{<" and not (c=="<" and False): d+=1;cur+=c
        elif c in ")]}": d-=1;cur+=c
        elif c==">" and s[i-1]!="=": d-=1;cur+=c
        elif c=="," and d==0: out.append(cur);cur=""
        else: cur+=c
        i+=1
    if cur.strip(): out.append(cur)
    return out
def match_close(s,i,o,c):
    d=0;q=None
    while i<len(s):
        ch=s[i]
        if q:
            if ch=="\\": i+=1
            elif ch==q: q=None
        elif ch in "\"'`": q=ch
        elif ch==o: d+=1
        elif ch==c:
            d-=1
            if d==0: return i
        i+=1
    raise ValueError("unbalanced")
def convert(src):
    changed=0;pos=0;out=""
    for m in re.finditer(r'\bconstructor\s*\(',src):
        if m.start()<pos: continue
        popen=m.end()-1
        pclose=match_close(src,popen,"(",")")
        params=split_params(src[popen+1:pclose])
        props=[]
        newp=[]
        for p in params:
            mm=MOD.match(p)
            if mm:
                mods=mm.group(1).split(); rest=mm.group(2).strip()
                name=re.match(r'([A-Za-z_$][\w$]*)(\??)',rest)
                props.append((mods,rest,name.group(1)))
                newp.append(rest)
            else: newp.append(p)
        if not props: continue
        # body
        j=pclose+1
        while src[j] in " \n\t": j+=1
        if src[j]==":":  # return type not allowed on ctor; skip
            continue
        if src[j]!="{": continue
        bclose=match_close(src,j,"{","}")
        body=src[j+1:bclose]
        # class header
        cls_start=src.rfind("class ",0,m.start())
        header=src[cls_start:m.start()]
        extends=" extends " in header.split("{")[-2] if "{" in header else False
        # field declarations
        indent=re.search(r'\n([ \t]*)$',src[:m.start()]); ind=indent.group(1) if indent else "  "
        decls=""
        for mods,rest,name in props:
            typ=re.match(r'[A-Za-z_$][\w$]*\??\s*(?::\s*(.*?))?(?:\s*=\s*.*)?$',rest,re.S)
            t=None
            mt=re.match(r'([A-Za-z_$][\w$]*)(\??)\s*:\s*(.*?)(\s*=\s*(.*))?$',rest,re.S)
            if mt: t=mt.group(3).strip(); opt=mt.group(2)
            else: opt=""
            vis=" ".join(x for x in mods if x in("private","public","protected"))
            ro="readonly " if "readonly" in mods else ""
            if t is None:
                # inferred from default: cannot infer type; fall back to unknown-safe via typeof
                mt2=re.match(r'([A-Za-z_$][\w$]*)\s*=\s*(.*)$',rest,re.S)
                decls+=f"{ind}{(vis+' ') if vis else ''}{ro}{name}{'' };\n" if False else ""
                t="__INFER__"
            decls+=f"{ind}{(vis+' ') if vis else ''}{ro}{name}{opt}: {t};\n" if t!="__INFER__" else f"{ind}{(vis+' ') if vis else ''}{ro}{name} = {mt2.group(2)};\n" if False else f"{ind}{(vis+' ') if vis else ''}{ro}{name}{opt}: {t};\n"
        if "__INFER__" in decls: 
            sys.stderr.write("untyped parameter property needs manual conversion\n"); continue
        assigns="".join(f"{ind}  this.{n} = {n};\n" for _,_,n in props)
        if extends:
            sm=re.search(r'\bsuper\s*\(',body)
            if not sm: continue
            sclose=match_close(body,sm.end()-1,"(",")")
            k=sclose+1
            while k<len(body) and body[k] in " \t": k+=1
            if k<len(body) and body[k]==";": k+=1
            newbody=body[:k]+"\n"+assigns.rstrip("\n")+body[k:]
        else:
            newbody="\n"+assigns.rstrip("\n")+("\n" if not body.startswith("\n") else "")+body
        out+=src[pos:m.start()]+decls.lstrip(" \t")+(ind if decls else "")+"constructor("+",".join(newp)+") {"+newbody+"}"
        pos=bclose+1;changed+=1
    out+=src[pos:]
    return out,changed
if __name__=="__main__":
    p=sys.argv[1]; s=open(p,encoding='utf-8').read(); n,c=convert(s)
    if c: open(p,'w',encoding='utf-8').write(n)
    print(p[-60:],c)
