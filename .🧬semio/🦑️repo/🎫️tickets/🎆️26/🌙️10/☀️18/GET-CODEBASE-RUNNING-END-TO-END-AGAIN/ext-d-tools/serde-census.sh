#!/bin/bash
# usage: serde-census.sh <crate dir...>: counts serde/serde_json usage in rust sources (non-test dirs and test dirs separately)
for d in "$@"; do
  all=$(rg -c "serde_json|serde::|Serialize|Deserialize|serde\(" "$d" --glob '*.rs' 2>/dev/null | awk -F: '{s+=$NF} END{print s+0}')
  tst=$(rg -c "serde_json|serde::|Serialize|Deserialize|serde\(" "$d" --glob '*.rs' --glob '*🧪️tests*/**' 2>/dev/null | awk -F: '{s+=$NF} END{print s+0}')
  echo "$all total, $tst in tests :: ${d: -80}"
done
