#!/bin/zsh
# 🩺️ The check CLI for the given assets in one grant of 1000 MB, asked for again when the queue gives up: cli.sh <log name> <asset id>…
source "${0:A:h}/env.sh"; cd "$R"; mkdir -p "$GEN/run"; out="$GEN/run/$1"; shift; : > "$out"
for a in "$@"; do
  until SEMIO_LANGUAGE=en repo capacity run --memory=1000 -- timeout 2400 zsh -c 'cd "$1" && exec "$2" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$3"' x "$S" "$B" "$a" > "$GEN/run/cli-$a.log" 2>&1 && grep -qE '^(✓|✗)' "$GEN/run/cli-$a.log"; do sleep 5; done
  echo "$a $(grep -m1 -E '^(✓|✗)' "$GEN/run/cli-$a.log" | cut -c1-140)" >> "$out"
done
echo "cli done" >> "$out"
