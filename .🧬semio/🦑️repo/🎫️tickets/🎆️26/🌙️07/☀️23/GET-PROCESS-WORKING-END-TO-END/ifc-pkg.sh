#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims:$PATH"  # managed cargo entry
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
export PATH="$HOME/.bun/bin:$PATH"
cd /Users/ueli/Documents/semio || exit 1
L=$1; shift
cargo test --offline --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml -p $PKG --lib "$@" 2>&1 | awk '/work quote denied/{c[$0]++;next}{print substr($0,1,700)}END{for(k in c)print "[DEBUG-SUMMARY] " c[k] "x " k}' > "/private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/$L.txt"; echo exit >> "/private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/$L.txt"
