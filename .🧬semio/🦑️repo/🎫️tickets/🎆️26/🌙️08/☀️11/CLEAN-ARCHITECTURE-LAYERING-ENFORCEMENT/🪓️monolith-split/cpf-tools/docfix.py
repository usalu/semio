import os,re,sys,json,regex,collections,hashlib
sys.path.insert(0,os.path.dirname(__file__))
from edit_lib import ROOT,read,write
G=os.environ["G"]
APPLY="--apply" in sys.argv
SKIP=("◻️2d","🖐️5d","bim")
MARK=regex.compile(r"^(?:\p{Extended_Pictographic}|\p{So}|\p{Sm}|\p{Regional_Indicator}|[0-9#*]️?⃣)")
GRAPH=regex.compile(r"\X")
GENERIC={"tests","unit","quick","long","packages","typescript","rust","bin-unit","units","testing","test","src","tests-1","laws"}
POOL_TEST=["🧪️","🔬️","⚖️","🧾️","🔎️","🎯️","🧮️","🛡️","📏️","🧱️","🧩️","🔗️","🧵️","🪢️","🧭️","📋️","🗂️","🧷️","🪞️","🔖️","🏷️","📌️","🧲️","🪤️","🧫️","🔭️","🧬️","🪛️","🔧️","🔩️"]
POOL_SRC=["📦️","🧩️","🧰️","🔗️","🧭️","🧱️","📐️","🗂️","🏷️","📌️","🧵️","🪢️","🔧️","🔩️","🧬️","📋️"]
def first_grapheme(s):
    m=GRAPH.match(s)
    return m.group(0) if m else ""
def norm_emoji(e):
    e=e.replace("️","")
    return e
def with_fe0f(e):
    b=e.replace("️","")
    return b+"️" if not b.endswith("⃣") else e
def folder_emoji(name):
    g=first_grapheme(name)
    if g and MARK.match(g) : return with_fe0f(g)
    return None
def words(name):
    s=re.sub(r"^[^A-Za-z0-9]+","",name)
    s=re.sub(r"\.(test\.)?(ts|tsx|rs)$","",s)
    s=re.sub(r"[-_]+"," ",s)
    s=re.sub(r"(?<=[a-z0-9])(?=[A-Z])"," ",s)
    return s.strip().lower()
def openers_of(text,path):
    out=[]
    for ln in text.split("\n"):
        s=ln.strip()
        m=re.match(r"^(?:///|//!)\s*(.*)$",s) if path.endswith(".rs") else None
        if m and m.group(1): out.append(m.group(1))
        m2=re.match(r"^/\*\*\s*(.*?)(?:\*/)?$",s)
        if m2 and m2.group(1): out.append(m2.group(1))
    emos=[]
    for o in out:
        if MARK.match(o): emos.append(norm_emoji(first_grapheme(o)))
    return emos
def sibling_emojis(path):
    d=os.path.join(ROOT,os.path.dirname(path)); used=set()
    try: names=os.listdir(d)
    except Exception: return used
    for n in names:
        if n==os.path.basename(path) or not re.search(r"\.(rs|ts|tsx)$",n) or n.endswith(".d.ts"): continue
        try: t=open(os.path.join(d,n),encoding="utf-8").read(4000)
        except Exception: continue
        lines=[l for l in t.split("\n") if l.strip()]
        for l in lines[:6]:
            s=l.strip()
            m=re.match(r"^(?:///|//!|//|/\*\*)\s*(.*?)$",s)
            if m and m.group(1) and MARK.match(m.group(1)):
                used.add(norm_emoji(first_grapheme(m.group(1)))); break
    return used
def pick_emoji(path,text,pool,forced=None):
    taken=sibling_emojis(path)|set(openers_of(text,path))
    cands=[]
    parts=path.split("/")
    base=parts[-1]
    if forced: cands.append(forced)
    for part in reversed(parts[:-1]):
        fe=folder_emoji(part)
        if fe: cands.append(fe)
    fe=folder_emoji(base)
    if fe and not base.startswith(("🦀","🟦","📜")): cands.insert(0,fe)
    cands+=pool
    for c in cands:
        if norm_emoji(c) not in taken and MARK.match(c): return with_fe0f(c)
    raise RuntimeError("no emoji for "+path)
def subject(path):
    parts=path.split("/"); base=parts[-1]
    stem=words(base)
    if not base.startswith(("🦀️.rs","🟦️.ts","🟦️.tsx")) and stem and stem not in ("script","test"):
        pass
    for part in reversed(parts[:-1]):
        w=words(part)
        if w and w not in GENERIC: return w
    return words(parts[-2])
def titles_ts(text):
    out=[]
    for m in re.finditer(r"\b(?:test|it|describe)(?:\.\w+)*\(\s*(?:\"([^\"\n]{4,140})\"|'([^'\n]{4,140})'|`([^`\n]{4,140})`)",text):
        out.append(next(g for g in m.groups() if g))
    return out
def titles_rs(text):
    out=[]
    lines=text.split("\n")
    for i,l in enumerate(lines):
        m=re.match(r"\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)",l)
        if m:
            j=i-1
            while j>=0 and lines[j].strip().startswith(("#[","//","///")): 
                if "#[test]" in lines[j] or "#[tokio::test" in lines[j] or "#[wasm_bindgen_test" in lines[j]:
                    out.append(m.group(1).replace("_"," ")); break
                j-=1
    return out
def exports_ts(text):
    return re.findall(r"\bexport\s+(?:async\s+)?(?:function|const|class|type|interface|enum)\s+([A-Za-z_$][\w$]*)",text)
def declared_ts(text):
    names=re.findall(r"(?m)^\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)",text)
    names+=re.findall(r"(?m)^\s*(?:export\s+)?const\s+([A-Za-z_$][\w$]*)\s*[=:]",text)
    return names
def describe(path,text):
    is_test=("🧪️tests" in path or "/tests/" in path or ".test." in path or "🧪️testing" in path)
    subj=subject(path)
    cap=subj[0].upper()+subj[1:]
    base=os.path.basename(path)
    if path.endswith(".rs"): ts=titles_rs(text)
    else: ts=titles_ts(text)
    if is_test and ts:
        pick=ts[:3]
        s=f"{cap}: {len(ts)} test{'s' if len(ts)!=1 else ''} covering "+"; ".join(p.rstrip(".") for p in pick)
        if len(ts)>3: s+="; and more"
        s+="."
    elif base=="📜️script.ts" and "receiveScriptProcessInvocation" in text:
        s=f"{cap}: child-process script that receives the issued script-process invocation and runs the case its arguments select."
    elif base=="📜️script.ts":
        s=f"{cap}: command script run by its tests."
    else:
        ex=[] if path.endswith(".rs") else exports_ts(text)
        dc=[] if path.endswith(".rs") else declared_ts(text)
        names=ex or dc
        if names:
            uniq=list(dict.fromkeys(names))
            kind="fixtures" if is_test else "exports"
            s=f"{cap}: {'test support' if is_test else 'module'} {'declaring' if not ex else 'exporting'} "+", ".join(f"`{e}`" for e in uniq[:4])+("." if len(uniq)<=4 else " and more.")
        elif is_test:
            s=f"{cap}: test support."
        else:
            s=f"{cap}."
    s=s.replace("*/","* /")
    if len(s)>230: s=s[:227].rstrip()+"…"
    return s,is_test
def rs_wiring(path):
    folder=os.path.basename(os.path.dirname(path)); needle=f"{folder}/{os.path.basename(path)}"
    d=os.path.dirname(path)
    kinds=set()
    for _ in range(5):
        d=os.path.dirname(d)
        full=os.path.join(ROOT,d)
        if not d or not os.path.isdir(full): break
        for n in os.listdir(full):
            if not n.endswith(".rs"): continue
            try: t=open(os.path.join(full,n),encoding="utf-8").read()
            except Exception: continue
            for m in re.finditer(re.escape(needle),t):
                ls=t.rfind("\n",0,m.start())+1; line=t[ls:t.find("\n",m.end())]
                prev=t[max(0,ls-200):ls]
                if "include!" in line: kinds.add("include")
                elif "#[path" in line or "#[path" in prev[-120:]: kinds.add("mod")
                else: kinds.add("other")
        if kinds: break
    return kinds
def build(kind,p):
    t=read(p); nl="\n"
    lines=t.split("\n")
    i=0
    if p.endswith(".rs"):
        wiring=rs_wiring(p)
        if kind=="plain-comment":
            ind=re.match(r"\s*",lines[0]).group(0)
            # leading comment run
            j=0
            while j<len(lines) and re.match(r"^\s*//(?!/|!)",lines[j]) and not re.match(r"^\s*//\s*#?(end)?region",lines[j]) and not re.match(r"^\s*//#(end)?region",lines[j]): j+=1
            marker="///" if ("include" in wiring and "mod" not in wiring) or ind else "//!"
            if wiring=={"mod"} : marker="//!"
            if not wiring and not ind: marker="//!"
            new=[re.sub(r"^(\s*)//\s?",r"\1"+marker+" ",l) for l in lines[:j]]
            return "\n".join(new+lines[j:]),marker,wiring,None
        else:
            desc,is_test=describe(p,t)
            emo=pick_emoji(p,t,POOL_TEST if is_test else POOL_SRC)
            if "include" in wiring and "mod" not in wiring: marker="///"
            elif wiring=={"mod"}: marker="//!"
            else:
                marker="//!" if not t.lstrip().startswith("use super::*") else "//!"
            head=f"{marker} {emo} {desc}"
            return head+"\n"+t,marker,wiring,head
    else:
        desc,is_test=describe(p,t)
        emo=pick_emoji(p,t,POOL_TEST if is_test else POOL_SRC)
        head=f"/** {emo} {desc} */"
        # directives
        k=0
        while k<len(lines) and (lines[k].startswith("#!") or lines[k].startswith("/// <reference") or re.match(r"^//\s*@vitest",lines[k])): k+=1
        out=lines[:k]+[head]+lines[k:]
        return "\n".join(out),"/**",set(),head
if __name__=="__main__":
    res=json.load(open(G+"/cpf-docscan.json",encoding="utf-8"))
    todo=[(k,p) for k,p in res if k in("none","plain-comment")]
    log=[]; skipped=[]
    for k,p in todo:
        if any(s in p for s in SKIP): skipped.append(p); continue
        try: new,marker,wiring,head=build(k,p)
        except Exception as e:
            log.append(("ERR",p,str(e))); continue
        log.append((k,p,marker,sorted(wiring),head))
        if APPLY:
            cur=read(p)
            assert hashlib.sha256(cur.encode()).hexdigest()==hashlib.sha256(read(p).encode()).hexdigest()
            log[-1]=log[-1]+(hashlib.sha256(cur.encode()).hexdigest()[:16],)
            write(p,new)
    json.dump({"log":log,"skipped":skipped},open(G+("/cpf-docfix-applied.json" if APPLY else "/cpf-docfix-plan.json"),"w"),ensure_ascii=False,indent=1)
    print(len(todo),len(skipped),collections.Counter((l[0],l[2] if len(l)>2 else "") for l in log))
