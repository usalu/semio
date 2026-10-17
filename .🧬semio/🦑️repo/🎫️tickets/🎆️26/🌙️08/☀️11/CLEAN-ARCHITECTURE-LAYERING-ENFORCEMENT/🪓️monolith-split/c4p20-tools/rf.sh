#!/bin/zsh
cd /private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p20/${1:-stage}
bad=0; n=0
find . -name "🦀️.rs" | while read f; do
  rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config skip_children=true "$f" > /dev/null 2> /private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p20/rf.err || { echo "FAIL $f"; head -3 /private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p20/rf.err; }
done
echo done
