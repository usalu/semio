#!/bin/zsh
# 💾️ Copies each given repo-relative file into the session scratchpad (outside the repo, first copy wins) so any edit of this slice can be diffed or restored; ticket paths would exceed the path-length limit.
cd /Users/ueli/Documents/semio || exit 1
PRE=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/dm2pre
for f in "$@"; do
  [ -f "$f" ] || continue
  dest="$PRE/$f"
  [ -e "$dest" ] && continue
  mkdir -p "$(dirname "$dest")" && cp -p "$f" "$dest"
done
