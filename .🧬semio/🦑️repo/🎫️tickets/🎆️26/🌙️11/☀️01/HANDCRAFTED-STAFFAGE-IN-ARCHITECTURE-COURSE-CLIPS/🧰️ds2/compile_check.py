import os, sys
ok = bad = 0
for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    p = p.strip()
    if not p.endswith(".py") or not os.path.exists(p):
        continue
    try:
        compile(open(p, encoding="utf-8").read(), p, "exec", dont_inherit=True)
        ok += 1
    except SyntaxError as e:
        bad += 1
        print("FAIL", p, str(e)[:200])
print("compiled", ok, "failed", bad)
