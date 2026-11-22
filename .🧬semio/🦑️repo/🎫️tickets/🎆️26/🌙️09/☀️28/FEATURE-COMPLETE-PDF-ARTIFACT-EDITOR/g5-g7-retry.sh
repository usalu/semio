#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR/🗑️generated"
name=$1; shift
for attempt in $(seq 1 40); do
  repo cargo --lang=en --store=shared --configuration=prototype --parallelism=2 -- "$@" > "$G/$name.log" 2>&1
  if grep -qE "Internal error|Export named|couldn't find file|could not compile \`socket2|cranelift|Unterminated string|SyntaxError|Bun v1|stamps moved|edited by another process|SyntaxError|Closed host capacity|MutationLeaf source authority failed|No space left" "$G/$name.log" && ! grep -q "test result" "$G/$name.log"; then
    echo "[DEBUG] attempt $attempt hit tooling error" >> "$G/$name.attempts"; sleep 90; continue
  fi
  break
done
grep -v "source-admission\|foreign-source\|planning-capture\|unit-metadata\|capacity\]\|repo\] still" "$G/$name.log" | tail -60 | cut -c1-400
