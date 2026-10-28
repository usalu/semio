#!/usr/bin/env bash
# rs-win.sh <root> <outdir>: type-check every std-only Rust source of the repo build tool for the Windows targets (rustc --emit=metadata, library and test harness)
set -uo pipefail
root="$1"; out="$2"; mkdir -p "$out"
L="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P="$root/🧰️framework/🔨️modules/🏃️process"
for t in x86_64-pc-windows-msvc aarch64-pc-windows-msvc; do
  for spec in "watcher|$L/🏗️build/🏡️resident/🔭️watch/🦀️rust/🦀️.rs|bin" "graph|$L/🏗️build/🕸️graph/🦀️rust/🦀️.rs|lib" "workspace|$P/🧭️routing/🏠️workspace/🦀️.rs|lib" "resources|$P/📦️artifacts/🏗️native-build/📥️resources/🦀️.rs|lib" "compiler|$P/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs|lib" "admission|$P/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs|bin"; do
    IFS='|' read -r n f k <<<"$spec"
    for mode in "$k" test; do
      flags=(--crate-type "$k"); [ "$mode" = test ] && flags=(--test)
      printf '%s %-9s %-4s: ' "$t" "$n" "$mode"
      ed=2021; [ "$n" = admission ] && ed=2024; rustc +nightly-2026-07-20 --edition "$ed" --crate-name "$n" "${flags[@]}" --target "$t" --emit=metadata -o "$out/$n-$t-$mode.rmeta" "$f" 2>"$out/$n-$t-$mode.log"; status=$?
      echo "exit $status, $(grep -c '^warning' "$out/$n-$t-$mode.log") warnings, $(grep -c '^error' "$out/$n-$t-$mode.log") errors"
    done
  done
done
