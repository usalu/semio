#!/bin/zsh
# ⛓️ The whole generator chain of one person of HW-10 … HW-18, one process after the other: the worker's generator (writes the catalog file), the finisher's named patch where one exists, then the repair ops `HW-1x/repair/<slug>.json` (apply.ts). The repair ops are idempotent, so the chain may be run again on its own result. chain.sh <slug>…   (STEP=repair runs the last step only)
source "${0:A:h}/env.sh"
typeset -A GENERATOR PATCH
GENERATOR=(
  picnicker-sitting-on-ground "HW-10|python3 picnicker-sitting-on-ground.py ship"
  person-eating-standing "HW-10|python3 person-eating-standing.py ship"
  person-drinking-coffee "HW-10|python3 person-drinking-coffee.py ship"
  waving-person "HW-11|$B build-waving-person.ts"
  photographer-with-camera "HW-11|$B build-photographer-with-camera.ts"
  person-taking-selfie "HW-11|$B build-person-taking-selfie.ts"
  standing-violinist "HW-12|$B build-violinist.ts"
  painter-at-easel "HW-12|$B build-painter.ts"
  dancer "HW-12|$B build-dancer.ts"
  person-sweeping "HW-13|python3 sweeping.py"
  person-hanging-laundry "HW-13|python3 laundry.py"
  person-carrying-shopping-bags "HW-13|python3 bags.py"
  postal-carrier "HW-14|$B gen-postal-carrier.ts"
  person-with-guide-dog "HW-15|$B gen-guide-dog.ts"
  person-signing "HW-15|$B gen-signing.ts"
  person-with-arm-sling "HW-15|$B gen-arm-sling.ts"
  person-in-rain-poncho "HW-16|$B poncho.ts"
  person-in-winter-coat "HW-16|$B winter.ts"
  sunbather-on-towel "HW-16|$B sunbather.ts"
  person-queuing "HW-17|python3 gen_person_queuing.py"
  person-checking-watch "HW-17|python3 gen_person_checking_watch.py"
  parent-with-child-on-shoulders "HW-17|python3 gen_parent_with_child_on_shoulders.py"
  worker-with-pallet-jack "HW-18|$B pallet-jack.ts"
  person-carrying-ladder "HW-18|$B ladder.ts"
)
PATCH=(dancer 1 person-in-rain-poncho 1 person-in-winter-coat 1 sunbather-on-towel 1 person-checking-watch 1 person-carrying-shopping-bags 1 picnicker-sitting-on-ground 1 parent-with-child-on-shoulders 1 person-with-guide-dog 1)
for a in "$@"; do
  entry="${GENERATOR[$a]}"; [[ -z "$entry" ]] && { echo "$a: not a person of HW-10 … HW-18"; continue; }
  folder="${entry%%|*}"; command="${entry#*|}"
  if [[ "$STEP" != repair ]]; then
    mkdir -p "$K/🗑️generated/$folder"
    (cd "$K/🧰️scripts/$folder" && ${=command}) > "$GEN/chain-$a.log" 2>&1 || { echo "$a: generator failed, see chain-$a.log"; continue; }
    [[ -n "${PATCH[$a]}" ]] && (cd "$K/🧰️scripts/FIN-B" && python3 patch.py "$a" --ship) >> "$GEN/chain-$a.log" 2>&1
  fi
  if [[ -f "$K/🧰️scripts/$folder/repair/$a.json" ]]; then (cd "$W" && "$B" apply.ts "$a") | tail -1; else echo "$a: no repair ops"; fi
done
