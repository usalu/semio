#!/bin/sh
. /g/t/logs/env.sh
cd "/c/semio/✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust"
rm -f /g/t/logs-cp/build.done
cargo test -j 4 --no-run --message-format short > /g/t/logs-cp/build.log 2>&1
echo "exit=$?" > /g/t/logs-cp/build.done
