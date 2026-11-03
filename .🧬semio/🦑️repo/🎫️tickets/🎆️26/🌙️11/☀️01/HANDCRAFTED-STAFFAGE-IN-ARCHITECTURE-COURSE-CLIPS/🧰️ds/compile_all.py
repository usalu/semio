import py_compile, sys, tempfile, os

bad = 0
n = 0
for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    if p.endswith(".py") and os.path.exists(p):
        n += 1
        try:
            py_compile.compile(p, cfile=os.path.join(tempfile.gettempdir(), "ds.pyc"), doraise=True)
        except Exception as e:
            bad += 1
            print("FAIL", p, e)
print("compiled", n, "failed", bad)
