#!/usr/bin/env bash
# usage: stopframe.sh <script relative to ENERGY> <SceneClass> <hold key> <out name> [language] [resolution]
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
WIN="$(cd "$ROOT" && pwd -W)"
ENERGY="$WIN/🎓️teaching/🏛️architecture/⚡️energy"
OUT="$WIN/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/🗑️generated/consistency-out/shots"
mkdir -p "$OUT"
export SEMIO_TEACHING_PROFILE=preview
export TRACE_TARGET="$ENERGY/$1::$2" STOP_KEY="$3" OUT_PNG="$OUT/$4-${5:-de}.png"
cd "$ROOT" || exit 2
bash "$HERE/run-py.sh" "$HERE/frame_probe.py" Trace "$WIN/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/consistency/trace_scene.py" "${5:-de}" "${6:-1280,720}" 2>&1 | tail -3
ls -la "$OUT_PNG"
