#!/bin/zsh
. "$(dirname "$0")/env.sh"
"$T/vitest.sh" kernel > "$G/ikt-vitest-kernel-baseline.log" 2>&1
echo "kernel done $?" >> "$G/ikt-vitest-baseline.done"
"$T/vitest.sh" renderer > "$G/ikt-vitest-renderer-baseline.log" 2>&1
echo "renderer done $?" >> "$G/ikt-vitest-baseline.done"
