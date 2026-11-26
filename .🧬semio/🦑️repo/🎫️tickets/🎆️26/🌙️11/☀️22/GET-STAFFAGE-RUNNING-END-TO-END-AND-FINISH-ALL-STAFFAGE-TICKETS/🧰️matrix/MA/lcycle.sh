#!/bin/zsh
# 🧶️ The dry run of the loft repair over several assets in ONE grant of the memory queue (800 MB), nothing of the catalog is written: lcycle.sh <id>…   STEPS="propose build gate measure look" selects the steps; LBASE=<dir> the folder of the base documents (default the backup).
#   propose   `loft/<id>.json` from the base by `loft.ts --propose` when it does not exist yet (hand-made ops of `loft/<id>.extra.json` appended)
#   build     the trial copy `$GEN/loft/<id>.json` = base + `loft/<id>.json`, then a second run that must change nothing; then, when `loft/<id>.lofts.json` exists, the second step `../MX-2/loft.ts` with it, twice
#   gate      `../MR/gate.ts --twins` of the copy (poses, clip instants, transitions); POSES_ONLY=1 keeps the poses
#   measure   `../MR/measure.ts` of backup and copy
#   prints    `../MC/prints.ts` of the copy (every authored band at its key and at both ends of its range)
#   look      sheets of backup and copy by `sheets.sh`
BATCH=MA source "${0:A:h}/../MR/env.sh"
g="$GEN/loft"; mkdir -p "$g" "$W/loft"
for a in "$@"; do
  base="${LBASE:-$GEN/backup}/$a.json"
  for step in ${=STEPS:-propose build gate measure}; do
    case "$step" in
      propose) [[ -f "$W/loft/$a.json" ]] || { (cd "$T" && "$B" "$W/loft.ts" "$base" --propose --turns "$W/turns/$a.json" > "$g/$a.ops" 2> "$g/$a.propose.log"); python3 -c "import json,sys,os; ops=json.load(open(sys.argv[1])); e=json.load(open(sys.argv[2])) if os.path.exists(sys.argv[2]) else []; open(sys.argv[3],'w').write('[\n'+',\n'.join(' '+json.dumps(o,ensure_ascii=False) for o in ops+e)+'\n]\n'); print(len(ops),'ops proposed')" "$g/$a.ops" "$W/loft/$a.extra.json" "$W/loft/$a.json"; } ;;
      build) cp "$base" "$g/$a.json"; chmod u+w "$g/$a.json"
        (cd "$T" && "$B" "$W/loft.ts" "$g/$a.json" --ops "$W/loft/$a.json" --write > "$g/$a.build1.log" 2>&1); (cd "$T" && "$B" "$W/loft.ts" "$g/$a.json" --ops "$W/loft/$a.json" --write > "$g/$a.build2.log" 2>&1)
        [[ -f "$W/loft/$a.lofts.json" ]] && for i in 1 2; do (cd "$S/📦️packages/🟦️typescript" && "$B" "$M/../MX-2/loft.ts" "$g/$a.json" --ops "$W/loft/$a.lofts.json" --write > "$g/$a.body$i.log" 2>&1); done
        [[ -f "$W/loft/$a.lofts.json" ]] && echo "$a body: $(tail -2 "$g/$a.body1.log" | tr '\n' ' ') | second: $(tail -2 "$g/$a.body2.log" | tr '\n' ' ')"
        grep -q "changed something" "$g/$a.build1.log" || { echo "$a BUILD FAILED: $(grep -m1 -i error "$g/$a.build1.log" | cut -c1-200)"; continue 2; }
        echo "$a build: $(tail -2 "$g/$a.build1.log" | tr '\n' ' ') | second: $(tail -2 "$g/$a.build2.log" | tr '\n' ' ')" ;;
      gate) (cd "$T" && timeout 3600 "$B" "$M/gate.ts" "$g/$a.json" --twins ${POSES_ONLY:+--poses} > "$g/$a.gate.log" 2>&1); echo "$a $(tail -1 "$g/$a.gate.log")"; grep HARD "$g/$a.gate.log" | cut -c1-230 ;;
      measure) (cd "$S/📦️packages/🟦️typescript" && timeout 1500 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$g/$a.json" > "$g/$a.measure.log" 2>&1); cut -c1-260 "$g/$a.measure.log" ;;
      prints) (cd "$T" && timeout 900 "$B" "$U/🧰️matrix/MC/prints.ts" "$g/$a.json" 2>&1 | grep -v '^\[' | cut -c1-220 | tee "$g/$a.prints.log") ;;
      look) "$W/sheets.sh" "loft:$g/$a.json" ;;
    esac
  done
done
