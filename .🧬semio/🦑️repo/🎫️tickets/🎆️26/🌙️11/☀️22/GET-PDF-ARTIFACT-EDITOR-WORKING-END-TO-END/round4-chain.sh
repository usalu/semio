#!/bin/sh
# usage: round4-chain.sh  (one after another, never two cargo runs: the PDF editor laws of round 4, a quiet tree, the measured wasm build r4g, the full journey pdf-journey8; markers in 🗑️generated/round4-chain.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
G="$here/🗑️generated"
cd "$here"
echo "laws $(date +%T)" >> "$G/round4-chain.log"
sh ./run-media-in-save.sh mis-5 the_pdf17_save_action_names_a_declared_native_output_port media_in_of_a_small_pdf_then_edit_and_save
echo "laws $(cat "$G/mis-5.done") $(date +%T)" >> "$G/round4-chain.log"
sh ./await-quiet-tree.sh 5
echo "quiet $(date +%T)" >> "$G/round4-chain.log"
sh ./measured-wasm-build.sh r4g
echo "wasm $(cat "$G/wasm-r4g.done") $(date +%T)" >> "$G/round4-chain.log"
grep -q "exit 0" "$G/wasm-r4g.done" || exit 1
sh ./run-round4-journey.sh pdf-journey8
echo "journey $(cat "$G/pdf-journey8.done") $(date +%T)" >> "$G/round4-chain.log"
