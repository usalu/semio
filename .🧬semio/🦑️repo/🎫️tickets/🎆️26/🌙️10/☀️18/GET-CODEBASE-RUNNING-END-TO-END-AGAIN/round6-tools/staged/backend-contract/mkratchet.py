#!/usr/bin/env python3
"""Re-cuts the literal ratchet of src/backends_test.ts against a tree (a combined overlay after the chain): clean files that gained a literal move to the caps, caps are set to the current counts (a cap that shrank is lowered, one that grew is reported). Usage: mkratchet.py <root> [--write]; then build.py."""
import json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
root=sys.argv[1];write="--write" in sys.argv
build=os.path.join(root,names.B)
path=os.path.join(here,"src","backends_test.ts");text=open(path,encoding="utf8").read()
literal=re.compile(r"""(["'`])(nx|bazel)\1|\b(?:tool|selected\.tool)\s*[!=]==\s*["'](nx|bazel)["']""")
clean=json.loads(re.search(r"const NEUTRAL_CLEAN=(\[.*?\]);",text,re.S).group(1))
leaks=json.loads(re.search(r"const KNOWN_LEAKS:Record<string,number>=(\{.*?\});",text,re.S).group(1))
def count(rel):
    p=os.path.join(build,rel)
    return None if not os.path.exists(p) else len(literal.findall(open(p,encoding="utf8").read()))
new_clean=[];new_leaks={}
for rel in clean:
    c=count(rel)
    if c in (None,0):new_clean.append(rel)
    else:new_leaks[rel]=c;print(f"clean file gained {c} literal(s): {rel}")
for rel,cap in leaks.items():
    c=count(rel)
    if c is None:print(f"cap file missing: {rel}");new_leaks[rel]=cap;continue
    if c>cap:print(f"cap grew {cap} -> {c}: {rel}")
    elif c<cap:print(f"cap shrank {cap} -> {c}: {rel}")
    if c==0:new_clean.append(rel);print(f"cap file now clean: {rel}")
    else:new_leaks[rel]=c
text=re.sub(r"const NEUTRAL_CLEAN=\[.*?\];","const NEUTRAL_CLEAN="+json.dumps(new_clean,ensure_ascii=False,separators=(",",":"))+";",text,count=1,flags=re.S)
text=re.sub(r"const KNOWN_LEAKS:Record<string,number>=\{.*?\};","const KNOWN_LEAKS:Record<string,number>="+json.dumps(new_leaks,ensure_ascii=False,separators=(",",":"))+";",text,count=1,flags=re.S)
if write:open(path,"w",encoding="utf8").write(text);print("written")
