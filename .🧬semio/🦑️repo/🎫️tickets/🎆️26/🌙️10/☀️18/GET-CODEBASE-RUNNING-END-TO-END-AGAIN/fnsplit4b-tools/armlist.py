#!/usr/bin/env python3
"""📋 Lists the arms of the match at a line of a Rust function. usage: armlist.py <file> <fn> <matchLine> [--nth N]"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn
file, fname, mline = sys.argv[1], sys.argv[2], int(sys.argv[3])
nth = int(sys.argv[sys.argv.index("--nth") + 1]) if "--nth" in sys.argv else 0
text = open(file, encoding="utf-8").read()
toks = rsfn.lex(text)
k, op, cl = rsfn.find_fn(toks, fname, nth)
mq = next(q for q in range(op, cl) if toks[q][1] == "match" and toks[q][2] == mline)
mopen = next(q for q in range(mq, cl) if toks[q][1] == "{")
b = 0
for q in range(mopen, len(toks)):
    if toks[q][1] == "{": b += 1
    elif toks[q][1] == "}":
        b -= 1
        if b == 0:
            mclose = q
            break
arms = rsfn.match_arms(toks, mopen, mclose)
for a in arms:
    pat = text[toks[a["ps"]][3]:toks[a["pe"]][3]].strip().replace("\n", " ")
    print("%d-%d\t%d\t%s" % (toks[a["ps"]][2], toks[a["be"]][2], toks[a["be"]][2] - toks[a["ps"]][2] + 1, pat[:110]))
