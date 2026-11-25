#!/bin/bash
# Starts one process that looks like a pinned rustc (argv0 is the pinned toolchain path, no repo runtime among its ancestors) of an unlisted library unit and holds 1700 MB, above the 1536 MB library ceiling; the live warden must cancel it within a few seconds; then prints the typed failure the owner of the build gets.
set -eu
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
RC="$(ls -d "$ROOT"/.cache/b/t/m/r/*/toolchains/*/bin/rustc | head -1)"
BUN="$(ls -d "$ROOT"/.cache/b/t/m/bun/*/bun | head -1)"
OUT="$(dirname "$0")/🗑️generated/proof"
mkdir -p "$OUT"
SINCE="$(python3 -c 'import time;print(int(time.time()*1000))')"
( exec -a "$RC" "$BUN" -e 'const b=Buffer.alloc(1700*1048576,1);setInterval(()=>{b[0]=1;},500);' --crate-name warden_ceiling_proof --crate-type lib --target wasm32-wasip2 ) &
CHILD=$!
for i in $(seq 1 30); do kill -0 "$CHILD" 2>/dev/null || break; sleep 1; done
if kill -0 "$CHILD" 2>/dev/null; then echo "NOT CANCELLED after 30 s"; kill -9 "$CHILD"; else echo "cancelled after about $i s"; fi
wait "$CHILD" 2>/dev/null || echo "exit status $?"
grep -h "warden_ceiling_proof" "$HOME/Library/Caches/semio/host-capacity/warden/warden.log" | tail -2 | cut -c1-420
cd "$ROOT/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden"
"$BUN" -e "import {requireNoHostWardenCancelBelowV1} from './🔁️loop/🟦️.ts';try{requireNoHostWardenCancelBelowV1($$,$SINCE);console.log('no cancel found')}catch(e){console.log(e.name,e.code);console.log(e.message)}"
