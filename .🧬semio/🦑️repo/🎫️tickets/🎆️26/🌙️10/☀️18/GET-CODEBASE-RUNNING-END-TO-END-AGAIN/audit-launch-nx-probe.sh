#!/bin/bash
cd /Users/ueli/Documents/semio
for p in "$@"; do
  echo "=== $p"
  NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true NO_COLOR=1 ./node_modules/.bin/nx show project "$p" --json 2>&1 | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{try{const j=JSON.parse(s);console.log(JSON.stringify({name:j.name,root:j.root,targets:Object.keys(j.targets??{}).sort()}))}catch(e){console.log("PARSEFAIL",s.slice(0,300))}})'
done
echo ALLDONE
