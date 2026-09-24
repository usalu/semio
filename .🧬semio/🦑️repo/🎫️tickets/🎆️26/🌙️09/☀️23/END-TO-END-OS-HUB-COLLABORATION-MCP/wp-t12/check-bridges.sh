#!/bin/zsh
# 🏭️ Checks the framework kernel/derive/replication test targets, then every standalone bridge workspace, one cargo at a time.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
echo "== kernel tests"
cargo check -p semio-framework-replication -p semio-framework-os-kernel-dsl-derive -p semio-framework-os-kernel --tests --keep-going --message-format=short 2>&1 | /usr/bin/grep -E '^(error|warning: unused)|error\[|: error|Finished|could not compile' ; echo "KERNEL_EXIT=${pipestatus[1]}"
while read -r manifest; do
  name=$(/usr/bin/grep -m1 '^name' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
  echo "== $name ($manifest)"
  cargo check --manifest-path "$manifest" -p "$name" --keep-going --message-format=short 2>&1 | /usr/bin/grep -E 'error|Finished|could not compile' | head -40
  echo "BRIDGE_EXIT $name ${pipestatus[1]}"
done < .tmp-ticket/wp-t12/bridge-manifests.txt
echo ALL_DONE
