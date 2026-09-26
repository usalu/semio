#!/bin/zsh
# 🔎️ G11: niced `cargo check` of the given crates (lib + tests). usage: zsh g11-check.sh <crate>… [-- extra cargo args]
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
P=(); X=(); SEEN=0
for a in "$@"; do if [ "$a" = "--" ]; then SEEN=1; elif [ $SEEN -eq 0 ]; then P+=(-p "$a"); else X+=("$a"); fi; done
date; S=$(date +%s)
nice -n 10 cargo check "${P[@]}" --lib --tests "${X[@]}" 2>&1 | /usr/bin/grep -E "^(error|warning: unused|warning: \`)|^\s+--> |Finished|could not compile" | head -80
echo "CHECK_RC=${pipestatus[1]} secs=$(( $(date +%s)-S ))"; date
