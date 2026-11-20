#!/bin/zsh
# R225 staging of the landing state: base copy of the live library plugin file, the new version (two edits), the new member files, land.tsv. Usage: R225_SCRATCH=<scratch dir> zsh prepare.sh
S=${R225_SCRATCH:?scratch dir}
HERE=${0:A:h}
LIVE=${R225_LIVE:-/Users/ueli/Documents/semio}
L=🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
MEMBER=$L/🏗️build/🧮️task-model/🧿️hash-view
ST=$S/state
rm -rf "$ST"; mkdir -p "$ST/base/$L" "$ST/new/$L"
cp "$LIVE/$L/🟨️.mjs" "$ST/base/$L/🟨️.mjs"
python3 -I - "$ST/base/$L/🟨️.mjs" "$ST/new/$L/🟨️.mjs" <<'PY'
import sys
s=open(sys.argv[1]).read()
a='import {remoteGuardInputsV1} from "./🏗️build/🧮️task-model/🛡️remote-guard/🟦️.ts";\n'
b='return {projects:shareNxProjectInputsV1(splitNxPlatformInputsV1(liftNxNamedInputReferencesV1(projects))),externalNodes:graph.externalNodes??{}};'
assert s.count(a)==1 and s.count(b)==1 and "holdNxHashViewV1" not in s
s=s.replace(a,a+'import {holdNxHashViewV1} from "./🏗️build/🧮️task-model/🧿️hash-view/🟦️.ts";\n')
s=s.replace(b,'return {projects:holdNxHashViewV1(shareNxProjectInputsV1(splitNxPlatformInputsV1(liftNxNamedInputReferencesV1(projects))),{...graph.namedInputs,[engineNamedInput]:engine,[engineNativeNamedInput]:engineNative}),externalNodes:graph.externalNodes??{}};')
open(sys.argv[2],"w").write(s)
PY
[ $? = 0 ] || exit 1
for f in 🟦️.ts 🧬️schema/🔣️.json 🧪️tests/🟦️.ts 🧪️tests/🐍️oracle/🐍️.py 🧪️tests/🧫️corpus/🔣️.json; do mkdir -p "$(dirname "$ST/new/$MEMBER/$f")"; cp "$S/mirror/$MEMBER/$f" "$ST/new/$MEMBER/$f"; done
printf 'N\t%s\n' "$MEMBER/🟦️.ts" "$MEMBER/🧬️schema/🔣️.json" "$MEMBER/🧪️tests/🐍️oracle/🐍️.py" "$MEMBER/🧪️tests/🧫️corpus/🔣️.json" "$MEMBER/🧪️tests/🟦️.ts" > "$ST/land.tsv"
printf 'M\t%s\n' "$L/🟨️.mjs" >> "$ST/land.tsv"
echo "state ready: $ST"; cat "$ST/land.tsv"
