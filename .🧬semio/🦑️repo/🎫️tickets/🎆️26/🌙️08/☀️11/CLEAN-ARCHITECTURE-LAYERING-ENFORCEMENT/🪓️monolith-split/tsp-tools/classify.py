import sys,re,collections,os
root="/Users/ueli/Documents/semio/"
rows=[]
for p in open(sys.argv[1]).read().split("\n"):
    if not p: continue
    try: t=open(root+p,encoding="utf8").read()
    except Exception as e: rows.append(("MISSING",p)); continue
    if re.search(r'from\s+["\']bun:test["\']',t): k="bun"
    elif re.search(r'from\s+["\']vitest["\']',t): k="vitest"
    elif re.search(r'(^|\n)\s*def test_|import pytest|unittest',t): k="py"
    elif p.endswith(".py"): k="py-other"
    else: k="ts-other"
    rows.append((k,p))
c=collections.Counter(k for k,_ in rows)
print(c)
open(sys.argv[2],"w").write("\n".join(k+"\t"+p for k,p in rows)+"\n")
