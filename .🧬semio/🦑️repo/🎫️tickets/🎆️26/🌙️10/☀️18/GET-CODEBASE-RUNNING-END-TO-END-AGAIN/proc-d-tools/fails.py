"""fails.py <log> [maxlines] — per failing test: name, then the first message lines before its (fail) marker, skipping source echo lines"""
import sys,re
log=open(sys.argv[1],encoding="utf8",errors="replace").read().split("\n")
mx=int(sys.argv[2]) if len(sys.argv)>2 else 6
start=0
for i,l in enumerate(log):
    if l.startswith("(fail) ") or l.startswith("(pass) "):
        if l.startswith("(fail) "):
            chunk=[x for x in log[start:i] if x.strip() and not re.match(r"^\d+ \|",x) and not x.startswith("[script]") and not x.strip().startswith("^") and "..." not in x[:4]]
            print("FAIL",l[7:120])
            for x in chunk[:mx]: print("    ",x[:230])
        start=i+1
