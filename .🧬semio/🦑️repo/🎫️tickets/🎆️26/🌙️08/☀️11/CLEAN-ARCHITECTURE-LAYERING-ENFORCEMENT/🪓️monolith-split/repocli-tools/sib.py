import re,sys
sys.path.insert(0,'.')
from strip import strip
CLI="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/"
files={
 "command_framework":"🧭️repo-cli/🎮️command-framework/🦀️.rs",
 "command_tree":"🧭️repo-cli/🌳️command-tree/🦀️.rs",
 "tree_source":"🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs",
 "verbs":"🧭️repo-cli/🚦️verbs/🦀️.rs",
 "run_proj":"🧭️repo-cli/🚀️run-and-projection/🦀️.rs",
 "event_stream":"📡️event-stream/🦀️.rs",
 "mcp_verb":"🔌️mcp-verb/🦀️.rs",
}
raw={k:open(CLI+v).read() for k,v in files.items()}
src={k:strip(t) for k,t in raw.items()}
defre=re.compile(r'^(pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|struct|enum|const|static|type|trait)\s+([A-Za-z_][A-Za-z0-9_]*)',re.M)
defs={}
for k,t in src.items():
    d={}
    for m in defre.finditer(t):
        d[m.group(2)]=m.group(1) or 'private'
    defs[k]=d
if __name__=="__main__":
    for k,t in src.items():
        lines=t.split('\n')
        for o,d in defs.items():
            if o==k: continue
            hits={}
            for n,vis in d.items():
                pat=re.compile(r'(?<![A-Za-z0-9_])'+n+r'(?![A-Za-z0-9_])')
                ls=[i for i,l in enumerate(lines) if pat.search(l)]
                if ls: hits[n]=(vis,ls)
            if hits:
                print(f"== {k} -> {o}")
                for n,(vis,ls) in sorted(hits.items()):
                    if len(n)<5 or n[0].islower():
                        i=ls[0]; print(f"   {n} [{vis.strip()}] x{len(ls)} L{i+1}: {lines[i].strip()[:110]}")
                    else:
                        print(f"   {n} [{vis.strip()}] x{len(ls)}")
