#!/bin/zsh
# 🔬️ The cases of 🧊️model-catalog for the given assets in one grant of 2500 MB with the full failure text, asked for again when the queue gives up: case.sh <asset id>[,<asset id>…]
me="${0:A}"; source "${0:A:h}/env.sh"; ids="$1"; mkdir -p "$GEN/run"; out="$GEN/run/case-${ids%%,*}.log"
if [[ "$2" != "--inside" ]]; then cd "$R"; : > "$out"; until SEMIO_LANGUAGE=en repo capacity run --memory=2500 -- timeout 3000 "$me" "$ids" --inside >> "$out" 2>&1 && grep -qE '^ +Tests ' "$out"; do sleep 5; done; echo "case done" >> "$out"; exit; fi
cd "$S/📦️packages/🟦️typescript" && SEMIO_STAFFAGE_MODEL_ASSETS="$ids" /usr/bin/time -l timeout 2700 "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🧊️model-catalog/🟦️.ts"
