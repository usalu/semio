#!/usr/bin/env bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"; A="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"; S="🏅️standards/🔖️1/🪆️subsets/✳️any"
run() { # dir log args...
  d="$1"; l="$2"; shift 2
  for try in 1 2 3 4 5 6 7 8 9 10 11 12; do
    "$T/carve-test.sh" "$d" "$l" "$@"
    if grep -q "module dbg has no single matching file" "$T/🗑️generated/$l.log"; then sleep 240; else break; fi
  done
}
run "$A/🔩️en1993/$S/🧬️schema/💡️inferences/⚖️checks/🧬️core/📦️packages/🦀️rust" run-core -p semio-s-artifact-norm-en1993-checks-core
run "$A/🏛️en1992/$S/🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust" run-en1992 -p semio-s-artifact-norm-en1992-checks -p semio-s-artifact-norm-en1992-checks-reach-and-trace
run "$A/🔩️en1993/$S/🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust" run-en1993-checks -p semio-s-artifact-norm-en1993-checks -p semio-s-artifact-norm-en1993-checks-laws
run "$A/🔩️en1993/$S/✏️editor/📦️packages/🦀️rust" run-en1993-editor -p semio-s-artifact-norm-en1993-editor -p semio-s-artifact-norm-en1993-editor-laws
run "$A/🧩️en1994/$S/✏️editor/📦️packages/🦀️rust" run-en1994-editor -p semio-s-artifact-norm-en1994-editor -p semio-s-artifact-norm-en1994-editor-laws
run "$A/🪵️en1995/$S/🧬️schema/⚖️kernel/📦️packages/🦀️rust" run-en1995-kernel -p semio-s-artifact-norm-en1995-kernel
run "$A/🫨️en1998/$S/🧬️schema/📐️formulas/📦️packages/🦀️rust" run-en1998 -p semio-s-artifact-norm-en1998-schema-formulas -p semio-s-artifact-norm-en1998-schema-formulas-worked-examples
run "$A/🏭️vdi3805/$S/🧬️schema/💡️inferences/📦️packages/🦀️rust" run-vdi3805 -p semio-s-artifact-norm-vdi3805-inferences -p semio-s-artifact-norm-vdi3805-inferences-laws
echo ALLDONE > "$T/🗑️generated/run-all.done"
