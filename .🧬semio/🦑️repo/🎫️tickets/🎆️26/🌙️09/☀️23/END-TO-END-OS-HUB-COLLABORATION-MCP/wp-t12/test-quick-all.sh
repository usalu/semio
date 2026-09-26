#!/bin/zsh
# 🧪️ T12 session 12: per-plugin `bun ./📜️script.ts test quick` for every plugin and extension package, one at a time.
# Captures `.🧬semio/🌐hub/s12-t12-captures/quick/<name>.txt` (EXIT/SECONDS appended); progress in `quick/progress.txt`.
# A package whose capture already ends in EXIT=0 is skipped, so a re-run resumes. `touch quick/PAUSE` holds the chain
# between packages. usage: zsh test-quick-all.sh [package-dir …]  (default: every plugin, then every extension)
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-t12-captures/quick"
mkdir -p "$OUT"
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t12/target
if [ $# -gt 0 ]; then dirs=("$@"); else
  dirs=(✏️s/🔌️plugins/*/📦️packages/🦀️rust(N) ✏️s/🔌️plugins/*/🧩️extensions/*/📦️packages/🦀️rust(N))
fi
for dir in "${dirs[@]}"; do
  name=$(print -r -- "$dir" | sed -e 's#✏️s/🔌️plugins/##' -e 's#/📦️packages/🦀️rust##' -e 's#/🧩️extensions/#+#')
  capture="$OUT/$name.txt"
  if [ -f "$capture" ] && tail -1 "$capture" | /usr/bin/grep -q "^EXIT=0 "; then continue; fi
  while [ -f "$OUT/PAUSE" ]; do sleep 20; done
  echo "START $name $(date '+%T')" >> "$OUT/progress.txt"
  start=$(date +%s)
  ( cd "$dir" && nice -n 15 bun ./📜️script.ts test quick ) > "$capture" 2>&1
  code=$?
  echo "EXIT=$code SECONDS=$(( $(date +%s) - start ))" >> "$capture"
  echo "END $name EXIT=$code $(( $(date +%s) - start ))s $(date '+%T')" >> "$OUT/progress.txt"
done
echo "ALL_DONE $(date '+%T')" >> "$OUT/progress.txt"
