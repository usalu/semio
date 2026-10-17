#!/bin/zsh
# usage: run.sh <label> [APPLY] -> rescans candidates, rebuilds owners, runs rewrite (dry or APPLY)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/shu-tools/env.sh
export BARREL T G
cd $R
git -c core.quotepath=false grep --untracked -l -F 'ShellHelpers' -- . ':!.🧬semio' 2>/dev/null | sort -u > $G/shu-candidates-$1.txt
find "$E/🛠️ShellHelpers" -type f \( -name '*.ts' -o -name '*.tsx' \) >> $G/shu-candidates-$1.txt
node $T/scan.cjs $G/shu-candidates-$1.txt > $G/shu-scan-$1.json
if [ -f "$BARREL" ]; then nice -n 10 node --max-old-space-size=8000 $T/ownermap.cjs $G/shu-tsconfig.json $G/shu-owners-$1.json > /dev/null; else cp $G/shu-owners0.json $G/shu-owners-$1.json; fi
REPORT_OUT=$G/shu-applied-$1.json node $T/rewrite.cjs $G/shu-owners-$1.json $G/shu-scan-$1.json $2
