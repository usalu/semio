#!/bin/sh
# usage: refresh.sh <tag>  -> writes 🗑️generated/pi2w7-<tag>-{closure-lib.tsv,closure-test.tsv,tops.json,top-entries.json,top-closures.json}
cd "$(dirname "$0")"
T=$1; G=../🗑️generated
export PKGDIR='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust'
CFG=lib bun closure.ts > $G/pi2w7-$T-closure-lib.tsv 2>$G/pi2w7-$T-closure.err
CFG=test bun closure.ts > $G/pi2w7-$T-closure-test.tsv 2>>$G/pi2w7-$T-closure.err
tail -1 $G/pi2w7-$T-closure-lib.tsv; tail -1 $G/pi2w7-$T-closure-test.tsv
sed -i '' "s#/pi2w7[-a-z0-9]*closure-lib.tsv#/pi2w7-$T-closure-lib.tsv#; " tops.py
python3 tops.py $G/pi2w7-$T-tops.json
python3 - <<PY
import json
from lib import *
t=json.load(open(GEN+"/pi2w7-$T-tops.json"))
P="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
json.dump([P+x["top"] for x in t], open(GEN+"/pi2w7-$T-top-entries.json","w"), ensure_ascii=False)
PY
ENTRIES=$PWD/$G/pi2w7-$T-top-entries.json OUT=$PWD/$G/pi2w7-$T-top-closures.json bun topclosure.ts
