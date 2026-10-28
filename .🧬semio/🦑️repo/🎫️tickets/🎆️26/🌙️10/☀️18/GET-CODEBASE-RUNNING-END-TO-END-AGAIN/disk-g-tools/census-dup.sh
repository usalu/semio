#!/bin/sh
# usage: census-dup.sh <out-prefix> <root>...   streams: find -> stat -> sort -> awk -> shasum; no tree held in memory
cd /Users/ueli/Documents/semio || exit 1
out="$1"; shift
find "$@" -xdev -type f -size +1M -print0 2>/dev/null | xargs -0 stat -f '%z %d:%i %N' 2>/dev/null | sort -k1,1n -k2,2 -u > "$out.files"
awk '{n[$1]++} END{for(s in n) if(n[s]>1) print s}' "$out.files" | sort -n > "$out.sizes"
awk 'NR==FNR{d[$1]=1;next} ($1 in d)' "$out.sizes" "$out.files" > "$out.cand"
: > "$out.hash"
while IFS= read -r line; do
  size=${line%% *}; rest=${line#* }; path=${rest#* }
  h=$(shasum -a 256 -- "$path" 2>/dev/null | cut -d' ' -f1)
  printf '%s %s %s\n' "$h" "$size" "$path" >> "$out.hash"
done < "$out.cand"
sort "$out.hash" | awk '{c[$1]++; s[$1]=$2; if(c[$1]==1) f[$1]=$3} END{for(h in c) if(c[h]>1){t=(c[h]-1)*s[h]; tot+=t; print t, c[h], s[h], f[h]}; print tot, "TOTAL"}' | sort -rn > "$out.report"
echo finished > "$out.done"
