#!/bin/zsh
# 🧫️ One variant of the repair of an asset inside ONE grant (start it with qsh), on a copy that never touches the catalog or the dry run of in-try.sh: the earlier step of chain.tsv, the ops file with fit.ts until a run changes nothing, `<ops>.vol.json` (if it exists) with volume.ts, `<ops>.loft.json` (if it exists) with ../MX-2/loft.ts, then the gate with the given flags (the peak memory of the gate process is printed; NOGATE=1 skips it), and with MEASURE=1 (MSCALES=50,100 limits it to those scales) the silhouette guard, validateAsset, the sweep and the print suitability; the copy is $GEN/var/<tag>.json, the output $GEN/var/<tag>.out and the hard findings are printed: MEASURE=1 qsh in-var.sh <id> <ops.json> <tag> [gate flags…]
[[ -n "$MH3_GRANT" ]] || { echo "in-var.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
a="$1"; ops="$2"; tag="$3"; shift 3
d="$GEN/var"; mkdir -p "$d"; doc="$d/$tag.json"
{
  src="$a"; pre="$(awk -F'\t' -v a="$a" '$1 == a { print $2 }' "$here/chain.tsv")"
  if [[ -n "$pre" ]]; then (cd "$T" && "$B" "$here/$pre" "$a" --out "$doc") > "$d/$tag.pre.log" 2>&1; echo "$pre exit $?"; src="$doc"; fi
  for n in 1 2 3 4 5; do
    (cd "$T" && "$B" "$here/fit.ts" "$src" --ops "$ops" --out "$doc") > "$d/$tag.fit$n.log" 2>&1
    echo "fit run $n: exit $? $(tail -1 "$d/$tag.fit$n.log")"; src="$doc"
    [[ $n -eq 1 ]] && grep -E "REFUSED" "$d/$tag.fit$n.log" | cut -c1-330
    grep -q " 0 changed something" "$d/$tag.fit$n.log" && break
  done
  if [[ -f "${ops%.json}.vol.json" ]]; then
    for n in 1 2 3; do
      (cd "$T" && "$B" "$here/volume.ts" "$doc" --ops "${ops%.json}.vol.json" --out "$doc") > "$d/$tag.vol$n.log" 2>&1
      echo "volume run $n: exit $? $(tail -1 "$d/$tag.vol$n.log")"; [[ $n -eq 1 ]] && cut -c1-400 "$d/$tag.vol$n.log"
      grep -q " 0 changed something" "$d/$tag.vol$n.log" && break
    done
  fi
  if [[ -f "${ops%.json}.loft.json" ]]; then
    for n in 1 2 3; do
      (cd "$S/📦️packages/🟦️typescript" && "$B" "$here/../MX-2/loft.ts" "$doc" --ops "${ops%.json}.loft.json" --out "$d/$tag.loft.json") > "$d/$tag.loft$n.log" 2>&1
      echo "loft run $n: exit $? $(tail -1 "$d/$tag.loft$n.log" | cut -c1-200)"; [[ $n -eq 1 ]] && cut -c1-260 "$d/$tag.loft$n.log"
      cmp -s "$doc" "$d/$tag.loft.json" && break; mv "$d/$tag.loft.json" "$doc"
    done
  fi
  if [[ -z "$NOGATE" ]]; then
  start=$(date +%s)
  (cd "$T" && /usr/bin/time -l timeout ${GATE_TIMEOUT:-2400} "$B" "$M/gate.ts" "$doc" --twins "$@") > "$d/$tag.gate.log" 2>&1
  echo "gate exit $? $(grep '^hard findings' "$d/$tag.gate.log") ($(( $(date +%s) - start )) s; peak $(awk '/maximum resident set size/ { printf "%d MB", $1 / 1048576 }' "$d/$tag.gate.log"))"
  grep -A6 "HARD" "$d/$tag.gate.log" | grep -v " \(twin\|reported\) " | cut -c1-250
  fi
  if [[ -n "$MEASURE" ]]; then
    (cd "$S/📦️packages/🟦️typescript" && /usr/bin/time -l timeout 900 "$B" "$M/measure.ts" "$doc" ${MSCALES:+--scales} ${MSCALES}) > "$d/$tag.measure.log" 2>&1
    python3 "$here/silcmp.py" "$GEN/measure/$a.before.log" "$d/$tag.measure.log"; echo "measure peak $(awk '/maximum resident set size/ { printf "%d MB", $1 / 1048576 }' "$d/$tag.measure.log")"
    (cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$here/suit.ts" "$doc") 2>&1 | grep -E "UNSUITABLE|^suitability:" | cut -c1-300
  fi
} > "$d/$tag.out" 2>&1
echo "== $tag"; grep -v "^fit run [1-9]: exit 0.* 0 refused$" "$d/$tag.out" | head -${LINES_MAX:-40}
