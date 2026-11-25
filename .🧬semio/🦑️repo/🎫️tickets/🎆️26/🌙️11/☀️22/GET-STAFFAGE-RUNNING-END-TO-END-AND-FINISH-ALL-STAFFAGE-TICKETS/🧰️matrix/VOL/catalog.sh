#!/bin/zsh
# 🧊️ The per-asset cases of 🧊️model-catalog, one vitest process per slice of at most $SLICE (default 8) assets or 6 MB of documents, named by SEMIO_STAFFAGE_MODEL_ASSETS, JSON results in $GEN/catalog/<time>-<first>.json; an asset with a verdict of its current document (catalog/done.txt: id sha verdict) is skipped, so the run resumes, and two chains share the catalog by claims: catalog.sh <MB> [<id>…]   (default: every asset but the 27, ORDER=small ascending up to $MAXB bytes or ORDER=large descending)
me="${0:A}"; source "${0:A:h}/env.sh"
if [[ "$1" != "--inside" ]]; then mb="$1"; shift; cd "$R"; until false; do SEMIO_LANGUAGE=en repo capacity run --memory="$mb" -- timeout 80000 "$me" --inside "$@" && exit; sleep 5; done; fi
shift; mkdir -p "$GEN/catalog/claim"; done="$GEN/catalog/done.txt"; touch "$done"
ids=("$@"); mine=""; if [[ ${#ids} -eq 0 ]]; then mine=1; ids=($(ls -l "$S"/🗂️catalog/*/*/🔣️.json | awk '{n=$5; sub(/^.*🗂️catalog\//,""); split($0,p,"/"); print n, p[2]}' | sort -n$([[ "${ORDER:-small}" == large ]] && echo r) | awk -v m="${MAXB:-900000}" -v o="${ORDER:-small}" 'o=="large"||$1<=m {print $2}')); fi
slice=(); bytes=0
flush() {
  [[ ${#slice} -eq 0 ]] && return
  n="$(date +%H%M%S)-$slice[1]"
  (cd "$S/📦️packages/🟦️typescript" && SEMIO_STAFFAGE_MODEL_ASSETS="${(j:,:)slice}" /usr/bin/time -l timeout 14000 "$B" x vitest run --reporter json --outputFile "$GEN/catalog/$n.json" --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🧊️model-catalog/🟦️.ts") > "$GEN/catalog/$n.log" 2>&1
  code=$?
  python3 -I "$W/catalog-done.py" "$GEN/catalog/$n.json" "$GEN/catalog/shas.txt" "$done" "$code" "$(grep -m1 'maximum resident' "$GEN/catalog/$n.log" | awk '{print int($1/1048576)}')"
  slice=(); bytes=0
}
for a in $ids; do
  [[ -n "$mine" ]] && (( ${IDS[(Ie)$a]} )) && continue
  f="$(asset "$a")"; sha="$(shasum -a 256 "$f" | cut -c1-16)"
  grep -q "^$a $sha " "$done" && continue
  [[ -n "$mine" ]] && { mkdir "$GEN/catalog/claim/$a" 2>/dev/null || continue; }
  echo "$a $sha" >> "$GEN/catalog/shas.txt"; slice+=("$a"); bytes=$((bytes + $(stat -f %z "$f")))
  (( ${#slice} >= ${SLICE:-8} || bytes > 6000000 )) && flush
done
flush; echo "catalog run ended $(date +%H:%M)"
