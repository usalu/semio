#!/bin/zsh
# 🔁️ Regenerates the P9 patch set from the stage (base → stage) over its fixed file list, then dry-runs it on the live tree.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-p9 || exit 2
P="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"; S="✏️s/🔌️plugins"; E="🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
python3 p9-hunks.py p9-agent-lane patches/p9-agent-lane.doc.txt \
 "🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs" "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs" "$P/🦀️.rs" "$P/🧵️retained-command/🦀️.rs" "$P/🧵️retained-command/🧪️tests/🔬️unit/🦀️.rs" "$P/🧪️tests/🔬️app-app-builder/🦀️.rs" "$P/🧪️tests/🤖️agent-lane-preview/🦀️.rs" "$P/🧪️tests/🤖️agent-lane-preview/🟦️.ts" "$P/🧫️fixtures/🤖️agent-lane-preview-verdicts.json" "$P/📦️packages/🦀️rust/📜️script.ts" \
 "$S/🔱️trinity/🗿️artifacts/🔌️jack/$E/🦀️.rs" "$S/🔱️trinity/🗿️artifacts/🔌️jack/$E/🎮️commands/🩹️patch-nodes/🦀️.rs" "$S/🔱️trinity/🗿️artifacts/🔌️jack/$E/🧪️tests/🔬️unit/🦀️.rs" \
 "$S/🔱️trinity/🗿️artifacts/♻️rewriting/$E/🦀️.rs" "$S/🔱️trinity/🗿️artifacts/♻️rewriting/$E/🎮️commands/🩹️patch-nodes/🦀️.rs" "$S/🔱️trinity/🗿️artifacts/♻️rewriting/$E/🧪️tests/🔬️unit/🦀️.rs" \
 "$S/🧩️puzzle/🗿️artifacts/◻️2d/$E/🦀️.rs" "$S/🧩️puzzle/🗿️artifacts/◻️2d/$E/🧪️tests/🤖️agent-lane/🦀️.rs" "$S/🧩️puzzle/🗿️artifacts/🧊️3d/$E/🦀️.rs" "$S/🧩️puzzle/🗿️artifacts/🧊️3d/$E/🧪️tests/🤖️agent-lane/🦀️.rs" "$S/🧩️puzzle/🗿️artifacts/🖐️5d/$E/🦀️.rs" "$S/🧩️puzzle/🗿️artifacts/🖐️5d/$E/🧪️tests/🤖️agent-lane/🦀️.rs" "$S/✒️writer/🗿️artifacts/✒️writer/$E/🦀️.rs" "$S/✒️writer/🗿️artifacts/✒️writer/$E/🧪️tests/🤖️agent-lane/🦀️.rs" || exit 1
cd patches && python3 p9-agent-lane.py --dry-run | tail -2
