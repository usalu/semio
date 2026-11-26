#!/bin/zsh
# 🧪️ The matrix of assets one after the other in one queue grant (800 MB): matrices.sh <id>…
source "${0:A:h}/env.sh"; mkdir -p "$GEN"; me="${0:A}"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=800 -- "$me" --inside "$@"; fi
shift
for a in "$@"; do
  rm -rf "$GEN/matrix-$a"; (cd "$T" && timeout 14400 "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$GEN/matrix-$a.log" 2>&1
  echo "$a matrix exit $? $(tail -1 "$GEN/matrix-$a.log" | cut -c1-200) $(date +%H:%M)"
done
