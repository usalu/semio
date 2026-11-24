#!/bin/zsh
# ✍️ Writes the repair of one asset into the catalog: the earlier step named in chain.tsv (if any), then ops/<id>.json with fit.ts of this folder (it refuses when the file changed on disk since it was read), a second run that must report "0 changed something", then the product sheets of the given scales before (from $GEN/backup) and after, in the default pose and in every pose of POSES: POSES="a b" apply.sh <id> <scale>…
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
a="$1"; shift
pre="$(awk -F'\t' -v a="$a" '$1 == a { print $2 }' "$here/chain.tsv")"
if [[ -n "$pre" ]]; then (cd "$T" && timeout 600 "$B" "$here/$pre" "$a" --write) > "$GEN/try/$a.prewrite.log" 2>&1; echo "$pre exit $? $(tr '\n' ' ' < "$GEN/try/$a.prewrite.log")"; fi
(cd "$T" && timeout 600 "$B" "$here/fit.ts" "$a" --ops "$here/ops/$a.json" --write) > "$GEN/try/$a.write.log" 2>&1
echo "write exit $? $(tail -2 "$GEN/try/$a.write.log" | tr '\n' ' ')"
(cd "$T" && timeout 600 "$B" "$here/fit.ts" "$a" --ops "$here/ops/$a.json") > "$GEN/try/$a.write2.log" 2>&1
echo "second run: $(tail -1 "$GEN/try/$a.write2.log")"
[[ $# -eq 0 ]] && exit 0
TAG=before "$M/look.sh" "$a" "$@"
"$M/look.sh" "$a" "$@"
for p in ${=POSES}; do TAG=before POSE="$p" "$M/look.sh" "$a" "$@"; POSE="$p" "$M/look.sh" "$a" "$@"; done
