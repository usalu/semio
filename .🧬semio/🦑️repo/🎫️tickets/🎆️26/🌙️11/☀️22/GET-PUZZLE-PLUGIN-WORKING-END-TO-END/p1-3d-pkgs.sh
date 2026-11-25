#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
for p in session window viewer model snapshot editor; do
  n=semio-s-artifact-puzzle-3d-$p; [ $p = session ] || [ $p = window ] || [ $p = snapshot ] && n=semio-s-artifact-puzzle-3d-editor-$p
  BUILDER_MEM=2500 BUILDER_TIMEOUT=10000 "$T/builder.sh" exec "cd '/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d' && cargo test --jobs 3 --offline --color never -p $n --no-fail-fast -- --test-threads=1 > '$T/🗑️generated/p1-pkg-$p.log' 2>&1; echo RC=\$? >> '$T/🗑️generated/p1-pkg-$p.log'"
done
