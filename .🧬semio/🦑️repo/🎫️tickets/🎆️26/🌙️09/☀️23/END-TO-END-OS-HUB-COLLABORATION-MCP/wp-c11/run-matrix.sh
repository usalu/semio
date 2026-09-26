#!/bin/zsh
# 🧮️ C11: two-browser collaboration matrix against a hub (default 7800) through two serves. Durable log under s13-c11-logs/<tag>.txt
# usage: zsh run-matrix.sh <tag> <locale en-US|de-DE> <url1> <url2> [spaceId|-] [kindId,…] [hubUrl] [adminCapabilityFile]
TAG="$1"; LOC="$2"; U1="$3"; U2="$4"; SPACE="${5:--}"; KINDS="${6:-}"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
export S_MATRIX_LOCALE="$LOC" S_MATRIX_HUB="${7:-http://127.0.0.1:7800}" S_MATRIX_ADMIN_FILE="${8:-$H/s12-w2-state-7800/admin-capability.json}"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-c11 || exit 1
echo "START $(date '+%F %T') tag=$TAG locale=$LOC pid=$$"
nice -n 10 bun c11-collab-matrix.mjs "$TAG" "$U1" "$U2" "$SPACE" "$KINDS"
echo "EXIT rc=$? $(date '+%F %T')"
