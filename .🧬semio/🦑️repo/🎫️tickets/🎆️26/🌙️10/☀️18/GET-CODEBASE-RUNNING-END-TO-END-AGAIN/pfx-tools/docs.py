import rs,os,re,collections
FE="️"
CAND={
 "option":["🔘","☑","🎚","🪞"],"optional":["☑","🔘","🎚","🪞"],"class":["🏷","🔖","📛"],"kind":["🏷","🔖"],"geometry":["📍","🗺","📌"],"value":["💠","🔷","🔹"],"relation":["🤝","🔁","↔"],
 "run":["🏃","🔤","✍"],"block":["🧱","🧊","🏗"],"shape":["🔶","🔷","⬟"],"placeholder":["📌","🪧","🔖"],"node":["🌿","🌱","🍃"],"reference":["🔗","⛓","🪝"],
 "child":["🪆","🧸","🎎"],"type":["🧪","🧫","🔬"],"piece":["🧩","♟","🪀"],"connection":["🔌","🪢","🧵"],"design":["📐","📏","🧭"],"pin":["📍","📌","🪛"],
 "link":["🖇","🔗","⛓"],"curve":["〰","➰","🪢"],"surface":["🏞","🌐","🗾"],"numbers":["🔢","🧮","➗"],"points":["📍","🎯","⭕"],"pset":["🗃","🗂","🧰"],
 "property":["🏷","🧾","📇"],"entity":["🧬","🔬","🧫"],"record":["📇","🗒","📒"],"properties":["🧾","📇","🗒"],"hex":["🔡","🔣","🔢"],"list":["🗂","🗒","📜"],
 "empty":["🕳"],"decode":["🛬"],"text":["📝","🔠","🔤"],"binary":["💾","📀","🧮"],"fill":["🧩","🪣","🫗"],"into":["🧵","🪡","🧶"],
}
def pick(name,used):
    words=[w for w in re.split(r"_+",name) if w not in("into","binary","text","document")]+["binary" if "binary" in name else "text" if "text" in name else "into","fill","into"]
    for w in words:
        for e in CAND.get(w,[]):
            if e+FE not in used:return e+FE
    for pool in CAND.values():
        for e in pool:
            if e+FE not in used:return e+FE
def norm(e):return e.replace("\ufe0f","")
def dedup(path):
    s=open(path,encoding="utf-8").read()
    mod=re.match(r"//!\s*(\S+)",s)
    docs=list(re.finditer(r"(?m)^([ \t]*///[ \t]*)(\S+)([^\n]*\n)([ \t]*(?:pub(?:\([a-z]+\))?[ \t]*)?(?:async[ \t]+)?(?:fn|struct|enum|impl|mod|use|const|type)[ \t]*(?:\w+)?)",s))
    seen=set([norm(mod.group(1))]) if mod else set()
    edits=[]
    for d in docs:
        e=norm(d.group(2))
        if e in seen:
            m=re.search(r"\b(?:fn|struct|enum)[ \t]+(\w+)",d.group(4))
            name=m.group(1) if m else "fill"
            used=seen|set(norm(x.group(2)) for x in docs)
            n=pick(name,set(u for u in used)|{u+FE for u in used})
            edits.append((d.start(2),d.end(2),n));seen.add(norm(n))
        else:seen.add(e)
    for a,b,n in sorted(edits,reverse=True):s=s[:a]+n+s[b:]
    open(path,"w",encoding="utf-8").write(s);return len(edits)
if __name__=="__main__":
    files=[rs.SUBSETS+d+"/"+rs.SNAP+"🛬️native/🦀️.rs" for d in sorted(os.listdir(rs.SUBSETS)) if os.path.exists(rs.SUBSETS+d+"/"+rs.SNAP+"🛬️native/🦀️.rs")]
    files.append(rs.SUBSETS+"✉️base/"+rs.SNAP+"🛫️native/🦀️.rs")
    for f in files:print(f.split("subsets/")[-1][:14],dedup(f))
