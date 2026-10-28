#!/bin/sh
# 🔤 laws of batch `typedecl` (lane audit12): sh verify.sh <root>  (after apply). tsc 5.9 over the G46-touched TS files (C/audit12-tools/tsconfig.g46.json, re-rooted at <root>): the 5 errors G46 added (service tests x3, door, suites test) are gone and only the 3 pre-G46 errors stay (operands, followup, optional-cache); the touched suites keep their results.
ROOT="$(cd "${1:?root}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; . "$HERE/../../verify.common.sh"; applied
cfg="$LOGS/tsconfig.json"; python3 -I -c 'import json,os,sys;c=json.load(open(sys.argv[1],encoding="utf8"));old,new=sys.argv[2],sys.argv[3];c["extends"]=c["extends"].replace(old,new,1);c["files"]=[f for f in (f.replace(old,new,1) for f in c["files"]) if os.path.exists(f)];json.dump(c,open(sys.argv[4],"w",encoding="utf8"),ensure_ascii=False)' "$C/audit12-tools/tsconfig.g46.json" /Users/ueli/Documents/semio "$ROOT" "$cfg"
(cd "$ROOT" && nice -n 10 node --max-old-space-size=8192 node_modules/typescript/bin/tsc -p "$cfg" --pretty false > "$LOGS/tsc.txt" 2>&1)
errs=$(grep -c 'error TS' "$LOGS/tsc.txt"); new=$(grep 'error TS' "$LOGS/tsc.txt" | grep -cvE '🚧️operands/🟦️.ts|🌤️followup/🟦️.ts|🧊️optional-cache/🟦️.ts')
[ "$new" = 0 ] && echo "ok   tsc: $errs errors, all pre-G46" || { echo "FAIL tsc: $new errors beyond the pre-G46 set"; grep 'error TS' "$LOGS/tsc.txt" | grep -vE '🚧️operands|🌤️followup|🧊️optional-cache' | cut -c1-200; FAILED=1; }
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
LAWENV="-u SEMIO_TEST_ARTIFACT_DIR" law 9/0 "$B/🚪️door/🧪️tests/🟦️.ts"
LAWENV="-u SEMIO_TEST_ARTIFACT_DIR" law 119/0 "$B/📥️caller/🛰️service/🧪️tests/🟦️.ts"
LAWENV="-u SEMIO_TEST_ARTIFACT_DIR" law 9/0 "$B/🗂️suites/🧪️tests/🟦️.ts"
parse; mode
[ $FAILED = 0 ] && echo "VERIFY OK typedecl" || echo "VERIFY FAIL typedecl"; exit $FAILED
