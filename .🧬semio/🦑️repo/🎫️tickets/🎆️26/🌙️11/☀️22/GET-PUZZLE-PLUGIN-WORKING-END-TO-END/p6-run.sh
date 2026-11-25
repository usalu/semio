#!/usr/bin/env bash
# usage: p6-run.sh <workdir> <log> <cargo test args...>   (masked per brief rule 12)
T=/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END
cd /home/deck/semio
wd="$1"; log="$2"; shift 2
exec repo capacity run --memory=3500 --cpu=3 -- timeout 3300 "$T/native-cargo.sh" "$wd" "source \"\$NATIVE_REPO_ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/mask-dbg.sh\" > $T/🗑️generated/$log 2>&1; cargo test $* --offline --jobs 2 >> $T/🗑️generated/$log 2>&1; echo EXIT \$? >> $T/🗑️generated/$log"
