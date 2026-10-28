#!/bin/sh
# 🌱️ seed.sh <mirror root>: copies into a mirror the git-ignored generated sources and the out-of-closure data the Play and demonstrator unit suites read
# (mirror.py copies tracked + untracked-not-ignored only). Read-only on live; clone copies on APFS.
M="${1:?usage: seed.sh <mirror root>}"; R=/Users/ueli/Documents/semio
for d in "✏️s/🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript/🤖️generated" "🧰️framework/🔨️modules/🛂️manifest/🤖️generated" \
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards" "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript"; do
  [ -e "$M/$d" ] && continue; mkdir -p "$M/$(dirname "$d")" && cp -cR "$R/$d" "$M/$d" 2>/dev/null || cp -R "$R/$d" "$M/$d"; echo "[DEBUG] seeded $d"
done
