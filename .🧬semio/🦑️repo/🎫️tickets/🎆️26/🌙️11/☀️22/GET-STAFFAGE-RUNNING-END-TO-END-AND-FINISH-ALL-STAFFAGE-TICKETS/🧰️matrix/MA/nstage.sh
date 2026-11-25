#!/bin/zsh
# 🪆️ One work stage of the nested-prism repair (`nest.ts`) over several assets as ONE process tree, started through the machine-wide memory queue by `nq.sh`; logs in `$GEN/nest`. nstage.sh <stage> <id>…
#   propose   `nest/<id>.json` from the backup (the asset before this batch): one `nest` op per closed front and plan shape with an elevation twin on a bone that turns; hand-made ops of `nest/<id>.extra.json` are appended
#   build     the trial copy `$GEN/nest/<id>.json` = backup (or, with NEST_BASE=<dir>, `<dir>/<id>.json`: the copy the single-solid chain made) + `nest/<id>.json`, then a second run on the copy that must change nothing
#   gate      `../MR/gate.ts --twins` of the trial copy (poses, clip instants, transitions)
#   measure   `../MR/measure.ts` (silhouette guard, validateAsset, three-views sweep) of backup and trial copy
#   which     `which.ts` of the trial copy (solids the evaluator cannot draw)
#   cycle     the stages of STEPS (default: propose build gate measure) per asset, one asset after the other, in one grant of the queue
#   proof     the proof stages of `stage.sh` named in STEPS over the assets (catalog files), in one grant
#   matrix    `T/📊️matrix.ts --language en --jobs 1 --fresh --assets <id>` once per asset into `$GEN/matrix-<id>`, summary by `../MR/cells.py`
#   look      product sheets (side, front, plan) of backup and trial copy (CATALOG=1: catalog file) at every authored scale in the default pose and at POSE_SCALES in the poses of POSES, into `$GEN/sheets`
#   apply     `nest.ts <id> --ops nest/<id>.json --write` on the catalog file, a second run that must change nothing, then compared with the trial copy
BATCH=MA source "${0:A:h}/../MR/env.sh"
stage="$1"; shift; d="$GEN/nest"; mkdir -p "$d" "$W/nest"
case "$stage" in
  propose)
    for a in "$@"; do
      (cd "$T" && "$B" "$W/nest.ts" "$GEN/backup/$a.json" --propose > "$d/$a.ops.json" 2> "$d/$a.propose.log")
      python3 -c "import json,sys,os; ops=json.load(open(sys.argv[1])); extra=json.load(open(sys.argv[2])) if os.path.exists(sys.argv[2]) else []; json.dump(ops+extra, open(sys.argv[3],'w'), indent=1, ensure_ascii=False); print(len(ops), 'nest ops,', len(extra), 'by hand')" "$d/$a.ops.json" "$W/nest/$a.extra.json" "$W/nest/$a.json"
      echo "$a: $(wc -l < "$d/$a.propose.log" | tr -d ' ') shapes without a twin"
    done ;;
  build)
    for a in "$@"; do
      cp "${NEST_BASE:-$GEN/backup}/$a.json" "$d/$a.json"
      (cd "$T" && "$B" "$W/nest.ts" "$d/$a.json" --ops "$W/nest/$a.json" --turns "$W/turns/$a.json" --write > "$d/$a.build1.log" 2>&1)
      (cd "$T" && "$B" "$W/nest.ts" "$d/$a.json" --ops "$W/nest/$a.json" --turns "$W/turns/$a.json" --write > "$d/$a.build2.log" 2>&1)
      echo "$a: $(tail -1 "$d/$a.build1.log") | second run: $(tail -1 "$d/$a.build2.log") | $(grep -c 'OVER TOLERANCE' "$d/$a.build1.log") over tolerance, $(grep -c REFUSED "$d/$a.build1.log") refused"
    done ;;
  gate) for a in "$@"; do (cd "$T" && timeout 1800 "$B" "$M/gate.ts" "$d/$a.json" --twins > "$d/$a.gate.log" 2>&1); echo "$a gate exit $? $(tail -1 "$d/$a.gate.log")"; grep -E "HARD|reported" "$d/$a.gate.log" | cut -c1-${CUT:-260}; done ;;
  measure) for a in "$@"; do (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$d/$a.json" > "$d/$a.measure.log" 2>&1); cut -c1-330 "$d/$a.measure.log"; done ;;
  which) for a in "$@"; do (cd "$T" && timeout 1500 "$B" "$W/which.ts" "$d/$a.json" > "$d/$a.which.jsonl" 2>&1); echo "$a: $(wc -l < "$d/$a.which.jsonl" | tr -d ' ') shapes the evaluator cannot draw"; head -20 "$d/$a.which.jsonl" | cut -c1-200; done ;;
  apply)
    for a in "$@"; do
      (cd "$T" && "$B" "$W/nest.ts" "$a" --ops "$W/nest/$a.json" --turns "$W/turns/$a.json" --write > "$d/$a.apply1.log" 2>&1)
      (cd "$T" && "$B" "$W/nest.ts" "$a" --ops "$W/nest/$a.json" --turns "$W/turns/$a.json" --write > "$d/$a.apply2.log" 2>&1)
      echo "$a: $(tail -1 "$d/$a.apply1.log") | second run: $(tail -1 "$d/$a.apply2.log") | $(cmp -s "$d/$a.json" "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" && echo "catalog equals the trial copy" || echo "CATALOG DIFFERS FROM THE TRIAL COPY")"
    done ;;
  cycle) for a in "$@"; do for step in ${=STEPS:-propose build gate measure}; do "$0" "$step" "$a"; done; grep -q "evaluate-error" "$d/$a.gate.log" && [[ -z "$NOWHICH" ]] && "$0" which "$a"; done ;;
  proof) for step in ${=STEPS:-gate scan measure vt-silhouette vt-three-views vt-physics vt-breathing vt-validation vt-power}; do "$W/stage.sh" "$step" "$@"; done ;;
  matrix)
    for a in "$@"; do
      rm -rf "$GEN/matrix-$a"; (cd "$T" && timeout ${MATRIX_TIMEOUT:-7200} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$GEN/proof/$a-matrix.log" 2>&1); echo "$a matrix exit $? $(tail -1 "$GEN/proof/$a-matrix.log" | cut -c1-300)"
      python3 "$M/cells.py" "$GEN/matrix-$a" > "$GEN/proof/$a-matrix-cells.txt" 2>&1; head -12 "$GEN/proof/$a-matrix-cells.txt" | cut -c1-330
    done ;;
  look)
    T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
    mkdir -p "$GEN/sheets"
    for a in "$@"; do
      scales=($(python3 -c "import json,sys; print(' '.join(k for k,r in json.load(open(sys.argv[1]))['asset']['representations'].items() if r.get('status')=='authored'))" "$GEN/backup/$a.json"))
      rest="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$GEN/backup/$a.json")"
      for tag in before after; do
        work="$GEN/sheets/work-$tag/$a"; mkdir -p "$work"
        if [[ "$tag" == before ]]; then cp "$GEN/backup/$a.json" "$work/🔣️.json"; elif [[ -n "$CATALOG" ]]; then cp "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$work/🔣️.json"; else cp "$d/$a.json" "$work/🔣️.json"; fi
        for pose in $rest ${=POSES}; do
          for s in ${=SCALES:-$scales}; do
            [[ "$pose" != "$rest" && -n "$POSE_SCALES" && " $POSE_SCALES " != *" $s "* ]] && continue
            png="$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"
            [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$a.s$s.$pose.$tag.png" && echo "$GEN/sheets/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"
          done
        done
        rm -rf "$GEN/sheets/work-$tag"
      done
    done ;;
  *) echo "unknown stage $stage"; exit 2 ;;
esac
