#!/bin/sh
cd "$(git rev-parse --show-toplevel)" || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
python3 "$T/real-closure-jobs.py" "$@" > "$T/🗑️generated/closure-jobs.json" || exit 1
node --no-warnings "$T/real-closure-probe.mjs" "$T/🗑️generated/closure-jobs.json" > "$T/🗑️generated/closure-rows.json" || exit 1
python3 - "$T/🗑️generated/closure-rows.json" <<'P'
import json,sys
for r in json.load(open(sys.argv[1])):
    f=r.get("files");b=r.get("bytes")
    flag="" if f is None else ("OVER" if f>32 or b>262144 else "ok")
    print(f"{flag:<5}{r['crate']:<58}{r['unit']:<44}files={f} bytes={b} lines={r.get('lines')} miss={len(r.get('missing',[]))} {r.get('error','')}")
P
