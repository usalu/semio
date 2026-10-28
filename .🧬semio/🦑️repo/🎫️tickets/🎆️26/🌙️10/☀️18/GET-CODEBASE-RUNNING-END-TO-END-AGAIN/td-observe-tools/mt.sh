#!/bin/sh
# 👁️ [td-observe] runs one observation test file sealed inside a mirror: usage mt.sh <mirror|live> <bare|art|env> <file relative to 👁️observation/🧪️tests> [bun test args]; prints the summary line and the failing names; full output in 🗑️generated/td-observe/<mirror>-<mode>-<slug>.txt
export SEMIO_MIRROR_HOME="${SEMIO_MIRROR_HOME:-/private/tmp/sm}"; C="$(cd "$(dirname "$0")/.." && pwd)"; want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' /Users/ueli/Documents/semio/package.json); PINNED="${BUN:-}"; [ -n "$PINNED" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun "$HOME"/.cache/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && PINNED="$b" && break; done; [ -n "$PINNED" ] || { echo "pinned bun $want not found (set BUN)"; exit 2; }; name="$1"; mode="$2"; f="$3"; shift 3
O="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests"; G="$C/🗑️generated/td-observe"; mkdir -p "$G"
slug=$(printf '%s' "$f" | shasum | cut -c1-8); out="$G/$name-$mode-$slug.txt"
dir="$(dirname "$O/$f")"; leaf="./$(basename "$f")"
case "$mode" in art) A="$C/🗑️generated/to/$name"; mkdir -p "$A"; pre="SEMIO_TEST_ARTIFACT_DIR=$A";; *) pre="";; esac
if [ "$mode" = env ]; then cmd="sh $C/red-g-tools/mrun.sh $name env '$O/$f' $*"; elif [ -n "$pre" ]; then cmd="cd '$dir' && $pre nice -n 10 '$PINNED' test --timeout 170000 '$leaf' $*"; else cmd="cd '$dir' && env -u SEMIO_TEST_ARTIFACT_DIR nice -n 10 '$PINNED' test --timeout 170000 '$leaf' $*"; fi
if [ "$name" = live ]; then (cd /Users/ueli/Documents/semio && sh -c "$cmd") > "$out" 2>&1
else python3 -I "$C/mirror-tools/mirror.py" run "$name" --seal-allow "$C/🗑️generated" --peers-ok -- sh -c "$cmd" > "$out" 2>&1; fi
printf '%s %s %s: %s\n' "$name" "$mode" "$f" "$(grep -E '^ *[0-9]+ (pass|fail|skip)$|env: ' "$out" | tr -s ' \n' ' ')"
grep -E '^\(fail\)|GUARD|^error:' "$out" | cut -c1-200 | head -30
