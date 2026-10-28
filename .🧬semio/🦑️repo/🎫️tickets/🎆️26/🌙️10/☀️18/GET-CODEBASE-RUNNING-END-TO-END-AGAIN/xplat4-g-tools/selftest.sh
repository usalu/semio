#!/bin/sh
# usage: sh selftest.sh  -> applies, re-checks and reverts the batches in the order xplat3-g, platforms, freespace, corpus on a scratch root that holds copies of the live files named by the bases (nothing live is touched) and prints the states
H="$(cd "$(dirname "$0")" && pwd)"
R="${XPLAT4_SCRATCH:-/private/tmp/claude-501/xplat4-selftest}"
rm -rf "$R"; mkdir -p "$R"
for list in "$H/../xplat3-g-tools/bases.tsv" "$H/staged/platforms/bases.tsv" "$H/staged/freespace/bases.tsv" "$H/staged/corpus/bases.tsv"; do
  while IFS="$(printf '\t')" read -r base rel; do
    [ -z "$rel" ] && continue
    [ -e "/Users/ueli/Documents/semio/$rel" ] && [ ! -e "$R/$rel" ] && { mkdir -p "$(dirname "$R/$rel")"; cp "/Users/ueli/Documents/semio/$rel" "$R/$rel"; }
  done < "$list"
done
for step in apply check revert; do
  echo "== $step"
  if [ "$step" = revert ]; then ORDER="corpus freespace platforms xplat3"; else ORDER="xplat3 platforms freespace corpus"; fi
  for b in $ORDER; do
    case $b in xplat3) A="$H/../xplat3-g-tools/apply.py";; *) A="$H/staged/$b/apply.py";; esac
    python3 -I "$A" $step --root "$R" | cut -d' ' -f1 | sort | uniq -c | tr '\n' ' '; echo " <- $b"
  done
done
rm -rf "$R"
