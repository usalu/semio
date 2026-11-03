import json, sys
for f in sys.argv[1:]:
    raw = open(f, "rb").read()
    t = raw.decode("utf-8-sig")
    d = json.loads(t)
    r = json.dumps(d, indent=2, ensure_ascii=False)
    print(f.split("/")[-2], raw[:3] == b"\xef\xbb\xbf", b"\r\n" in raw, t == r, t == r + "\n", len(t), len(r))
