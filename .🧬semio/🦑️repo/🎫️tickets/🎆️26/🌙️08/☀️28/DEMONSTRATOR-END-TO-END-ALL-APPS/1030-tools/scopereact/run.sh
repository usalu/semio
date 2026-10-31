#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio || exit 1
SEMIO_LANGUAGE=en repo tools exec tsc -- -p "$C/1030-tools/e2eprep/tsconfig.json" > "$C/🗑️generated/1030/scopereact-tsc2.txt" 2>&1
echo $? > "$C/🗑️generated/1030/scopereact-tsc2.rc"
for t in test canonical-architecture; do
  SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:$t > "$C/🗑️generated/1030/scopereact-$t.txt" 2>&1
  echo $? > "$C/🗑️generated/1030/scopereact-$t.rc"
done
echo done > "$C/🗑️generated/1030/scopereact-all.rc"
