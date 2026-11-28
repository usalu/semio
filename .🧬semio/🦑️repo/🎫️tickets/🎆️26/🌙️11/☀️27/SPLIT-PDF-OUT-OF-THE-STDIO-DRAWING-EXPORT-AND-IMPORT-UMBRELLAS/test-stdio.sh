#!/usr/bin/env bash
# 🧪️ Lib tests of the stdio drawing export/import umbrellas and their pdf crates.
exec "$(dirname "$0")/cargo-lane.sh" 4 test --manifest-path "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml" --no-fail-fast --lib --message-format short \
  -p semio-s-artifact-stdio-semio-drawing-export -p semio-s-artifact-stdio-semio-drawing-import -p semio-s-artifact-stdio-semio-drawing-pdf-export -p semio-s-artifact-stdio-semio-drawing-pdf-import
