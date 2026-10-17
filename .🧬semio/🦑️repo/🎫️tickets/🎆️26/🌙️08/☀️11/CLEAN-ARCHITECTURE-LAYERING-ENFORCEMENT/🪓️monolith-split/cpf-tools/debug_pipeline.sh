source "$(dirname "$0")/env.sh"
nice -n 10 bun $T/debug-census.ts > $G/cpf-debug-census-p.tsv 2> $G/cpf-debug-census-p.err
tail -1 $G/cpf-debug-census-p.err
python3 - <<'PY'
import os,re,json
G=os.environ["G"]
fs=sorted(set(l.split("\t")[0].rsplit(":",1)[0] for l in open(G+"/cpf-debug-census-p.tsv",encoding="utf-8")))
json.dump([p for p in fs if re.search(r"\.(ts|tsx|mjs)$",p)],open(G+"/cpf-debug-ts-files.json","w"),ensure_ascii=False)
PY
(cd $T && bun ts_debug.ts $G/cpf-debug-ts-files.json)
python3 - <<'PY'
import os,json
G=os.environ["G"]
occ=json.load(open(G+"/cpf-ts-debug-occ.json",encoding="utf-8"))
head=set(l.strip()[:200] for l in open(G+"/cpf-head-debug-lines.txt",encoding="utf-8",errors="replace"))
cache={}
for o in occ:
    p=o["path"]
    if p not in cache: cache[p]=open(p,encoding="utf-8").read().split("\n")
    o["new"]=cache[p][o["line"]-1].strip()[:200] not in head
json.dump(occ,open(G+"/cpf-ts-debug-occ2.json","w"),ensure_ascii=False)
print("new occ",sum(1 for o in occ if o["new"]))
PY
(cd $T && bun ts_debug_apply.ts "$@")
