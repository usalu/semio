import sys, os, re, json
sys.path.insert(0, os.path.dirname(__file__))
import w3_ndp_values_scan as s
code = sys.argv[1]
d = json.load(open(sys.argv[2]))[code]
reads, _ = s.scan(code)
for i in sorted(set(d["miss_en"]) | set(d["miss_de"])):
    for w in reads[i][:2]:
        path, ln = w.rsplit(":", 1)
        line = open(os.path.join(s.ROOT, path), encoding="utf-8").read().split("\n")[int(ln) - 1].strip()
        j = line.find(i)
        print(i, "|", path.split("/")[-2], ln, "|", line[max(0, j - 70): j + len(i) + 40])
