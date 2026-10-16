#!/bin/zsh
# 🧮 lib + test closure census of the flow-flow carve crates (work tree or real tree)
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/osflow-tools/closure.ts
W=${WORKDIR:-/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/ff/work}
export ROOT_DIR=$W
for spec in "model:🧱️model" "document:🧾️document" "copy:🧵️retained/📑️copy" "mutation-laws:🌿️vcs/⚖️laws" "snapshot-laws:🚪️io/⚖️laws" "facade:."; do
  n=${spec%%:*}; d=${spec##*:}
  [ -f "$W/$d/🦀️.rs" ] || continue
  pk="$W/$d/📦️packages/🦀️rust"
  l=$(TOTAL_ONLY=1 bun $T "$W/$d/🦀️.rs" "$pk" | tail -1 | awk '{print $3" files "$5" bytes "$7" lines "$8}')
  t=$(EXTRA=test,'feature=protocol-laws' TOTAL_ONLY=1 bun $T "$W/$d/🦀️.rs" "$pk" | tail -1 | awk '{print $3" files "$5" bytes "$7" lines "$8}')
  echo "$n  LIB: $l   TEST: $t"
done
