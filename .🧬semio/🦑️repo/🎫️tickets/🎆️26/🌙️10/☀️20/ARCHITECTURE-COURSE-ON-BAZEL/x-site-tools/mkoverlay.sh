#!/bin/sh
# usage: mkoverlay.sh <overlay-dir>; a scratch checkout whose course tree is a private copy and everything else a symlink to the real checkout
set -e
real="$(cd "$(dirname "$0")/../../../../../../../.." && pwd)"
ov="$1"
rm -rf "$ov"; mkdir -p "$ov"
for entry in "$real"/* "$real"/.[!.]*; do
  name="$(basename "$entry")"
  case "$name" in .git|.cache|.🧬semio|🎓️teaching) continue;; esac
  ln -s "$entry" "$ov/$name"
done
mkdir -p "$ov/.🧬semio"
mkdir -p "$ov/🎓️teaching/🏛️architecture"
for entry in "$real"/🎓️teaching/* ; do
  name="$(basename "$entry")"
  [ "$name" = "🏛️architecture" ] || ln -s "$entry" "$ov/🎓️teaching/$name"
done
for entry in "$real"/🎓️teaching/🏛️architecture/*; do
  name="$(basename "$entry")"
  [ "$name" = "📚️course" ] || ln -s "$entry" "$ov/🎓️teaching/🏛️architecture/$name"
done
rsync -a --exclude node_modules "$real/🎓️teaching/🏛️architecture/📚️course/" "$ov/🎓️teaching/🏛️architecture/📚️course/"
[ -e "$real/🎓️teaching/🏛️architecture/📚️course/node_modules" ] && ln -s "$real/🎓️teaching/🏛️architecture/📚️course/node_modules" "$ov/🎓️teaching/🏛️architecture/📚️course/node_modules"
[ -e "$real/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/node_modules" ] && ln -s "$real/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/node_modules" "$ov/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/node_modules"
echo "$ov"
