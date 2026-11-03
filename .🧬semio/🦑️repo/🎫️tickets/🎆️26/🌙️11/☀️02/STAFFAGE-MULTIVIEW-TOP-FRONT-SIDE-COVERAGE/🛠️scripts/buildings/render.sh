#!/bin/sh
S="/c/git/semio/🎓️teaching/🏛️architecture/🧍️staffage"
O="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🗑️generated/buildings"
cd "$S"
id=$1; shift
for sc in "$@"; do
  bun 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $id --scale $sc --views --out "$O/$id" >/dev/null 2>&1 || echo "FAIL $id $sc"
done
