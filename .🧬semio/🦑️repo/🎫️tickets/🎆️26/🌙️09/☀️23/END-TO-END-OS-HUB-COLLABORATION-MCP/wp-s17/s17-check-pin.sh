#!/bin/zsh
# 🧪️ S17 compile-atomic native gate of the VersionPin landing set, sequential phases (preamble rules 25/26: fleet-b
# build-dir, nice 10): A = SDK core + procedural + flow artifact + all 26 extensions; B = demonstrator; C = stdio + hub.
# usage: zsh s17-check-pin.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
a=(semio-framework semio-framework-os-kernel semio-framework-plugin semio-s-plugin-procedural semio-s-artifact-flow-flow)
for c in ✏️s/🔌️plugins/*/🧩️extensions/*/📦️packages/🦀️rust/Cargo.toml(N); do a+=($(/usr/bin/grep -m1 '^name' "$c" | sed -e 's/.*"\(.*\)".*/\1/')); done
echo "START $(date '+%T') phaseA=${#a}" > "$1"
for phase in A B C; do
  case $phase in
    A) crates=($a) ;;
    B) crates=(semio-s-plugin-demonstrator) ;;
    C) crates=(semio-s-plugin-stdio semio-hub) ;;
  esac
  args=(); for c in $crates; do args+=(-p $c); done
  until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -le 14 ]; do sleep 30; done
  echo "PHASE $phase START $(date '+%T')" >> "$1"
  nice -n 10 cargo check $args --lib --tests --message-format short >> "$1" 2>&1
  code=$?
  echo "PHASE $phase EXIT=$code $(date '+%T')" >> "$1"
  [ $code -ne 0 ] && break
done
echo "ALL_DONE $(date '+%T')" >> "$1"
