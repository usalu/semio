"""🔓️ Name-keyed visibility widening for definitions other crates reference (the narrowest cross-crate visibility is `pub`)."""
import re
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
IDENT=re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
def strip_comments(t):
    t=re.sub(r"/\*.*?\*/","",t,flags=re.S)
    t=re.sub(r'"(?:\\.|[^"\\\n])*"',lambda m:'""',t)
    t=re.sub(r"//[^\n]*","",t)
    return t
class Others:
    def __init__(self,paths):
        self.text="\n".join(strip_comments(open(p,encoding="utf-8").read()) for p in paths)
        self.idents=set(IDENT.findall(self.text))
        self._c={}
    def item(self,name): return name in self.idents
    def field(self,struct,name):
        k=("f",struct,name)
        if k in self._c: return self._c[k]
        t=self.text
        r=bool(re.search(r"\."+re.escape(name)+r"(?![A-Za-z0-9_(])",t)) or bool(re.search(re.escape(struct)+r"\s*(?:<[^>{]*>)?\s*\{[^}]{0,2000}?(?<![A-Za-z0-9_])"+re.escape(name)+r"\s*[:,}]",t,re.S)) or bool(re.search(r"(?<![A-Za-z0-9_])"+re.escape(name)+r"\s*:\s*[^:]",t) and False)
        self._c[k]=r;return r
    def method(self,name):
        t=self.text
        return bool(re.search(r"(?:\.|::)"+re.escape(name)+r"(?![A-Za-z0-9_])",t))
VISRE=re.compile(r"^pub(?:\([^)]*\))?\s+")
def with_pub(line,indent):
    body=line[len(indent):]
    if body.startswith("pub "): return line
    return indent+"pub "+VISRE.sub("",body)
HEAD=re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(fn|struct|enum|const|static|type|trait|impl|union)\b")
def widen_file(path,oth,apply=False,skip_types=()):
    lines=open(path,encoding="utf-8").read().split("\n")
    out=[];changes=[]
    block=None
    for idx,l in enumerate(lines):
        new=l
        if l and not l[0].isspace():
            if l.startswith(("//","#[","}",")",";","]","use ","mod ","pub use ","pub mod ","#!","/*")):
                if l.startswith("}"): block=None
                out.append(l); continue
            m=HEAD.match(l)
            block=None
            if m:
                kind=m.group(1)
                if kind=="impl":
                    block=("impl",) if " for " not in l.split("{")[0] else ("trait",)
                    ih=re.match(r"^impl(?:<[^>]*>)?\s+([A-Za-z_]\w*)",l)
                    if ih and ih.group(1) in skip_types: block=("skip",)
                else:
                    nm=re.search(r"\b"+kind+r"\s+([A-Za-z_][A-Za-z0-9_]*)",l)
                    name=nm.group(1) if nm else None
                    if kind=="struct": block=("struct",name)
                    if name and name!="_" and oth.item(name): new=with_pub(l,"")
                    if kind=="struct" and name in skip_types: block=("skip",)
        elif block and l.startswith("    ") and not l.startswith("     "):
            if block[0]=="struct":
                m=re.match(r"^    (?:pub(?:\([^)]*\))?\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:",l)
                if m and oth.field(block[1],m.group(1)): new=with_pub(l,"    ")
            elif block[0]=="impl":
                m=re.match(r"^    (?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:unsafe\s+)?(?:async\s+)?(?:fn|const)\s+([A-Za-z_][A-Za-z0-9_]*)",l)
                if m and oth.method(m.group(1)): new=with_pub(l,"    ")
        if new!=l: changes.append((idx+1,l.strip()[:100],new.strip()[:100]))
        out.append(new)
    if apply and changes: open(path,"w",encoding="utf-8").write("\n".join(out))
    return changes
