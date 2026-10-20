#!/bin/sh
# 🧪 Second-set verification: overlay with the hygiene and facade-tail sources, apply.py then apply2.py on it, manifest differential, rustfmt parse, ring check, oracle on a 30-crate family and two seeded breaks.
# usage: sh overlay2.sh  (run overlay.sh first; read-only on the checkout; one heavy process at a time)
set -eu
R=/Users/ueli/Documents/semio
H="$(cd "$(dirname "$0")" && pwd)"
O=/private/tmp/gran-value
F="🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
G="$H/../../🗑️generated/gran-value/oracle"
rm -rf "$O/overlay/pre2" "$O/overlay/post2" "$O/ledger-post2a" "$O/ledger-post2b"
cp -R "$O/overlay/pre" "$O/overlay/pre2"
python3 "$H/mkoverlay2.py" "$O/overlay/pre2"
cp -R "$O/overlay/pre2" "$O/overlay/post2"
python3 "$H/apply.py" --root "$O/overlay/post2" --write --ledger "$O/ledger-post2a" | tail -1
python3 "$H/apply2.py" --root "$O/overlay/post2" --write --ledger "$O/ledger-post2b" | tail -2
bun "$H/verify_manifests2.ts" "$O/overlay/pre2" "$O/overlay/post2"
python3 "$H/check_rings.py" "$O/overlay/post2"
python3 - <<'PY'
import json
fam = json.load(open("/private/tmp/gran-value/fam/valuecore.json"))["crates"]
extra = ["semio-framework-time-travel", "semio-framework-manifest-mutation-inputs", "semio-framework-os-store-one-item-batch", "semio-framework-pack-record-encode", "semio-framework-plugin-tool-run", "semio-framework-replication-mutation-retained"]
json.dump({"name": "valuetail", "crates": sorted(set(fam) | set(extra))}, open("/private/tmp/gran-value/fam/valuetail.json", "w"))
PY
python3 "$H/oracle.py" run "$O/overlay/pre2" "$O/fam/valuetail.json" tailpre --cfgs lib > "$O/oracle-tailpre.log" 2>&1
python3 "$H/oracle.py" run "$O/overlay/post2" "$O/fam/valuetail.json" tailpost --cfgs lib > "$O/oracle-tailpost.log" 2>&1
python3 "$H/oracle_diff.py" "$G/gv-valuetail-tailpre-cur-lib-final.err" "$G/gv-valuetail-tailpost-cur-lib-final.err" | head -3
T="🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/📨️command/🦀️.rs"
python3 "$H/oracle.py" run "$O/overlay/post2" "$O/fam/valuetail.json" tailseedname --cfgs lib --mutate "semio-framework-time-travel::$T@@__m_semio_framework_value_retirement_queue::RetirementQueue;@@__m_semio_framework_value_retirement_queue::RetirementQueueX;" > "$O/oracle-seed1.log" 2>&1
python3 "$H/oracle.py" run "$O/overlay/post2" "$O/fam/valuetail.json" tailseedfacade --cfgs lib --mutate "semio-framework-time-travel::$T@@__m_semio_framework_value_retirement_queue::RetirementQueue;@@__m_semio_framework_value::retirement_queue::RetirementQueue;" > "$O/oracle-seed2.log" 2>&1
for s in tailseedname tailseedfacade; do echo "== $s"; python3 "$H/oracle_diff.py" "$G/gv-valuetail-tailpre-cur-lib-final.err" "$G/gv-valuetail-$s-cur-lib-final.err" | head -3; done
