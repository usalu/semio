import sys, re, os, shutil
src = sys.argv[1]; out = sys.argv[2]; apply = len(sys.argv) > 3 and sys.argv[3] == "apply"
lines = open(src, encoding="utf8").read().split("\n")
if lines[-1] == "": lines.pop()
PKG = lines[0]
assert PKG == "package semio:framework@1.0.0;", PKG
assert lines[1] == ""
chunks = []
i = 2
n = len(lines)
while i < n:
    if lines[i] == "":
        i += 1; continue
    start = i
    while lines[i].startswith("///") or lines[i].startswith("//"): i += 1
    m = re.match(r"^(interface|world) ([a-z0-9-]+) \{$", lines[i])
    assert m, (i + 1, lines[i])
    kind, name = m.groups()
    j = i
    while lines[j] != "}": j += 1
    chunks.append((kind, name, start, j))
    i = j + 1
for kind, name, s, e in chunks:
    print(f"{kind} {name} {s+1}-{e+1} ({e-s+1} lines)")
rebuilt = []
for kind, name, s, e in chunks:
    rebuilt += lines[s:e+1]
orig = [l for l in lines[2:] if l != ""]
assert [l for l in rebuilt if l != ""] == orig, "lossy"
def fname(kind, name): return "📜️.wit" if kind == "world" else f"📜️{name}.wit"
for kind, name, s, e in chunks:
    text = PKG + "\n\n" + "\n".join(lines[s:e+1]) + "\n"
    p = os.path.join(out, fname(kind, name))
    if apply:
        open(p + ".new", "w", encoding="utf8").write(text)
print("ok", len(chunks))
