import json, sys
r = json.load(open("real.json"))
for c in sys.argv[1:]:
    if c not in r: r.append(c)
json.dump(sorted(r), open("real.json", "w"), ensure_ascii=False, indent=1)
