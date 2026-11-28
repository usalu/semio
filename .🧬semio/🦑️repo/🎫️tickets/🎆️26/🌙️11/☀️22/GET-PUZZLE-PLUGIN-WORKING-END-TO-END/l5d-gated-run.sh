#!/usr/bin/env bash
# usage: l5d-gated-run.sh <needMB> <grantMB> <log> <cargo args...>
need=$1; grant=$2; log=$3; shift 3
T="$(cd "$(dirname "$0")" && pwd)"
while [ "$(awk '/MemAvailable/{print int($2/1024)}' /proc/meminfo)" -lt "$need" ]; do sleep 30; done
cd "$T/../../../../../../.."
SEMIO_NATIVE_CARGO_JOBS=1 SEMIO_NATIVE_REFRESH_LOCK=1 SEMIO_NATIVE_LANE=5d repo capacity run --memory="$grant" -- timeout 14000 bash "$T/current-native-tests.sh" "🖐️5d" "$@" > "$log" 2>&1
echo "RC:$?" >> "$log"
