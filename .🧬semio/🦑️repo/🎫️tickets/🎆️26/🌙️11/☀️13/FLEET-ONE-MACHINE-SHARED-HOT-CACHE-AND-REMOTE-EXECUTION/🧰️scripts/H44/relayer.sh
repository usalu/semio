#!/bin/zsh
# re-applies apply.py to a fresh overlay copy of the live files (the staged patch is checked against scratch copies only)
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H44
H=${0:A:h}
REL="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
rm -rf $S/overlay; mkdir -p $S/overlay/$REL; cd /Users/ueli/Documents/semio/$REL || exit 1
for f in 🌐️remote-execution/🚀️offload/🔬️checkout-test/🧬️schema/🔣️.json 🌐️remote-execution/🚀️offload/🔬️checkout-test/📐️policy/🟦️.ts 🌐️remote-execution/🚀️offload/🔬️checkout-test/🚦️gate/🟦️.ts 🌐️remote-execution/🚀️offload/🔬️checkout-test/👷️worker/🟦️.ts 🌐️remote-execution/🚀️offload/🔬️checkout-test/🔌️client/🟦️.ts 🌐️remote-execution/👷️worker/🟦️.ts 🔬️test-file/🌐️remote/🟦️.ts 🔬️test-file/🏃️run/🟦️.ts 🌐️remote-execution/🚀️offload/🔬️checkout-test/🧪️tests/🟦️.ts 🌐️remote-execution/🚀️offload/🔬️checkout-test/🧪️tests/🔣️.json 🌐️remote-execution/🚀️offload/🔬️checkout-test/🧪️tests/🐍️.py 🔬️test-file/🧪️tests/🌐️remote/🟦️.ts 🌐️remote-execution/🚀️offload/🔬️checkout-test/🧪️tests/🌐️scheduler/🟦️.ts; do mkdir -p "$S/overlay/$REL/$(dirname "$f")"; cp "$f" "$S/overlay/$REL/$f"; done
python3 $H/apply.py $S/overlay/$REL
