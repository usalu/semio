#!/bin/sh
# 🔁 Reruns the Draw editor test crate until it builds (other agents edit dependencies live); usage: retry-draw-editor-tests.sh <log-prefix> <test filter...>
set -u
T="$(cd "$(dirname "$0")" && pwd)"
P="$1"; shift
cd "$T/../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing" || exit 1
for n in 1 2 3 4 5 6 7 8; do
  SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en repo cargo --store=isolated --configuration=prototype -- test --no-fail-fast -p semio-s-artifact-draw-drawing-editor-tests "$@" > "$T/🗑️generated/$P-$n.log" 2>&1
  if grep -aq "test result" "$T/🗑️generated/$P-$n.log"; then exit 0; fi
  sleep 240
done
