"""🧹 Token-level unused-import check for the carved crate roots (external imports cannot warn in the analysis oracle)."""
import re,sys
sys.path.insert(0,'.')
from strip import strip
CLI="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/"
FILES=["📡️event-stream/🦀️.rs","🧭️repo-cli/🎮️command-framework/🦀️.rs","🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs","🔌️mcp-verb/🦀️.rs","🧭️repo-cli/🚦️verbs/🦀️.rs","🧭️repo-cli/🌳️command-tree/🦀️.rs","🧭️repo-cli/🚀️run-and-projection/🦀️.rs"]
TRAITS={"RepoContext","Write","Repository","Read"}
bad=0
for f in FILES:
    t=strip(open(CLI+f).read())
    uses=list(re.finditer(r'^use\s+([^;]+);',t,re.M))
    rest=t
    for u in uses: rest=rest.replace(u.group(0),' '*len(u.group(0)))
    names=[]
    for u in uses:
        body=u.group(1)
        m=re.match(r'(.*?)::\{(.*)\}$',body,re.S)
        items=[x.strip() for x in m.group(2).split(',')] if m else [body.split('::')[-1].strip()]
        for it in items:
            if not it: continue
            alias=it.split(' as ')[-1].strip()
            names.append(alias)
    for n in names:
        c=len(re.findall(r'(?<![A-Za-z0-9_])'+re.escape(n)+r'(?![A-Za-z0-9_])',rest))
        if c==0:
            tag="TRAIT-OK?" if n in TRAITS else "UNUSED"
            print(f"{f}: {n}: {tag}"); bad+= n not in TRAITS
print("unused:",bad)
