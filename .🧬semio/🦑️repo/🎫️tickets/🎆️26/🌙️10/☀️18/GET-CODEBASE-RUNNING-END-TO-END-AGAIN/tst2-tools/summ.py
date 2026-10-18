import json,sys,collections
rows=[json.loads(l) for l in open(sys.argv[1],encoding="utf-8")]
ok=[r for r in rows if r["rc"]==0]; bad=[r for r in rows if r["rc"]!=0]
print("files",len(rows),"ok",len(ok),"bad",len(bad),"pass",sum(r["pass_"] for r in rows),"fail",sum(r["fail"] for r in rows),"skip",sum(r["skip"] for r in rows))
for r in bad:
    print(r["rc"],f'p{r["pass_"]} f{r["fail"]} e{r["err"]}',r["secs"],r["file"].replace("🧰️framework/🛍️products/💻️os/","OS/").replace("🧰️framework/🔨️modules/","M/").replace("✏️s/🔌️plugins/","PL/"))
