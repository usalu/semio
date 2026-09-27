#!/bin/zsh
# 📏️ WG11 item 3: measures every function's own stack frame in an aarch64 Mach-O test binary (no cargo): the prologue's
# `sub sp, sp, #imm[, lsl #12]`, Rust's inline stack-probe target (`sub x9, sp, #imm[, lsl #12]`) and the `x15` operand of
# `___chkstk_darwin` (frame = x15 × 16). Prints the N largest frames.
# usage: zsh wg11-stack-frames.sh <binary> [N] [symbol regex]
BIN="$1"; N="${2:-80}"; FILTER="${3:-.}"
[ -f "$BIN" ] || { echo "usage: wg11-stack-frames.sh <binary> [N] [regex]"; exit 2; }
nice -n 5 /usr/bin/objdump -d --no-show-raw-insn -C "$BIN" 2>/dev/null | awk -v filter="$FILTER" '
function hex(s,   v, i, c) { sub(/^#?0x/, "", s); v = 0; for (i = 1; i <= length(s); i++) { c = index("0123456789abcdef", tolower(substr(s, i, 1))) - 1; if (c < 0) break; v = v * 16 + c } return v }
function num(s) { gsub(/[#,]/, "", s); return (s ~ /^0x/) ? hex(s) : s + 0 }
function flush() { if (name != "" && size > 0 && name ~ filter) printf "%d\t%s\n", size, name; }
/^[0-9a-f]+ <.*>:$/ { flush(); name = substr($0, index($0, "<") + 1); sub(/>:$/, "", name); size = 0; x15 = 0; seen = 0; probe = 0; probed = 0; next }
{
  seen++
  if (seen > 40) next
  line = $0
  if (line ~ /mov[ \t]+x15, #/) { split(line, parts, "#"); x15 = num("#" parts[2]) }
  else if (line ~ /movk[ \t]+x15, #/) { split(line, parts, "#"); v = num("#" parts[2]); if (line ~ /lsl #16/) x15 += v * 65536; else if (line ~ /lsl #32/) x15 += v * 4294967296 }
  else if (line ~ /___chkstk_darwin/) { if (x15 * 16 > size) size = x15 * 16 }
  else if (line ~ /sub[ \t]+sp, sp, #/ && probe == 0) { split(line, parts, "#"); v = num("#" parts[2]); if (line ~ /lsl #12/) v = v * 4096; size += v }
  else if (line ~ /sub[ \t]+sp, sp, #/ && probe == 1 && line !~ /lsl #12/) { split(line, parts, "#"); size = probed + num("#" parts[2]); probe = 2 }
  else if (line ~ /sub[ \t]+x9, (sp|x9), #/) { split(line, parts, "#"); v = num("#" parts[2]); if (line ~ /lsl #12/) v = v * 4096; probed += v; probe = 1; if (probed > size) size = probed }
}
END { flush() }
' | sort -rn -k1,1 | head -n "$N"
