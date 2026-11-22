#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR/🗑️generated/g14"
name=$1; shift
for attempt in $(seq 1 20); do
  repo cargo --lang=en --store=shared --configuration=prototype --parallelism=2 -- check --offline --manifest-path '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.toml' "$@" > "$G/$name.log" 2>&1
  if grep -qE "Internal error|Export named|couldn't find file|could not compile \`socket2|cranelift|Unterminated string|SyntaxError|Bun v1|stamps moved|edited by another process|Closed host capacity|MutationLeaf source authority failed|No space left" "$G/$name.log" && ! grep -qE "^error(\[|:) " "$G/$name.log"; then echo "attempt $attempt tooling" >> "$G/$name.attempts"; sleep 60; continue; fi
  break
done
echo FINISHED >> "$G/$name.log"
