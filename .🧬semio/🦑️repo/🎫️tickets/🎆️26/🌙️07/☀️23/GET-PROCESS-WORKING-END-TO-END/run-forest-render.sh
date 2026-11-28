#!/bin/zsh
# 🦊️ Runs one browser scenario against the served Process 3D app on its own BiDi port, Firefox profile and output tag; writes $TAG-out.txt ("fin <status>"), $TAG-report.json, $TAG.png and $TAG.status.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; G=$T/🗑️generated
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd /Users/ueli/Documents/semio; mkdir -p $G
export TAG=${TAG:-forest} BIDI_PORT=${BIDI_PORT:-9433} SCENARIO=${SCENARIO:-process-forest-render.json}
export HUB_RECEIPT=${HUB_RECEIPT:-/Users/ueli/Documents/semio/.cache/b/p/m/browser/prototype/modules/🏭️process/🧾️component.json}
rm -f $G/$TAG-out.txt $G/$TAG-report.json $G/$TAG.png $G/$TAG.status $G/$TAG-live.txt
if lsof -nP -iTCP:$BIDI_PORT -sTCP:LISTEN >/dev/null 2>&1; then echo "fin 3 port $BIDI_PORT is owned by $(lsof -nP -t -iTCP:$BIDI_PORT -sTCP:LISTEN)" > $G/$TAG-out.txt; echo 3 > $G/$TAG.status; exit 3; fi
[ -n "$LIGHT" ] && export FF_PREFS="$(cat $T/light-prefs.js)"
if [ -n "$PRELOAD_FILE" ]; then export PRELOAD="$(cat $T/$PRELOAD_FILE)"; else export PRELOAD='() => { window.__ctx = []; for (const t of ["webglcontextlost","webglcontextrestored","webglcontextcreationerror"]) document.addEventListener(t, e => window.__ctx.push([t, Math.round(performance.now())]), true); }'; fi
export STEPS="$(cat $T/$SCENARIO)"
LIVE_LOG=$G/$TAG-live.txt $B $T/verify-process-firefox-bidi.ts > $G/$TAG-out.txt 2>&1
s=$?; echo "fin $s" >> $G/$TAG-out.txt; echo $s > $G/$TAG.status; exit $s
