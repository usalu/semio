#!/bin/sh
# 🗂 schemaidx verify: `sh verify.sh <root>` = the check-schema-index law (the committed index equals the projection of the root); red on live 23:40 (missing `semio:laws/test-store-isolation/rules`, added by G49 isolate without regenerating), green after apply.
HERE=$(cd "$(dirname "$0")" && pwd); bun "$HERE/regen.ts" "$1" --check
