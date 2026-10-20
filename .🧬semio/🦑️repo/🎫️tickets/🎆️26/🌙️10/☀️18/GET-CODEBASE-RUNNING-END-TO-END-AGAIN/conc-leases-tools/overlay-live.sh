#!/bin/sh
set -e
T=$(cd "$(dirname "$0")" && pwd)
ROOT=/Users/ueli/Documents/semio
OV=${1:-/private/tmp/conc-leases/ovl}
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
sh -c 'sed "/cp -R \"\$T\/staged\/\./d" "'"$T"'/overlay.sh" > /private/tmp/conc-leases/overlay-nostage.sh'
sh /private/tmp/conc-leases/overlay-nostage.sh "$OV" >/dev/null
mirror() {
  rm -f "$OV/$1"; mkdir -p "$OV/$1"
  for e in "$ROOT/$1"/* "$ROOT/$1"/.[!.]*; do [ -e "$e" ] || continue; n=$(basename "$e"); [ "$n" = "$2" ] || ln -s "$e" "$OV/$1/$n"; done
}
C="$L/⚡️caching"; S="$C/💾️storage"; E="$S/🔒️enforcement"; N="$E/🔌️nx"
mirror "$C" "💾️storage"; mirror "$S" "🔒️enforcement"; mirror "$E" "🔌️nx"; mirror "$N" "🎟️put-slots"
cp -R "$ROOT/$N/🎟️put-slots" "$OV/$N/🎟️put-slots"
cp "$T/wrap.ts" "$T/wrapfn.ts" "$T/driver.ts" "$T/validate-catalog.ts" "$T/regen-help.ts" "$OV/"
mkdir -p "$OV/🧰️framework/🔨️modules/🏃️process/.cache/b/c"
echo "$OV"
