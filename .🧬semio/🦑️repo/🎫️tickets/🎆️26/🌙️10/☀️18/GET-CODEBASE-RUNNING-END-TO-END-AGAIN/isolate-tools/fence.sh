#!/bin/sh
# 🚧 [isolate] toggles the store fence preload in the bunfig.toml of a MIRROR (never live) for base runs: usage fence.sh on|off <mirror-root>
m="$2"; case "$m" in /private/tmp/m/*) ;; *) echo "mirror only" >&2; exit 2;; esac
f="$m/bunfig.toml"; e='"./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🚧️fence/📜️script.ts"'
python3 -I - "$1" "$f" "$e" <<'PY'
import sys
mode,f,e=sys.argv[1:]
s=open(f,encoding="utf-8").read();has=(", "+e) in s
if mode=="off" and has: s=s.replace(", "+e,"",1)
if mode=="on" and not has: s=s.replace('📜️script.ts"]\n\n[install]','📜️script.ts", '+e+']\n\n[install]',1)
open(f,"w",encoding="utf-8").write(s);print("fence",mode,("fence/📜️script.ts" in s))
PY
