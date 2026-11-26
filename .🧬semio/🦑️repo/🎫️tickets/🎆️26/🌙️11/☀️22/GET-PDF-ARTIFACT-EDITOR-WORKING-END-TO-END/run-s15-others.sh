#!/bin/sh
# usage: run-s15-others.sh  (slice 15: gated cargo check of the three other workspaces that build PDF page content: semio drawing export, layout PDF interchange, BIM sheets; logs in 🗑️generated)
cd "$(dirname "$0")"
root=$(CDPATH= cd -- "$(pwd)/../../../../../../.." && pwd)
G=🗑️generated
rm -f $G/s15-others.done
for w in "semio:✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio" "layout:✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout" "bim:✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model"; do
  n=${w%%:*}; d=${w#*:}
  for attempt in 1 2 3; do
    NATIVE_CARGO_DIR="$root/$d" sh ./native-cargo.sh check --workspace --tests --keep-going --message-format short > $G/s15-others-$n.log 2>&1
    echo "exit $?" > $G/s15-others-$n.done
    grep -aq "error\[E\|^error: could not compile" $G/s15-others-$n.log || break
    grep -aq "error\[E" $G/s15-others-$n.log && break
    sleep 60
  done
done
echo done > $G/s15-others.done
