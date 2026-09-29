#!/bin/zsh
# 🧪️ RS1 overlay proof under ONE overlay-lane hold (private build-dir inside the overlay, rules 3/23/25/28):
#   R  semio-framework-raster (CPU tier unit laws, fixture digests, resvg oracle; GPU tier compiles natively)
#   F  semio-framework-fonts (reader laws; swash oracle)
#   S  stdio drawing png/pdf leaf laws (conversion-drawing)
#   H  shooting photos:out + scene svg laws
#   W  wasm32-wasip2: raster + fonts laws under wasmtime (every-target digest law)
# usage: zsh rs1-proof.sh <tag> [sections…]   capture .🧬semio/🌐hub/s15-rs1-logs/<tag>.txt
tag="$1"; shift
sections=("$@"); [ $# -eq 0 ] && sections=(R F S H)
HUB="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OV="$HUB/s15-rs1-overlay"
LOGS="$HUB/s15-rs1-logs"; mkdir -p "$LOGS"
OUT="$LOGS/$tag.txt"
export CARGO_BUILD_BUILD_DIR="$OV/.rs1-build" CARGO_TARGET_DIR="$OV/.rs1-target" CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=33554432 CARGO_NET_OFFLINE=true RS1_SECTIONS="${sections[*]}"
export CARGO_TARGET_WASM32_WASIP2_RUNNER="wasmtime run --dir=."
echo "QUEUED $(date '+%F %T') sections=${sections[*]}" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay rs1 -- zsh -c '
  free=$(df -g / | awk "NR==2 {print \$4}")
  echo "=== lane $(date "+%T") load=$(sysctl -n vm.loadavg) free=${free}GiB"
  [ "$free" -ge 40 ] || { echo "DISK GUARD: ${free} GiB free < 40"; exit 3; }
  [ -d "$CARGO_BUILD_BUILD_DIR/debug" ] || python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-t14/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$CARGO_BUILD_BUILD_DIR/debug"
  cd "'"$OV"'" || exit 6
  for s in ${=RS1_SECTIONS}; do
    echo "=== $s $(date "+%T")"
    case $s in
      R) /usr/bin/time -l nice -n 15 cargo test -p semio-framework-raster --no-fail-fast -- --nocapture ;;
      F) /usr/bin/time -l nice -n 15 cargo test -p semio-framework-fonts --no-fail-fast ;;
      S) /usr/bin/time -l nice -n 15 cargo test -p semio-s-artifact-stdio-semio --features conversion-drawing --lib --no-fail-fast -- png::v1_2::any pdf::v1_7::any ;;
      H) /usr/bin/time -l nice -n 15 cargo test -p semio-s-artifact-shooting-shooting --lib --no-fail-fast -- photo scene_svg scene_png export_svg ;;
      W) /usr/bin/time -l nice -n 15 cargo test -p semio-framework-raster -p semio-framework-fonts --target wasm32-wasip2 --no-fail-fast ;;
      M) for size in 256 1080p 4096; do /usr/bin/time -l nice -n 15 cargo test -p semio-framework-raster --release --lib -- --ignored --nocapture --exact component::cpu::tests::debug_measure_$size; done ;;
    esac
    echo "$s rc=$? $(date "+%T")"
  done' >> "$OUT" 2>&1
echo "DONE $(date '+%T')" >> "$OUT"
