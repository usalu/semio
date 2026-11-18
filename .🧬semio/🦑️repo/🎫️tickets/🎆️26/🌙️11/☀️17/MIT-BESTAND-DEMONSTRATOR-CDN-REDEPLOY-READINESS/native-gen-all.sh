#!/bin/bash
HERE=$(cd "$(dirname "$0")" && pwd); P="✏️s/🔌️plugins"
for s in "trinity-jack-rs=$P/🔱️trinity/🗿️artifacts/🔌️jack" "trinity-rewriting-rs=$P/🔱️trinity/🗿️artifacts/♻️rewriting" "reasoning-wires-rs=$P/💡️reasoning/🗿️artifacts/🔌️wires" "writer-writer-rs=$P/✒️writer/🗿️artifacts/✒️writer" "draw-drawing-rs=$P/🖍️draw/🗿️artifacts/🖍️drawing" "puzzle-5d-rs=$P/🧩️puzzle/🗿️artifacts/🖐️5d" "flow-plugin=🌎️hub/🧩️compositions/🌊️flow"; do
  n=${s%%=*}; r=${s#*=}; MEM=1500 "$HERE/native-wasm.sh" "gen:@semio-tech/$n:graph-generate=$r/📦️packages/🦀️rust"
done
