#!/bin/zsh
# usage: build.sh <clone root> <scratch dir>   regenerates the patched files, list.tsv and the .patch files from the clone's CURRENT files (the clone is only read).
set -e
clone=${1:?clone root}; scratch=${2:?scratch dir}
here=$(cd "$(dirname "$0")" && pwd)
live=${LIVE:-/Users/ueli/Documents/semio}
NB="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass"
JS="$NB/🧵️job-slots"
DS="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts"
T="$JS/🧪️tests/🌐️fleet"
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }
rm -rf "$scratch/root"; mkdir -p "$scratch/root"
typeset -A names=("$JS/🟦️.ts" job-slots.ts "$JS/🧬️schema/🔣️.json" job-slots-schema.json "$NB/🟦️.ts" component-pass.ts "$DS" driver-stages.ts)
: > "$here/list.tsv"
for target in "$JS/🧬️schema/🔣️.json" "$JS/🟦️.ts" "$NB/🟦️.ts" "$DS"; do
 mkdir -p "$(dirname "$scratch/root/$target")"; cp "$clone/$target" "$scratch/root/$target"
done
python3 "$here/edit-o24.py" "$scratch/root"
for target in "$JS/🧬️schema/🔣️.json" "$JS/🟦️.ts" "$NB/🟦️.ts" "$DS"; do
 name=${names[$target]}
 cp "$scratch/root/$target" "$here/$name"
 diff -u "$clone/$target" "$scratch/root/$target" > "$here/${name%.*}.patch" || true
 printf '%s\t%s\t%s\t%s\n' "$name" "$target" "$(sha "$clone/$target")" "$(sha "$here/$name")" >> "$here/list.tsv"
done
cp "$live/$T/🟦️.ts" "$here/fleet-test.ts"; cp "$live/$T/🐍️oracle/🐍️.py" "$here/fleet-oracle.py"
printf '%s\t%s\t%s\t%s\n' fleet-test.ts "$T/🟦️.ts" NEW "$(sha "$here/fleet-test.ts")" >> "$here/list.tsv"
printf '%s\t%s\t%s\t%s\n' fleet-oracle.py "$T/🐍️oracle/🐍️.py" NEW "$(sha "$here/fleet-oracle.py")" >> "$here/list.tsv"
cat "$here/list.tsv" | cut -f1,3,4 | cut -c1-120
