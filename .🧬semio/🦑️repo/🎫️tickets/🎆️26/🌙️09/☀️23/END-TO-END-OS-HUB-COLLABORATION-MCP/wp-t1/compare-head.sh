#!/bin/zsh
# 🔍️ Compares every changed fixture under $2 with its HEAD blob through the reader binary $1; prints equal/different counts.
reader=$1; scope=$2; tmp=$(mktemp -d); ok=0; bad=0
git diff --name-only -- "$scope" | /usr/bin/grep "🧫️fixtures/.*\.json$" | while IFS= read -r f; do
  git show "HEAD:$f" > $tmp/h.json
  r=$($reader compare $tmp/h.json "$f" | python3 -c "import json,sys; print(json.load(sys.stdin)['measurements']['equal'])")
  if [ "$r" = True ]; then ok=$((ok+1)); else bad=$((bad+1)); echo "DIFF $f"; fi
done
echo "equal=$ok different=$bad"
rm -rf $tmp
