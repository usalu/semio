#!/bin/sh
# 🧪 Oracle differential of an overlay built by mkoverlay2.py: pre family on the original sources, post family on the overlay with the plans applied. usage: sh oracle2.sh <tag>  (one process at a time)
set -eu
H="$(cd "$(dirname "$0")" && pwd)"; O=/private/tmp/gran-hubs; tag="$1"
G="$H/../../🗑️generated/gran-hubs/oracle"
nice -n 10 python3 "$H/oracle.py" run "$O/overlay-$tag/pre" "$O/fam/$tag-pre.json" "$tag-pre" --cfgs lib > "$O/oracle-$tag-pre.log" 2>&1
nice -n 10 python3 "$H/oracle.py" run "$O/overlay-$tag/post" "$O/fam/$tag-post.json" "$tag-post" --cfgs lib > "$O/oracle-$tag-post.log" 2>&1
echo "== $tag pre vs post"; python3 "$H/oracle_diff.py" "$G/gh-${tag}pre-$tag-pre-cur-lib-final.err" "$G/gh-${tag}post-$tag-post-cur-lib-final.err" | head -12
