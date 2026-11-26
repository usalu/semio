#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
B=🧰️framework/🛍️products/🦑️repo/🔨️modules; C=$B/📚️library; R=$B/🤝️shared-worktree; RC=$C/🏗️build/🌐️remote-cache
OUT=$1; : > $OUT
for f in \
 $C/🏗️build/🗄️cas/🧪️tests/🟦️.ts $C/🏗️build/🗄️cas/📌️pins/🧪️tests/🟦️.ts \
 $C/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🧪️tests/🟦️.ts \
 $R/📒️journal/🧪️tests/🟦️.ts $R/🛡️resilience/🧪️tests/🟦️.ts $R/🛡️resilience/🧪️tests/💓️beat.ts $R/🛡️resilience/🧪️tests/💥️killpoints.ts \
 $R/🔁️replica/🧱️consolidate/🧪️tests/🟦️.ts $R/🔁️replica/🧷️sidecars/🧪️tests/🟦️.ts \
 $R/🔁️replica/🧪️tests/🟦️.ts $R/🔁️replica/🧪️tests/🛡️resilience.ts $R/🔁️replica/🧪️tests/🛟️boot.ts $R/🔁️replica/🧪️tests/💥️killpoints.ts $R/🔁️replica/🧪️tests/🧷️sidecars.ts \
 $RC/🏷️retention/🧪️tests/🟦️.ts $RC/🖥️server/🧪️tests/🟦️.ts $RC/🧹️collector/🧪️tests/🟦️.ts $RC/💾️store/🧪️tests/🟦️.ts ; do
  [ -e "$f" ] || { echo "MISSING $f" >> $OUT; continue; }
  repo test-file "$f" 2>&1 | grep -aE "^(passed|FAILED|refused)|\(fail\)|error:" | cut -c1-300 >> $OUT
done
echo DONE >> $OUT
