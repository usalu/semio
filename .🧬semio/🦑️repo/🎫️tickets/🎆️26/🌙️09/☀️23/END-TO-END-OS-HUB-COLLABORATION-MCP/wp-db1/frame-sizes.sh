#!/bin/zsh
# DB1: print the stack-frame size of every function whose symbol matches <regex> in the db test binary (debug poll-fn frames).
# usage: frame-sizes.sh <binary> <regex>
bin="$1"; pat="$2"
nm "$bin" | /usr/bin/grep -E "$pat" | awk '{print $1, $3}' | while read addr sym; do
  size=$(xcrun objdump --disassemble-symbols="$sym" --no-show-raw-insn "$bin" 2>/dev/null | sed -n 4,12p | /usr/bin/grep -oE "sub	(x9|sp), sp, #0x[0-9a-f]+(, lsl #12)?" | head -1)
  echo "$size :: $(echo $sym | swift demangle 2>/dev/null | cut -c1-40) ${sym:0:160}"
done
