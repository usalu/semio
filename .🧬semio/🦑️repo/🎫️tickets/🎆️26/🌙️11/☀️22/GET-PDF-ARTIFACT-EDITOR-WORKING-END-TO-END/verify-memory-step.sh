#!/bin/sh
# usage: verify-memory-step.sh <tag>  (gated: PDF lib suite, io-export, page-editor tests, snapshot-value `editor::pdf14`, XLSX/XML lib type-check; 5 s rustc sampler; 🗑️generated/verify-<tag>.{log,mem.log,peaks.txt,done})
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
G="$here/🗑️generated"
tag=${1:?tag}
P=semio-s-artifact-stdio-pdf
bun=$(ls -d "$HOME"/Library/Caches/semio/runtimes/bun/*/ 2>/dev/null | head -1)
export PATH="${bun%/}:$PATH" RUST_MIN_STACK=67108864
stop="$G/verify-$tag.stop"
log="$G/verify-$tag.log"
rm -f "$stop" "$G/verify-$tag.done"
: > "$log"
sh "$here/sample-rustc-memory.sh" "$G/verify-$tag.mem.log" "$stop" 0 &
sampler=$!
run() { echo "== $*" >> "$log"; sh "$here/native-cargo.sh" "$@" >> "$log" 2>&1; echo "== exit $?" >> "$log"; }
run test --no-fail-fast -p $P -p $P-io-export -p $P-page-editor-tests --lib
run test -p $P-pdf-snapshot-value-tests --lib -- editor::pdf14 --test-threads=1
NATIVE_CARGO_DIR="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx" run check -p semio-s-artifact-stdio-xlsx --lib --message-format short
NATIVE_CARGO_DIR="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml" run check -p semio-s-artifact-stdio-xml --lib --message-format short
touch "$stop"
wait "$sampler" 2>/dev/null || true
python3 "$here/peak-rustc-memory.py" "$G/verify-$tag.mem.log" > "$G/verify-$tag.peaks.txt"
grep -a -E "^== |^test result|FAILED|panicked|^error" "$log" > "$G/verify-$tag.summary.txt"
echo done > "$G/verify-$tag.done"
