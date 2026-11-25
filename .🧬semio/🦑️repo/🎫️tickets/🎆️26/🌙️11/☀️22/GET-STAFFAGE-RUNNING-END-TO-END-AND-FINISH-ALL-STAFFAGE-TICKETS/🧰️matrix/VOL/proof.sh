#!/bin/zsh
# 🧾️ The proof list of lane VOL on the catalog documents in one queue grant of 2500 MB: per asset `MR/measure.ts` before (backup) and after, the check CLI and `checkVolumes` at the written scales (`volumes.ts`); then every suite once with the ids as its name filter: proof.sh [<id>…]   (STEPS="measure volumes check tests")
me="${0:A}"; source "${0:A:h}/env.sh"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=2500 -- timeout 30000 "$me" --inside "$@"; fi
shift; ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS); d="$GEN/proof"; mkdir -p "$d"; P="$S/📦️packages/🟦️typescript"; steps="${STEPS:-measure volumes check tests}"
for a in $ids; do
  echo "== $a $(date +%H:%M) $(shasum -a 256 "$(asset "$a")" | cut -c1-16)"
  if [[ " $steps " == *" measure "* ]]; then
    (cd "$P" && timeout 900 "$B" "$M/measure.ts" "$GEN/backup/$a.json") > "$d/$a-before.log" 2>&1
    (cd "$P" && timeout 900 "$B" "$M/measure.ts" "$a") > "$d/$a-after.log" 2>&1
    diff <(sed "s/$a//" "$d/$a-before.log") <(sed "s/$a//" "$d/$a-after.log") > "$d/$a-diff.log" && echo "measure: same as before: $(grep -E 'silhouette:|validate:|sweep:' "$d/$a-after.log" | sed "s/$a //" | tr '\n' ';')" || { echo "measure DIFFERS"; head -20 "$d/$a-diff.log"; }
  fi
  [[ " $steps " == *" volumes "* ]] && (cd "$R" && timeout 900 "$B" "$W/volumes.ts" "$a") 2>&1 | tail -12
  [[ " $steps " == *" check "* ]] && { (cd "$S" && timeout 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"; }
done
if [[ " $steps " == *" tests "* ]]; then
  pat="(${(j:|:)ids})( |:|$)"
  for suite in "🧊️catalog-silhouette" "🧊️three-views" "🌡️catalog-physics"; do
    log="$d/vt-$suite.log"; (cd "$P" && /usr/bin/time -l timeout 7200 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$log" 2>&1; echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')"; grep -E '×|FAIL|AssertionError' "$log" | head -12
  done
  for suite in "🧊️volume-schema" "🧊️volumes-rules" "🧊️model-bounds" "🧬️schema-conformance"; do
    log="$d/vt-$suite.log"; (cd "$P" && /usr/bin/time -l timeout 7200 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts") > "$log" 2>&1; echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')"; grep -E '×|FAIL|AssertionError' "$log" | head -12
  done
fi
echo "proof ended $(date +%H:%M)"
