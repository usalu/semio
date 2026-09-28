#!/bin/zsh
# 🏠️ SH2: runs the permanent `verify home` harness (os-dev) with the hub test credential in env only (sh2-hub-env.sh).
# usage: zsh sh2-verify-home.sh --serve <url> [--hub <url>] [--locale en|de] [--tag <t>]
exec zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/sh2-hub-env.sh bun '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts' verify home --out /Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/generated/home-e2e "$@"
