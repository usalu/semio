import sys,difflib
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tfe-tools"
idx=[l.split("\t") for l in open(F+"/orig/index.tsv",encoding="utf8").read().split("\n") if l]
pat=sys.argv[1]
for s,p in idx:
    if pat in p and s!="NEW":
        a=open(s,encoding="utf8").read().split("\n"); b=open(p,encoding="utf8").read().split("\n")
        print("###",p[-90:])
        for l in difflib.unified_diff(a,b,lineterm="",n=0): print(l[:300])
