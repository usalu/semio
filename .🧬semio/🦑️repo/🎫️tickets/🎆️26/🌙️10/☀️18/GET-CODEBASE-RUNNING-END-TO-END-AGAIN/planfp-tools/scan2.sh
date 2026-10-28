#!/bin/sh
# 🔎 Runs every bazel test file that reads 📜️script.ts as text (plain bun test, artifact dir set) and prints pass/fail per file: sh scan2.sh <root> <label> <outdir>
root=$1; label=$2; out=$3
cd "$root" || exit 1
for f in 📥️actions 📥️purpose/🧪️tests/🌱️birth 🔏️implementation 📥️environment 📥️inventory 📤️arguments 📥️platform 📥️delegation 📥️fingerprint ""; do
 p="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/${f:+$f/}🟦️.ts"
 SEMIO_TEST_ARTIFACT_DIR="$out/art" bun test "$p" > "$out/scan.txt" 2>&1
 printf "%s %-40s %s | %s\n" "$label" "${f:-🧪️tests}" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$out/scan.txt" | tr -s ' \n' ' ')" "$(grep -E '^\(fail\)' "$out/scan.txt" | cut -c1-90 | tr '\n' ';')"
done
