"""[gram3-g] prints the regex literals (text + enclosing line) of the hottest `literal <file>:<line>` rows of a .count file. usage: rxsites.py <count> [top]"""
import re, sys
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
rows = []
for line in open(sys.argv[1], encoding="utf8"):
    n, _, key = line.rstrip("\n").split("\t", 2)
    if key.startswith("literal "):
        rows.append((int(n), key[8:]))
rows.sort(reverse=True)
for n, site in rows[: int(sys.argv[2]) if len(sys.argv) > 2 else 20]:
    file, ln = site.rsplit(":", 1)
    import os
    patched = "/private/tmp/gram3-g/base/" + file
    path = patched if os.path.exists(patched) else B + file if not file.startswith("/") else file
    try:
        text = open(path, encoding="utf8").read().split("\n")[int(ln) - 1]
    except Exception as e:
        text = "?" + str(e)
    lits = re.findall(r"(?<![\w)\]])/(?![/*])(?:\\.|\[(?:\\.|[^\]\\])*\]|[^/\\\n])+/[dgimsuvy]*", text)
    print(f"{n:>9} {file}:{ln}  {lits[:4]}")
