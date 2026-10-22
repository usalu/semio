#!/bin/sh
# usage: live.sh <id>: edit the dag-host manifest (adds one dev-dependency), run hub and dag locked fetches through the repo cargo ingress, undo the edit, run both again
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/lock-auto-tools"; G="$C/🗑️generated/lock-auto"; id=$1
HUB="🌎️hub/Cargo.toml"; DAG="✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml"
cp "🌎️hub/Cargo.lock" "$G/$id.hub.lock.before"; cp "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.lock" "$G/$id.dag.lock.before"
python3 "$T/edit2.py" add
sh "$T/cargo.sh" $id-hub-edited fetch --locked --offline --manifest-path "$HUB"
sh "$T/cargo.sh" $id-dag-edited fetch --locked --offline --manifest-path "$DAG"
python3 "$T/edit2.py" remove
sh "$T/cargo.sh" $id-hub-restored fetch --locked --offline --manifest-path "$HUB"
sh "$T/cargo.sh" $id-dag-restored fetch --locked --offline --manifest-path "$DAG"
cp "🌎️hub/Cargo.lock" "$G/$id.hub.lock.after"; cp "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.lock" "$G/$id.dag.lock.after"
echo done > "$G/$id.done"
