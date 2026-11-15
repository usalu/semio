#!/bin/bash
python3 - <<'PY'
import json,glob
f=glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/roe-deer/🔣️.json")[0]
old=[t for t in json.load(open(f))["asset"]["volumes"]["500"]["thermal"] if t["id"]=="th-eyes-head"]
json.dump(old,open("🗑️generated/author-29/roe-eyes500.json","w"))
PY
./author-29-bun.sh kit-thermal.ts animals/roe-deer --scale 500 --write --map "hooves=far-hind-leg-hoof+near-hind-leg-hoof+far-fore-leg-hoof+near-fore-leg-hoof,coat-hindlimbs=far-thigh+near-thigh+far-hind-leg+near-hind-leg+far-hind-leg-cannon+near-hind-leg-cannon,coat-forelimbs=far-fore-upper+near-fore-upper+far-fore-leg+near-fore-leg+far-fore-leg-cannon+near-fore-leg-cannon,coat-trunk=hindbody+chest+neck,coat-head=head,nose-skin=head,eyes=head,ears=ears" > /dev/null 2>&1
python3 - <<'PY'
import json
f="volumes/animals/roe-deer.json"
d=json.load(open(f))
th=d["volumes"]["500"]["thermal"]
eyes=json.load(open("🗑️generated/author-29/roe-eyes500.json"))
th=[t for t in th if not (t["region"] in ("eyes",) or (t["region"]=="nose-skin" and "of" in t))]
th+=eyes
d["volumes"]["500"]["thermal"]=th
json.dump(d,open(f,"w"),separators=(",",":"),ensure_ascii=False)
print("thermal 500:",len(th),[t["id"] for t in th if t["region"] in ("eyes","nose-skin")])
PY
