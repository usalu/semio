#!/bin/sh
# runs real Bazel lanes one at a time after the first one finished
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/🗑️generated/repo-tests-b/lanes"
while ! grep -q '^rc=' "$S/selected-graph.err" 2>/dev/null; do sleep 15; done
for pair in "locale-law @semio-tech/repo-lib:test-locale-law-ownership" "build-mode @semio-tech/repo-lib:test-build-mode" "build-surfaces @semio-tech/repo-lib:test-build-surfaces" "build-source-demand @semio-tech/repo-lib:test-build-source-demand" "caller-birth @semio-tech/repo-lib:test-build-caller-birth" "bazel-storage repo:test-bazel-storage" "graph-seal repo:test-graph-seal" "tools-partition repo:test-tools-partition" "javascript-storage repo:test-javascript-storage" "cache-scope repo:test-cache-scope"; do
  name=${pair%% *}; target=${pair#* }
  nice -n 10 bun repo --tool=bazel run "$target" > "$S/$name.out" 2> "$S/$name.err"
  echo "rc=$?" >> "$S/$name.err"
done
echo done > "$S/chain.done"
