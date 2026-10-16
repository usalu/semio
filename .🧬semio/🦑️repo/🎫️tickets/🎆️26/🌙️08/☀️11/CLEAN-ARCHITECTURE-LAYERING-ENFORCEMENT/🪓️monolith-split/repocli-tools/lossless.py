"""📏 Losslessness: trimmed non-blank line multiset of the nine files before (scratch copy of HEAD working tree) and after."""
import collections,sys
CLI="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/"
ORIG="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/repocli/orig/"
FILES=["🦀️.rs","🧭️repo-cli/🦀️.rs","📡️event-stream/🦀️.rs","🔌️mcp-verb/🦀️.rs","🧭️repo-cli/🎮️command-framework/🦀️.rs","🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs","🧭️repo-cli/🚦️verbs/🦀️.rs","🧭️repo-cli/🌳️command-tree/🦀️.rs","🧭️repo-cli/🚀️run-and-projection/🦀️.rs"]
def ms(base):
    c=collections.Counter()
    for f in FILES:
        for l in open(base+f,encoding="utf-8").read().split("\n"):
            l=l.strip()
            if l: c[l]+=1
    return c
a,b=ms(ORIG),ms(CLI)
print("missing (in old, not in new):")
for k,v in sorted((a-b).items()): print("  -",v,k[:170])
print("extra (in new, not in old):")
for k,v in sorted((b-a).items()): print("  +",v,k[:170])
