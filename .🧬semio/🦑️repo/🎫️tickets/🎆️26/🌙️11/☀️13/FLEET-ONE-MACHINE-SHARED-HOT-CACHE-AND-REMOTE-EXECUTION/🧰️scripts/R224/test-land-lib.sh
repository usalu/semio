#!/bin/zsh
# 🧪️ Laws of land-lib.sh on a scratch root and a fake store: new files first, modified files only after the peers acknowledged delivery, bases checked under the lock, exact restore, lock removed by the exit trap. Run: zsh test-land-lib.sh
HERE=${0:A:h}
SCR=$(mktemp -d /private/tmp/land-lib-test.XXXXXX); trap 'rm -rf "$SCR"' EXIT
PASS=0; FAIL=0
ok(){ if eval "$2"; then PASS=$((PASS+1)); else FAIL=$((FAIL+1)); echo "FAIL: $1"; fi; }

mk(){
 ROOT=$SCR/$1/root; ST=$SCR/$1/state; LB=$SCR/$1/lockbase; STORE=$SCR/$1/store
 mkdir -p "$ROOT/mod/🌡️host" "$ROOT/.cache/f" "$ST/base/mod/🌡️host" "$ST/new/mod/🪟️new/🧪️tests" "$ST/new/mod/🌡️host" "$LB" "$STORE/v1/origins/me/segments" "$STORE/v1/origins/p1"
 echo "old" > "$ROOT/mod/🌡️host/🟦️.ts"; echo "old" > "$ST/base/mod/🌡️host/🟦️.ts"; echo "new" > "$ST/new/mod/🌡️host/🟦️.ts"
 echo "mem" > "$ST/new/mod/🪟️new/🟦️.ts"; echo "t" > "$ST/new/mod/🪟️new/🧪️tests/🟦️.ts"
 printf 'N\tmod/🪟️new/🟦️.ts\nN\tmod/🪟️new/🧪️tests/🟦️.ts\nM\tmod/🌡️host/🟦️.ts\n' > "$ST/land.tsv"
 echo "{\"originId\":\"me\",\"storeRoot\":\"$STORE\"}" > "$ROOT/.cache/f/machine.json"
 echo '{"version":1,"segment":1,"bytes":0,"events":0,"hlc":"x"}' > "$STORE/v1/origins/me/head.json"; : > "$STORE/v1/origins/me/segments/000000000001.jsonl"
 echo "{\"sync\":true,\"wallMs\":$(python3 -c 'import time;print(int(time.time()*1000))'),\"cursors\":{},\"queue\":{\"incoming\":0}}" > "$STORE/v1/origins/p1/attest.json"
}
daemon(){
 ( until [ -e "$ROOT/mod/🪟️new/🧪️tests/🟦️.ts" ]; do sleep 0.1; done
   [ -z "$NOPUBLISH" ] || exit 0
   for p in "mod/🪟️new/🟦️.ts" "mod/🪟️new/🧪️tests/🟦️.ts"; do python3 -c 'import json,sys;print(json.dumps({"operation":"put","path":sys.argv[1]},ensure_ascii=False))' "$p" >> "$STORE/v1/origins/me/segments/000000000001.jsonl"; done
   size=$(wc -c < "$STORE/v1/origins/me/segments/000000000001.jsonl" | tr -d ' ')
   echo "{\"version\":1,\"segment\":1,\"bytes\":$size}" > "$STORE/v1/origins/me/head.json"
   [ -z "$LAG" ] || exit 0
   sleep 2; cp "$ROOT/mod/🌡️host/🟦️.ts" "$SCR/modified-at-ack"
   echo "{\"sync\":true,\"wallMs\":$(python3 -c 'import time;print(int(time.time()*1000))'),\"cursors\":{\"me\":{\"segment\":1,\"bytes\":$size}},\"queue\":{\"incoming\":0}}" > "$STORE/v1/origins/p1/attest.json" ) &
 DPID=$!
}
run(){ ( export LAND_STATE=$ST LAND_ROOT=$ROOT LAND_LOCKBASE=$LB LAND_STORE=$STORE LAND_NOWINDOW=1 LAND_LANG=en LAND_DELIVERY_POLL=0.2 ${=LANDENV}; . "$HERE/land-lib.sh"; [ -z "$TESTFN" ] || eval "$TESTFN"; land_run R999 "law" ) > "$SCR/out" 2>&1; RC=$?; }

mk a; daemon; LANDENV="LAND_DELIVERY_LIMIT=20"; run; wait $DPID
ok "green path exits 0" "[ $RC = 0 ]"
ok "modified file written" "[ \"\$(cat \$ROOT/mod/🌡️host/🟦️.ts)\" = new ]"
ok "new files written" "[ -f \$ROOT/mod/🪟️new/🟦️.ts ] && [ -f \$ROOT/mod/🪟️new/🧪️tests/🟦️.ts ]"
ok "modified file was still the base while the peer acknowledged" "[ \"\$(cat \$SCR/modified-at-ack)\" = old ]"
ok "delivery acknowledged by the peer" "grep -q 'acked p1' \$SCR/out"
ok "exit trap removed the lock" "[ ! -e \$LB/landing.lock ]"

mk b; echo "peer edit" > "$ROOT/mod/🌡️host/🟦️.ts"; LANDENV=""; run
ok "base differs: exit 2, nothing written" "[ $RC = 2 ] && [ ! -e \$ROOT/mod/🪟️new ] && [ \"\$(cat \$ROOT/mod/🌡️host/🟦️.ts)\" = 'peer edit' ]"
ok "base differs: lock gone" "[ ! -e \$LB/landing.lock ]"

mk c; mkdir -p "$ROOT/mod/🪟️new"; echo mine > "$ROOT/mod/🪟️new/🟦️.ts"; run
ok "new file exists: exit 2, nothing overwritten" "[ $RC = 2 ] && [ \"\$(cat \$ROOT/mod/🪟️new/🟦️.ts)\" = mine ]"

mk d; daemon; LANDENV="LAND_DELIVERY_LIMIT=20 LAND_FORCE_RED=1"; run; wait $DPID
ok "forced red: exit 3" "[ $RC = 3 ]"
ok "forced red: exact restore (new dirs gone, modified file back)" "[ ! -e \$ROOT/mod/🪟️new ] && [ \"\$(cat \$ROOT/mod/🌡️host/🟦️.ts)\" = old ]"
ok "forced red: lock gone" "[ ! -e \$LB/landing.lock ]"

mk e; daemon; LANDENV="LAND_DELIVERY_LIMIT=20"; TESTFN='land_tests(){ return 1; }'; run; wait $DPID; TESTFN=""
ok "red staged tests restore exactly" "[ $RC = 3 ] && [ ! -e \$ROOT/mod/🪟️new ] && [ \"\$(cat \$ROOT/mod/🌡️host/🟦️.ts)\" = old ]"

mk f; NOPUBLISH=1; daemon; LANDENV="LAND_DELIVERY_LIMIT=2"; run; wait $DPID; NOPUBLISH=""
ok "files never published: exit 5, restored, nothing modified" "[ $RC = 5 ] && [ ! -e \$ROOT/mod/🪟️new ] && [ \"\$(cat \$ROOT/mod/🌡️host/🟦️.ts)\" = old ]"

mk g; LAG=1; daemon; LANDENV="LAND_DELIVERY_LIMIT=2"; run; wait $DPID; LAG=""
ok "a peer that does not acknowledge is named and the landing proceeds after the bound" "[ $RC = 0 ] && grep -q 'lagging p1' \$SCR/out && [ \"\$(cat \$ROOT/mod/🌡️host/🟦️.ts)\" = new ]"

mk h; echo "{\"sync\":true,\"wallMs\":1,\"cursors\":{},\"queue\":{\"incoming\":0}}" > "$STORE/v1/origins/p1/attest.json"; daemon; LANDENV="LAND_DELIVERY_LIMIT=2"; run; wait $DPID
ok "a stale attestation is skipped, not awaited" "[ $RC = 0 ] && grep -q 'skipped p1 stale-attestation' \$SCR/out"

mk i; LANDENV="LAND_ACK=skip LAND_NOWINDOW="; TESTFN='land_minute(){ echo 10; }'; run; TESTFN=""
ok "outside the window and far from the next one: exit 4, nothing written" "[ $RC = 4 ] && [ ! -e \$ROOT/mod/🪟️new ]"

mk j; mkdir "$LB/landing.lock"; echo "claude-other x" > "$LB/landing.lock/owner"; LANDENV="LAND_LOCK_WAIT=3 LAND_ACK=skip"; run
ok "a held lock is waited for, bounded, never taken over; its owner is named" "[ $RC = 1 ] && grep -q 'claude-other' \$SCR/out && [ -e \$LB/landing.lock/owner ] && [ ! -e \$ROOT/mod/🪟️new ]"

mk k; echo 'X\tbad' > "$ST/land.tsv"; run
ok "a manifest line outside the schema pattern is refused" "[ $RC = 1 ]"

mk l; daemon; LANDENV="LAND_DELIVERY_LIMIT=30"; ( export LAND_STATE=$ST LAND_ROOT=$ROOT LAND_LOCKBASE=$LB LAND_STORE=$STORE LAND_NOWINDOW=1 LAND_LANG=en LAND_DELIVERY_POLL=0.2 LAND_DELIVERY_LIMIT=30; . "$HERE/land-lib.sh"; land_run R999 law ) > "$SCR/out" 2>&1 &
LPID=$!; until [ -e "$ROOT/mod/🪟️new/🟦️.ts" ]; do sleep 0.1; done; kill -TERM $LPID; wait $LPID; kill $DPID 2>/dev/null; wait $DPID 2>/dev/null
ok "a termination in the middle of a landing restores exactly and releases the lock" "[ ! -e \$ROOT/mod/🪟️new ] && [ \"\$(cat \$ROOT/mod/🌡️host/🟦️.ts)\" = old ] && [ ! -e \$LB/landing.lock ]"

echo "pass $PASS fail $FAIL"; [ $FAIL = 0 ]
