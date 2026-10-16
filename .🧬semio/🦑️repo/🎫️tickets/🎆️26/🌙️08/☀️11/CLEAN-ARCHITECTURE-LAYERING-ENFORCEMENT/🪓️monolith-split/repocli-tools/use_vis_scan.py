"""🔎 Definition scan: every name imported from a sibling carved crate must be a top-level `pub` item of that crate; plus seeded sensitivity."""
import re,sys,os,shutil
sys.path.insert(0,'.')
from strip import strip
CLI=os.environ.get("CLI_TREE","/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/")
CR={
 "event_stream":"📡️event-stream/🦀️.rs",
 "command_framework":"🧭️repo-cli/🎮️command-framework/🦀️.rs",
 "tree_source_and_search":"🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs",
 "mcp_verb":"🔌️mcp-verb/🦀️.rs",
 "verbs":"🧭️repo-cli/🚦️verbs/🦀️.rs",
}
PARENT=["🧭️repo-cli/🌳️command-tree/🦀️.rs","🧭️repo-cli/🚀️run-and-projection/🦀️.rs","🧭️repo-cli/🦀️.rs","🦀️.rs"]
def defs(text):
    d={}
    for m in re.finditer(r'^(pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(fn|struct|enum|const|static|type|trait|mod)\s+([A-Za-z_][A-Za-z0-9_]*)',text,re.M):
        d[m.group(3)]=(m.group(1) or 'private').strip()
    return d
D={k:defs(strip(open(CLI+v,encoding="utf-8").read())) for k,v in CR.items()}
bad=0;checked=0
for f in list(CR.values())+PARENT:
    t=strip(open(CLI+f,encoding="utf-8").read())
    for m in re.finditer(r'^(?:pub\s+)?use\s+semio_framework_repo_cli_([a-z_]+?)(?:\s+as\s+\w+)?;|^(?:pub\s+)?use\s+semio_framework_repo_cli_([a-z_]+?)::(\{[^}]*\}|[A-Za-z_]\w*)\s*;',t,re.M|re.S):
        crate=m.group(1) or m.group(2)
        if crate not in D: print("UNKNOWN CRATE",crate,f); bad+=1; continue
        if m.group(3) is None: continue
        items=[x.strip().split(' as ')[0] for x in m.group(3).strip('{}').split(',') if x.strip()]
        for it in items:
            checked+=1
            vis=D[crate].get(it)
            if vis!='pub':
                print(f"NOT PUB: {f} imports {crate}::{it} (defined as {vis})"); bad+=1
print("checked",checked,"imports; violations",bad)
sys.exit(1 if bad else 0)
