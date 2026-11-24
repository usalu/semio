#!/bin/zsh
A="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated"
run() { fam=$1; log=$2; shift 2; for try in $(seq 1 90); do (cd "$A/$fam" && repo cargo --store=shared --configuration=prototype -- test --offline --no-fail-fast "$@" > "$G/$log" 2>&1); if grep -aq "^test result" "$G/$log" || grep -aq "could not compile \`semio-s-artifact\|Rust crate semio-s-artifact" "$G/$log"; then break; fi; if grep -aq "exceeds its source boundary\|could not compile\|command failed" "$G/$log"; then sleep 120; continue; fi; break; done; }
run 🪶️en1999 kit-f-en1999-pkg.log -p semio-s-artifact-norm-en1999
echo done > "$G/kit-chain4.done"
