#!/bin/sh
# 🔬️ [td-observe] verify.sh <root>: parse-checks every staged TS file and runs the laws this batch changes INSIDE <root> (a mirror with the batch applied; never live) with the PINNED Bun (package.json packageManager; the PATH bun may be older). obsbare: bare `bun test ./🟦️.ts` of the main observation law + the build suite route law. obslaws: the block reader rows and the presence refusal/terminal rows beneath the genuine `setup` birth (artifacts under <root>/.tmp/to). Exit 0 only when every run reports 0 fail.
R="$(cd "${1:?usage: verify.sh <root>}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; C="$(cd "$HERE/../../.." && pwd)"; batch="$(basename "$HERE")"
want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' "$R/package.json"); B="${BUN:-}"
[ -n "$B" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun "$HOME"/.cache/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && B="$b" && break; done
[ -n "$B" ] || { echo "pinned bun $want not found (set BUN)"; exit 2; }
O="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests"; GB="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/📜️script.ts"
G="$C/🗑️generated/td-observe/verify-$batch"; mkdir -p "$G" "$R/.tmp/to"; rc=0
python3 -I -c 'import json,sys;[print(sys.argv[2]+"/"+k) for k in json.load(open(sys.argv[1]+"/manifest.json")) if k.endswith(".ts")]' "$HERE" "$R" > "$G/scan.txt"
"$B" "$C/gate-g-tools/scan.ts" "$G/scan.txt" "$G/scan" | tail -1 | grep -q "bad 0" || { echo "PARSE FAIL"; rc=1; }
summary() { printf '%s: %s\n' "$1" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$2" | tr -s ' \n' ' ')"; grep -qE '^ *[1-9][0-9]* fail$' "$2" && rc=1; grep -qE '^ *[0-9]+ pass$' "$2" || rc=1; }
birth() { (cd "$R" && SEMIO_TEST_ARTIFACT_DIR="$R/.tmp/to" nice -n 10 "$B" "$GB" "--file=$O/$1" --birth=setup "--root=$R" "--pattern=$2") > "$G/$3.txt" 2>&1; summary "$3" "$G/$3.txt"; for d in $(mount | grep "$R/.cache/b/o/m/tmp/gf-" | awk '{print $1}'); do hdiutil detach "$d" >/dev/null 2>&1; done; }
case "$batch" in
 obsbare)
  (cd "$O" && env -u SEMIO_TEST_ARTIFACT_DIR nice -n 10 "$B" test --timeout 170000 ./🟦️.ts) > "$G/main-bare.txt" 2>&1; summary "main bare" "$G/main-bare.txt"
  (cd "$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🧪️tests" && SEMIO_TEST_ARTIFACT_DIR="$R/.tmp/to" nice -n 10 "$B" test --timeout 170000 ./🟦️.ts) > "$G/suites.txt" 2>&1; summary "suite route law" "$G/suites.txt";;
 obslaws)
  birth "🧱️blocks/🟦️.ts" "^observation.block.reader" blocks-readers
  birth "🪪️presence/🟦️.ts" "^inference.reuse.(classifies|refuses.a.[a-z0-9-]+.source)" presence-source
  birth "🪪️presence/🟦️.ts" "^inference.reuse.refuses.a.[a-z0-9-]+.lock" presence-lock
  birth "🪪️presence/🟦️.ts" "^inference.reuse.(revalidates|preserves)" presence-reuse;;
 *) echo "unknown batch $batch"; exit 2;;
esac
echo "verify $batch rc=$rc (logs $G)"; exit $rc
