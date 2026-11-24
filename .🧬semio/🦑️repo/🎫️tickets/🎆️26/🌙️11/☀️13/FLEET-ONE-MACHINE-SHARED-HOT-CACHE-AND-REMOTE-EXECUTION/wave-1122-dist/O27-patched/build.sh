#!/bin/zsh
# usage: build.sh <clone root> <scratch dir>   regenerates the post-images, list.tsv and .patch files from the clone's CURRENT files (the clone is only read).
set -e
clone=${1:?clone root}; scratch=${2:?scratch dir}
here=$(cd "$(dirname "$0")" && pwd)
P="🧰️framework/🔨️modules/🏃️process"
JS="$P/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots"
U="$P/🏗️build/🎟️capacity/🧮️units"
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }
rm -rf "$scratch/root"; mkdir -p "$scratch/root"
typeset -A names=("$JS/🧬️schema/🔣️.json" job-slots-schema.json "$JS/🟦️.ts" job-slots.ts "$U/🧬️schema/🔣️.json" units-schema.json "$U/🟦️.ts" units.ts "$U/🧪️tests/🛰️fleet-jobs/🧫️fixtures/🔣️.json" fleet-jobs-fixtures.json "$U/🧪️tests/🛰️fleet-jobs/🐍️oracle/🐍️.py" fleet-jobs-oracle.py "$U/🧪️tests/🐍️oracle/🐍️.py" units-oracle.py)
order=("$JS/🧬️schema/🔣️.json" "$U/🧬️schema/🔣️.json" "$JS/🟦️.ts" "$U/🟦️.ts" "$U/🧪️tests/🛰️fleet-jobs/🧫️fixtures/🔣️.json" "$U/🧪️tests/🛰️fleet-jobs/🐍️oracle/🐍️.py" "$U/🧪️tests/🐍️oracle/🐍️.py")
for target in $order; do mkdir -p "$(dirname "$scratch/root/$target")"; cp "$clone/$target" "$scratch/root/$target"; done
python3 "$here/edit-o27.py" "$scratch/root/$P"
python3 "$here/edit-o27-tests.py" "$scratch/root/$P"
: > "$here/list.tsv"
for target in $order; do
 name=${names[$target]}
 cp "$scratch/root/$target" "$here/$name"
 diff -u "$clone/$target" "$scratch/root/$target" > "$here/${name%.*}.patch" || true
 printf '%s\t%s\t%s\t%s\n' "$name" "$target" "$(sha "$clone/$target")" "$(sha "$here/$name")" >> "$here/list.tsv"
done
cut -f1,3,4 "$here/list.tsv" | cut -c1-120
