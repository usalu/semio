#!/bin/zsh
# H66 landing: image growth plan + lane sizing. Run ONLY inside a landing slot (:00-:10). Locks: process, caching-cargo (released right after the writes).
set -e
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
H="$T/🧰️scripts/H66";FW="/Users/ueli/Documents/semio/🧰️framework"
IMG="$FW/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image"
POOL="$FW/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧩️image"
DOC="$FW/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor"
for area in process caching-cargo; do
  mkdir "$T/landing.lock.$area" || { echo "lock $area held: $(cat $T/landing.lock.$area/owner)"; exit 1; }
  echo "H66 $(date +%T)" > "$T/landing.lock.$area/owner"
done
trap 'rm -rf "$T/landing.lock.process" "$T/landing.lock.caching-cargo"' EXIT
[ ! -e "$IMG/🪜️growth" ] || { echo "growth module exists"; exit 1; }
mkdir -p "$IMG/🪜️growth/🧬️schema" "$IMG/🪜️growth/🧪️tests"
cp "$H/stage/🍎️image/🪜️growth/🟦️.ts" "$IMG/🪜️growth/🟦️.ts"
cp "$H/stage/🍎️image/🪜️growth/🧬️schema/🔣️.json" "$IMG/🪜️growth/🧬️schema/🔣️.json"
for f in 🟦️.ts 🐍️.py 🔣️.json; do cp "$H/stage/🍎️image/🪜️growth/🧪️tests/$f" "$IMG/🪜️growth/🧪️tests/$f"; done
python3 "$H/pool-patch.py" "$POOL/🟦️.ts"
python3 - "$POOL/🧪️tests/🟦️.ts" "$H/pool-test-append.ts" <<'PY'
import os,sys
p,a=sys.argv[1:3];s=open(p).read()
assert s.count("laneMeasuredNeedBytesV1,repairLaneImagesV1")==1
s=s.replace("laneMeasuredNeedBytesV1,repairLaneImagesV1","laneMeasuredNeedBytesV1,laneSizesV1,repairLaneImagesV1")+open(a).read()
open(p+".h66","w").write(s);os.replace(p+".h66",p)
PY
python3 "$H/doctor-patch.py" "$DOC/🔎️probes/💾️storage/🟦️.ts"
python3 - "$DOC/🧪️tests/🔎️probes/🟦️.ts" "$H/doctor-test-append.ts" <<'PY'
import os,sys
p,a=sys.argv[1:3];s=open(p).read()
x='import {elasticGrowTargetV1,imageGrowPlanV1,imageRepairPlanV1,storageProbeV1} from';assert s.count(x)==1
s=s.replace(x,'import {elasticGrowTargetV1,growShardImagesV1,imageGrowPlanV1,imageRepairPlanV1,storageProbeV1} from')
y='import {toolsProbeV1} from';assert s.count(y)==1
s=s.replace(y,'import {appendNativeImageShardEventV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";\nimport {toolsProbeV1} from')+open(a).read()
open(p+".h66","w").write(s);os.replace(p+".h66",p)
PY
echo landed
