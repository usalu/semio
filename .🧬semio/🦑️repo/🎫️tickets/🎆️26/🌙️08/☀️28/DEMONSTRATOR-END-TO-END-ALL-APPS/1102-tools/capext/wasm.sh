#!/bin/bash
# usage: wasm.sh <tag> <batch>...   wasm32-wasip2 --lib check, one crate at a time
source "$(dirname "$0")/lib.sh"
tag=$1; shift
rm -f "$G/capwasm-run-$tag.rc"
for b in "$@"; do
  read -r dir pkg <<< "$(spec "$b")"
  out="$G/capwasm-$b-$tag"; rm -f "$out.rc"
  cd "$ROOT" || exit 1
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$dir/Cargo.toml" --target wasm32-wasip2 --keep-going --lib -p "$pkg" > "$out.txt" 2>&1
  echo $? > "$out.rc"
done
echo done > "$G/capwasm-run-$tag.rc"
