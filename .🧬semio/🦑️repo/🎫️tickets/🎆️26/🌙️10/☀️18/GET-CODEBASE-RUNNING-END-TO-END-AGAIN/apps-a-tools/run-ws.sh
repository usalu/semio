#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/apps-a-x"
OUT="$L/ws-results2.tsv"
run() {
  local log="$1"; shift
  for t in 1 2 3 4 5 6 7 8; do
    nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$log" 2>&1
    rc=$?
    [ $rc = 0 ] && return 0
    /usr/bin/grep -qE "could not compile|due to [0-9]+ previous" "$log" && return $rc
    sleep 60
  done
  return $rc
}
while IFS=$'\t' read -r plugin dir; do
  key=$(echo "$dir" | md5 -q | cut -c1-8)
  /usr/bin/grep -qF "$dir	" "$OUT" 2>/dev/null && continue
  start=$(date +%s); upd=skip
  if [ ! -f "$L/$key-update.done" ]; then run "$L/$key-update.log" update --workspace --offline --manifest-path "$dir/Cargo.toml"; upd=rc=$?; [ $? = 0 ] && touch "$L/$key-update.done"; fi
  run "$L/$key-libs.log" check --workspace --keep-going --message-format=short --manifest-path "$dir/Cargo.toml"; libs=$?
  run "$L/$key-check.log" check --workspace --all-targets --keep-going --message-format=short --manifest-path "$dir/Cargo.toml"; all=$?
  echo "$dir	$plugin	$key	update=$upd	libs_rc=$libs	alltargets_rc=$all	$(( $(date +%s) - start ))s" >> "$OUT"
done < "$C/apps-a-tools/workspaces.tsv"
echo DONE >> "$OUT"
