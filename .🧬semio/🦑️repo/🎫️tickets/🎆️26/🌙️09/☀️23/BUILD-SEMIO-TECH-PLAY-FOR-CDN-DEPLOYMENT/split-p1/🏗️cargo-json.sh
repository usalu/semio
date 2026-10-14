#!/bin/bash
# 🏗️ Lane-P1 cargo invocation emitting rustc JSON diagnostics (for the fix tools) through the fleet slot and private dirs;
# JSON lines go to `⚡️cache/p1/logs/<tag>.json`, the human summary of errors to stdout.
# usage: 🏗️cargo-json.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P1="$ROOT/.🧬semio/🦑️repo/⚡️cache/p1"; mkdir -p "$P1/logs"
export CARGO_TARGET_DIR="$P1/target" CARGO_BUILD_BUILD_DIR="$P1/build" CARGO_TERM_COLOR=never CARGO_INCREMENTAL=0
cd "$ROOT" || exit 2
bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p1-$TAG" -- cargo "$@" --message-format=json > "$P1/logs/$TAG.json" 2> "$P1/logs/$TAG.stderr"
rc=$?
python3 -c "
import json,sys,collections
c=collections.Counter(); w=0
for l in open(sys.argv[1],encoding='utf-8'):
    try: d=json.loads(l)
    except Exception: continue
    if d.get('reason')=='compiler-message':
        m=d['message']
        if m['level']=='error': c[(m.get('code') or {}).get('code')]+=1
        elif m['level']=='warning': w+=1
print('rc=$rc errors',sum(c.values()),dict(c),'warnings',w)
" "$P1/logs/$TAG.json"
exit $rc
