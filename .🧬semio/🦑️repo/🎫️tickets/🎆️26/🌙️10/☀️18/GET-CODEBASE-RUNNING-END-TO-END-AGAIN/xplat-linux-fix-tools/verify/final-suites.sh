#!/bin/zsh
OV=${1:-/private/tmp/xplat-linux-fix/ov-all}
F="🧰️framework"; P="$F/🔨️modules/🏃️process"; NB="$P/📦️artifacts/🏗️native-build/💾️storage"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; H="$P/🏗️build/🌡️host-capacity"; U="$L/⚡️caching/🦀️cargo/🗃️unit-cache"; R="$NB/🌐️platform/🥾️boot/🩹️recovery"
files=(
"$NB/📁️directory/🧪️tests/🟦️.ts" "$NB/📁️directory/📏️usage/🧪️tests/🟦️.ts" "$NB/🌐️platform/📋️contract/📐️statfs/🧪️tests/🟦️.ts" "$NB/🌐️platform/📋️contract/📐️statfs/🧪️tests/🔒️sites/🟦️.ts" "$NB/🌐️platform/🧪️tests/🧭️dispatch/🟦️.ts"
"$R/🕰️clock/🧪️tests/🟦️.ts" "$R/🧪️tests/🟦️.ts" "$R/🧪️tests/🩺️heal/🟦️.ts" "$R/🧪️tests/🔗️wiring/🟦️.ts"
"$NB/🧱️fixed/🧪️tests/🟦️.ts" "$NB/🧱️fixed/🧪️tests/🌱️birth/🟦️.ts" "$NB/📤️publication/🧱️fixed/🧪️tests/🟦️.ts" "$NB/🧪️tests/🟦️.ts"
"$H/🧪️tests/📡️sampling/🟦️.ts" "$H/🧪️tests/📡️sampling/🐧️linux/🟦️.ts" "$H/🧪️tests/🚪️gate/🟦️.ts" "$H/🧪️tests/🟦️.ts"
"$U/🧵️lane/🧪️tests/🟦️.ts"
"$L/🏗️build/🏃️execution/🔒️hermetic/🧪️tests/🟦️.ts" "$L/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/📥️strategies/🟦️.ts"
"$F/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts"
"$L/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts"
)
for f in $files; do
  line=$(/private/tmp/xplat-linux-fix/tw.sh "$OV" "$f" 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail|skip)|^\(fail\)" | tr '\n' ' ')
  echo "${f: -66} :: $line"
done
