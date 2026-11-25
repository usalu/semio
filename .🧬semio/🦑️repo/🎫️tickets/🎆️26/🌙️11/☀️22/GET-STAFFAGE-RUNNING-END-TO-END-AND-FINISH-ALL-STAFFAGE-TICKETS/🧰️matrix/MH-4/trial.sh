#!/bin/zsh
# 🔬️ One loft trial of the third takeover inside a held grant: trial.sh <id>   runs loft-try.sh <id> full (lofts with lofta.ts, the after ops, silhouette, validate, sweep, gate over every subject, authors' scan) ; the summary (where.ts on a marked view is run by hand: it takes minutes at 1:50) goes to $GEN/try/<id>.trial.out.
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/try"
{
  "$W/loft-try.sh" "$a" full
} > "$d/$a.trial.out" 2>&1
