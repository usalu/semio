import os, re, sys
# 🧭️ nearest.py <repo-rel file>... : ancestors' package manifests (<dir>/Cargo.toml and <dir>/📦️packages/🦀️rust/Cargo.toml), nearest first, with their lines matching PAT (default semio-framework-job\b).
R = "/Users/ueli/Documents/semio"
pat = re.compile(os.environ.get("PAT", r"semio-framework-job\b"))
for f in sys.argv[1:]:
    d = os.path.dirname(os.path.join(R, f))
    print("##", f[-70:])
    n = 0
    while d.startswith(R) and d != R and n < 3:
        for c in (os.path.join(d, "Cargo.toml"), os.path.join(d, "📦️packages/🦀️rust/Cargo.toml")):
            if os.path.isfile(c):
                txt = open(c, encoding="utf8").read()
                if "[package]" not in txt: continue
                n += 1
                nm = re.search(r'^name = "([^"]+)"', txt, re.M).group(1)
                lib = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', txt)
                print("  ", os.path.relpath(c, R)[-110:], "|", nm, "| lib", lib.group(1) if lib else None)
                sec = ""
                for i, line in enumerate(txt.split("\n"), 1):
                    s = line.strip()
                    if s.startswith("["): sec = s
                    if pat.search(line) and not s.startswith("[workspace"): print("      ", i, sec, "|", s[:200])
        d = os.path.dirname(d)
