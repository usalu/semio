#!/bin/zsh
# usage: privloop.sh <family> <pkg-for-files> [rounds]  — oracle, print new errors, apply privacy fixes, repeat until no privacy diagnostics remain
fam=$1; pkg=$2; rounds=${3:-4}
cd "$(dirname "$0")"
for r in $(seq 1 $rounds); do
  perl -e 'alarm 900; exec @ARGV' -- python3 ftc.py oracle families/$fam.json --cfgs test --label p$r >/dev/null 2>&1
  python3 newerr.py ../🗑️generated/ftc-pos-$fam-p$r-result.json 6 | head -9
  export AUTOPRIV_FILES="$(python3 ftc.py files $pkg)"
  out=$(python3 autopriv.py $fam "ftc-pos-$fam-p$r-cur-test-it*.err" --apply 2>&1)
  echo "$out" | /usr/bin/grep -c '^fixed' | sed 's/^/fixed: /'
  echo "$out" | /usr/bin/grep 'SKIPPED\|UNLOCATED' | head
  [ "$(echo "$out" | /usr/bin/grep -c '^fixed')" = "0" ] && break
done
