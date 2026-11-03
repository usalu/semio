import sys,re
rows=[l.rstrip("\n").split("\t") for l in open(sys.argv[1],encoding="utf8")][1:]
pat=re.compile(sys.argv[2])
w=int(sys.argv[3]) if len(sys.argv)>3 else 100
for r in rows:
    if pat.search(r[2]):
        f=r[0].split("\\")[-2] if "\\" in r[0] else r[0]
        print(f"{f}|{r[1][-45:]}|{r[2]}|{r[4][:w]}")
