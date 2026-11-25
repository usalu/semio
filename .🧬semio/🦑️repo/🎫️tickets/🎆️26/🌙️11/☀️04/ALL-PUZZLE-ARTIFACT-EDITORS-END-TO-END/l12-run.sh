#!/usr/bin/env bash
# usage: l12-run.sh <2d|3d|5d|hub> <steps-file>; each steps-file line = cargo test args of one step. One 4g tmpfs namespace; overlay build dir (lower = prior 5d native cache); test executables deleted after each step; opt-level 1 + embed-metadata=no to fit 4g; L12_PLAIN=1 keeps the lower cache's own flags so unchanged crates come from the lower.
set -eu
t=$(cd "$(dirname "$0")" && pwd); dim=$1; steps=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
export L12_PKGS="${L12_PKGS:-semio-s-artifact-puzzle-3d-editor semio-s-artifact-puzzle-3d}" L12_DIM=$dim L12_REQ="$t/🗑️generated/l12-req-$dim" L12_STEPS=$steps L12_SPILL="${L12_SPILL:-$t/🗑️generated/l12-spill}"; mkdir -p "$L12_SPILL"
case $dim in
 2d) cd='cd "$PWD/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d"';;
 3d) cd='cd "$PWD/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d"';;
 5d) cd='cd "$P5"';;
 hub) cd='cd "$PWD/🌎️hub"';;
esac
[ -n "${L12_CD:-}" ] && cd="cd \"$L12_CD\""
body='G="$L12_SPILL"; for c in $L12_PKGS; do mkdir -p "$G/$c" /tmp/build/b/debug/build/$c; mount --bind "$G/$c" /tmp/build/b/debug/build/$c; done
[ -n "${L12_PLAIN:-}" ] || export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_CODEGEN_UNITS=16 CARGO_PROFILE_TEST_CODEGEN_UNITS=16 CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=true CARGO_PROFILE_DEV_OVERFLOW_CHECKS=true
export ZIG_GLOBAL_CACHE_DIR=/tmp/zig ZIG_LOCAL_CACHE_DIR=/tmp/zig-local
[ -n "${L12_PLAIN:-}" ] || export RUSTFLAGS="$RUSTFLAGS -C embed-bitcode=no -C strip=symbols -Zembed-metadata=no"
while IFS= read -r line; do
  [ -z "$line" ] && continue
  echo "L12-STEP $line"
  for a in 1 2 3 4 5; do eval "cargo test --no-run --jobs ${L12_JOBS:-2} --offline $line" && break; echo "L12-BUILD-RETRY $a"; done
  eval "cargo test --no-fail-fast --jobs ${L12_JOBS:-2} --offline $line" || echo "L12-EXIT $?"
  find /tmp/build/b/debug/build -path "*/semio-s-artifact-puzzle-*/out/*" -type f -perm -u+x ! -name "*.so" ! -name "*.rlib" ! -name "*.rmeta" ! -name "build_script*" ! -name "build-script*" -delete 2>/dev/null || true
  df -h /tmp | tail -1
done < "$L12_STEPS"
echo L12-DONE
while :; do
  while [ ! -s "$L12_REQ" ]; do sleep 3; done
  mv -f "$L12_REQ" "$L12_REQ.cur"
  [ "$(head -1 "$L12_REQ.cur")" = quit ] && exit 0
  while IFS= read -r line; do
    [ -z "$line" ] && continue
    echo "L12-STEP $line"
    for a in 1 2 3 4 5; do eval "cargo test --no-run --jobs ${L12_JOBS:-2} --offline $line" && break; echo "L12-BUILD-RETRY $a"; done
    eval "cargo test --no-fail-fast --jobs ${L12_JOBS:-2} --offline $line" || echo "L12-EXIT $?"
    find /tmp/build/b/debug/build -path "*/semio-s-artifact-puzzle-*/out/*" -type f -perm -u+x ! -name "*.so" ! -name "*.rlib" ! -name "*.rmeta" ! -name "build_script*" ! -name "build-script*" -delete 2>/dev/null || true
  done < "$L12_REQ.cur"
  echo L12-DONE
done'
L2_SIZE="${L12_SIZE:-4g}" exec "$t/l2-native.sh" x "$cd; $body"
