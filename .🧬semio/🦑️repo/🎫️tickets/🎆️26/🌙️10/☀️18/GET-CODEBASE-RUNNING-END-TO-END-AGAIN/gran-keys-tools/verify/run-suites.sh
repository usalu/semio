#!/bin/zsh
# usage: run-suites.sh <sandbox> <out>
SB=$1; OUT=$2; L="$SB/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
: > $OUT
for t in \
 "⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules" \
 "🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️closure" \
 "🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint" \
 "🏗️build/🔧️tools/🏗️bazel/🦀️cargo/🧪️tests" \
 "🏗️build/📥️inputs/🎯️precision/🧪️tests" \
 "🏗️build/📥️inputs/🦀️resolution/🧪️tests" \
 "🧪️tests/🏠️nx-workspace-root-file-reference" \
 "🧪️tests/⚡️workspace-negation-closure" \
 "🧪️tests/🧱️cargo-transaction-command-source" \
 "🏗️build/🧪️tests/🕸️graph/🌱️native-scope" \
 "🏗️build/🧪️tests/🕸️graph/🛂️admission" ; do
  if [ -d "$L/$t" ]; then
    r=$(cd "$L/$t" && bun test ./🟦️.ts 2>&1 | grep -E "^ *[0-9]+ (pass|fail|skip)$" | tr '\n' ' ')
    echo "$t :: $r" >> $OUT
  else echo "$t :: missing" >> $OUT; fi
done
cat $OUT
