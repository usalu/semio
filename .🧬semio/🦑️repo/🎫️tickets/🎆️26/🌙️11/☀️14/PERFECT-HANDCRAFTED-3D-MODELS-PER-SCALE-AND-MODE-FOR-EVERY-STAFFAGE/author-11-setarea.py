import re, sys, json


def setareas(path, values):
    s = open(path, encoding="utf8").read()
    i = s.index('\n    "thermal": {')
    j = s.index('"emitters"', i)
    blk = s[i:j]
    for rid, new in values.items():
        pat = re.compile(r'(\n          "id": "' + re.escape(rid) + r'",\n(?:(?!\n          "id": ).)*?"areaM2": )([0-9.e-]+)', re.S)
        m = pat.search(blk)
        assert m, rid
        print(path.split("/")[-2], rid, m.group(2), "->", new)
        blk = blk[: m.start(2)] + str(new) + blk[m.end(2):]
    s = s[:i] + blk + s[j:]
    json.loads(s)
    open(path, "w", encoding="utf8").write(s)


if __name__ == "__main__":
    setareas(sys.argv[1], json.loads(sys.argv[2]))
