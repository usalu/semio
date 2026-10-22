#!/bin/sh
# usage: sh regress-both.sh BASE NEW   baseline and new mirror: the standard list (regress-new.sh) and the extra list (regress-extra.sh); writes /private/tmp/pm-seam/both.done
HERE=$(cd "$(dirname "$0")" && pwd)
rm -f /private/tmp/pm-seam/both.done
for m in "$1" "$2"; do
  sh "$HERE/regress-new.sh" "$m" "$m" > /dev/null 2>&1
  cp /private/tmp/pm-seam/regall-$m.tsv /private/tmp/pm-seam/std-$m.tsv
  rm -f /private/tmp/pm-seam/ex-$m.tsv.done
  sh "$HERE/regress-extra.sh" /private/tmp/pm-seam/$m /private/tmp/pm-seam/ex-$m.tsv > /dev/null 2>&1
done
echo done > /private/tmp/pm-seam/both.done
