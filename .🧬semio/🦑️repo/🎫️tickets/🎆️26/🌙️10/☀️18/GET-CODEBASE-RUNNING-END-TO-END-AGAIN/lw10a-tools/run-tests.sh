#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
export SEMIO_TEST_ARTIFACT_DIR="$PWD/$C/🗑️generated/lw10a-run/artifacts"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" "$C/🗑️generated/lw10a-run/tests"
i=0
while read -r f; do
  i=$((i+1))
  out="$C/🗑️generated/lw10a-run/tests/$i.log"
  echo "$f" > "$out"
  ( bun test --timeout 60000 "./$f" >> "$out" 2>&1; echo "exit $?" >> "$out" ) &
  if (( i % 4 == 0 )); then wait; fi
done < <(grep -v "^$" $C/lw10a-tools/changed-tests.txt)
wait
echo ALLDONE > "$C/🗑️generated/lw10a-run/tests/DONE"
