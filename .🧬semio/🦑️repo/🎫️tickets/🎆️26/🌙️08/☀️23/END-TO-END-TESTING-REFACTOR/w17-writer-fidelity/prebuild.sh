#!/bin/zsh
H="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/hosts"
build() {
  local n=$1; shift
  echo "=== BUILD $n $*"
  cargo build --quiet --manifest-path "$H/test-s-plugins-stdio-artifacts-$n/Cargo.toml" "$@" 2>&1 | grep -E "^error" -A 10
  echo "=== EXIT $n ${pipestatus[1]}"
}
for n in "$@"; do
  case "$n" in
    *-subject-rust) build "$n" --features sut ;;
    *) build "$n" ;;
  esac
done
echo ALLBUILT
