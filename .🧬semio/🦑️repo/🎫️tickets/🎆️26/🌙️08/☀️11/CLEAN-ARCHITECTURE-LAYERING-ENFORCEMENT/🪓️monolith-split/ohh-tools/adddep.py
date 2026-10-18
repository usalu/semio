"""➕ OHH manifest dependency adder: appends one dependency line to the end of a section of a Cargo.toml (idempotent, anchor-checked).
usage: adddep.py <Cargo.toml> <dependencies|dev-dependencies> '<line>'"""
import re, sys
p, sec, line = sys.argv[1:4]
t = open(p, encoding="utf-8").read()
name = line.split("=")[0].strip()
m = re.search(r"^\[%s\][ \t]*\n" % re.escape(sec), t, re.M)
assert m, f"no [{sec}] in {p}"
nxt = re.search(r"^\[", t[m.end():], re.M)
stop = m.end() + nxt.start() if nxt else len(t)
body = t[m.end():stop]
if any(re.match(r"^%s\s*(=|\.)" % re.escape(name), l.strip()) for l in body.split("\n")):
    print("present:", p[-60:], name)
    sys.exit(0)
tail = body.rstrip("\n")
gap = body[len(tail):]
out = t[:m.end()] + tail + ("\n" if tail else "") + line + ("\n" if not gap else gap) + t[stop:]
open(p, "w", encoding="utf-8").write(out)
print("added:", p[-60:], line[:60])
