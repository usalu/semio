#!/bin/zsh
# 🧪️ Runs one built test binary as a whole suite, then re-runs every red alone (`--exact`, one thread) so a red that only
# fails beside its neighbours (process-global arena) is told apart from a red of its own. suite.sh <label> <test-binary>
label="$1"; bin="$2"
reds=(${(f)"$("$bin" --test-threads 4 2>&1 | sed -n 's/^test \(.*\) \.\.\. FAILED$/\1/p')"})
echo "SUITE $label reds=${#reds}"
for name in $reds; do
  if "$bin" --exact "$name" --test-threads 1 >/dev/null 2>&1; then echo "ALONE-OK $name"; else echo "RED $name"; fi
done
