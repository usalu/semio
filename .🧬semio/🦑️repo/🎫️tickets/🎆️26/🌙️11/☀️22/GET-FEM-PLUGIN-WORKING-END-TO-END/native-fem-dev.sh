#!/usr/bin/env bash
# Native (repo-free) FEM 2D / 3D dev route. Usage: native-fem-dev.sh <step>
#   wasm      cargo build of the semio:fem component (wasm32-wasip2, prototype-component profile)
#   serve2d   vite on 6086 (FEM 2D)        serve3d   vite on 6087 (FEM 3D)
#   check     curl readiness of both servers
# Each step writes its log to $T/🗑️generated/nfd-<step>.log. Replicates what the repo targets run (see native-fem-dev.md).
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"
R=/c/semio
VITE=$(ls -dt "$R"/.cache/b/d/m/j/*/node_modules/vite/bin/vite.js | head -1)
mkdir -p "$G" /c/t/wd /c/t/wdb /c/t/wdtmp

case "$1" in
wasm)
  cd "$R/🌎️hub" || exit 2
  export TMP='C:\t\wdtmp' TEMP='C:\t\wdtmp' TMPDIR='C:\t\wdtmp' CARGO_INCREMENTAL=0
  export CARGO_TARGET_DIR='C:\t\wd' CARGO_BUILD_BUILD_DIR='C:\t\wdb'
  export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= SEMIO_BUILD_MODE=prototype RUST_MIN_STACK=134217728
  export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=selected
  cargo build --offline -j 3 --manifest-path "$R/🌎️hub/Cargo.toml" --profile prototype --target wasm32-wasip2 \
    -p semio-hub-fem-component --lib --config "$(cygpath -m "$T/native-fem-profile.toml")" \
    > "$G/nfd-wasm.log" 2>&1
  echo "rc=$?" >> "$G/nfd-wasm.log"
  ;;
serve2d|serve3d)
  v=fem${1#serve}; v=${v/fem2d/fem2d}; port=6086; [ "$v" = fem3d ] && port=6087
  cd "$R" || exit 2
  export SEMIO_PLUGIN=$v SEMIO_RENDERER=react SEMIO_BUILD_MODE=prototype VITE_SEMIO_PLUGIN=$v VITE_SEMIO_RENDERER=react
  export VITE_SEMIO_APP_ID="s.fem.$v@1/*#editor" SEMIO_VITE_HMR=0 S_OS_PORT=$port S_HUB_URL= SEMIO_BRAND=
  export SEMIO_VITE_CACHE_DIRECTORY="C:\\semio\\.cache\\b\\p\\m\\vite\\prototype\\native-$v"
  nohup bun "$VITE" --mode prototype --config "$R/✏️s/🧑‍💻dev/🎭️variants/🏗️fem/🏗️builder/🌐️vite/🟦️.ts" \
    --configLoader native --host 127.0.0.1 --port $port --strictPort --clearScreen false \
    > "$G/nfd-$1.log" 2>&1 &
  echo "$1 pid=$!"
  ;;
check)
  for p in 6086 6087; do echo "== $p"; curl -s -o /dev/null -w "html %{http_code} %{size_download}\n" "http://127.0.0.1:$p/"; done
  ;;
*) echo "usage: $0 wasm|serve2d|serve3d|check"; exit 2;;
esac
