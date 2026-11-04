#!/usr/bin/env bash
A="/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
while pgrep -f "stdio-txt-tests --lib editor" >/dev/null; do sleep 10; done
run() { (cd "$A/$1" && repo cargo --store=shared --configuration=prototype -- test -p "$2" --lib "${3:-editor}" > "$G/a-$2.log" 2>&1; echo "$2 exit $?" >> "$G/a-chain.log"); }
run 🔤️txt semio-s-artifact-stdio-txt-tests
run 📝️md semio-s-artifact-stdio-md-tests
run 🧾️json semio-s-artifact-stdio-json-mutations
run 📰️xml semio-s-artifact-stdio-xml-mutations
run 🌐️html semio-s-artifact-stdio-html-mutations
