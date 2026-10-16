import re,sys,os
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
src={k:open(CLI+v).read() for k,v in files.items()}
defre=re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:fn|struct|enum|const|static|type|trait)\s+([A-Za-z_][A-Za-z0-9_]*)',re.M)
defs={}
for k,t in src.items():
    # top-level only (col 0), except verbs which is dedented? check
    d=set(defre.findall(t))
    defs[k]=d
    print(k,len(d))
ident=re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
for k,t in src.items():
    toks=set(ident.findall(re.sub(r'//.*','',t)))
    for o,d in defs.items():
        if o==k: continue
        used=sorted((toks&d))
        if used: print(f"{k} -> {o}: {len(used)} {used[:40]}")
