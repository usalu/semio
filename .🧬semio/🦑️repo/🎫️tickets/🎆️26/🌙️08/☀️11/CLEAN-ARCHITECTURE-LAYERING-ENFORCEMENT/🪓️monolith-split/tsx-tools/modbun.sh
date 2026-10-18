#!/bin/zsh
# usage: modbun.sh <label> <module-prefix-regex> [part N/M]: runs bun test over bunfiles.txt rows whose path matches the regex
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tsx-tools/env.sh
label="$1"; pat="$2"; part="${3:-1/1}"
files=("${(@f)$(python3 - "$T/bunfiles.txt" "$pat" "$part" <<'PY'
import re,sys
rows=[l.rstrip("\n") for l in open(sys.argv[1],encoding="utf8") if l.strip()]
rows=[r for r in rows if re.search(sys.argv[2],r)]
i,n=map(int,sys.argv[3].split("/"))
print("\n".join("./"+r for k,r in enumerate(rows) if k%n==i-1))
PY
)}")
echo "files: ${#files}"
"$T/buntest.sh" "$label" "$R" "${files[@]}"
