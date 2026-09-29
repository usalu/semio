#!/bin/zsh
# 🔁️ Regenerates P9's T6 set `p9-fail-closed` from its own stage (base → stage), then dry-runs it on the live tree.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-p9 || exit 2
export P9_STAGE=".🧬semio/🌐hub/s14-p9-stage-t6"
P="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"; S="✏️s/🔌️plugins"; E="🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
python3 p9-hunks.py p9-fail-closed patches/p9-fail-closed.doc.txt \
 "$P/🦀️.rs" "$P/🧫️fixtures/⚖️declared-verb-verdicts.json" "$P/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts" "$P/🧪️tests/⚖️declared-verb-verdicts/🦀️.rs" \
 "$P/🧪️tests/🤖️agent-lane-preview/🦀️.rs" "$P/🧪️tests/🤖️agent-lane-preview/🟦️.ts" "$P/🧫️fixtures/🤖️agent-lane-carriage.json" "$P/📦️packages/🦀️rust/📜️script.ts" \
 "$S/🧩️puzzle/🗿️artifacts/◻️2d/$E/🧪️tests/🤖️agent-lane/🦀️.rs" "$S/🧩️puzzle/🗿️artifacts/🧊️3d/$E/🧪️tests/🤖️agent-lane/🦀️.rs" "$S/🧩️puzzle/🗿️artifacts/🖐️5d/$E/🧪️tests/🤖️agent-lane/🦀️.rs" \
 "$S/🌊️flow/🗿️artifacts/🌊️flow/$E/🧪️tests/⚖️declared-verbs/🦀️.rs" \
 "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs" "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs" || exit 1
cd patches && python3 p9-fail-closed.py --dry-run | tail -12
