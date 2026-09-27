#!/bin/zsh
# 🦀️ AV2 overlay proof: every Rust crate the video-render slice touches, built and tested in the AV scratch overlay with
# PRIVATE build/target dirs (never the shared build-dir), one cargo at a time. Run it through the overlay lane:
#   setopt no_bg_nice; nohup zsh .tmp-ticket/📜️fleet-mutex.sh overlay av2 -- zsh .tmp-ticket/wp-av2/av2-overlay-cargo.sh <log-dir> [steps] > <capture> 2>&1 & disown
# steps (default all): kernel raster plugin host animate-artifact animate-plugin
setopt no_bg_nice
OVERLAY="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay"
LOGS="${1:-/Users/ueli/Documents/semio/.tmp-ticket/wp-av2/generated/cargo}"
shift
STEPS=("$@")
[ ${#STEPS[@]} -eq 0 ] && STEPS=(kernel raster plugin host animate-artifact animate-plugin)
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-target"
export CARGO_INCREMENTAL=0
mkdir -p "$LOGS"
cd "$OVERLAY" || exit 2
run() {
  local name="$1"; shift
  until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -lt 14 ]; do sleep 30; done
  echo "[av2-cargo] $name start $(date +%H:%M:%S) cargo $*" > "$LOGS/$name.txt"
  nice -n 15 cargo "$@" >> "$LOGS/$name.txt" 2>&1
  local rc=$?
  echo "[av2-cargo] $name exit=$rc end $(date +%H:%M:%S)" >> "$LOGS/$name.txt"
  echo "$name exit=$rc $(date +%H:%M:%S)"
}
for step in "${STEPS[@]}"; do
  case "$step" in
    kernel) run kernel test -p semio-framework --lib --no-fail-fast -- video_render ;;
    raster) run raster test -p semio-framework-raster --lib --no-fail-fast -- video ;;
    plugin) run plugin test -p semio-framework-plugin --lib --no-fail-fast -- wire_effect_round_trip ;;
    host) run host check -p semio-framework-plugin-host --lib --tests ;;
    animate-artifact) run animate-artifact test -p semio-s-artifact-animate-presentation --lib --no-fail-fast -- export_video program_unit a_deck a_tileless a_stated every_command retained ;;
    animate-plugin) run animate-plugin check -p semio-s-plugin-animate --lib --tests ;;
  esac
done
echo "[av2-cargo] all done $(date +%H:%M:%S)"
