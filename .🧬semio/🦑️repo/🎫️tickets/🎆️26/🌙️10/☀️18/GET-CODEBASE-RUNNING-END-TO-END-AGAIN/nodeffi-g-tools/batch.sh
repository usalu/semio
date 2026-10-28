#!/bin/sh
# 🧲️ nodeffi-g batch (ALREADY LIVE in the tree; not under library/process/launcher). usage: batch.sh check|verify|revert
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/nodeffi-g-tools/batch-files.txt"
case "$1" in
 check) while IFS="$(printf '\t')" read -r k p; do [ -e "$p" ] && echo "present $k $p" || echo "MISSING $k $p"; done < "$L"; shasum -a 256 $(cut -f2 "$L") | cut -c1-16,65- ;;
 verify)
  node node_modules/.bin/tsc -p "$C/nodeffi-g-tools/tsconfig.strict.json" || exit 1
  R="🧰️framework/🔨️modules/⚙️runtime"
  node --test "$R/🟢️node/🧲️native/🧮️system/🧪️tests/🟦️.ts" "$R/🟢️node/🧲️native/🏗️addon/🧪️tests/🟦️.ts" "$R/🟢️node/🧲️native/🧪️tests/🟦️.ts" || exit 1 ;;
 revert) while IFS="$(printf '\t')" read -r k p; do if [ "$k" = M ]; then git show "HEAD:$p" > "$p"; else rm -f "$p"; fi; done < "$L"; echo "reverted (remove empty dirs by hand)" ;;
 *) echo "usage: batch.sh check|verify|revert"; exit 2 ;;
esac
