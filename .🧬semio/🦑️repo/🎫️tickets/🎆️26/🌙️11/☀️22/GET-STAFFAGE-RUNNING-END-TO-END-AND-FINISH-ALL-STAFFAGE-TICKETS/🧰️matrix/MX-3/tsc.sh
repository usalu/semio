#!/bin/zsh
# 🧪️ Type check of the staffage package in one queue grant: tsc.sh
source "${0:A:h}/env.sh"; mkdir -p "$GEN"
cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=1500 -- /bin/zsh -c "cd $P && /usr/bin/time -l timeout 1800 $B x tsc --noEmit -p tsconfig.json > $GEN/tsc.log 2>&1; echo tsc exit \$? >> $GEN/tsc.log"
