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
}
facade=["Codebase","CodebaseScope","RecordingContext","RepoContext","FsRepoContext","emoji_text","identity_entity","flat","entity","DraftCreateInput","TicketStatus","TreeFilter","TreeNode","TreeNodeKind","json","Map","Json","BTreeMap","BTreeSet","Write"]
for k,v in files.items():
    t=strip(open(CLI+v).read())
    print("==",k)
    for n in facade:
        c=len(re.findall(r'(?<![A-Za-z0-9_])'+n+r'(?![A-Za-z0-9_])',t))
        if c: print("  ",n,c)
