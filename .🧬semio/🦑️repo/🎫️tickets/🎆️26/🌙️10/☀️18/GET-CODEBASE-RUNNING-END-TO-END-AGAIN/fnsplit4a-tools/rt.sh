#!/bin/zsh
# usage: rt.sh <file>... : bun test with the scratch tsconfig override and artifact dir (explicit ./ paths), failures only
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
T=$C/🗑️generated/fnsplit4a-census
cd /Users/ueli/Documents/semio
mkdir -p $T/art
args=(); for f in "$@"; do args+=("./$f"); done
SEMIO_TEST_ARTIFACT_DIR=$T/art perl -e 'alarm 580; exec @ARGV' -- bun --tsconfig-override=$T/ov.json test "${args[@]}" 2>&1 | command grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$|^error: .*(Cannot find|Missing)" | sort | uniq -c | sort -rn | head -${RT_HEAD:-40}
