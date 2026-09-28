#!/bin/zsh
# 🧾️ L1 combined proof through the fleet lanes. usage:
#   zsh l1-check.sh native <capture> <crates-file> [<features-file>]   cargo check --keep-going --lib --tests (build-fleet-b, private target)
#   zsh l1-check.sh wasm   <capture> <crates-file> [<target>]          cargo check --keep-going --lib --target <wasm32-wasip2> (default build-dir)
# <crates-file>: one package per line (# comments ok). Capture: full cargo output + START/END lines with rc and wall time.
setopt no_bg_nice
R=/Users/ueli/Documents/semio; T=$R/.tmp-ticket
lane="$1"; out="${2:A}"; list="${3:A}"; extra="${4:-}"; [ "$lane" = native ] && [ -n "$extra" ] && extra="${extra:A}"
crates=(${(f)"$(/usr/bin/grep -v '^#' "$list" | /usr/bin/grep -v '^ *$')"})
args=(); for c in $crates; do args+=(-p "$c"); done
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-6}"
cd $R || exit 2
case "$lane" in
  native)
    feats=(); [ -n "$extra" ] && feats=(--features "${(j:,:)${(f)"$(/usr/bin/grep -v '^#' "$extra" | /usr/bin/grep -v '^ *$')"}}")
    export CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="$T/wp-l1/target"
    echo "QUEUED native ${#crates} crates $(date '+%F %T')" > "$out"
    zsh "$T/📜️fleet-mutex.sh" native l1 -- zsh -c 'echo "START $(date "+%F %T")"; s=$(date +%s); nice -n 5 cargo check --keep-going --message-format short --lib --tests "$@"; rc=$?; echo "END rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc' l1 $args $feats >> "$out" 2>&1 ;;
  wasm)
    target="${extra:-wasm32-wasip2}"
    unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
    echo "QUEUED wasm $target ${#crates} crates $(date '+%F %T')" > "$out"
    zsh "$T/📜️fleet-mutex.sh" wasm l1 -- zsh -c 'echo "START $(date "+%F %T")"; s=$(date +%s); nice -n 5 cargo check --keep-going --message-format short --lib --target "$0" "$@"; rc=$?; echo "END rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc' "$target" $args >> "$out" 2>&1 ;;
esac
rc=$?
echo "LANE-EXIT rc=$rc $(date '+%F %T')" >> "$out"
exit $rc
