#!/bin/zsh
# 🧪️ Runs every test of one already-built test binary in its OWN process (process-global UI arena state cannot leak between
# tests) and prints `FAIL <name>` per red plus a total. each-test.sh <test-binary>
bin="$1"; fails=0; total=0
for name in ${(f)"$("$bin" --list --format terse 2>/dev/null | sed -n 's/: test$//p')"}; do
  total=$((total + 1))
  if ! "$bin" --exact "$name" --test-threads 1 >/dev/null 2>&1; then fails=$((fails + 1)); echo "FAIL $name"; fi
done
echo "EACH total=$total fails=$fails"
