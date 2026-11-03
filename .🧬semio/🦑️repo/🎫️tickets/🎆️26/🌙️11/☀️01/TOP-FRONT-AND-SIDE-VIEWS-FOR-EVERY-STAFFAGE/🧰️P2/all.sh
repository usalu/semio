#!/bin/sh
cd "$(dirname "$0")"
export PYTHONUTF8=1
for m in a_hot_water_cylinder a_kitchen_extractor_hood a_laptop a_mvhr_unit a_pendant_lamp a_room_thermostat a_thermostatic_radiator_valve a_toaster a_shower_head a_server_rack a_vacuum_cleaner; do python $m.py; done > "../🗑️generated/P2/[DEBUG] apply.txt" 2>&1
for a in "$@"; do ./run.sh $a 50,100,200 all; done
