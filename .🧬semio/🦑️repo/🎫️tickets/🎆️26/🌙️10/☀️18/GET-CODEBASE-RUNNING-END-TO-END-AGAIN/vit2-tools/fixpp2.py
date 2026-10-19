import re,sys
MOD=r'(?:(?:public|private|protected|readonly|override)\s+)+'
def skip_str(s,i):
    q=s[i];i+=1
    while i<len(s):
        if s[i]=="\\": i+=2; continue
        if s[i]==q: return i+1
        if q=="`" and s[i]=="$" and s[i+1:i+2]=="{":
            d=1;i+=2
            while i<len(s) and d:
                if s[i] in "\"'`": i=skip_str(s,i); continue
                if s[i]=="{": d+=1
                elif s[i]=="}": d-=1
                i+=1
            continue
        i+=1
    return i
def match(s,i,o,c):
    d=0
    while i<len(s):
        ch=s[i]
        if ch in "\"'`": i=skip_str(s,i); continue
        if s.startswith("//",i): i=s.index("\n",i); continue
        if s.startswith("/*",i): i=s.index("*/",i)+2; continue
        if ch==o: d+=1
        elif ch==c:
            d-=1
            if d==0: return i
        i+=1
    return -1
def split(p):
    out=[];d=0;cur="";prev="";i=0
    while i<len(p):
        ch=p[i]
        if ch in "\"'`": j=skip_str(p,i); cur+=p[i:j]; i=j; prev=ch; continue
        if ch in "<([{": d+=1
        elif ch in ">)]}" and not (ch==">" and prev=="="): d-=1
        prev=ch
        if ch=="," and d==0: out.append(cur);cur=""
        else: cur+=ch
        i+=1
    if cur.strip(): out.append(cur)
    return out
def fix(path):
    s=open(path,encoding="utf-8").read(); changed=0; pos=0
    while True:
        m=re.compile(r'^([ \t]*)constructor\s*\(',re.M).search(s,pos)
        if not m: break
        po=m.end()-1; pc=match(s,po,"(",")")
        params=s[po+1:pc]; ps=split(params)
        if not any(re.match(r'\s*(?://[^\n]*\n\s*|/\*.*?\*/\s*)*'+MOD,p,re.S) for p in ps): pos=pc; continue
        bo=s.index("{",pc); bc=match(s,bo,"{","}")
        body=s[bo+1:bc]; ind=m.group(1); fields=[];assigns=[];newps=[]
        for p in ps:
            lead=re.match(r'(\s*(?://[^\n]*\n\s*|/\*.*?\*/\s*)*)(.*)$',p,re.S)
            pre,rest=lead.group(1),lead.group(2)
            mm=re.match(r'('+MOD+r')(.*)$',rest,re.S)
            if not mm: newps.append(p);continue
            mods=mm.group(1).split(); rest=mm.group(2).strip()
            nm=re.match(r'(\w+)(\?)?\s*(?::\s*(.*?))?(?:\s*=(?!>)\s*(.*))?$',rest,re.S)
            name,opt,typ,default=nm.groups()
            if not typ and default is None: print("MANUAL untyped",name); return 0
            if not typ:
                if re.fullmatch(r'-?\d[\d_.e]*|true|false',default.strip()): typ="number" if default.strip()[0] in "-0123456789" else "boolean"
                elif default.strip()[0] in "\"'`": typ="string"
                else: print("MANUAL untyped default",name); return 0
            fields.append(f"{ind}{' '.join(mods)} {name}{'?' if opt else ''}: {typ};")
            assigns.append(f"this.{name} = {name};")
            newps.append(pre+name+(opt or "")+": "+typ+(" = "+default.strip() if default is not None else ""))
        a=" ".join(assigns)
        sup=re.search(r'\bsuper\s*\(',body)
        if sup:
            sc=match(body,sup.end()-1,"(",")"); e=body.index(";",sc)+1 if body[sc+1:].lstrip().startswith(";") else sc+1
            nb=body[:e]+"\n"+ind+"  "+a+body[e:]
        else: nb="\n"+ind+"  "+a+body if body.strip() else "\n"+ind+"  "+a+"\n"+ind
        new="\n".join(fields)+"\n"+ind+"constructor("+",".join(newps)+") {"+nb+"}"
        s=s[:m.start()]+new+s[bc+1:]; pos=m.start()+len(new); changed+=1
    if changed: open(path,"w",encoding="utf-8").write(s)
    return changed
for p in sys.argv[1:]: print(p,fix(p))
