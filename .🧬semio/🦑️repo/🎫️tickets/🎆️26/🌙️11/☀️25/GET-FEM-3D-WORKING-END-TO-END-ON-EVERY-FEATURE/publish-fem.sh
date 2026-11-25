#!/usr/bin/env bash
# 🚀️ After a green wasm: re-describes with a rebuilt describe tool, materializes the browser module, restarts both FEM vite servers.
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; G="$M/🗑️generated"
REBUILD=1 "$M/describe-fem.sh" || exit 1
for p in 6086 6087; do powershell -NoProfile -c "Get-NetTCPConnection -LocalPort $p -State Listen -ErrorAction SilentlyContinue | % { Stop-Process -Id \$_.OwningProcess -Force }"; done
sleep 2; "$M/native-fem-dev.sh" serve2d; "$M/native-fem-dev.sh" serve3d
for i in $(seq 1 60); do curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:6087/ | grep -q 200 && break; sleep 2; done
"$M/native-fem-dev.sh" check
echo "publish rc=0"
