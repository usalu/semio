#!/bin/zsh
# 📌️ S19 overlay pin: puts files a peer is mid-edit on back to HEAD INSIDE THE OVERLAY ONLY, so overlay proofs compile
# against the last committed state of those files. usage: zsh s19-overlay-pin.sh <relpath…>
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"
cd /Users/ueli/Documents/semio || exit 2
for rel in "$@"; do
  git show "HEAD:$rel" > "$O/$rel" && echo "pinned $rel"
done
