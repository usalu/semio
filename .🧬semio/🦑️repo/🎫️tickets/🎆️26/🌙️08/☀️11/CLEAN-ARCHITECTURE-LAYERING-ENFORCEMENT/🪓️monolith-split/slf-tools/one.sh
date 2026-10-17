#!/bin/zsh
# usage: one.sh <phase> <label>  runs one route and prints per-file status
. "$(dirname "$0")/env.sh"
zsh "$T/vt.sh" "$1" "$2"
node "$T/vsum.mjs" "$1" "$2" >/dev/null
node -e '
const s=JSON.parse(require("fs").readFileSync(process.env.G+"/slf-sum-"+process.argv[1]+".json","utf8"))[process.argv[2]];
console.log(process.argv[2],"files",s.files,"loaded",s.loaded,"tests",s.tests,"passed",s.passed,"failed",s.failed);
for(const [f,r] of Object.entries(s.perFile)) if(!r.loaded||r.failed) console.log(" ",r.loaded?"FAILS("+r.failed+")":"NOLOAD",f.slice(-100),"\n      ",r.loadError??"");
' "$1" "$2"
