#!/bin/sh
set -e
T=$(cd "$(dirname "$0")" && pwd)
ROOT=/Users/ueli/Documents/semio
P="🧰️framework/🔨️modules/🏃️process"
L="$P/🔒️leases"
I="$P/🧭️routing/📥️invocation/🏃️process"
S="$T/staged"
rm -rf "$S"; mkdir -p "$S"
MODIFIED="$L/🟦️.ts $I/🟦️.ts $I/🧬️schema/📈️progress.json $I/🧪️tests/📈️progress/🟦️.ts $I/🧪️tests/📈️progress/🔣️.json $P/📜️script.ts $P/📋️project.json"
NEW="$L/⏳️wait/🧬️schema/📜️policy.json $L/⚖️fairness/🧪️tests/🟦️.ts $L/⚖️fairness/🧪️tests/🧫️fixtures/🔣️.json $L/⚖️fairness/🧬️schema/🔣️.json $L/⚖️fairness/🧪️tests/📥️worker/🟦️.ts"
python3 - "$ROOT" "$S" "$T" <<'PY'
import sys,os,shutil,hashlib,json
root,stage,tools=sys.argv[1:4]
P="🧰️framework/🔨️modules/🏃️process"
L=P+"/🔒️leases"
I=P+"/🧭️routing/📥️invocation/🏃️process"
modified=[L+"/🟦️.ts",I+"/🟦️.ts",I+"/🧬️schema/📈️progress.json",I+"/🧪️tests/📈️progress/🟦️.ts",I+"/🧪️tests/📈️progress/🔣️.json",P+"/📜️script.ts",P+"/📋️project.json"]
new={L+"/⏳️wait/🧬️schema/📜️policy.json":"policy.json",L+"/⚖️fairness/🧪️tests/🟦️.ts":"fairness-test.ts",L+"/⚖️fairness/🧪️tests/🧫️fixtures/🔣️.json":"fairness-fixture.json",L+"/⚖️fairness/🧬️schema/🔣️.json":"fairness-schema.json",L+"/⚖️fairness/🧪️tests/📥️worker/🟦️.ts":"fairness-worker.ts"}
bases={}
for rel in modified:
    data=open(os.path.join(root,rel),'rb').read()
    bases[rel]=hashlib.sha256(data).hexdigest()
    dst=os.path.join(stage,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);open(dst,'wb').write(data)
for rel,src in new.items():
    assert not os.path.exists(os.path.join(root,rel)),rel
    bases[rel]=None
    dst=os.path.join(stage,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copy(os.path.join(tools,src),dst)
json.dump(bases,open(os.path.join(tools,"bases.json"),'w',encoding='utf8'),indent=2,ensure_ascii=False)
PY
python3 "$T/patch_leases.py" "$S/$L/🟦️.ts" "$T/queue_block.ts"
python3 "$T/patch_progress.py" "$S/$I"
python3 "$T/patch_register.py" "$S/$P"
echo staged
