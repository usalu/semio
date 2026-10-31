#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio || exit 1
for t in test-javascript-storage test-javascript-adopt test-javascript-generation test-javascript-incremental-sync test-javascript-provider-walk test-javascript-concurrent-generations test-javascript-installed-paths test-bootstrap-dependencies; do
  SEMIO_LANGUAGE=en repo run repo:$t > "$C/🗑️generated/1030/scopereact-$t.txt" 2>&1
  echo $? > "$C/🗑️generated/1030/scopereact-$t.rc"
done
echo done > "$C/🗑️generated/1030/scopereact-tests.rc"
