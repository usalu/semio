s = open(r"c:\git\semio\semio\client\lib\rs\lib.rs", encoding="utf-8").read().splitlines()
for i, line in enumerate(s):
    if "fn relay_auth_designs_piece_ids" in line:
        q = s[i + 1].strip().strip(",").strip('"')
        print(q)
        print("opens", q.count("{"), "closes", q.count("}"))
        d = 0
        for c in q:
            d += 1 if c == "{" else (-1 if c == "}" else 0)
        print("depth", d)
        break
