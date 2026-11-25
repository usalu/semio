#!/bin/zsh
# 📟️ The state of the proof chain of batch MV-1: per plant the number of proof lines, whether it is done, and every line that is not green (an exit code other than 0, a failing cell, new issues, findings). status.sh
BATCH=MV-1 source "${0:A:h}/../MR/env.sh"
for f in "$GEN"/proof/*.txt(N); do
  a="${f:t:r}"; bad="$(grep -vE "^done|exit 0|silhouette: OK|validate: 0 issues|sweep: 0 findings" "$f"; grep -E " [1-9][0-9]* fail|[1-9][0-9]* hard findings|failed \(|[1-9][0-9]* failed" "$f")"
  echo "$a: $(wc -l < "$f" | tr -d ' ') lines$(grep -q '^done' "$f" && echo ', done')${bad:+ | NOT GREEN: ${bad[1,300]}}"
done
