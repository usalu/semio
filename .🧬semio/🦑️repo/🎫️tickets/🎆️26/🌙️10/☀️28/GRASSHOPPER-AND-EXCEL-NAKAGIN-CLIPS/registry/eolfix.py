import os, re, sys
p = os.path.abspath(sys.argv[1])
d = open(p, "rb").read()
d = re.sub(rb"(\r{2,})\n", lambda m: m.group(1) + b"\r\r\n", d)
with open(p, "r+b") as f:
    f.seek(0); f.write(d); f.truncate()
print("ok", len(d))
