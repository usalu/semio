#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd /Users/ueli/Documents/semio
export BIDI_PORT=${BIDI_PORT:-9433}
[ -n "$LIGHT" ] && export FF_PREFS="$(cat $T/light-prefs.js)"
[ -n "$MEMTRACK" ] && export PRELOAD="$(cat $T/wasm-mem-preload.js)" || export PRELOAD='() => { window.__ctx = []; for (const t of ["webglcontextlost","webglcontextrestored","webglcontextcreationerror"]) document.addEventListener(t, e => window.__ctx.push([t, Math.round(performance.now())]), true); }'
[ -n "$LIVE_ALL" ] && export LIVE_ALL
export STEPS="$(cat $T/${SCENARIO:-process-forest-render.json})"
LIVE_LOG=$T/🗑️generated/${TAG:-forest}-live.txt $B $T/verify-process-firefox-bidi.ts > $T/🗑️generated/${TAG:-forest}-out.txt 2>&1
echo "fin $?" >> $T/🗑️generated/${TAG:-forest}-out.txt
cp $T/🗑️generated/e2e-process.png $T/🗑️generated/${TAG:-forest}.png
cp $T/🗑️generated/e2e-report.json $T/🗑️generated/${TAG:-forest}-report.json
pkill -f "remote-debugging-port=$BIDI_PORT"
