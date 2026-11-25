#!/bin/zsh
# 📐️ The derived-view measure of the suite on the catalog documents of the three leftovers after their repair, into $GEN/left/derived-after.txt: left-after.sh   (run it through q.sh with 800 MB or inside a larger grant)
source "${0:h}/env.sh"
cd "$T" && timeout 600 "$B" "$W/derived.ts" steel-column glulam-beam storage-crate-stack > "$GEN/left/derived-after.txt" 2>&1
grep -E "^==|FLAGGED|plan against elevation" "$GEN/left/derived-after.txt" | cut -c1-160
