#!/bin/zsh
cd /Users/ueli/Documents/semio
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
for n in $(seq 1 12); do
  repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies > "$TF/🗑️generated/pub-occ-component-r$n.txt" 2>&1
  code=$?
  if grep -q "BUILD_SOURCE_UNPARSEABLE" "$TF/🗑️generated/pub-occ-component-r$n.txt"; then echo "retry $n: source unparseable"; sleep 240; continue; fi
  echo "run $n exit $code"; echo DONE $n $code > "$TF/🗑️generated/pub-occ-build.done"; exit $code
done
echo DONE 0 gaveup > "$TF/🗑️generated/pub-occ-build.done"
