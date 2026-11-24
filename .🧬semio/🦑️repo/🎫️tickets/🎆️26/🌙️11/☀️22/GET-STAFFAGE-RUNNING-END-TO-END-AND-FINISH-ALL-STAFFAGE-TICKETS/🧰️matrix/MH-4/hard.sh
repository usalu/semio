#!/bin/zsh
# 📋️ The hard lines of a gate log with their bones, and the worst twins: hard.sh <log>…
for f in "$@"; do
  awk '/^[^ ]/ { show = ($0 ~ / HARD /) ; twin = ($0 ~ / twin /); n = 0 } { n++ } show || (twin && n <= 9) || /^hard findings/ || / subjects /' "$f" | cut -c1-${COLS:-250}
done
