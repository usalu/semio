#!/bin/zsh
# 🏁️ The takeover proof chains of batch MH-2 (2026-11-25), each asset under as few grants of the memory queue as possible (a grant took 5 to 40 minutes of waiting): `takeover.sh matrix <id>…` runs `T/📊️matrix.ts --jobs 1 --fresh --assets <id>` per asset, one after the other, reserving the megabytes written in $GEN/matrix-mb and held to nine tenths of them by `guard.sh`, which also records the peak of the resident size, log in $GEN/proof/<id>-matrix.log, cells by status appended to $GEN/proof/matrix-summary.txt; `takeover.sh proof <id>…` runs under ONE grant of 2500 MB for all the assets given, one after the other, per asset the repair once more (`fit2.ts`, must change nothing), the gate over every subject (`../MR/gate.ts --twins`), the official scan and ONE vitest run over the six real test files filtered by the asset id and the two manifest cases, then under ONE grant of 5000 MB the check CLI of every asset (both grants under `guard.sh`), block appended to $GEN/proof/summary.txt; `takeover.sh sheets <id>…` draws under ONE grant for all the assets given the product sheets of the backup and of the catalog file at every scale the ops name (default pose) and writes their pixel difference.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
mode="$1"; shift; d="$GEN/proof"; mkdir -p "$d" "$GEN/vt" "$GEN/sheets"; P="$S/📦️packages/🟦️typescript"
T14="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
for a in "$@"; do case "$mode" in
  matrix)
    mb="$(cat "$GEN/matrix-mb" 2>/dev/null || echo 7000)"; s=$SECONDS
    rm -rf "$GEN/matrix-$a"; NOLOCK=1 "$W/q.sh" "$mb" ${MATRIX_TIMEOUT:-14400} "$T" "$W/guard.sh" $((mb*9/10)) "$d/$a-matrix.peak" "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1
    { echo "=== C $a exit $? reserved $mb MB, $(cat "$d/$a-matrix.peak" | tr '\n' ' '), $((SECONDS-s)) s with the wait, ended $(date +%H:%M): $(tail -1 "$d/$a-matrix.log")"; python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300; } >> "$d/matrix-summary.txt" ;;
  proof)
    inner="$d/$a-inner.sh"
    { print -r -- "cd \"$T\""
      print -r -- "\"$B\" \"$W/fit2.ts\" \"$a\" --ops \"$W/ops/$a.json\" > \"$d/$a-refit.log\" 2>&1; echo \"refit exit \$? \$(tail -1 \"$d/$a-refit.log\" | cut -c1-160)\""
      print -r -- "\"$B\" \"$M/gate.ts\" \"$a\" --twins > \"$d/$a-gate.log\" 2>&1; echo \"gate exit \$? \$(tail -1 \"$d/$a-gate.log\")\""
      print -r -- "\"$B\" 🔎️scan.ts --language en \"$a\" > \"$d/$a-scan.log\" 2>&1; echo \"scan exit \$? \$(tail -1 \"$d/$a-scan.log\" | cut -c1-200)\""
      print -r -- "cd \"$P\""
      print -r -- "\"$B\" x vitest run --config \"../../🧪️tests/🎚️config/🟦️.ts\" \"../../🧪️tests/🧊️catalog-silhouette/🟦️.ts\" \"../../🧪️tests/🧊️three-views/🟦️.ts\" \"../../🧪️tests/🌡️catalog-physics/🟦️.ts\" \"../../🧪️tests/🫁️breathing-catalog/🟦️.ts\" \"../../🧪️tests/✅️asset-validation/🟦️.ts\" \"../../🧪️tests/⚡️asset-power/🟦️.ts\" -t \"(accepts the sample figure|validates the manifest|$a)\" > \"$GEN/vt/$a-tests.log\" 2>&1"
      print -r -- "echo \"tests exit \$? \$(grep -E '^ *(Test Files|Tests) ' \"$GEN/vt/$a-tests.log\" | tr -s ' ' | tr '\\n' ';') \$(grep -E '^ *(❯|×|FAIL) ' \"$GEN/vt/$a-tests.log\" | tr -s ' ' | tr '\\n' ';' | cut -c1-600)\""
    } > "$inner"
    { print -r -- "cd \"$S\""
      print -r -- "\"$B\" \"🔨️modules/🗂️catalog/🩺️check/🟦️.ts\" --asset \"$a\" > \"$d/$a-check.log\" 2>&1; echo \"check $a exit \$?: \$(grep -m1 -E '^(✓|✗)' \"$d/$a-check.log\" | cut -c1-160)\""
    } >> "$d/check-master-$$.sh"
    print -r -- "echo \"=== A $a \$(date +%H:%M)\"; /bin/zsh \"$inner\"; echo \"--- A $a \$(date +%H:%M)\"" >> "$d/proof-master-$$.sh" ;;
  sheets)
    inner="$GEN/sheets/$a-inner.sh"; : > "$inner"
    pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$GEN/backup/$a.json")"
    for s in ${=SCALES:-$(python3 -c "import json,sys; print(' '.join(sorted({o['scale'] for o in json.load(open(sys.argv[1])) if 'scale' in o}, key=int)))" "$W/ops/$a.json")}; do for tag in before after; do
      src="$GEN/backup/$a.json"; [[ $tag == after ]] && src="$S/🗂️catalog/👤️humans/$a/🔣️.json"
      work="$GEN/sheets/work-$tag-$s/$a"
      print -r -- "mkdir -p \"$work\"; cp \"$src\" \"$work/🔣️.json\"; png=\"\$(\"$B\" r3-2d.ts --file \"$work/🔣️.json\" --scale $s --pose \"${POSE:-$pose}\" 2>/dev/null | tail -1)\"; [[ -f \"\$png\" ]] && mv \"\$png\" \"$GEN/sheets/$a.s$s.${POSE:-$pose}.$tag.png\"; rm -rf \"$GEN/sheets/work-$tag-$s\"" >> "$inner"
    done; done
    print -r -- "cd \"$T14\"; /bin/zsh \"$inner\"; echo \"=== B $a \$(date +%H:%M)\"; \"$R/.venv/bin/python3\" -W ignore \"$W/sheetdiff.py\" \"$GEN/sheets\" \"$a\"" >> "$d/sheets-master-$$.sh" ;;
esac; done
[[ "$mode" == sheets ]] && { "$W/q.sh" 1000 ${SHEETS_TIMEOUT:-7200} "$T14" /bin/zsh "$d/sheets-master-$$.sh" >> "$d/sheets-summary.txt" 2>&1; rm -f "$d/sheets-master-$$.sh"; }
[[ "$mode" == proof ]] && { "$W/q.sh" 2500 ${PROOF_TIMEOUT:-14400} "$T" "$W/guard.sh" 2300 "$d/proof-$$.peak" /bin/zsh "$d/proof-master-$$.sh" >> "$d/summary.txt" 2>&1; echo "proof grant: $(cat "$d/proof-$$.peak")" >> "$d/summary.txt"
  "$W/q.sh" 5000 ${PROOF_TIMEOUT:-14400} "$S" "$W/guard.sh" 4700 "$d/check-$$.peak" /bin/zsh "$d/check-master-$$.sh" >> "$d/summary.txt" 2>&1; echo "check grant: $(cat "$d/check-$$.peak")" >> "$d/summary.txt"; rm -f "$d/proof-master-$$.sh" "$d/check-master-$$.sh"; }
echo "TAKEOVER $mode done $(date +%H:%M)" >> "$d/chain.out"
