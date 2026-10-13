#!/bin/bash
# 🔎️ Lane C variant of the ticket's manifest sweep: checks every plugin artifact/extension package natively (lib) in the
# private lane-c dirs, resumable (manifests already recorded are skipped), stopping after a time budget.
# usage: 🔎️manifest-sweep.sh <seconds>
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
OUT="$T/🗑️generated/lane-c/manifest-sweep.tsv"; touch "$OUT"
BUDGET="$1"; start=$(date +%s)
find ✏️s/🔌️plugins -path '*/📦️packages/🦀️rust/Cargo.toml' -not -path '*/target/*' | sort | while read -r m; do
  grep -qF "	$m	" "$OUT" && continue; grep -qF "	$m"$'\n' <(cut -f2 "$OUT"; echo) 2>/dev/null
  cut -f2 "$OUT" | grep -qxF "$m" && continue
  left=$(( BUDGET - ($(date +%s) - start) )); [ "$left" -lt 60 ] && { echo "budget exhausted"; break; }
  f=""; grep -q '^component-app-assembly' "$m" && f="--features component-app-assembly"
  log=$(bash "$T/lane-c-semio-split/🦀️cargo.sh" "$left" check --manifest-path "$m" --lib $f --message-format short 2>&1); rc=$?
  [ "$rc" = 124 ] && { echo "deadline in $m"; break; }
  if [ "$rc" != 0 ]; then printf "FAIL\t%s\t%s\n" "$m" "$(echo "$log" | grep -m1 -E ': error|^error' | cut -c1-240)" >> "$OUT"; else printf "ok\t%s\n" "$m" >> "$OUT"; fi
done
echo "recorded $(wc -l < "$OUT") ok=$(grep -c '^ok' "$OUT") fail=$(grep -c '^FAIL' "$OUT") of $(find ✏️s/🔌️plugins -path '*/📦️packages/🦀️rust/Cargo.toml' -not -path '*/target/*' | wc -l)"
