#!/usr/bin/env python3
"""🔁️ Appends `?` (production) or `.unwrap()` (tests) after the exact call expressions `with_host(..)` / `with_host_from_snapshot(..)` whose helper now returns a Result: finds the call by name, matches its parentheses, and skips definitions, doc lines and calls that are already followed by `?`/`.unwrap()`. Prints every edited site."""
import re,sys
def is_test(p):return bool(re.search(r"🧪|/tests?/|_tests?\.rs|tests\.rs",p))
NAMES=re.compile(r"(?<![\w:.])(with_host_from_snapshot|with_host)\(")
def rip(f):
    t=open(f,encoding="utf8").read();out=[];i=0;n=0
    for m in NAMES.finditer(t):
        s=m.start()
        ls=t.rfind("\n",0,s)+1
        pre=t[ls:s]
        if re.search(r"\bfn\s+$",pre) or pre.lstrip().startswith("//") :continue
        depth=0;j=m.end()-1;
        while j<len(t):
            c=t[j]
            if c=="(":depth+=1
            elif c==")":
                depth-=1
                if depth==0:break
            j+=1
        end=j+1
        if t[end:end+1]=="?" or t[end:end+9]==".unwrap()":continue
        out.append((end,"" if False else (".unwrap()" if is_test(f) else "?")));n+=1
    for end,suf in sorted(out,reverse=True):t=t[:end]+suf+t[end:]
    if n:open(f,"w",encoding="utf8").write(t)
    return n
if __name__=="__main__":
    for f in sys.argv[1:]:print(rip(f),f.split("/")[-3][:20],f.split("/")[-2][:20])
