import sys,re
rows=[l.rstrip("\n").split("\t") for l in open(sys.argv[1],encoding="utf8")][1:]
pat=re.compile(sys.argv[2])
for r in rows:
    if pat.search(r[2]):
        f=r[0].split("\\")[-2] if "\\" in r[0] else r[0]
        t=r[3] if r[2].startswith("en") else r[4]
        print(f"{f}|{r[1][-40:]}|{r[2][:22]}|...{t[-90:]}")
