#!/bin/sh
# ⏳️ Waits for the stdio plugin lib to compile (a peer session is mid-refactor on 📄️pdf and 🖼️tiff),
# then runs per-case parity for every case wave 17 touched plus its regression neighbours.
PROBE="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w17-dwg-probe"
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w17-parity-out"
mkdir -p "$OUT"
tries=0
while [ "$tries" -lt 60 ]; do
  out=$(cd "$PROBE" && cargo build --message-format=short 2>&1)
  if ! printf '%s' "$out" | grep -qE 'error(\[|:)'; then
    echo "BUILD-OK after $tries wait(s)"
    break
  fi
  n=$(printf '%s' "$out" | grep -cE 'error(\[|:)')
  echo "waiting: $n error(s) still in the tree"
  tries=$((tries + 1))
  sleep 120
done
echo "=== dwg probe on the real fixture"
"$PROBE/target/debug/probe" "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg" 2>&1 | tail -5
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
for c in mutate-dwg-ac1018 mutate-dwg-ac1024 mutate-jpg-jfif-1-01-baseline mutate-tiff-6-0-baseline mutate-json-rfc8259-i-json mutate-docx-ecma-376-strict mutate-xlsx-ecma-376-strict create-and-read-jpeg create-and-round-trip-stl mutate-jpg-jfif-1-01 mutate-tiff-6-0 mutate-stl-ascii mutate-json-rfc8259; do
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$c" > "$OUT/$c.txt" 2>&1
  code=$?
  line=$(grep -m1 '^\[test\] level=' "$OUT/$c.txt")
  echo "RESULT $c exit=$code ${line:-NO-SUMMARY-LINE}"
done
echo "ALL-CASES-DONE"
