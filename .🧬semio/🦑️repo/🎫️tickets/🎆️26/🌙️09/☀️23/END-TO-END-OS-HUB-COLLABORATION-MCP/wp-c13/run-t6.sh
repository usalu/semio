#!/bin/zsh
# 🚀️ C13 (session 15): live re-checks on hub 7800 (catalog t6) through ONE serve 6670 and one browser at a time, sequentially:
# viewer en + de (block, note, writer, draw), cross-undo en + de (note, writer, draw — B's crafted foreign Revert must be refused by
# the HUB, never relayed), presence en + de (note, draw, 3d puzzle, 2d puzzle: both mount — 3d puzzle opens —, A's pointer in B,
# A's selection painted in B, draw's layer count moves on both views). The serve is stopped at the end.
# usage: zsh run-t6.sh <tagPrefix> <adminCapabilityFile> [hubUrl=http://127.0.0.1:7800] [steps=viewer,undo,presence]
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-c13
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-logs"
TAG="$1"; CAP="$2"; HUB="${3:-http://127.0.0.1:7800}"; STEPS="${4:-viewer,undo,presence}"
SERVE=http://127.0.0.1:6670/
cd "$W" || exit 1
echo "RUN-T6 START $(date '+%F %T') tag=$TAG hub=$HUB steps=$STEPS"
[ "$(curl -s -o /dev/null -m 5 -w "%{http_code}" "$HUB/")" != "000" ] || { echo "HUB DOWN $HUB"; exit 2; }
[ -f "$CAP" ] || { echo "NO ADMIN CAPABILITY $CAP"; exit 2; }
hold=""
if ! curl -sf -o /dev/null -m 5 "$SERVE"; then
  hold=$(python3 ../wp-w2/w2-detach.py "$L/serve-hold-6670-$TAG.txt" zsh "$W/serve-hold.sh" 6670 "$HUB")
  echo "serve-hold pid $hold"
  for i in {1..40}; do curl -sf -o /dev/null -m 5 "$SERVE" && break; sleep 15; done
  curl -sf -o /dev/null -m 5 "$SERVE" || { echo "SERVE NEVER ANSWERED"; kill "$hold" 2>/dev/null; exit 2; }
fi
echo "serve up $(date '+%T')"
KINDS_VIEWER="2d.block,s.note.note,text.document,2d.drawing"
KINDS_UNDO="s.note.note,text.document,2d.drawing"
KINDS_PRESENCE="s.note.note,2d.drawing,3d.puzzle,2d.puzzle"
for loc in en de; do
  [[ ",$STEPS," == *,viewer,* ]] && zsh run-two-human.sh "$TAG-viewer-$loc" viewer "$loc" "$HUB" "$SERVE" "$SERVE" "$CAP" "$KINDS_VIEWER" 300000
  [[ ",$STEPS," == *,undo,* ]] && zsh run-two-human.sh "$TAG-undo-$loc" cross-undo "$loc" "$HUB" "$SERVE" "$SERVE" "$CAP" "$KINDS_UNDO" 300000
  if [[ ",$STEPS," == *,presence,* ]]; then
    (source ../wp-c12/env.sh; nice -n 10 bun probe-c13-presence.mjs "$TAG-presence-$loc" "$SERVE" "$KINDS_PRESENCE" "$([ $loc = de ] && echo de-DE || echo en-US)"; echo "PRESENCE $loc rc=$?")
  fi
done
[ -n "$hold" ] && kill "$hold" 2>/dev/null && echo "serve-hold $hold stopped"
echo "RUN-T6 DONE $(date '+%F %T')"
