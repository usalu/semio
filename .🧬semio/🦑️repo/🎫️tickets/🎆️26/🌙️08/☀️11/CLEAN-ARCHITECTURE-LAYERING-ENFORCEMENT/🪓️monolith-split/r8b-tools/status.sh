#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 99
for a in 🗒️note/🗿️artifacts/🗒️note 🎥️shooting/🗿️artifacts/🎥️shooting 📋️forms/🗿️artifacts/📋️forms 🎞️animate/🗿️artifacts/🎬️presentation; do
  p="✏️s/🔌️plugins/$a"
  echo "== $p"
  git -c core.fsmonitor=false status --porcelain --untracked-files=all -- "$p" | cut -c1-2 | sort | uniq -c
done
