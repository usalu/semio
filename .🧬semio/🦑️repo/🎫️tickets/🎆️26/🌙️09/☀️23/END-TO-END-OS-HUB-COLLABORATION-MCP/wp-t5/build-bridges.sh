#!/bin/zsh
# 🏭️ Builds every generated mutation bridge, one cargo at a time, into the slice's private target.
cd /Users/ueli/Documents/semio
out=.tmp-ticket/wp-t5/generated/bridges
mkdir -p $out
for manifest in $(find "✏️s/🔌️plugins" "🧰️framework/🛍️products/💻️os/🎚️config" -path "*/🏭️bridge/Cargo.toml" -not -path "*/🪆️subsets/*" | sort); do
  dir=${manifest:h}
  name=${${dir:h}:t}
  /usr/bin/grep -q "^$name exit 0" $out/summary.txt 2>/dev/null && continue
  ( cd "$dir" && CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t5/target-bridge cargo build --offline > "/Users/ueli/Documents/semio/$out/$name.txt" 2>&1; echo "exit $?" >> "/Users/ueli/Documents/semio/$out/$name.txt" )
  echo "$name $(tail -1 $out/$name.txt)" >> $out/summary.txt
done
echo done >> $out/summary.txt
