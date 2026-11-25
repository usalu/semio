#!/bin/zsh
# 🚦️ The dry run of in-try.sh over every subject and, only when it is clean, the write of in-apply.sh, inside ONE grant (start it with qsh): an asset is written when its trial copy has 0 hard findings in the gate over every pose, clip instant and transition, the silhouette guard is OK, validateAsset has 0 issues, the sweep 0 findings and every authored scale is suitable for print; the output of an earlier dry run is removed first, so that only this run can pass the test (on 2026-11-25 11:47 a failed start of the dry run was judged by the output of an older one and an unmeasured document was written; it was put back at once); otherwise the catalog file is left alone and the reason is printed: qsh in-try-apply.sh <id>…
[[ -n "$MH3_GRANT" ]] || { echo "in-try-apply.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
for a in "$@"; do
  out="$GEN/try/$a.out"
  rm -f "$out"
  GATE_FLAGS="" "$here/in-try.sh" "${a}"
  if [[ -f "$out" ]] && grep -q "^gate exit 0 hard findings: 0 " "$out" && grep -q "^$a silhouette: OK" "$out" && grep -q "^$a validate: 0 issues" "$out" && grep -q "^$a three-views sweep: 0 findings" "$out" && grep -q "^suitability: OK" "$out"; then
    "$here/in-apply.sh" "$a"
  else
    echo "$a NOT written: $(grep -E '^gate exit|silhouette:|validate:|sweep:|^suitability:|UNSUITABLE' "$out" | tr '\n' ' ' | cut -c1-400)"
  fi
done
