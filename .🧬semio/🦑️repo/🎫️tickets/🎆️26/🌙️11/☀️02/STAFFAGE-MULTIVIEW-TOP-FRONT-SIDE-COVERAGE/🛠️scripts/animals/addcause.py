import re, sys
cause = "below-printable-feature"
for f in sys.argv[1:]:
    raw = open(f, "rb").read()
    t = raw.decode("utf-8")
    if "\r\n" in t: raise SystemExit("crlf " + f)
    n = 0
    def sub(m):
        global n
        n += 1
        return m.group(0) + "\n" + m.group(1) + '"cause": "' + cause + '",'
    t2 = re.sub(r'^( *)"status": "unavailable",', sub, t, flags=re.M)
    t2 = re.sub(r'("cause": "[^"]+",)\n *"cause": "[^"]+",', r"\1", t2)
    open(f, "wb").write(t2.encode("utf-8"))
    print(f.split("/")[-2], n)
