#!/bin/bash
# 🔗 Sequential test chain: build once per package (`--no-run`, no timeout, the shared compiler queues), then run every test binary directly under a hard 29 minute timeout with explicit threads and without the benches.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
A="🏅️standards/🔖️1/🪆️subsets/✳️any"
run() { # dir pkg name [skips...]
  local dir="$1" pkg="$2" name="$3"; shift 3
  local skips=(); for s in "$@"; do skips+=(--skip "$s"); done
  cd "$dir" || return
  repo cargo --store=shared --configuration=prototype -- test -p "$pkg" --offline --no-run > "$T/🗑️generated/build-$name.log" 2>&1
  local n=0
  grep -o "(/[^)]*)" "$T/🗑️generated/build-$name.log" | tr -d '()' | while read -r exe; do
    [ -f "$exe" ] && [ -x "$exe" ] || continue
    n=$((n+1))
    timeout 1740 "$exe" --test-threads=4 "${skips[@]}" > "$T/🗑️generated/run-$name-$n.log" 2>&1
    echo "exit $? $exe" >> "$T/🗑️generated/run-$name.exits"
  done
}
rm -f "$T"/🗑️generated/run-*.exits
run "$N/🫨️en1998/$A/🎬️scene/📦️packages/🦀️rust" semio-s-artifact-norm-en1998-scene en1998-scene latency
run "$N/🫨️en1998/$A/✏️editor/📦️packages/🦀️rust" semio-s-artifact-norm-en1998-editor en1998-editor latency_bench
run "$N/🏛️en1992/$A/✏️editor/📦️packages/🦀️rust" semio-s-artifact-norm-en1992-editor en1992-editor latency_bench
echo done > "$T/🗑️generated/chain.done"
