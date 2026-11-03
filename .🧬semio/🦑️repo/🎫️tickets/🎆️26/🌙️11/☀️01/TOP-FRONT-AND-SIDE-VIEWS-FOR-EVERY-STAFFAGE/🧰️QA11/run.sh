#!/bin/sh
# Rebuilds the nine vehicles: VE builders, then the QA11 rig pass. Retries while the file is locked by another process.
cd "$(dirname "$0")/../🧰️VE" || exit 1
export PYTHONIOENCODING=utf-8
for b in bike bus car cargo kick ms tram van vintage; do
  n=0; until python build_$b.py > /dev/null 2>&1; do n=$((n+1)); [ $n -gt 30 ] && { echo "build $b failed"; exit 1; }; sleep 1; done
done
cd ../🧰️QA11 || exit 1
for a in city-bicycle electric-kick-scooter motor-scooter delivery-van cargo-bike; do
  python rig.py $a 2>&1 | tail -1
done
