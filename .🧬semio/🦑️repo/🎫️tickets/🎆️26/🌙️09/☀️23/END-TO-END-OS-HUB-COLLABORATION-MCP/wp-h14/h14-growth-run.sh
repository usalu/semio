#!/bin/zsh
# 📈️ H14 14c: runs h14-commit-growth.ts against <origin> detached and samples the hub <pid> (3 s every 20 s) meanwhile.
#   zsh h14-growth-run.sh <origin> <hub-pid> <tag> [perBatch] [batches] [kind]
setopt no_bg_nice
set -u
R=/Users/ueli/Documents/semio; W="$R/.tmp-ticket/wp-h14"; D="$R/.🧬semio/🌐hub/s14-h14-logs"
ORIGIN=$1; PID=$2; TAG=$3; PER=${4:-256}; N=${5:-40}; KIND=${6:-s.note.note}
export OS_HUB_PROBE_EMAIL=user1@semio.dev
export OS_HUB_PROBE_PASSWORD="$(sed -n 's/.*user1@semio.dev|User One|\([^"|]*\).*/\1/p' "$R/.tmp-ticket/wp-w3/w3-restart-7800.sh" | head -1)"
OUT="$D/commit-growth-$TAG.txt"; mkdir -p "$D/samples-$TAG"
HUB_PID=$PID nohup bun "$W/h14-commit-growth.ts" "$ORIGIN" $PER $N $KIND > "$OUT" 2>&1 &
PROBE=$!; disown
echo "probe pid $PROBE → $OUT"
i=0
while kill -0 $PROBE 2>/dev/null; do
  sleep 20; i=$((i+1))
  kill -0 $PROBE 2>/dev/null || break
  echo "$(date '+%T') sample $i $(tail -1 "$OUT" | cut -c1-120)" >> "$D/samples-$TAG/index.txt"
  sample $PID 3 -file "$D/samples-$TAG/s$i.txt" >/dev/null 2>&1
  [ $i -ge 30 ] && break
done
tail -5 "$OUT"
