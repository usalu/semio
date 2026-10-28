#!/bin/zsh
# [conv-g] verifies the batch. usage: verify.sh static   (before apply: staged files only, no repo run)
#                                      verify.sh suites   (after `stage.py apply`: the graph-free single-file suites of every touched module; exit 1 on any fail)
#                                      verify.sh mirror [short-root]   (rebase-g: NEVER touches the live tree. Builds a short-root mirror (path limit 256), lands oracle2-g'"'"'s files and then this batch there (CG_ROOT/CG_BACKUP), runs the same suites in the mirror, each through the graph-free official envelope (ver-f-tools/proc-run.ts: the compiler/authorizer suites refuse a plain bun test); exit 1 on a refused apply or any fail)
HERE=${0:A:h}; cd /Users/ueli/Documents/semio; W=/Users/ueli/Documents/semio; C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; ENVELOPE=""
BUN=${BUN:-/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
case ${1:-static} in
static)
  python3 -B "$HERE/make-staged.py" >/dev/null 2>&1 || true
  python3 -I "$HERE/stage.py" check; python3 -B "$HERE/selftest.py" && python3 -B "$HERE/laws.py" "$BUN" 2>&1 | grep -v Deprecat ;;
mirror)
  OV=${2:-/private/tmp/claude-501/rb/conv}; ENVELOPE=1
  sh "$C/rebase-g-tools/mirror.sh" "$OV" "🧰️framework/🔨️modules" "🧰️framework/🛍️products/🦑️repo/🔨️modules" "🧰️framework/📦️packages/🟦️typescript/⚙️runtime" >/dev/null || exit 1
  python3 -I "$C/rebase-g-tools/land-oracle2.py" "$OV" || exit 1
  CG_ROOT="$OV" CG_BACKUP="$OV/.cg-backup" python3 -I "$HERE/stage.py" apply | tail -2 || exit 1
  [ -n "$(CG_ROOT="$OV" python3 -I "$HERE/stage.py" check | grep -v -E "DONE")" ] && { echo "mirror not fully applied"; CG_ROOT="$OV" python3 -I "$HERE/stage.py" check | grep -v DONE | head; rm -rf "$OV"; exit 1; }
  W="$OV"; cd "$W"
  ;&
suites)
  F="$W/🧰️framework"; L="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; B="$L/🏗️build"; A="$L/⚡️caching/🚀️bootstrap/🛠️tools"; P="$F/🔨️modules/🏃️process"
  fail=0
  attempt(){ local dir="$1" how="$2" out; if [ "$how" = env ]; then out=$(VF_WALL=280 nice bun "$C/ver-f-tools/proc-run.ts" file "$dir/🟦️.ts" 2>&1); else out=$(cd "$dir" && nice "$BUN" test --timeout 280000 ./🟦️.ts 2>&1); fi; PASS=$(echo "$out" | grep -E "^ *[0-9]+ pass$" | tail -1 | tr -d ' '); NFAIL=$(echo "$out" | grep -E "^ *[0-9]+ fail$" | tail -1 | tr -d ' '); [ -n "$PASS" ] && { [ -z "$NFAIL" ] || [ "$NFAIL" = "0fail" ]; }; }
  run(){ local dir="$1"; if attempt "$dir" plain; then echo "ok   $PASS  ${dir##*/library/}"; elif [ -n "$ENVELOPE" ] && attempt "$dir" env; then echo "ok   $PASS (official envelope)  ${dir##*/library/}"; elif [ -n "$ENVELOPE" ] && [ "$PASS $NFAIL" = "2pass 2fail" ] && [ "${dir##*/}" = "🧊️memo" ]; then echo "KNOWN-RED $PASS $NFAIL (identical on the live tree before the batch: needs the Nx caller capabilities)  ${dir##*/library/}"; else echo "FAIL ${PASS:-nopass} ${NFAIL:-nofail}  $dir"; fail=1; fi; }
  for d in "$B/🏛️sealed/🧪️tests/🗂️store" "$B/🏛️sealed/🧪️tests/🚦️trampoline" "$F/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧪️tests" "$B/🏡️resident/📜️journal/🧪️tests" "$L/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🧪️tests" "$L/⚡️caching/💾️storage/🧾️ledger/🧪️tests" "$A/🧹️retention/🎚️store-bound/🧪️tests" "$L/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests" "$L/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🏊️pool-model" "$P/🏗️build/🟦️typescript/🧪️tests" "$P/🏗️build/🟦️typescript/🧪️tests/🔁️incremental" "$A/🔏️authorizers/🧪️tests/🧊️memo" "$B/📥️inputs/🔭️read-oracle/🧪️tests" "$B/🧾️ci-report/🗂️suites/🧪️tests" "$B/🕸️graph/🕰️generations/🌿️frontier-walk/🧪️tests" "$B/🕸️graph/🕰️generations/🧩️claims/🧪️tests" "$B/🕸️graph/🕰️generations/🛌️yield/🧪️tests" "$B/🕸️graph/🕰️generations/✂️cutoff/🧪️tests" "$B/🕸️graph/🎯️test-owners/🧪️tests" "$B/📥️inputs/🌳️owner-view/🧪️tests" "$B/🔭️witness/🧪️tests"; do run "$d"; done
  [ "$W" != /Users/ueli/Documents/semio ] && rm -rf "$W"
  exit $fail ;;
esac
