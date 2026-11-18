#!/bin/bash
scales=${1:-50,100,200,500}
cd "$(dirname "$0")"
python3 vehicles-restore.py 🚜️vehicles/cargo-bike && python3 vehicles-cb0.py || exit 1
for sc in ${scales//,/ }; do
  python3 vehicles-cb6.py $sc && python3 vehicles-cb1.py $sc elevation >/dev/null && python3 vehicles-cb2.py $sc elevation >/dev/null && python3 vehicles-cb1.py $sc front,plan >/dev/null && python3 vehicles-cb1.py $sc front,plan >/dev/null && python3 vehicles-cb2.py $sc front,plan >/dev/null || exit 1
  [ -f vehicles-cb7-$sc.sh ] && bash vehicles-cb7-$sc.sh
  python3 vehicles-cb4.py $sc front,plan 8 | tail -1
done
