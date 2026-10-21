#!/bin/sh
# Set 2 (collect-errors): overlay from the live files + staged ops, then the collect laws, the settle law with the new scenario, the python oracles and mutations. Live tree untouched.
set -e
T="$(cd "$(dirname "$0")" && pwd)"; W="$(cd "$T" && git rev-parse --show-toplevel)"; cd "$W"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; G="$L/🏗️build/🕸️graph"; OV=/private/tmp/loop-hunt/ov2; R="$T/🗑️generated"
rm -rf "$OV" "$R"; mkdir -p "$R"
for f in "$L/🏗️build/👁️observation/🟨️.mjs" "$G/📥️inference/📦️project-nodes/🟨️.mjs" "$L/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat/🔣️.json" "$L/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat/🐍️oracle.py"; do mkdir -p "$OV/$(dirname "$f")"; cp "$f" "$OV/$f"; done
python3 "$T/apply.py" apply --root "$OV" --write
for f in "$OV/$G/📥️inference/📦️project-nodes/🟨️.mjs" "$OV/$G/🧾️collect-errors/🟨️.mjs" "$OV/$L/🏗️build/👁️observation/🟨️.mjs"; do bun build --no-bundle "$f" >/dev/null || { echo "PARSE FAIL $f"; exit 1; }; done; echo "parse ok"
# collect law runs inside the repository so ajv resolves
mkdir -p "$R/c"; cp -R "$OV/$G/🧾️collect-errors/." "$R/c/"
echo "--- collect law"; (cd "$R/c/🧪️tests" && bun test ./🟦️.ts 2>&1 | tail -6)
echo "--- collect oracle"; python3 "$R/c/🧪️tests/🐍️oracle.py" "$R/c/🧪️tests/🔣️.json" > "$R/oracle.json"; python3 - "$R" <<'PY'
import json,sys
r=sys.argv[1];o=json.load(open(r+"/oracle.json"));f=json.load(open(r+"/c/🧪️tests/🔣️.json"))
bad=[c["message"][:30] for c,x in zip(f["causes"],o["causes"]) if c["cause"]!=x]
g=[{"cause":x["cause"],"count":x["count"],"projects":x["projects"]} for x in f["expectGroups"]]
print("oracle mismatches:",bad,"groups equal:",g==o["groups"]);sys.exit(1 if bad or g!=o["groups"] else 0)
PY
echo "--- settle law (with collected-errors-are-final)"; mkdir -p "$R/s/🧬️schema"; S="$OV/$L/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat"
cp "$T/staged/new/settle-repeat/test.ts" "$R/s/🟦️.ts"; cp "$S/🔣️.json" "$R/s/🔣️.json"; cp "$T/staged/new/settle-repeat/schema.json" "$R/s/🧬️schema/🔣️.json"
(cd "$R/s" && SETTLE_SOURCE="$OV/$L/🏗️build/👁️observation/🟨️.mjs" bun test ./🟦️.ts 2>&1 | tail -5)
python3 "$S/🐍️oracle.py" "$S/🔣️.json" > "$R/so.json"; python3 -c "
import json;o=json.load(open('$R/so.json'));f=json.load(open('$S/🔣️.json'))
bad=[s['name'] for s in f['scenarios'] if {k:v for k,v in s['expect'].items() if k!='failure' or s['expect']['settled']=='failure'}!=o[s['name']]]
print('settle oracle mismatches:',bad)"
echo "--- mutations of the collector"
python3 - "$R" "$OV/$G/🧾️collect-errors/🟨️.mjs" <<'PY'
import subprocess,sys,pathlib,shutil
R=pathlib.Path(sys.argv[1]);src=pathlib.Path(sys.argv[2]);text=src.read_text(encoding="utf8");run=R/"c/🧪️tests";mod=R/"c/🟨️.mjs"
muts={"swallow-final":('if (isFinal(error)) throw error;',''),"no-final-error":('throw Object.assign(Error(`','return Object.assign(Error(`'),"no-grouping":('const group = grouped.get(row.cause) ??','const group = undefined ??'),"path-before-project":(None,None)}
killed=0;total=0
for name,(a,b) in muts.items():
    if a is None:continue
    assert text.count(a)==1,name;total+=1
    mod.write_text(text.replace(a,b),encoding="utf8")
    r=subprocess.run(["bun","test","./🟦️.ts"],cwd=run,capture_output=True,text=True);dead=r.returncode!=0;killed+=dead;print("killed  " if dead else "SURVIVED",name)
mod.write_text(text,encoding="utf8");print(killed,"of",total,"killed");sys.exit(0 if killed==total else 1)
PY
