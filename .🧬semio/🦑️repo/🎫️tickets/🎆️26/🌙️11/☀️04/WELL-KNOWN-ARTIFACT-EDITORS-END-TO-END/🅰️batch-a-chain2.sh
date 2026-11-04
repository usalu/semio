#!/usr/bin/env bash
A="/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
while pgrep -f "batch-a-chain.sh" >/dev/null; do sleep 10; done
run() { (cd "$A/$1" && repo cargo --store=shared --configuration=prototype -- test -p "$2" --lib "${3:-editor}" > "$G/a2-$2.log" 2>&1; echo "$2 exit $?" >> "$G/a2-chain.log"); }
run 🌐️html semio-s-artifact-stdio-html-mutations
run 🧾️json semio-s-artifact-stdio-json-mutations-2
