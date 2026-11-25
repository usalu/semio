#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad
export PATH="$HOME/.bun/bin:$PATH"
cd /Users/ueli/Documents/semio || exit 1
for i in {1..20}; do
  cargo test --offline --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml -p "$1" --lib > $S/$2.txt 2>&1
  grep -q "test result" $S/$2.txt && break
  sleep 90
done
echo done > $S/$2.done
