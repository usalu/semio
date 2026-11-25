#!/bin/zsh
# 📟️ One screen of the state of the batch: status.sh prints per asset the kept and open shapes of the search, the result lines of the landing (STOP, gate, matrix) and what holds the lock now.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
for a in $(cut -f2 "$U/🧰️matrix/batches/MH-5.tsv"); do
  log="$GEN/auto/$a.log"; out="$GEN/land/$a.all.out"; [[ "$a" == presenter-at-flipchart ]] && out="$GEN/land-presenter.out"
  print -r -- "$a: search $([[ -f $log ]] && echo "$(grep -c '^KEPT' $log)k/$(grep -c '^OPEN' $log)o$(grep -q ' done: ' $log && echo ' done')" || echo -) | land $([[ -f $out ]] && (cat $out "$GEN/land/$a.matrix.out" 2>/dev/null | grep -E '^STOP|^gate: |^gate exit|^matrix exit|^status' | cut -c1-90 | tr '\n' ';') || echo -)"
done
ps -eo etime,command | grep -E "lockf -k .*MH-5" | grep -v grep | sed -E 's/.*\.lock //' | cut -c1-60,200-290
date
