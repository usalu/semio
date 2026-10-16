"""🔎 Cross-crate visibility scan: non-pub fields and methods of lower crates that dependent crates mention."""
import re,sys
sys.path.insert(0,'.')
from strip import strip
import os
CLI=os.environ.get("CLI_TREE","/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/")
F={
 "event_stream":"📡️event-stream/🦀️.rs",
 "command_framework":"🧭️repo-cli/🎮️command-framework/🦀️.rs",
 "tree_source":"🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs",
 "mcp_verb":"🔌️mcp-verb/🦀️.rs",
 "verbs":"🧭️repo-cli/🚦️verbs/🦀️.rs",
 "command_tree":"🧭️repo-cli/🌳️command-tree/🦀️.rs",
 "run_proj":"🧭️repo-cli/🚀️run-and-projection/🦀️.rs",
}
DEP={"event_stream":[],"command_framework":["event_stream"],"tree_source":["command_framework","event_stream"],"mcp_verb":["event_stream"],"verbs":["command_framework","event_stream","mcp_verb","tree_source"],"command_tree":["command_framework","verbs"],"run_proj":["command_framework","event_stream","mcp_verb","tree_source","command_tree"]}
S={k:strip(open(CLI+v).read()) for k,v in F.items()}
def top_blocks(t):
    # yield (kind,name,body) for struct/enum/impl at col 0
    for m in re.finditer(r'^(?:pub(?:\([^)]*\))?\s+)?(struct|enum|impl)\b[^{;]*?(\{|;)',t,re.M):
        if m.group(2)==';': 
            yield (m.group(1),m.group(0),"")
            continue
        i=m.end(); d=1
        while d and i<len(t):
            if t[i]=='{': d+=1
            elif t[i]=='}': d-=1
            i+=1
        yield (m.group(1),m.group(0),t[m.end():i-1])
issues=[]
for k,t in S.items():
    names=set()
    for kind,head,body in top_blocks(t):
        if kind=="struct":
            # fields at depth 1
            for fm in re.finditer(r'^\s*(pub(?:\([^)]*\))?\s+)?([a-z_][a-z0-9_]*)\s*:',body,re.M):
                if not fm.group(1) or 'crate' in fm.group(1): names.add(("field",fm.group(2),fm.group(1) or 'private',head.strip()[:50]))
            for fm in re.finditer(r'\(\s*(pub(?:\([^)]*\))?\s+)?[A-Za-z]',head):
                pass
        elif kind=="impl":
            for fm in re.finditer(r'^    (pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([a-z_][a-z0-9_]*)',body,re.M):
                if not fm.group(1) or 'crate' in fm.group(1): names.add(("method",fm.group(2),fm.group(1) or 'private',head.strip()[:50]))
        elif kind=="enum":
            pass
    for d in [x for x,deps in DEP.items() if k in deps]:
        for (what,n,vis,head) in sorted(names):
            hits=[i+1 for i,l in enumerate(S[d].split('\n')) if re.search(r'(\.|::)'+n+r'\b|\b'+n+r'\s*:(?!:)' if what=="field" else r'(\.|::)'+n+r'\s*\(',l)]
            if hits: issues.append((k,d,what,n,vis,head,hits[:5]))
for i in issues: print(i)
print(len(issues),"candidates")
