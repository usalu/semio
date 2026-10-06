#!/bin/zsh
# 🔁️ Starts the unattended acceptance batch only on a compiling base: every ten minutes one `cargo check --lib` over the crates
# nearly every editor depends on and that outside peers keep mid-refactor (stdio dwg / txt / deflate / semio, the workflow
# artifact, kernel + plugin). The first green pass launches `🔁️acceptance-batch.sh` and ends; a red pass logs its first error.
# Private target + build dir `🗑️generated/coord/base-target` (check-only, no shared lock). Log: `🗑️generated/coord/base-green.txt`.
setopt no_bg_nice
ticket="${0:A:h}"; coord="$ticket/🗑️generated/coord"; log="$coord/base-green.txt"
cd /Users/ueli/Documents/semio || exit 2
export CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR="$coord/base-target" CARGO_BUILD_BUILD_DIR="$coord/base-target"
while true; do
  out="$coord/base-check.txt"
  cargo check -p semio-framework-os-kernel -p semio-framework-plugin -p semio-framework-artifact-workflow-workflow --lib --message-format=short > "$out" 2>&1
  first=$?
  cargo check --manifest-path "✏️s/Cargo.toml" -p semio-s-artifact-stdio-dwg -p semio-s-artifact-stdio-txt -p semio-s-artifact-stdio-deflate -p semio-s-artifact-stdio-semio --lib --message-format=short >> "$out" 2>&1
  second=$?
  if [ $first -eq 0 ] && [ $second -eq 0 ]; then
    print -r -- "$(date '+%F %T') BASE GREEN → acceptance batch launched" >> "$log"
    python3 "$ticket/🚀️detach.py" "$ticket/🗑️generated/s5-agnostic/batch.out" "$PWD" /bin/zsh "$ticket/🔁️acceptance-batch.sh"
    exit 0
  fi
  print -r -- "$(date '+%F %T') BASE RED ($first/$second, $(/usr/bin/grep -c -E ': error' "$out") errors): $(/usr/bin/grep -m1 -E ': error' "$out" | sed 's|/Users/ueli/Documents/semio/||' | cut -c1-170)" >> "$log"
  sleep 600
done
