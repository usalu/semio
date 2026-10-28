"""🔁 [backlog2] re-cuts the staged nxlock batch onto the LIVE leaseord `pin` class: no `graph-pin` class (live `pin`, rank 0, already owns `build-graph:` and `rust-code:` with explicit edges), the pin tests of nxlock go back to the base text (live's own pin tests stand); `daemon-startup` (rank 8) and the inert `daemon-slot` stay. Idempotent; originals kept as *.pre-backlog2. usage: python3 -I recut_nxlock.py"""
import json, os, re, shutil
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
D = C + "/nxlock-g-tools/staged/nxlock/files"; B = C + "/backlog-tools/merge/base0/nxlock"
def rd(p): return open(p, encoding="utf8").read()
def keep(p):
    if not os.path.exists(p + ".pre-backlog2"): shutil.copyfile(p, p + ".pre-backlog2")
def write(p, t):
    keep(p); h = os.stat(p).st_mode & 0o7777; open(p, "w", encoding="utf8").write(t); os.chmod(p, h)
policy = json.loads(rd(D + "/f00"))
policy["classes"] = [row for row in policy["classes"] if row["id"] != "graph-pin"]
write(D + "/f00", json.dumps(policy, indent=2, ensure_ascii=False) + "\n")
t = rd(D + "/f01"); base = rd(B + "/f001")
entry = '"build-graph:/r/snapshots/a.json":"graph-pin",'
assert t.count(entry) == 1; t = t.replace(entry, "")
first = t.index('test("the authored policy lets any holder wait for a pinned graph snapshot')
after = t.index('test("the start of an Nx daemon is the innermost lease')
b0 = base.index('test("the authored policy lets a graph holder wait for the pinned graph snapshot'); b1 = base.index("\n});\n", b0) + 5
t = t[:first] + base[b0:b1] + "\n" + t[after:]
assert "graph-pin" not in t
write(D + "/f01", t)
t = rd(D + "/f09"); assert t.count('"build-graph:":"graph-pin"') == 1
write(D + "/f09", t.replace('"build-graph:":"graph-pin"', '"build-graph:":"pin"'))
print("nxlock re-cut onto live pin: graph-pin removed from f00/f01/f09")
