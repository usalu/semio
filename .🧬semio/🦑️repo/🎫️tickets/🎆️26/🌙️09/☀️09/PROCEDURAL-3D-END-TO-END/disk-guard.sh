#!/bin/zsh
# 🛡️ Stops this ticket's native component build before the data volume fills (below 5 GiB free).
while pgrep -f wgpu-native-components-build >/dev/null; do
  free=$(df -k /Users/ueli/Documents/semio | awk 'NR==2{print int($4/1048576)}')
  if [ "$free" -lt 5 ]; then echo "[DEBUG] disk-guard: ${free} GiB free, stopping native build $(date)"; pkill -f wgpu-native-components-build; pkill -f "cargo build.*g3d-native/tp2"; pkill -f "g3d-native/tp2"; exit 0; fi
  sleep 10
done
