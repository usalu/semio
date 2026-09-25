#!/bin/zsh
# 🌐️ WG7 hub 8050: a fresh data root holding a copy of `s11-wg7-catalog-n`'s current generation, a signed copy of the os-hub binary
# that published it, both human credentials, and W2's `w2-hub-hold.ts` detached (setsid). Refuses an existing data root or a bound port.
# usage: zsh wg7-hub-8050.sh
set -u
R=/Users/ueli/Documents/semio
H=$R/.🧬semio/🌐hub
CAT=$H/s11-wg7-catalog-n
DATA=$H/s11-wg7-hub-8050
STATE=$H/s11-wg7-hub-8050-state
BINDIR=$H/s11-wg7-bin
HOLD=($R/.tmp-ticket/wp-w2/w2-hub-hold.ts)
step() { echo "[wg7-hub] $1 $(date '+%F %T')"; }
step "START hub-8050"
GEN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['generationId'])" $CAT/trusted-catalog/current.json)
[ -e $DATA ] && { step "FAIL data root $DATA exists"; exit 1; }
mkdir -p $DATA/trusted-catalog/generations && chmod 700 $DATA $DATA/trusted-catalog $DATA/trusted-catalog/generations
cp -Rp $CAT/trusted-catalog/generations/$GEN $DATA/trusted-catalog/generations/ && cp -p $CAT/trusted-catalog/current.json $DATA/trusted-catalog/current.json
diff -r $CAT/trusted-catalog/generations/$GEN $DATA/trusted-catalog/generations/$GEN || { step "FAIL generation copy differs"; exit 1; }
mkdir -p $BINDIR
BIN=$BINDIR/os-hub-8050
rm -f $BIN && cp $R/.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/os-hub $BIN && codesign -s - -f $BIN
for u in "user1@semio.dev|User One|gm1-local-dev-pass-1" "user2@semio.dev|User Two|gm1-local-dev-pass-2"; do
  E=${u%%|*}; REST=${u#*|}; N=${REST%%|*}; P=${REST#*|}
  printf '%s' "$P" | OS_HUB_DATA=$DATA $BIN credential set --email "$E" --display-name "$N" || { step "FAIL credential $E"; exit 1; }
done
lsof -nP -iTCP:8050 -sTCP:LISTEN && { step "FAIL port 8050 bound"; exit 1; }
mkdir -p $STATE
python3 $R/.tmp-ticket/wp-w2/w2-detach.py $STATE/hold.txt bun $HOLD[1] 8050 $DATA $BIN $STATE > $STATE/detach.txt
step "HOLD pid $(cat $STATE/detach.txt) generation $GEN"
for i in $(seq 1 120); do /usr/bin/grep -q "ADMIN issued\|WAIT_FAIL\|CHILD_EXIT" $STATE/status.txt 2>/dev/null && break; sleep 5; done
step "HUB $(cat $STATE/status.txt 2>/dev/null)"
step "DONE"
