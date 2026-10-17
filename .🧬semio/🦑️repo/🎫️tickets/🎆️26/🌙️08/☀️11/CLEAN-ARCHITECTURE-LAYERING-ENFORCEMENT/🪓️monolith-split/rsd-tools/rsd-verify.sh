#!/bin/sh
# 🔬 Baseline (pre-edit overlay) and post-edit oracle runs for the rii (renderer) and rsdp (plugin) families.
ulimit -n 10240
F="$(cd "$(dirname "$0")/.." && pwd)"
cd "$F/pos-tools" || exit 1
export POS_PREFIX=rsd
python3 ../rsd-tools/rsd_pos.py ../rsd-tools/rsd-overlay.json run ../rii-tools/rii.json --cfgs lib,test --label wb1 --no-base > ../🗑️generated/rsd-wb1-run.log 2>&1
python3 pos.py run ../rii-tools/rii.json --cfgs lib,test --label wa1 --no-base > ../🗑️generated/rsd-wa1-run.log 2>&1
python3 ../rsd-tools/rsd_pos.py ../rsd-tools/rsd-overlay.json run ../rsd-tools/rsdp.json --cfgs lib,test --label pb1 --no-base > ../🗑️generated/rsd-pb1-run.log 2>&1
python3 pos.py run ../rsd-tools/rsdp.json --cfgs lib,test --label pa1 --no-base > ../🗑️generated/rsd-pa1-run.log 2>&1
echo done > ../🗑️generated/rsd-verify.done
