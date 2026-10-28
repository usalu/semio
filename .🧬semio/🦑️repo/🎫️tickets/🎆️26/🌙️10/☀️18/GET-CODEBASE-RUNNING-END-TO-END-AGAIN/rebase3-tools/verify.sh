#!/bin/sh
# [rebase3] laws of reuse-once + plan-once2 + rustfast (graph-free single-file tests). usage: sh verify.sh <root>   (live root after landing, or a mirror root; run under `mirror.py run` for a mirror). Exit 0 = all green.
R=${1:?root}; cd "$R" || exit 1
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'; G="$L/🏗️build/🕸️graph"; U="$L/⚡️caching/🦀️cargo/🗃️unit-cache"; P='🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units'
status=0
for f in "$G/🕰️generations/🧩️claims/🧪️tests/💤️idle" "$G/🕰️generations/🔌️load/🧪️tests/🪑idle-builder" "$G/📥️inference/🧪️tests/🍱️shared-setup" "$G/🧪️tests/🧳️carried-closures" "$G/🕰️generations/🧪️tests/🪞️plan-once" \
  "$G/🕰️generations/🧩️claims/🧪️tests" "$G/🕰️generations/🔌️load/🧪️tests" "$G/🧪️tests/🕰️generation-protocol" "$G/🧪️tests/♻️tagged-retention" "$G/🧪️tests/🔗️edge-refresh" "$G/🧪️tests/🕊️graph-release" "$G/🕰️generations/🧪️tests" \
  "$U/📖️admission/🧪️tests/🔑️key-stability" "$U/📖️admission/📚️foreign/🧪️tests" "$U/🔏️key/🧪️tests/🧩️parts" "$L/🏃️process/🦀️cargo/⏱️timing/🧪️tests/📒️ledger" "$P/🧪️tests/🔁️plan-stability" "$P/🧪️tests/🔁️duplicate-edges" "$U/🧪️tests" "$U/🔏️key/🧪️tests"; do
  out=$(bun test --timeout 300000 "./$f/🟦️.ts" 2>&1); r=$(echo "$out" | grep -E "^ *[0-9]+ (pass|fail)" | tr '\n' ' '); echo "${f##*/}: $r"
  echo "$out" | grep -qE "^ *0 fail" || { status=1; echo "$out" | grep -E "^\(fail\)" | head -5; }
done
exit $status
