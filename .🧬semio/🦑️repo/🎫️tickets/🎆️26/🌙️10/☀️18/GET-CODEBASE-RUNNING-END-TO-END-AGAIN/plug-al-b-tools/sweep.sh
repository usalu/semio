#!/bin/bash
# usage: sweep.sh MODE LIST.tsv RESULTS.tsv   (MODE check|test; LIST rows: plugin<TAB>workspace-dir; one cargo at a time, retrying infra faults)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/plug-al-b"
MODE="$1"; LIST="$2"; OUT="$3"
while IFS=$'\t' read -r plugin dir; do
  [ -f "$G/stop" ] && break
  key=$(echo "$dir" | md5 -q | cut -c1-8)
  /usr/bin/grep -qF "$dir	$MODE	" "$OUT" 2>/dev/null && continue
  start=$(date +%s)
  if [ "$MODE" = check ]; then
    sh "$C/plug-al-b-tools/run.sh" "$G/$key-$MODE.log" check --workspace --all-targets --keep-going --message-format=short --manifest-path "$dir/Cargo.toml"; rc=$?
  else
    sh "$C/plug-al-b-tools/run.sh" "$G/$key-$MODE.log" test --workspace --no-fail-fast --message-format=short --manifest-path "$dir/Cargo.toml"; rc=$?
  fi
  errs=$(/usr/bin/grep -cE ': error(\[|:)' "$G/$key-$MODE.log")
  bad=$(/usr/bin/grep -oE "could not compile \`[^\`]*\`" "$G/$key-$MODE.log" | sed -E 's/could not compile `//; s/`//' | sort -u | tr '\n' ',')
  echo "$dir	$MODE	$plugin	$key	rc=$rc	errors=$errs	$(( $(date +%s) - start ))s	$bad" >> "$OUT"
done < "$LIST"
echo "DONE $MODE" >> "$OUT"
