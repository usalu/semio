import os, re, sys
p = os.path.abspath(sys.argv[1])
d = open(p, "rb").read()
d = re.sub(rb"(\r{3,})\n", lambda m: m.group(1)[:-1] + b"\n", d)
with open(p, "r+b") as f:
    f.seek(0); f.write(d); f.truncate()
