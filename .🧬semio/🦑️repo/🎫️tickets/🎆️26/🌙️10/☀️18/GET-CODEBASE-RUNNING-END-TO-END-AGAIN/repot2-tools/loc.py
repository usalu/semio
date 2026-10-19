import sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L=[l.strip() for l in open(C+"/repot2-tools/list-red.txt",encoding="utf-8") if l.strip()]
for s in sys.argv[1:]:
    for f in L:
        if s in f: print(f)
