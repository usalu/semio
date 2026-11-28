#!/bin/sh
# builds a scratch checkout view: real folders along the path to the build module, symlinks to the live siblings, staged files overlaid
set -e
LIVE=/Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H95
R=$S/run
BREL="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
rm -rf "$R"; mkdir -p "$R"
cur=""; old="$LIVE"
for part in $(echo "$BREL" | tr '/' ' '); do
  mkdir -p "$R/$cur"
  for e in "$old"/* "$old"/.[!.]*; do [ -e "$e" ] || continue; n=$(basename "$e"); [ "$n" = "$part" ] && continue; ln -s "$e" "$R/$cur/$n" 2>/dev/null || true; done
  cur="${cur:+$cur/}$part"; old="$old/$part"
done
mkdir -p "$R/$BREL"
for e in "$old"/*; do n=$(basename "$e"); [ -e "$R/$BREL/$n" ] || ln -s "$e" "$R/$BREL/$n"; done
for e in "$LIVE"/*; do n=$(basename "$e"); [ -e "$R/$n" ] || ln -s "$e" "$R/$n"; done
# overlay staged files (replace symlinked dirs by real dirs level by level)
(cd "$S/stage/tree/B" && find . -type f) | while read f; do
  f="${f#./}"; dest="$R/$BREL/$f"; d=$(dirname "$f"); cur2="$R/$BREL"
  for seg in $(echo "$d" | tr '/' ' '); do
    [ "$seg" = "." ] && continue
    nxt="$cur2/$seg"
    if [ -L "$nxt" ]; then tgt=$(readlink "$nxt"); rm "$nxt"; mkdir "$nxt"; for e in "$tgt"/*; do ln -s "$e" "$nxt/$(basename "$e")"; done; fi
    [ -e "$nxt" ] || mkdir -p "$nxt"
    cur2="$nxt"
  done
  rm -f "$dest"; cp "$S/stage/tree/B/$f" "$dest"
done
echo "$R/$BREL"
LB="$LIVE/$BREL"
python3 - "$R/$BREL" "$LB" <<'PY'
import os,shutil,sys
dst,src=sys.argv[1],sys.argv[2]
assert "scratchpad/H95/run" in dst
for d in ["🚪️door","⚙️operations","🚉️land","🚢️fleet-status","📸️snapshot","💥️broken"]:
    t=os.path.join(dst,d,"🧪️tests"); s=os.path.join(src,d,"🧪️tests")
    if os.path.isdir(s) and not os.path.isfile(os.path.join(t,"..","📸️.marker")):
        if os.path.lexists(t) and not os.path.islink(t) and any(os.path.islink(os.path.join(t,n)) for n in os.listdir(t)):
            shutil.rmtree(t)
        if os.path.islink(t): os.unlink(t)
        if not os.path.exists(t): shutil.copytree(s,t,symlinks=False)
PY
(cd "$S/stage/tree/B" && find . -type f) | while read f; do f="${f#./}"; dest="$R/$BREL/$f"; [ -L "$dest" ] && continue; mkdir -p "$(dirname "$dest")"; cp -f "$S/stage/tree/B/$f" "$dest"; done
for m in package.json 📋️project.json; do cp -f --remove-destination "$LIVE/$m" "$R/$m" 2>/dev/null || { rm "${R:?}/${m:?}"; cp "$LIVE/$m" "$R/$m"; }; done
