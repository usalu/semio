#!/usr/bin/env bash
# 🏭️ Native FEM component pipeline: non-incremental wasm build (thin LTO + incremental left undefined `.llvm.*` symbols after disk-full crashes), retried across peers' transient breakage, then describe + materialize + restart of both vite servers.
# Usage: fem-wasm-pipeline.sh [attempts] [--no-publish|--publish-only]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/pipe"; mkdir -p "$G" /c/t/wd /c/t/wdb /c/t/wdtmp
D="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"
R=/c/semio
export TMP='C:\t\wdtmp' TEMP='C:\t\wdtmp' TMPDIR='C:\t\wdtmp' CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR='C:\t\wd' CARGO_BUILD_BUILD_DIR='C:\t\wdb'
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= SEMIO_BUILD_MODE=prototype RUST_MIN_STACK=134217728
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=selected
if [ "$2" != "--publish-only" ]; then
rm -rf /c/t/wdb/wasm32-wasip2/prototype/incremental
wipe_fem() { rm -rf /c/t/wdb/wasm32-wasip2/prototype/build/semio-hub-fem-* /c/t/wdb/wasm32-wasip2/prototype/deps/*semio_hub_fem* /c/t/wdb/wasm32-wasip2/prototype/.fingerprint/semio-hub-fem-*; }
wipe_fem
touch /c/t/wdtmp/pipeline.stamp
for i in $(seq 1 "${1:-40}"); do
  echo "attempt $i start $(date +%H:%M)" >> "$G/attempts.log"
  (cd "$R/🌎️hub" && cargo build --offline -j 6 --manifest-path "$R/🌎️hub/Cargo.toml" --profile prototype --target wasm32-wasip2 \
    -p semio-hub-fem-component --lib --config "$(cygpath -m "$D/native-fem-profile.toml")") > "$G/wasm.log" 2>&1
  rc=$?; echo "rc=$rc" >> "$G/wasm.log"
  [ $rc -eq 0 ] && { echo "attempt $i rc=0 $(date +%H:%M)" >> "$G/attempts.log"; break; }
  echo "attempt $i failed $(date +%H:%M): $(grep -a -m3 -E '^error(\[E[0-9]+\])?:|rust-lld: error' "$G/wasm.log" | cut -c1-300 | tr '\n' ' ')" >> "$G/attempts.log"
  grep -aq "rust-lld: error\|not enough space\|unexpectedly panicked" "$G/wasm.log" && wipe_fem
  grep -aq "failed to write .*Cargo.lock" "$G/wasm.log" && { sleep 10; continue; }
  sleep 120
done
[ $rc -eq 0 ] || exit 1
fi
mapfile -t RL < <(find /c/t/wdb/wasm32-wasip2/prototype/build -name '*.rlib' -newer /c/t/wdtmp/pipeline.stamp)
[ "$2" = "--publish-only" ] || [ ${#RL[@]} -gt 0 ] || { echo "locals guard found no rebuilt rlibs $(date +%H:%M)" >> "$G/attempts.log"; exit 1; }
[ ${#RL[@]} -eq 0 ] || bash "$T/wasm-locals-guard.sh" 40000 "${RL[@]}" > "$G/locals-guard.out" 2>&1 || { echo "locals guard failed $(date +%H:%M): $(tail -3 "$G/locals-guard.out" | tr -s '[:space:]' ' ')" >> "$G/attempts.log"; exit 1; }
[ "$2" = "--no-publish" ] && exit 0
for j in 1 2 3 4 5 6; do bash "$D/describe-fem.sh" > "$G/describe.out" 2>&1 && break; echo "describe attempt $j failed $(date +%H:%M)" >> "$G/attempts.log"; [ $j -eq 6 ] && exit 1; sleep 20; done
for p in 6086 6087; do powershell -NoProfile -c "Get-NetTCPConnection -LocalPort $p -State Listen -ErrorAction SilentlyContinue | % { Stop-Process -Id \$_.OwningProcess -Force }"; done
sleep 2; bash "$D/native-fem-dev.sh" serve2d; sleep 60; bash "$D/native-fem-dev.sh" serve3d
for i in $(seq 1 90); do curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:6087/ | grep -q 200 && break; sleep 2; done
echo "published $(date +%H:%M)" >> "$G/attempts.log"
