#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G=$C/🗑️generated/bzl-pin-plain; mkdir -p $G
export SEMIO_WORKSPACE_ROOT=$PWD SEMIO_TEST_ARTIFACT_DIR=$PWD/$C/🗑️generated/bzl-pin-art SEMIO_TEST_BAZEL_BINARY=$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
files=(
"🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/📥️runtime/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/💾️storage/🗑️reset/🧪️tests/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📤️arguments/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📋️contract/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️delegation/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️environment/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🧪️tests/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️control/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📦️snapshot/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/🔏️implementation/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️framing/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️inventory/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions/🟦️.ts"
"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint/🟦️.ts"
"⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🧪️tests/🟦️.ts"
"⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🩺️readiness/🧪️tests/🟦️.ts"
)
i=0; : > $G/summary.txt
for f in $files; do i=$((i+1)); log=$G/file-$i.log
 perl -e 'alarm 590; exec @ARGV' -- bun test "./$L/$f" --timeout 580000 > $log 2>&1; rc=$?
 echo "$i rc=$rc $(/usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' $log | tr -s ' ' | tr '\n' ' ') :: ${f}" >> $G/summary.txt
done
echo DONE >> $G/summary.txt
