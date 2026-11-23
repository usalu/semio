#!/bin/zsh
# 🏁️ The final measurements of the 24 people, one process after the other, into `record/`: validator and check CLI (proof.txt), the gate of every scale and view with every `checkVolumes` rule (after.json), the real `catalog-silhouette` test (after-silhouette-test.txt), the other catalog suites that name a person in their test names (suites.txt), the power invariants and the manifest rows (rows.txt). final.sh [step…]   (steps: proof gate test suites rows; all by default)
source "${0:A:h}/env.sh"; cd "$W"
mkdir -p record
if (( $# )); then steps=("$@"); else steps=(proof gate test suites rows); fi
people=(${=SCOPE})
if (( ${steps[(I)proof]} )); then ./proof.sh $people > record/proof.txt; echo "proof: $(grep -c '| 0 issues | ✓' record/proof.txt) of ${#people} with 0 issues and ✓"; fi
if (( ${steps[(I)gate]} )); then ./gate.sh after $people > "$GEN/gate-after.txt"; python3 summarize.py after; fi
if (( ${steps[(I)test]} )); then
  ./vt.sh after-all 🧊️catalog-silhouette $people > "$GEN/vt-after.txt"
  grep -E "^AssertionError" "$GEN/vt/after-all.log" > record/after-silhouette-test.txt
  echo "real test: $(cat "$GEN/vt-after.txt")"
fi
if (( ${steps[(I)suites]} )); then
  : > record/suites.txt
  for suite in ✅️asset-validation ⚡️asset-power 🔭️multiview-coverage 🫁️breathing-catalog 🌡️catalog-physics 🧊️volumes-rules 🧊️volumes-connected 🧊️model-silhouette 🧊️three-views 📏️print-suitability 🧬️schema-conformance 🦴️lateral-rest 🧍️front-projection 🧍️plan-projection ✂️section-views 🧊️model-catalog 🧊️model-roundness; do
    line="$(SUFFIX= ./vt.sh "suite-$suite" "$suite" $people)"
    failed="$(grep -E '^ (FAIL|×)' "$GEN/vt/suite-$suite.log" | grep -oE "(${(j:|:)people})" | sort -u | tr '\n' ' ')"
    echo "- \`$suite\`: $line${failed:+ failing people: $failed}" >> record/suites.txt
    echo "$suite: $line"
  done
fi
if (( ${steps[(I)rows]} )); then
  "$B" power.ts $people > "$GEN/power.txt" 2>&1; "$B" rows.ts $people > "$GEN/rows.txt" 2>&1
  { echo "- Power invariants of \`⚡️asset-power\` (the loops over the whole catalog, run for the 24 by \`REP-B/power.ts\`): $(tail -1 "$GEN/power.txt"); $(grep -c 'manifest says' "$GEN/power.txt") people whose manifest entry carries another power than the asset evaluates."; echo "- Rows (\`REP-B/rows.ts\`): $(tail -1 "$GEN/rows.txt")"; grep "row changed" "$GEN/rows.txt" | sed 's/^/  - /'; } > record/rows.txt
  cat record/rows.txt | cut -c1-300
fi
echo FINAL-DONE
