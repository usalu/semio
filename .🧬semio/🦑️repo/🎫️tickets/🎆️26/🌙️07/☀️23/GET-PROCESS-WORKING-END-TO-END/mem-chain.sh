#!/bin/zsh
# Sequential measurement chain (one gated cargo at a time); S is the repository root.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; S="/Users/ueli/Documents/semio"
$T/mem-cargo.sh "$S/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" p3d2 test --no-run -p semio-s-artifact-process-process3d-editor
$T/mem-cargo.sh "$S/🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust" job test --no-run --features testkit
$T/mem-cargo.sh "$S/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📐️model/📦️packages/🦀️rust" rmodel test --no-run
$T/mem-cargo.sh "$S/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/📦️packages/🦀️rust" toolrun test --no-run
$T/mem-cargo.sh "$S/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👥️member/📦️packages/🦀️rust" brep test --no-run
$T/mem-cargo.sh "$S/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust" energy test --no-run
