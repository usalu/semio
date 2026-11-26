# 📍️ Where views of trial copies leave the silhouette guard (sourced by q.sh): where.sh <tag> <id>.<variant>:<scale>:<view>…   runs `far.ts` per entry on $GEN/try/<id>.<variant>.json (variant `base`: $GEN/base/<id>.json).
shift
for t in "$@"; do
  parts=("${(@s/:/)t}"); f="$GEN/try/${parts[1]}.json"; [[ "${parts[1]}" == *.base ]] && f="$GEN/base/${parts[1]%.base}.json"
  cap 1200 "$P" "$B" "$W/far.ts" "$f" --scale "${parts[2]}" --view "${parts[3]}" --top 6 2>&1 | cut -c1-400
done
