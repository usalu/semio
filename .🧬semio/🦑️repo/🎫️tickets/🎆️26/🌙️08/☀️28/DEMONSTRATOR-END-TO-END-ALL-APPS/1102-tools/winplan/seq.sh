#!/bin/zsh
export LC_ALL=en_US.UTF-8
HERE=${0:A:h}; C=${HERE:h:h}
V=${SEQV:-/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/winseq}
rm -rf $V; mkdir -p $V
cd /Users/ueli/Documents/semio
while read b p; do if /usr/bin/grep -q '^--- a/' $C/$p; then /usr/bin/grep -h '^--- a/' $C/$p | sed 's|^--- a/||;s|\t.*||'; else /usr/bin/grep -h '^--- ' $C/$p | sed 's|^--- ||;s|\t.*||'; fi; done < ${ORDER:-$HERE/order.txt} | sort -u | while IFS= read -r f; do mkdir -p "$V/${f:h}"; cp "$f" "$V/$f"; done
cd $V
while read b p; do
  pn=-p1; /usr/bin/grep -q '^--- a/' $C/$p || pn=-p0
  if git apply --check $pn $C/$p 2>/dev/null && git apply $pn $C/$p; then echo "$b OK $p"; else echo "$b FAIL $p: $(git apply --check $pn $C/$p 2>&1|head -1|cut -c1-200)"; fi
done < ${ORDER:-$HERE/order.txt}
