#!/bin/sh
# 🌱️ [td-observe] runs one observation test file beneath a declared genuine birth inside a mirror (sealed): usage gb.sh <mirror> <birth> <file relative to 👁️observation/🧪️tests> [--pattern=<regex>]; full output in 🗑️generated/td-observe/gb-<mirror>-<birth>-<slug>.txt
export SEMIO_MIRROR_HOME="${SEMIO_MIRROR_HOME:-/private/tmp/sm}"; C="$(cd "$(dirname "$0")/.." && pwd)"; want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' /Users/ueli/Documents/semio/package.json); PINNED="${BUN:-}"; [ -n "$PINNED" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun "$HOME"/.cache/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && PINNED="$b" && break; done; [ -n "$PINNED" ] || { echo "pinned bun $want not found (set BUN)"; exit 2; }; name="$1"; birth="$2"; f="$3"; shift 3; export TO_EXTRA="$*"
M="$(cd "$(python3 -I "$C/mirror-tools/mirror.py" list | awk -v n="$name" '$1==n{print $NF}')" && pwd -P)"
O="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests"; GB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
out="$C/🗑️generated/td-observe/gb-$name-$birth-$(printf '%s' "$f$*" | shasum | cut -c1-8).txt"
python3 -I "$C/mirror-tools/mirror.py" run "$name" --seal-allow "$C/🗑️generated" --peers-ok -- sh -c "cd '$M' && nice -n 10 '$PINNED' \$( [ -n \"\${SDK:-}\" ] && echo '$C/td-observe-tools/sdkdrive.ts $M $O/$f $birth' || echo '$M/$GB --file=$O/$f --birth=$birth --root=$M') \${TO_EXTRA:+\"\$TO_EXTRA\"}" > "$out" 2>&1
printf '%s %s %s: %s\n' "$name" "$birth" "$f" "$(grep -E '^ *[0-9]+ (pass|fail|skip)$' "$out" | tr -s ' \n' ' ')"
grep -E '^\(fail\)|GUARD|^error:|genuine-test failed' "$out" | cut -c1-200 | head -30
