#!/bin/zsh
# 🧪️ LB2 p17 baseline (overlay lane, inside the scratch): revert p17 → the stdio lib tests on the live-equivalent scratch → re-apply p17.
# Answers whether a p17-s2 red is the live baseline. `zsh lb2-p9-scratch.sh run <capture> zsh <this>`.
W="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2"
SC="$PWD"
python3 "$W/lb2-p17-pack-schema-identity.py" --revert --root "$SC" | tail -1
echo "REVERTED $(date '+%T')"
cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --lib 2>&1 | /usr/bin/grep -E "^test .*(FAILED|ok)$|^test result|panicked|is stale"
echo "BASELINE rc=$pipestatus[1] $(date '+%T')"
python3 "$W/lb2-p17-pack-schema-identity.py" --write --root "$SC" | tail -2
