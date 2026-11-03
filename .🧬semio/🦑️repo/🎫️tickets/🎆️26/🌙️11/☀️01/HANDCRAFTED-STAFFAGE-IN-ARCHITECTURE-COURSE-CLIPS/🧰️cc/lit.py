import json, sys
rows = json.load(open(sys.argv[1], encoding="utf-8"))
man = json.load(open(sys.argv[2], encoding="utf-8"))["casts"]
for r in rows:
    if "error" in r or r["call"] == "watt_anchor": continue
    c = r["cast"]
    f = r["file"].split("/🎬️clip/")[-1][:60]
    if c in man:
        m = man[c]
        print(f'{m["asset"]}|{c}|{m.get("view","-")}|{m.get("pose","")}|{f}|L{r["line"]}|{r["call"]}|{json.dumps(r["kw"], ensure_ascii=False)}')
    else:
        print(f'DYN|{c[:60]}|{f}|L{r["line"]}|{r["call"]}|{json.dumps(r["kw"], ensure_ascii=False)}')
