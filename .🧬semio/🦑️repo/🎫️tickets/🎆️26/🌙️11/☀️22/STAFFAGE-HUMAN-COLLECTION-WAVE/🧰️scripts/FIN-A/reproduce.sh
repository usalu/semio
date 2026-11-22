#!/bin/zsh
# ♻️ Runs the generator of a person again and tells whether it reproduces the shipped catalog file (the previous file is kept in 🗑️generated/FIN-A/backup/<slug>.json): reproduce.sh <slug>…
source "${0:A:h}/env.sh"
typeset -A RUN
K="$T/🧰️scripts"
RUN=(
  standing-desk-worker "cd $K/HW-01 && python3 build_desk.py"
  presenter-at-flipchart "cd $K/HW-01 && python3 build_presenter.py"
  office-worker-carrying-laptop "cd $K/HW-01 && python3 build_carrier.py"
  waiter-with-tray "cd $K/HW-02 && python3 waiter.py"
  barista "cd $K/HW-02 && python3 barista.py"
  nurse-walking "cd $K/HW-03 && $B nurse.ts && python3 ../FIN-A/fit_dims.py nurse-walking --write"
  doctor-with-clipboard "cd $K/HW-03 && $B doctor.ts"
  patient-with-iv-pole "cd $K/HW-03 && $B patient.ts"
  construction-worker-with-shovel "cd $K/HW-04 && python3 shovel.py --write"
  worker-with-jackhammer "cd $K/HW-04 && python3 jackhammer.py --write"
  surveyor-with-total-station "cd $K/HW-04 && python3 surveyor.py --write"
  walking-toddler "cd $K/HW-05 && python3 build_toddler.py"
  schoolchild-with-backpack "cd $K/HW-05 && python3 build_schoolchild.py"
  teenager-with-headphones "cd $K/HW-05 && python3 build_teen.py"
  elderly-person-knitting "cd $K/HW-06 && python3 knit_build.py"
  mobility-scooter-user "cd $K/HW-06 && python3 scoot_build.py"
  tennis-player "cd $K/HW-07 && $B tennis-player.ts"
  basketball-player "cd $K/HW-07 && $B basketball-player.ts"
  hiker-with-trekking-poles "cd $K/HW-08 && $B hiker.ts"
  person-stretching "cd $K/HW-08 && $B stretch.ts"
  e-scooter-rider "cd $K/HW-09 && python3 build_e_scooter_rider.py && $B poses_e_scooter_rider.ts"
  traveller-with-rolling-suitcase "cd $K/HW-09 && python3 build_traveller.py && $B poses_traveller.ts && python3 ../FIN-A/fit_dims.py traveller-with-rolling-suitcase --write"
  person-pushing-bicycle "cd $K/HW-09 && python3 build_person_pushing_bicycle.py && $B poses_person_pushing_bicycle.ts"
)
mkdir -p "$GEN/backup"
for a in "$@"; do
  cp "$CAT/$a/🔣️.json" "$GEN/backup/$a.json"
  t0=$(date +%s)
  if ! eval "${RUN[$a]}" > "$GEN/[DEBUG]-reproduce-$a.log" 2>&1; then echo "$a GENERATOR FAILED ($(tail -1 "$GEN/[DEBUG]-reproduce-$a.log" | cut -c1-200))"; cp "$GEN/backup/$a.json" "$CAT/$a/🔣️.json"; continue; fi
  python3 - "$CAT/$a/🔣️.json" "$GEN/backup/$a.json" "$a" $(( $(date +%s) - t0 )) <<'PY'
import json, sys
new, old = (json.load(open(p, encoding="utf-8"))["asset"] for p in sys.argv[1:3])
if new == old:
    print(sys.argv[3], "reproduced identically", sys.argv[4] + " s")
else:
    print(sys.argv[3], "DIFFERENT in", [k for k in new if new[k] != old.get(k)], sys.argv[4] + " s")
PY
done
