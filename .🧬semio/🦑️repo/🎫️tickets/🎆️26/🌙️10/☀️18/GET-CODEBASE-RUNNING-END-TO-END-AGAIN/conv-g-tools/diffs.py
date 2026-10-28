import sys, hashlib, difflib, os, build, ops
for a in sys.argv[1:]:
    p = build.resolve(a); key = hashlib.sha256(p.encode()).hexdigest()[:10]
    b = open("base/"+key, encoding="utf8").read().split("\n"); n = open("new/"+key, encoding="utf8").read().split("\n")
    print("##", a)
    for l in difflib.unified_diff(b, n, lineterm="", n=0):
        if l.startswith(("---", "+++", "@@")): continue
        print(l[:210])
