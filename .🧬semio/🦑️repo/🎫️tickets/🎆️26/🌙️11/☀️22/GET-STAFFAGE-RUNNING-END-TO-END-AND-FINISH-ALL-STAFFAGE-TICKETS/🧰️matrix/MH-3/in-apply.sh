#!/bin/zsh
# ✍️ Writes the repair of assets into the catalog inside ONE grant (start it with qsh), as apply.sh does: per asset the earlier step of chain.tsv (if any) and ops/<id>.json with fit.ts of this folder (`--write`; it refuses when the file changed on disk since it was read), further runs until one reports "0 changed something" (at most five), then ops2/<id>.json (if any) with volume.ts until a run changes nothing and fit.ts once more, then lofts/<id>.json (if any) with ../MX-2/loft.ts and fit.ts once more, then `cmp` of the catalog file with the dry-run document $GEN/try/<id>.json whose gate was measured; an asset whose catalog file is not that document is reported as DIFFERENT (measure it before it counts as repaired): qsh in-apply.sh <id>…
[[ -n "$MH3_GRANT" ]] || { echo "in-apply.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
for a in "$@"; do
  pre="$(awk -F'\t' -v a="$a" '$1 == a { print $2 }' "$here/chain.tsv")"
  if [[ -n "$pre" ]]; then (cd "$T" && "$B" "$here/$pre" "$a" --write) > "$GEN/try/$a.prewrite.log" 2>&1; echo "$a $pre exit $? $(tr '\n' ' ' < "$GEN/try/$a.prewrite.log" | cut -c1-200)"; fi
  (cd "$T" && "$B" "$here/fit.ts" "$a" --ops "$here/ops/$a.json" --write) > "$GEN/try/$a.write.log" 2>&1
  echo "$a write exit $? $(tail -2 "$GEN/try/$a.write.log" | tr '\n' ' ' | cut -c1-260)"
  for n in 2 3 4 5; do
    (cd "$T" && "$B" "$here/fit.ts" "$a" --ops "$here/ops/$a.json" --write) > "$GEN/try/$a.write2.log" 2>&1
    echo "$a run $n: $(tail -1 "$GEN/try/$a.write2.log")"
    grep -q " 0 changed something" "$GEN/try/$a.write2.log" && break
  done
  if [[ -f "$here/ops2/$a.json" ]]; then
    for n in 1 2 3; do
      (cd "$T" && "$B" "$here/volume.ts" "$a" --ops "$here/ops2/$a.json" --write) > "$GEN/try/$a.volwrite.log" 2>&1
      echo "$a volume run $n: exit $? $(tail -1 "$GEN/try/$a.volwrite.log")"
      grep -q " 0 changed something" "$GEN/try/$a.volwrite.log" && break
    done
    (cd "$T" && "$B" "$here/fit.ts" "$a" --ops "$here/ops/$a.json" --write) > "$GEN/try/$a.write3.log" 2>&1
    echo "$a fit after volume: $(tail -1 "$GEN/try/$a.write3.log")"
  fi
  if [[ -f "$here/lofts/$a.json" ]]; then
    for n in 1 2; do
      (cd "$S/📦️packages/🟦️typescript" && "$B" "$here/../MX-2/loft.ts" "$a" --ops "$here/lofts/$a.json" --write) > "$GEN/try/$a.loftwrite.log" 2>&1
      echo "$a loft run $n: exit $? $(tail -1 "$GEN/try/$a.loftwrite.log" | cut -c1-200)"
    done
    (cd "$T" && "$B" "$here/fit.ts" "$a" --ops "$here/ops/$a.json" --write) > "$GEN/try/$a.write4.log" 2>&1
    echo "$a fit after loft: $(tail -1 "$GEN/try/$a.write4.log")"
  fi
  cmp -s "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/try/$a.json" && echo "$a catalog file = dry run" || echo "$a catalog file DIFFERENT from the dry run"
done
