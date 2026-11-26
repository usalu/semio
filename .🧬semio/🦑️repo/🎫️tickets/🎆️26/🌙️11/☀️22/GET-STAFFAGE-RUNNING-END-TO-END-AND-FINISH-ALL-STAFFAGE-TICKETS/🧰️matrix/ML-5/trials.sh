# 🧫️ Several trials under one grant (sourced by q.sh): trials.sh <tag> <id>:<variant>[:<steps with +>]…
shift
for t in "$@"; do
  parts=("${(@s/:/)t}")
  source "$W/try.sh" "${parts[1]}" "${parts[2]}" "${${parts[3]:-loft+prints+guard+gate+scan}//+/ }"
  cat "$GEN/try/${parts[1]}.${parts[2]}.out"
done
