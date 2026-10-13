#!/bin/bash
# 🧪️ Runs every test of one libtest binary in its own process (`--exact <name> --test-threads=1`), RUST_MIN_STACK 64 MiB,
# per-test alarm; writes `<out>/results.tsv` (status, name, seconds) and one log per test.
# usage: 🧪️each-test.sh <bin> <cwd> <out-dir> [parallel=6] [alarm-s=300] [filter-regex=.] [shard=0/1]
# A shard `k/n` runs every n-th listed test starting at k and appends to results.tsv (shard 0 truncates it).
BIN=$1; CWD=$2; OUT=$3; PAR=${4:-6}; AL=${5:-300}; FILTER=${6:-.}; SHARD=${7:-0/1}
K=${SHARD%/*}; N=${SHARD#*/}
mkdir -p "$OUT"; [ "$K" = 0 ] && rm -f "$OUT/results.tsv"; export BIN CWD OUT AL
export RUST_MIN_STACK=67108864 CARGO_MANIFEST_DIR="$CWD" SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:-$OUT/artifacts}"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
cd "$CWD" || exit 2
one() {
  name="$1"; h=$(printf '%s' "$name" | md5 -q); s=$(date +%s)
  perl -e 'alarm shift; exec @ARGV' "$AL" "$BIN" --exact "$name" --test-threads=1 > "$OUT/$h.log" 2>&1
  code=$?
  if [ $code -eq 0 ]; then st=ok; elif [ $code -eq 142 ]; then st=TIMEOUT; elif [ $code -eq 134 ]; then st=ABORT; elif [ $code -eq 101 ]; then st=FAILED; else st="EXIT$code"; fi
  printf '%s\t%s\t%s\n' "$st" "$name" "$(( $(date +%s) - s ))" >> "$OUT/results.tsv"
}
export -f one
"$BIN" --list 2>/dev/null | grep ': test$' | sed 's/: test$//' | grep -E "$FILTER" | awk -v k="$K" -v n="$N" '(NR-1) % n == k' | xargs -P "$PAR" -I{} bash -c 'one "$@"' _ {}
awk -F'\t' '{print $1}' "$OUT/results.tsv" | sort | uniq -c
