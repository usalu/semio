#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
D="/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator"
export SEMIO_LANGUAGE=en
cd /Users/ueli/Documents/semio || exit 1
repo tools exec tsc -- -p "$C/1030-tools/e2eprep/tsconfig.json" > "$G/tsdemo-tsc.txt" 2>&1; echo $? > "$G/tsdemo-tsc.rc"
repo tools exec bun -- test "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🧪️tests/🟦️.ts" > "$G/tsdemo-buildtooling.txt" 2>&1; echo $? > "$G/tsdemo-buildtooling.rc"
cd "$D" || exit 1
for t in "test" "canonical-architecture" "runtime-catalog-test"; do
  case $t in
    test) repo ./📜️script.ts test > "$G/tsdemo-d-test.txt" 2>&1; echo $? > "$G/tsdemo-d-test.rc";;
    canonical-architecture) repo ./📜️script.ts test long "./🧪️tests/🏛️shell-brand/🟦️.ts" > "$G/tsdemo-d-canon.txt" 2>&1; echo $? > "$G/tsdemo-d-canon.rc";;
    runtime-catalog-test) repo ./📜️script.ts runtime-catalog-test > "$G/tsdemo-d-catalog.txt" 2>&1; echo $? > "$G/tsdemo-d-catalog.rc";;
  esac
done
echo done > "$G/tsdemo-d-all.rc"
