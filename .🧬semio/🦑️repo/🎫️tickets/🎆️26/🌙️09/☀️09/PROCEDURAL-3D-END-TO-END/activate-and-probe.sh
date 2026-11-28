#!/bin/sh
# 🧪️ Activates the React prototype, restarts the 6018 host and runs the journey probe with extension trace; usage: activate-and-probe.sh OUT [ONLY]
cd "$(dirname "$0")/../../../../../../.." || exit 1
W="$(pwd)"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
G="$T/🗑️generated"
export SEMIO_LANGUAGE=en
repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype --exclude-dependencies > "$G/activate-react-4.log" 2>&1 || { echo "activate failed"; exit 1; }
screen -S g3d -X quit 2>/dev/null; sleep 2; : > "$G/serve-native-6018.txt"; screen -dmS g3d "$T/serve-generation3d-native.sh"
i=0; until curl -s -o /dev/null http://127.0.0.1:6018/ || [ $i -ge 60 ]; do i=$((i+1)); sleep 5; done
cd "$T"
if [ -n "$2" ]; then export SEMIO_PROBE_ONLY="$2"; fi
SEMIO_PROBE_OUT="$1" SEMIO_PROBE_EXTENSION_TRACE=1 SEMIO_PROBE_BASELINE_FIRST=1 SEMIO_PROBE_BASELINE_MS=1200000 SEMIO_PROBE_MAX_MS=2400000 repo tools exec --light bun -- react-journey-probe.mjs > "$G/journey-$1.log" 2>&1
echo "exit $?" >> "$G/journey-$1.log"
