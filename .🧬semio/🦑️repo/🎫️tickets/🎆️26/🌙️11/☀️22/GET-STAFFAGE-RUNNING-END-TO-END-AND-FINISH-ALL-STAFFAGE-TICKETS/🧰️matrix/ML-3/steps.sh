# 🪜️ The steps of batch ML-3 as functions, for jobs that already run inside a grant of the memory queue (`worker.sh`); source it. Every step runs one process in the foreground under `timeout`, logs its peak memory to `$GEN/log/rss.txt` and appends one result line to `$GEN/proof/<id>.txt` where it proves something.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/log" "$GEN/proof" "$GEN/sheets" "$GEN/vt"
# run <timeout s> <dir> <command…>
run() { local t="$1" dir="$2" r="$GEN/log/rss-$$.txt" code; shift 2; (cd "$dir" && /usr/bin/time -l -o "$r" timeout "$t" "$@"); code=$?; echo "$(( $(awk '/maximum resident set size/{print $1}' "$r") / 1048576 )) MB ${${@[2,5]}:t}" >> "$GEN/log/rss.txt"; rm -f "$r"; return $code; }
# tool <log name> <script.ts of this folder or path> <args…>
tool() { local name="$1" script="$2"; shift 2; [[ "$script" == */* ]] || script="$W/$script"; run ${TOOL_TIMEOUT:-3000} "$T" "$B" "$script" "$@" > "$GEN/log/$name.log" 2>&1; echo "$name exit $? $(tail -1 "$GEN/log/$name.log" | cut -c1-300)"; }
# scan <tag> <id…>
scan() { local tag="$1"; shift; run 3000 "$T" "$B" 🔎️scan.ts --language en --report "$GEN/log/scan-$tag.json" "$@" > "$GEN/log/scan-$tag.log" 2>&1; local line="scan ($tag) exit $? $(tail -1 "$GEN/log/scan-$tag.log" | cut -c1-200)"; echo "$line"; [[ $# -eq 1 ]] && echo "$line" >> "$GEN/proof/$1.txt"; }
# gate <id|file> [flags…]: every pose, clip instant and transition
gate() { local a="$1" n="${${1:t}%.json}"; shift; run 2400 "$T" "$B" "$M/gate.ts" "$a" --soft "$@" > "$GEN/log/$n-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/log/$n-gate.log" | cut -c1-200)" | tee -a "$GEN/proof/$n.txt"; }
# look <id> <tag: after|before|try> <pose or -> <scale…>: the product 2D sheet (side, front, plan; line and solid)
look() { local a="$1" tag="$2" pose="$3"; shift 3; local work="$GEN/sheets/work-$tag/$a" s png; mkdir -p "$work"
  case "$tag" in before) cp "$GEN/backup/$a.json" "$work/🔣️.json";; try) cp "$GEN/try/$a.json" "$work/🔣️.json";; *) cp "$C/$a/🔣️.json" "$work/🔣️.json";; esac
  [[ "$pose" == "-" ]] && pose="$(python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['asset']['presentation']['defaultPose'])" "$work/🔣️.json")"
  for s in "$@"; do png="$(run 900 "$T14" "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" --pose "$pose" 2>&1 | tail -1)"; [[ -f "$png" ]] && mv "$png" "$GEN/sheets/$a.s$s.$pose.$tag.png" && echo "$GEN/sheets/$a.s$s.$pose.$tag.png" || echo "no sheet: $png"; done
  rm -rf "$GEN/sheets/work-$tag"; }
# vt <id> <label> <test folder> <filter words…>   (SUFFIX as in ../MR/vt.sh)
vt() { local a="$1" label="$2" suite="$3"; shift 3; local log="$GEN/vt/$a-$label.log" pat="(${(j:|:)@})${SUFFIX-( |$)}"
  run ${VT_TIMEOUT:-2400} "$S/📦️packages/🟦️typescript" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" > "$log" 2>&1
  echo "$label: exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" | tee -a "$GEN/proof/$a.txt"; }
# matrix <id>: the matrix of one asset to its end
matrix() { local a="$1" s=$(date +%s) code; rm -rf "$GEN/matrix-$a"
  run ${MATRIX_TIMEOUT:-9000} "$T" "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$GEN/proof/$a-matrix.log" 2>&1; code=$?
  echo "matrix exit $code $(grep '\[matrix\].*cells:' "$GEN/proof/$a-matrix.log" | tail -1 | cut -c1-200) ($(( $(date +%s)-s )) s) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | head -12 | tr '\n' ' ' | cut -c1-900)" | tee -a "$GEN/proof/$a.txt"; }
# prove <id>: the proof list of the multiview brief without the matrix
prove() { local a="$1" o="$GEN/proof/$1.txt"; echo "proof of $a, file $(shasum -a 256 "$C/$a/🔣️.json" | cut -c1-16), $(date +%F\ %H:%M)" >> "$o"
  scan "after-$a" "$a"; gate "$a"
  run 900 "$T" "$B" "$W3/suit.ts" "$a" > "$GEN/log/$a-suit.log" 2>&1; echo "suit exit $? $(tail -1 "$GEN/log/$a-suit.log" | cut -c1-120)" | tee -a "$o"
  run 2400 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$a" > "$GEN/log/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$GEN/log/$a-measure.log" | tee -a "$o"
  run 2400 "$S" "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$GEN/log/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$GEN/log/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$GEN/log/$a-check.log") suitability lines" | tee -a "$o"
  vt "$a" silhouette "🧊️catalog-silhouette" "$a"; vt "$a" three-views "🧊️three-views" "$a"; vt "$a" physics "🌡️catalog-physics" "$a"
  SUFFIX=":" vt "$a" breathing "🫁️breathing-catalog" "$a"; SUFFIX="" vt "$a" validation "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"; SUFFIX=":" vt "$a" power "⚡️asset-power" "$a"
  echo "proof done $(date +%H:%M)" >> "$o"; }
