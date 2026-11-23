#!/bin/zsh
# 🏗️ The whole generator chain of a person of HW-01 … HW-09: the worker's generator (the commands of FIN-A/reproduce.sh) writes its output, which is kept as 🗑️generated/REP-A/base/<slug>.json, then apply.py adds the repair ops of REP-A/ops and writes the catalog file. build.sh <slug>…
# NOGEN=1 skips the generator and builds from the kept generator output.
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
mkdir -p "$GEN/base"
for a in "$@"; do
  if [[ -z "$NOGEN" ]]; then
    eval "${RUN[$a]}" > "$GEN/[DEBUG]-generator-$a.log" 2>&1 || { echo "$a GENERATOR FAILED ($(tail -1 "$GEN/[DEBUG]-generator-$a.log" | cut -c1-200))"; continue; }
    cp "$CAT/$a/🔣️.json" "$GEN/base/$a.json"
  fi
  python3 "$W/apply.py" $a "$GEN/base/$a.json" "$CAT/$a/🔣️.json"
done
