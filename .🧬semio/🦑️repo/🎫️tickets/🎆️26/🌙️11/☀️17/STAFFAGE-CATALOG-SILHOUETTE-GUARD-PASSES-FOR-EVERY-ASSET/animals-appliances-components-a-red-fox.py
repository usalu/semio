import sys; sys.path.insert(0, ".")
exec(open("animals-appliances-components-a-lib.py").read())
def f(d):
    for sid in ("hindbody", "chest"):
        sections(d, "50", sid, lambda i, c: {"exponent": 8})
    for sid in ("near-forearm", "far-forearm"):
        sections(d, "50", sid, lambda i, c: {"rx": round(c["rx"] + 0.002, 5)} if i in (1, 2, 3) else {})
run("red-fox", f)
