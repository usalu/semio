#!/bin/zsh
# 🧫️ The body search of one plant on a copy, inside the grant of its caller: `trial.sh <id> [flags of ../MV-3/body.ts]` runs `body.ts` on `$GEN/backup/<id>.json` (or on `$GEN/base/<id>.json` when the loft step made one: the backup with the lofts of `loft/<id>.json`), rounds the shoulders and ends of the bodies when RAW is unset (`caps.ts`; its depth radius is DEEP, default 0.5, times the tolerance of the asset, further flags in CAPS, else in `flags/<id>.caps`; with RAW=1 the bodies of `body.ts` are taken as found) into `body/<id>.json`, applies them to the copy as `$GEN/try/<id>.json`, searches a place on the drawing of its bone for every anchor the bodies left beside it (`../MV-3/anchors.ts`, saved as `anchors/<id>.json` when one moved), then prints what the bodies show beside the outlines in the default pose (`rest.ts`) and in the wind poses (`../MV-3/visible.ts --prefix mvb-`, and the plates of the backup for comparison).
source "${0:A:h}/env.sh"
a="$1"; shift; mkdir -p "$W/body" "$GEN/try" "$GEN/log"; src="$GEN/backup/$a.json"; [[ -f "$GEN/base/$a.json" ]] && src="$GEN/base/$a.json"
cd "$T"
timeout 3000 "$B" "$W3/body.ts" "$src" --save "$GEN/try/$a-raw-ops.json" "$@" > "$GEN/log/$a-body.log" 2>&1; echo "body exit $? $(grep -c '^left' "$GEN/log/$a-body.log") left; $(tail -2 "$GEN/log/$a-body.log" | head -1 | cut -c1-200)"
if [[ -n "$RAW" ]]; then cp "$GEN/try/$a-raw-ops.json" "$W/body/$a.json"; else
  deep="$(python3 -c "import json,sys; d=json.load(open(sys.argv[1]))['asset']['dimensions']; print(round(max(0.015, 0.02*max(d['heightM'], d['widthM'], d['depthM']))*float(sys.argv[2]), 4))" "$src" "${DEEP:-0.5}")"
  timeout 300 "$B" "$W/caps.ts" "$GEN/try/$a-raw-ops.json" "$W/body/$a.json" --deep "$deep" ${=CAPS:-$(cat "$W/flags/$a.caps" 2>/dev/null)} | tail -1 | sed "s|$W/||"
fi
timeout 3000 "$B" "$W3/body.ts" "$src" --ops "$W/body/$a.json" --out "$GEN/try/$a.json" 2>&1 | tail -2 | cut -c1-200
timeout 3000 "$B" "$W3/anchors.ts" "$GEN/try/$a.json" --save "$GEN/try/$a-anchors.json" --out "$GEN/try/$a.json" > "$GEN/log/$a-anchors.log" 2>&1; echo "anchors exit $? $(grep -c '"op"' "$GEN/try/$a-anchors.json") moved; $(tail -1 "$GEN/log/$a-anchors.log" | cut -c1-200)"
rm -f "$W/anchors/$a.json"; grep -q '"op"' "$GEN/try/$a-anchors.json" && cp "$GEN/try/$a-anchors.json" "$W/anchors/$a.json"
timeout 1200 "$B" "$W/rest.ts" "$GEN/try/$a.json" > "$GEN/log/$a-try-rest.log" 2>&1; echo "rest exit $? $(tail -1 "$GEN/log/$a-try-rest.log")"
timeout 3000 "$B" "$W3/visible.ts" "$GEN/try/$a.json" --prefix mvb- > "$GEN/log/$a-try-visible.log" 2>&1; echo "visible exit $? $(tail -1 "$GEN/log/$a-try-visible.log" | cut -c1-260)"
[[ -f "$GEN/log/$a-before-visible.log" ]] || { timeout 3000 "$B" "$W3/visible.ts" "$src" --prefix mvc- > "$GEN/log/$a-before-visible.log" 2>&1; echo "plates before: $(tail -1 "$GEN/log/$a-before-visible.log" | cut -c1-260)"; }
