#!/bin/sh
# 🧪️ Rebuilds the Math and BRep flow extensions, activates the React prototype, restarts the 6018 host and runs the boot plus edit/undo/redo journey rows.
cd "$(dirname "$0")/../../../../../../.." || exit 1
W="$(pwd)"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
G="$T/🗑️generated"
export SEMIO_LANGUAGE=en
for project in flow-extension-math-rust flow-extension-brep-rust; do
  n=0
  until [ $n -ge 8 ]; do
    n=$((n+1))
    repo run "@semio-tech/$project:package" -c prototype --exclude-dependencies > "$G/ext-$project-$n.log" 2>&1 && { echo "$project ok attempt $n"; break; }
    echo "$project failed attempt $n: $(sed 's/\x1b\[[0-9;]*m//g' "$G/ext-$project-$n.log" | grep -E '^error|Internal error' | head -2 | cut -c1-250)"
    [ $n -ge 8 ] && exit 1
    sleep 150
  done
done
repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype --exclude-dependencies > "$G/activate-react-3.log" 2>&1 || { echo "activate failed: $(grep -E 'error' "$G/activate-react-3.log" | head -2 | cut -c1-250)"; exit 1; }
find "$W/.cache/b/p/m/browser/prototype/runtime/react/generation3d/extensions" -name "*.core.wasm" -exec ls -la {} \; | awk '{print $5,$6,$7,$8,$9}'
screen -S g3d -X quit 2>/dev/null; sleep 2; : > "$G/serve-native-6018.txt"; screen -dmS g3d "$T/serve-generation3d-native.sh"
i=0; until curl -s -o /dev/null http://127.0.0.1:6018/ || [ $i -ge 60 ]; do i=$((i+1)); sleep 5; done
cd "$T" && SEMIO_PROBE_OUT=react-journey-ext SEMIO_PROBE_ONLY=1,4 SEMIO_PROBE_EXTENSION_TRACE=1 SEMIO_PROBE_BASELINE_FIRST=1 SEMIO_PROBE_BASELINE_MS=1200000 SEMIO_PROBE_MAX_MS=2400000 repo tools exec --light bun -- react-journey-probe.mjs > "$G/journey-ext.log" 2>&1
grep -E '^\[DEBUG\] [0-9]+-' "$G/journey-ext.log" | cut -c1-400
grep -cE 'EXT_ANSWER' "$G/journey-ext.log"; grep -E 'EXT_FAULT' "$G/journey-ext.log" | head -3 | cut -c1-300
