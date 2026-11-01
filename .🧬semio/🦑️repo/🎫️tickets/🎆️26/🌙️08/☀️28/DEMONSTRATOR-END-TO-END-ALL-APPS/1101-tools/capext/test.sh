#!/bin/bash
# usage: test.sh <tag> <batch>...   native cargo test of the extension crate
source "$(dirname "$0")/lib.sh"
tag=$1; shift
rm -f "$G/captest-run-$tag.rc"
for b in "$@"; do
  read -r dir pkg <<< "$(spec "$b")"
  p=${pkg%-component}
  out="$G/captest-$b-$tag"; rm -f "$out.rc"
  cd "$ROOT" || exit 1
  RUST_BACKTRACE=1 SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$ROOT/$dir/Cargo.toml" --no-fail-fast -p "$p" > "$out.txt" 2>&1
  echo $? > "$out.rc"
done
echo done > "$G/captest-run-$tag.rc"
