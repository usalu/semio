#!/bin/zsh
# 🧾️ U6 native-lane proof (rule 3): one hold = `cargo check --keep-going --lib --tests` over the crate list, then
# `cargo test --lib --no-fail-fast` per crate (continues past a red crate). Private target + captures under `.🧬semio/🌐hub/s14-u6-*`
# (rule 26: the ticket's gitignored dirs are swept). Features mirror L1's T1 set
# (`<crate>/component-app-assembly`) so build-fleet-b units are reused.
# usage: zsh u6-lane.sh <capture> <mode: check|test|both> <crate…>
setopt no_bg_nice
R=/Users/ueli/Documents/semio; T=$R/.tmp-ticket; W=$T/wp-u6
out="${1:A}"; mode="$2"; shift 2
crates=("$@")
args=(); feats=(); for c in $crates; do args+=(-p "$c"); feats+=("$c/component-app-assembly"); done
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="$R/.🧬semio/🌐hub/s14-u6-target"
cd $R || exit 2
echo "QUEUED native $mode ${#crates} crates $(date '+%F %T')" > "$out"
zsh "$T/📜️fleet-mutex.sh" native u6 -- nice -n 15 zsh -c '
  mode="$1"; feats="$2"; shift 2; rc=0
  if [ "$mode" != test ]; then
    echo "START check $(date "+%F %T")"; s=$(date +%s)
    cargo check --keep-going --message-format short --lib --tests --features "$feats" "$@"; rc=$?
    echo "END check rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"
  fi
  if [ "$mode" != check ]; then
    while [ $# -gt 0 ]; do
      c="$2"; shift 2
      echo "START test $c $(date "+%F %T")"; s=$(date +%s)
      cargo test --lib --features "$c/component-app-assembly" -p "$c" --no-fail-fast -- --test-threads 4; t=$?
      echo "END test $c rc=$t wall=$(( $(date +%s) - s ))s $(date "+%F %T")"
      [ $t -ne 0 ] && rc=$t
    done
  fi
  exit $rc' u6 "$mode" "${(j:,:)feats}" $args >> "$out" 2>&1
rc=$?
echo "LANE-EXIT rc=$rc $(date '+%F %T')" >> "$out"
exit $rc
