#!/bin/zsh
# usage: build.sh <clone root> <scratch dir>   regenerates the combined O24+O26 files, list.tsv and the .patch files from the clone's CURRENT files (the clone is only read).
set -e
clone=${1:?clone root}; scratch=${2:?scratch dir}
here=$(cd "$(dirname "$0")" && pwd)
live=${LIVE:-/Users/ueli/Documents/semio}
NB="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass"
JS="$NB/🧵️job-slots"
SG="$NB/💾️storage-gate"
DS="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts"
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }
rm -rf "$scratch/root"; mkdir -p "$scratch/root"
typeset -A names=("$JS/🧬️schema/🔣️.json" job-slots-schema.json "$JS/🟦️.ts" job-slots.ts "$NB/🟦️.ts" component-pass.ts "$DS" driver-stages.ts)
order=("$JS/🧬️schema/🔣️.json" "$JS/🟦️.ts" "$NB/🟦️.ts" "$DS")
for target in $order; do
 mkdir -p "$(dirname "$scratch/root/$target")"; cp "$clone/$target" "$scratch/root/$target"
done
python3 "$here/edit-o24.py" "$scratch/root"
python3 "$here/edit-o26.py" "$scratch/root"
: > "$here/list.tsv"
new() { cp "$live/$2" "$here/$1"; printf '%s\t%s\tNEW\t%s\n' "$1" "$2" "$(sha "$here/$1")" >> "$here/list.tsv"; }
emit() {
 target=$1; name=${names[$target]}
 cp "$scratch/root/$target" "$here/$name"
 diff -u "$clone/$target" "$scratch/root/$target" > "$here/${name%.*}.patch" || true
 printf '%s\t%s\t%s\t%s\n' "$name" "$target" "$(sha "$clone/$target")" "$(sha "$here/$name")" >> "$here/list.tsv"
}
emit "$JS/🧬️schema/🔣️.json"
new storage-gate-schema.json "$SG/🧬️schema/🔣️.json"
new storage-gate.ts "$SG/🟦️.ts"
emit "$JS/🟦️.ts"
emit "$NB/🟦️.ts"
emit "$DS"
new fleet-test.ts "$JS/🧪️tests/🌐️fleet/🟦️.ts"
new fleet-oracle.py "$JS/🧪️tests/🌐️fleet/🐍️oracle/🐍️.py"
new storage-gate-test.ts "$SG/🧪️tests/🟦️.ts"
new storage-gate-oracle.py "$SG/🧪️tests/🐍️oracle/🐍️.py"
cut -f1,3,4 "$here/list.tsv" | cut -c1-120
