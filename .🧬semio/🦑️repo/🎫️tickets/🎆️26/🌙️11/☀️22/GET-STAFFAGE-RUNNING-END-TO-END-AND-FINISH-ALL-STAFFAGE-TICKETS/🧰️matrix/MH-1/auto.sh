#!/bin/zsh
# 🤖️ The whole chain of one asset after the other: pipe.sh (ops, tuner, trial), and when ready.sh agrees, land.sh (write and proof with the matrix); an asset that is not ready is left untouched for the fixer: auto.sh <id>…   (LANDONLY=1 skips the pipe and lands the trial that is already measured)
cd "${0:A:h}"
for a in "$@"; do
  [[ -z "$LANDONLY" ]] && ./pipe.sh "$a"
  if ./ready.sh "$a"; then ./land.sh "$a"; fi
done
