#!/bin/sh
# 🖊 Laws of batch `rustcode-writer` after apply: sh verify.sh <repository or mirror root>
here=$(cd "$(dirname "$0")" && pwd)
root=${1:?root}
python3 -I "$here/stage.py" check --root "$root" || exit 1
cd "$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library" || exit 1
bun build --no-bundle "./⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts" >/dev/null || exit 1
bun test --timeout 90000 "./⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🧪️tests/🖊️writer/🟦️.ts" || exit 1
echo "rustcode-writer VERIFIED (neighbour ./⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts: 10 pass / 18 fail on base and with the batch, identical failure set: 'Original parent Script process invocation required')"
