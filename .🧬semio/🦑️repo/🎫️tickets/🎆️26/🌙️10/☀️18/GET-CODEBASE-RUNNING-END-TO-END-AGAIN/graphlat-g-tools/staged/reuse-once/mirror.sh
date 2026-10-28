#!/bin/sh
# [graphlat-g] verification of reuse-once (on top of plan-once) on C/mirror-tools/mirror.py mirrors, never on the live tree.
#   sh mirror.sh create      builds the mirrors: glreuse (plan-once + reuse-once applied) and glbase (only the new law files, no edit)
#   sh mirror.sh laws <m>    runs the new laws in mirror <m> (glbase = red, glreuse = green)
#   sh mirror.sh suites <m>  runs the existing suites the edits can reach
#   sh mirror.sh tsc <m>     strict type check of the touched .ts file in mirror <m>
#   sh mirror.sh drop        removes both mirrors
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
T="$PWD/$C/graphlat-g-tools/staged"; MM="python3 -I $PWD/$C/mirror-tools/mirror.py"
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'; G="$L/🏗️build/🕸️graph"
LAWS="$G/🕰️generations/🧩️claims/🧪️tests/💤️idle $G/🕰️generations/🔌️load/🧪️tests/🪑idle-builder $G/📥️inference/🧪️tests/🍱️shared-setup $G/🧪️tests/🧳️carried-closures $G/🕰️generations/🧪️tests/🪞️plan-once"
SUITES="$G/🕰️generations/🧩️claims/🧪️tests $G/🕰️generations/🔌️load/🧪️tests $G/🕰️generations/🧪️tests $G/🧪️tests/🕰️generation-protocol $G/🧪️tests/♻️tagged-retention $G/🧪️tests/🔗️edge-refresh $G/🧪️tests/🕊️graph-release"
case "$1" in
 create)
  $MM create glreuse --batch "BATCH_ROOT={root} python3 -I '$T/batch_generic.py' '$T/plan-once' apply && BATCH_ROOT={root} python3 -I '$T/batch_generic.py' '$T/reuse-once' apply"
  $MM create glbase --batch "cp -R '$T/reuse-once/tree/.' {root}/ && cp -R '$T/plan-once/tree/.' {root}/" ;;
 laws)
  for D in $LAWS; do echo "== $D" | cut -c1-30,100-260; $MM run "$2" --cwd "$D" -- nice -n 5 bun test --timeout 300000 ./🟦️.ts 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)|error:|Error" | head -12; done ;;
 suites)
  for D in $SUITES; do echo "== $D" | cut -c1-30,100-260; $MM run "$2" --cwd "$D" -- nice -n 5 bun test --timeout 300000 ./🟦️.ts 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)|error:" | head -12; done ;;
 tsc)
  $MM run "$2" --cwd "$G" -- nice -n 5 bun x tsc --noEmit --strict --skipLibCheck --allowImportingTsExtensions --target esnext --module esnext --moduleResolution bundler ./🟦️.ts 2>&1 | head -30 ;;
 drop) $MM drop glreuse glbase ;;
 *) echo "usage: sh mirror.sh create|laws <m>|suites <m>|tsc <m>|drop" ;;
esac
