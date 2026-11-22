#!/bin/sh
# L15: run a repo 📜️script.ts natively under a fresh envelope; usage: l15-run.sh <script-dir> <args...>
T="$(cd "$(dirname "$0")" && pwd)"; R=/home/deck/semio
BUN=$(ls /home/deck/.cache/semio/runtimes/bun/*/bun | head -1)
export SEMIO_BUILD_MODE=prototype
cd "$R/$1"; shift
eval "$("$BUN" "$T/l15-envelope.ts" | "$BUN" -e 'const j=JSON.parse(require("fs").readFileSync(0,"utf8"));for(const k in j)console.log("export "+k+"="+JSON.stringify(j[k]).replace(/\$/g,"\\$")); ')"
exec "$BUN" ./📜️script.ts "$@"
