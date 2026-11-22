#!/bin/sh
# Runs the export command line once per format family into 🗑️generated/cli (real CLI process, real ffmpeg of the pinned wheel, rsvg-convert stand-in from ./tools) and lists what was written.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
CLI="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎬️cli/🟦️.ts"
OUT="$HERE/🗑️generated/cli"
export PATH="$HERE/tools:$PATH"
export SEMIO_TOOL_FFMPEG="$(ls "$HOME"/Library/Caches/semio/staffage-oracles/media/lib/python*/site-packages/imageio_ffmpeg/binaries/ffmpeg-* | head -1)"
rm -rf "$OUT"; mkdir -p "$OUT"
kettle() { "$BUN" "$CLI" --asset electric-kettle --pose boiling --view thermal --scale 20 --lang en --out "$OUT" --air-temp 15 --radiant-temp 15 --humidity 65 --wind 0.2 "$@" 2>"$OUT/stderr.txt" || { echo "FAILED: $*"; tail -3 "$OUT/stderr.txt"; }; }
adult() { "$BUN" "$CLI" --asset standing-adult --view thermal --scale 50 --lang de --out "$OUT" "$@" 2>"$OUT/stderr.txt" || { echo "FAILED: $*"; tail -3 "$OUT/stderr.txt"; }; }
kettle --viewpoint elevation --format svg
kettle --viewpoint axonometric --format svg
kettle --viewpoint elevation --format pdf
kettle --viewpoint elevation --format dxf
kettle --viewpoint elevation --format dwg
kettle --projection third --views elevation,front,plan,axonometric --format sheet-svg
kettle --projection first --format sheet-pdf
kettle --projection third --format sheet-dxf
kettle --projection third --format sheet-dwg
kettle --viewpoint elevation --format png-sequence --duration 1 --fps 4 --px-per-mm 4
kettle --viewpoint elevation --format mp4 --duration 2 --fps 10 --px-per-mm 4
kettle --viewpoint elevation --format webm --duration 1 --fps 4 --px-per-mm 4
adult --clip idle-breathing --viewpoint elevation --format mp4 --fps 5 --px-per-mm 4 --environment winter-clear-night
adult --pose relaxed --viewpoint elevation --format svg --air-temp 2 --humidity 85 --wind 1.5 --wind-direction 180 --wind-profile logarithmic --turbulence 20
grep -c "" "$OUT/stderr.txt" >/dev/null 2>&1
ls -la "$OUT" | awk '{print $5, $9}'
