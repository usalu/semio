#!/bin/sh
# 🩹 usage: verify.sh <mirror-root>   (mirror with the batch applied; the mirror holds 🧰️framework and is a git repository: git init)
M="$1"; T="$(cd "$(dirname "$0")/../.." && pwd)"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules"
cd "$T" && bun go-mirror.ts --root "$M" --module "$B/🔗️graphql/📦️packages/🐹️go" && bun go-mirror.ts --root "$M" --module "$B/📜️statutes/📦️packages/🐹️go"
