#!/bin/sh
cd "$(dirname "$0")"
root=$(CDPATH= cd -- "$(pwd)/../../../../../../.." && pwd)
NATIVE_CARGO_DIR="$root/🧰️framework" sh native-cargo.sh test -p semio-framework-pack-record-encode-cursor --lib -- "$@" --test-threads=1 --nocapture
