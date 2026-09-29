#!/bin/zsh
# 🔁️ Regenerates P9's T7 set `p9-jack-headless-query` from its own stage (base → stage), then dry-runs it on the live tree.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-p9 || exit 2
export P9_STAGE=".🧬semio/🌐hub/s14-p9-stage-t7"
J="✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
python3 p9-hunks.py p9-jack-headless-query patches/p9-jack-headless-query.doc.txt "$J/🦀️.rs" "$J/🎮️commands/▶️run-query/🧵️job/🦀️.rs" "$J/🧪️tests/🔬️unit/🦀️.rs" "$J/🎮️commands/▶️run-query/🧵️job/🧪️tests/🔬️unit/🦀️.rs" || exit 1
cd patches && python3 p9-jack-headless-query.py --dry-run | tail -6
