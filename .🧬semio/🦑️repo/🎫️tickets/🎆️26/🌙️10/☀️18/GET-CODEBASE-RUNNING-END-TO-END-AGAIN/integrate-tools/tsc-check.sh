#!/bin/bash
# usage: tsc-check.sh <overlay-root> <files-list.txt> <errors-out.txt>
# Type-checks the listed files in the overlay with the repo compiler and the repo options (strict); the few type packages are COPIES.
ov="$1"; list="$2"; out="$3"; live=/Users/ueli/Documents/semio
cd "$ov" || exit 2
mkdir -p node_modules/@types node_modules/bun-types
[ -d node_modules/@types/bun ] || cp -RL "$live/node_modules/@types/bun" node_modules/@types/bun
[ -d node_modules/@types/node ] || cp -RL "$live/node_modules/@types/node" node_modules/@types/node
[ -e node_modules/bun-types/index.d.ts ] || cp -RL "$live/node_modules/.bun/bun-types@1.4.3/node_modules/bun-types/." node_modules/bun-types/
for p in ajv fast-deep-equal json-schema-traverse require-from-string fast-uri minimatch brace-expansion balanced-match; do [ -d "node_modules/$p" ] || { [ -e "$live/node_modules/$p" ] && cp -RL "$live/node_modules/$p" "node_modules/$p"; } ; done
python3 - "$list" <<'PY'
import json,sys
files=[l.strip() for l in open(sys.argv[1],encoding="utf8") if l.strip()]
opts=json.load(open("/Users/ueli/Documents/semio/tsconfig.json"))["compilerOptions"]
opts["types"]=["bun","node"]
json.dump({"compilerOptions":opts,"files":files},open("tsconfig.integrate.json","w"),ensure_ascii=False,indent=1)
PY
"$live/node_modules/.bin/tsc" -p tsconfig.integrate.json > "$out" 2>&1
echo "tsc exit $? errors: $(grep -c 'error TS' "$out")"
