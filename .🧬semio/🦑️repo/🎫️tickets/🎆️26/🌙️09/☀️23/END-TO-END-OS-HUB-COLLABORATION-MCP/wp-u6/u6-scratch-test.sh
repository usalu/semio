#!/bin/zsh
# 🧾️ U6 prepared-set proof on the scratch copy (native lane, PRIVATE build-dir + target inside it, `--offline`): per crate
# `cargo check --lib --tests` then `cargo test --lib`, both with `<crate>/component-app-assembly`. The live tree stays untouched.
# usage: [U6_PLAIN="<crate …>"] zsh u6-scratch-test.sh <scratch-root> <capture> <crate…>   (U6_PLAIN: crates without that feature)
setopt no_bg_nice
SC="${1:A}"; out="${2:A}"; shift 2
T=/Users/ueli/Documents/semio/.tmp-ticket
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_BUILD_DIR="$SC/.u6-build" CARGO_TARGET_DIR="$SC/.u6-target"
cd "$SC" || exit 2
echo "QUEUED scratch-test $* $(date '+%F %T')" > "$out"
zsh "$T/📜️fleet-mutex.sh" native u6 -- nice -n 15 zsh -c '
  rc=0
  for c in "$@"; do
    echo "START $c $(date "+%F %T")"; s=$(date +%s)
    f=(--features "$c/component-app-assembly"); [[ " $U6_PLAIN " == *" $c "* ]] && f=()
    cargo check --offline --keep-going --message-format short --lib --tests $f -p "$c"; k=$?
    cargo test --offline --lib $f -p "$c" --no-fail-fast -- --test-threads 4; t=$?
    echo "END $c check rc=$k test rc=$t wall=$(( $(date +%s) - s ))s $(date "+%F %T")"
    [ $k -ne 0 ] && rc=$k; [ $t -ne 0 ] && rc=$t
  done
  exit $rc' u6 "$@" >> "$out" 2>&1
rc=$?
echo "LANE-EXIT rc=$rc $(date '+%F %T')" >> "$out"
exit $rc
