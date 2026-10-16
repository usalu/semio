#!/bin/zsh
# Save a pristine copy of each repo-relative path (file or directory) into the relo-manifest baseline mirror, once.
B=/private/tmp/claude-501/-Users-ueli-Documents-semio/35792af2-9c1e-4f70-b4e3-d5fa94336422/scratchpad/relo-baseline
cd /Users/ueli/Documents/semio || exit 1
for p in "$@"; do
  p=${p#./}
  if [[ -d "$p" ]]; then
    find "$p" -type f -not -path '*/node_modules/*' | while read -r f; do [[ -e "$B/$f" ]] || { mkdir -p "$B/${f:h}"; cp -p "$f" "$B/$f"; }; done
  elif [[ -f "$p" ]]; then
    [[ -e "$B/$p" ]] || { mkdir -p "$B/${p:h}"; cp -p "$p" "$B/$p"; }
  else
    echo "absent: $p" >> "$B/.absent"
  fi
done
