import re, sys, os, subprocess
R = "/Users/ueli/Documents/semio"
files = [l for l in open(sys.argv[1], encoding="utf8").read().split("\n") if l]
dry = len(sys.argv) > 2 and sys.argv[2] == "dry"
A = re.compile(r'((?:join|resolve)\((?![^\n"]*import\.meta)[^"\'`\n]*?,\s*")(🧫️fixtures/)')
C = re.compile(r'(library \+ "/)(🧫️fixtures/)')
B = re.compile(r'("⚡️caching/)(🧫️fixtures/)')
for f in files:
    p = os.path.join(R, f)
    s = open(p, encoding="utf8").read()
    n = A.sub(lambda m: m.group(1) + "🧪️tests/" + m.group(2), s)
    n = C.sub(lambda m: m.group(1) + "🧪️tests/" + m.group(2), n)
    n = B.sub(lambda m: m.group(1) + "🧪️tests/" + m.group(2), n)
    if n != s:
        d = sum(1 for a, b in zip(s.split("\n"), n.split("\n")) if a != b)
        print(d, "lines", f[-100:])
        if not dry: open(p, "w", encoding="utf8").write(n)
    else:
        print(0, "NOCHANGE", f[-100:])
