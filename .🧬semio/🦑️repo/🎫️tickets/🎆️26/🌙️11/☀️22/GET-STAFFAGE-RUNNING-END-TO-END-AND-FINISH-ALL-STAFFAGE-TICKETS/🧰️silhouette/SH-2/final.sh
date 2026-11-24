#!/bin/zsh
# 🏁️ The final measurements of the 25 people of SH-2, one process after the other, into `record/`: every check of the guard (gate.txt), validator, three-views sweep and product check CLI beside the backup (proof.txt), the real `🧊️catalog-silhouette` test (silhouette-test.txt with the first failing expectation per person) and the other suites with a name filter (suites.txt). final.sh [step…]   (steps: sheets gate proof test suites; all by default; `sheets` renders the product sheets of 1:50 and 1:500 of the catalog file and of the backup)
source "${0:A:h}/env.sh"; cd "$W"
mkdir -p record
if (( $# )); then steps=("$@"); else steps=(sheets gate proof test suites); fi
people=(${=SCOPE})
if (( ${steps[(I)sheets]} )); then for a in $people; do ./look.sh $a 50 500 > /dev/null; TAG=before ./look.sh $a 50 500 > /dev/null; done; echo "sheets: $(ls "$GEN/sheets" | grep -c after) after, $(ls "$GEN/sheets" | grep -c before) before"; fi
if (( ${steps[(I)gate]} )); then ./gate.sh final $people > record/gate.txt; echo "gate: $(grep -c ': OK' record/gate.txt) of ${#people} OK"; fi
if (( ${steps[(I)proof]} )); then ./proof.sh $people > record/proof.txt; echo "proof: $(grep -c '✓' record/proof.txt) of ${#people} with ✓"; fi
if (( ${steps[(I)test]} )); then
  line="$(VT_TIMEOUT=3000 ./vt.sh silhouette 🧊️catalog-silhouette $people)"
  { echo "$line"; grep -E "^AssertionError|^ +(✓|×|✗) " "$GEN/vt/silhouette.log" | cut -c1-240; } > record/silhouette-test.txt
  echo "real test: $line"
fi
if (( ${steps[(I)suites]} )); then
  : > record/suites.txt
  for suite in 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog ✅️asset-validation ⚡️asset-power; do
    line="$(SUFFIX= VT_TIMEOUT=2400 ./vt.sh "suite-$suite" "$suite" $people)"
    failed="$(grep -E '^ *(FAIL|×|✗)' "$GEN/vt/suite-$suite.log" | grep -oE "(${(j:|:)people})" | sort -u | tr '\n' ' ')"
    echo "- \`$suite\`: $line${failed:+ failing people: $failed}" >> record/suites.txt
    echo "$suite: $line ${failed:+failing: $failed}"
  done
fi
echo FINAL-DONE
