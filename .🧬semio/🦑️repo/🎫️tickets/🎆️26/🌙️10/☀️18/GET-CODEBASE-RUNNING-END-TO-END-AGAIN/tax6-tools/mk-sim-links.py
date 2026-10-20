# 🔗 Links the siblings of a scratch library copy to the live tree so its relative imports resolve. usage: mk-sim-links.py <scratch name under 🗑️generated>
import os,sys
C=os.path.abspath(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN")
S=C+"/🗑️generated/"+sys.argv[1]
live=os.path.abspath(".")
def link(real,dst):
    if not os.path.lexists(dst): os.symlink(real,dst)
for level in ["🧰️framework","🧰️framework/🛍️products","🧰️framework/🛍️products/🦑️repo","🧰️framework/🛍️products/🦑️repo/🔨️modules"]:
    keep=level.split("/")[-1]+"/"
    for e in os.listdir(f"{live}/{level}"):
        nxt=f"{level}/{e}"
        if any(x.startswith(nxt) and x!=nxt for x in ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"]): continue
        link(f"{live}/{nxt}",f"{S}/{nxt}")
