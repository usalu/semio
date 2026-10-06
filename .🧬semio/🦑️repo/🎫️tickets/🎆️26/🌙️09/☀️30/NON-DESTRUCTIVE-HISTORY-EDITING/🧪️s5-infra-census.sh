#!/bin/zsh
# 🧮️ Composition census #2 (design §21.4 / resume §0.3): one foreground batch of `cargo check --manifest-path 🌎️hub/Cargo.toml
# -p semio-hub-<plugin>… --target wasm32-wasip2 --lib --keep-going` for at most four compositions, under build gate v5 (fewer than
# 4 cargo, `CARGO_BUILD_JOBS=3`; never while `foundation.status` reads BUILDING or a fresh RED).
# Usage: `zsh 🧪️s5-infra-census.sh <batch> <plugin> [<plugin> …]`. Cargo's JSON messages are summarized by
# `🧪️s5-infra-census-summary.ts` into `🗑️generated/s5-infra/census2-<batch>.txt`: `GREEN <hub>` (its library was produced),
# `RED <crate> errors=<n> first=<file:line: error>`, `BLOCKED <hub> by=[red crates it depends on]`; one batch line goes to
# `🗑️generated/s5-infra/census2.events`. Exit 5 = the gate stayed busy for 4 minutes (re-issue), exit 6 = cargo ended without a
# verdict (killed).
root="/Users/ueli/Documents/semio"
ticket="${0:A:h}"
out="$ticket/🗑️generated/s5-infra"
state="$ticket/🗑️generated/coord/foundation.status"
batch="$1"
shift
[ -n "$batch" ] && [ $# -ge 1 ] && [ $# -le 4 ] || { echo "usage: <batch> <plugin> [<plugin> …] (at most four)"; exit 2 }
cd "$root" || exit 2
mkdir -p "$out"
export CARGO_BUILD_JOBS=3
foundation="$(cat "$state" 2>/dev/null)"
[ "${foundation%% *}" = GREEN ] || { echo "foundation is not GREEN: $foundation"; exit 5 }
waited=0
until [ "$(pgrep -x cargo | wc -l | tr -d ' ')" -lt 4 ]; do
  [ $waited -ge 240 ] && { echo "GATE BUSY $(date '+%T') cargo=$(pgrep -x cargo | wc -l | tr -d ' ')"; exit 5 }
  sleep 20
  waited=$((waited + 20))
done
file="$out/census2-$batch.jsonl"
packages=()
for plugin in "$@"; do packages+=(-p "semio-hub-$plugin"); done
echo "[census2 $batch] start $(date '+%T'): $*"
cargo check --manifest-path "🌎️hub/Cargo.toml" $packages --target wasm32-wasip2 --lib --keep-going --message-format=json-diagnostic-short > "$file" 2> "$out/census2-$batch.err"
code=$?
/usr/bin/grep -q '"reason":"build-finished"' "$file" || { echo "NO VERDICT $(date '+%T') cargo exit=$code without a build-finished message: $(tail -1 "$out/census2-$batch.err" | cut -c1-200)"; exit 6 }
bun "$ticket/🧪️s5-infra-census-summary.ts" "$file" "$@" > "$out/census2-$batch.txt"
echo "$(date '+%F %T') census2 $batch exit=$code plugins=[$*] green=$(/usr/bin/grep -c '^GREEN' "$out/census2-$batch.txt") red-crates=$(/usr/bin/grep -c '^RED ' "$out/census2-$batch.txt") blocked=$(/usr/bin/grep -c '^BLOCKED' "$out/census2-$batch.txt")" | tee -a "$out/census2.events"
cat "$out/census2-$batch.txt"
rm -f "$file" "$out/census2-$batch.err"
