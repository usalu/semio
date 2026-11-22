#!/bin/sh
# Author 14: renders every sheet of the look step for one asset, one process at a time.
# usage: rework-14-look.sh <category/slug> [quick]
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1
for s in 50 100 200 500; do for l in body thermal structure; do "$BUN" r3-sheet.ts "$A" --scale $s --list $l 2>&1 | grep -E "NOT DRAWN|rror" ; done; done
"$BUN" r3-sheet.ts "$A" --scale 50 --list thermal --cut 2>&1 | grep -E "NOT DRAWN|rror"
"$BUN" -e '
import { load } from "./r3-lib.ts";
const { asset } = load(process.argv[1], "");
for (const p of asset.poses) if (p.id !== asset.presentation.defaultPose) console.log("--pose " + p.id);
for (const c of asset.clips) { console.log("--clip " + c.id + " --t " + (c.durationS * 0.25).toFixed(2)); console.log("--clip " + c.id + " --t " + (c.durationS * 0.6).toFixed(2)); }
' "$A" | while read -r args; do "$BUN" r3-sheet.ts "$A" --scale 50 $args 2>&1 | grep -E "NOT DRAWN|rror|png" | sed 's/.*r3-sheet.//'; done
