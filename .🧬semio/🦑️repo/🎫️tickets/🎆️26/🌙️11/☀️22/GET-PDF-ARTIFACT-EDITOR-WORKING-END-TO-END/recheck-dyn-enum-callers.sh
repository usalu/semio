#!/bin/sh
# usage: recheck-dyn-enum-callers.sh  (waits until the store test-support file was quiet for 15 minutes, at most 40, then checks the four test crates that close PluginApp without the prelude; results in 🗑️generated/t6.summary)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
f="$root/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs"
waited=0
while [ "$waited" -lt 40 ] && [ -n "$(find "$f" -mmin -15 2>/dev/null)" ]; do sleep 60; waited=$((waited+1)); done
c="$root/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions"
rm -f "$here/🗑️generated/t6.summary"
for t in 🍌️tests-05 🍈️tests-10 🔧️tests-11-2 🥝️tests-16; do
  NATIVE_CARGO_DIR="$root" sh "$here/native-cargo.sh" check --tests --lib --manifest-path "$c/$t/📦️packages/🦀️rust/Cargo.toml" > "$here/🗑️generated/t6-$t.log" 2>&1
  echo "$t exit $?" >> "$here/🗑️generated/t6.summary"
done
echo done >> "$here/🗑️generated/t6.summary"
