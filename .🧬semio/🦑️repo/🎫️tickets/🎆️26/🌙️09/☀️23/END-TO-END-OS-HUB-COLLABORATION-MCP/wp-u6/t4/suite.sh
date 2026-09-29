#!/bin/zsh
# 🧪️ Runs one built test binary as a whole suite, then re-runs every red alone (`--exact`, one thread) so a red that only
# fails beside its neighbours (process-global arena) is told apart from a red of its own. A binary that ends without its
# `test result` line (abort, signal) is reported `INCOMPLETE rc=<rc>` — never as a green suite. suite.sh <label> <test-binary>
label="$1"; bin="$2"
out=$("$bin" --test-threads 4 2>&1); rc=$?
reds=(${(f)"$(print -r -- "$out" | sed -n 's/^test \(.*\) \.\.\. FAILED$/\1/p')"})
result=$(print -r -- "$out" | /usr/bin/grep '^test result' | tail -1)
echo "SUITE $label reds=${#reds} rc=$rc"
[[ -n "$result" ]] || echo "INCOMPLETE $label rc=$rc last: $(print -r -- "$out" | /usr/bin/grep -E 'panicked at|aborting|signal' | tail -2 | tr '\n' ' ' | cut -c1-300)"
for name in $reds; do
  if "$bin" --exact "$name" --test-threads 1 >/dev/null 2>&1; then echo "ALONE-OK $name"; else echo "RED $name"; fi
done
