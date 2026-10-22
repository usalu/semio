#!/bin/sh
# 🔁 Regenerates every staged plan in apply order and checks the composition (one process at a time, cwd independent).
cd "$(dirname "$0")" || exit 1
for b in build_reserved build_macro_plan build_oskernel_macros build_plugin_macros build_derive_plan; do nice -n 10 python3 $b.py 2>&1 | tail -1 | cut -c1-200; done
for h in umbrella plugin os-kernel; do nice -n 10 python3 plan_hub.py $h 2>&1 | tail -2 | head -1 | cut -c1-300; nice -n 10 python3 assemble2.py $h 2>&1 | head -1; done
for h in umbrella plugin os-kernel job; do nice -n 10 python3 plan_dead.py $h 2>&1 | tail -1; done
nice -n 10 python3 compose_check.py umbrella-reserved,macros-elsewhere,umbrella,os-kernel-macros,os-kernel-derive,os-kernel,plugin-macros,plugin,umbrella-dead,plugin-dead,os-kernel-dead,job-dead 2>&1 | tail -13
