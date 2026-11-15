#!/bin/bash
# 🎠 Drives the tries of a round (rules of rounds 13 to 15) on g1-m07: P12 sampler around every try, launcher identity before and after, 3 minutes between tries; ends on native_rc 0 or 4, or after two tries in a row with the same other error. Usage: drive-round.sh <round> <first try> <last try>
set -u
round="$1"; k="$2"; last="$3"
M="$(cd "$(dirname "$0")" && pwd)"; out="$M/round-$round.tries.txt"; ids="$M/round-$round.launcher.txt"
L="$HOME/.local/share/semio/launcher/bin/repo"; prev=""
identity() { { echo "## $1 $(date -u +%Y-%m-%dT%H:%M:%SZ)"; ls -li "$L"; stat -L -f 'target inode %i mtime %Sm' -t %Y-%m-%dT%H:%M:%S "$L"; codesign -dvvv "$L" 2>&1 | grep -E "^Identifier|^CDHash"; } >> "$ids" 2>&1; }
while [ "$k" -le "$last" ]; do
  suffix=""; [ "$k" -gt 1 ] && suffix="-try-$k"
  p12="$M/round-$round$suffix.p12-samples.txt"
  identity "before try $k"
  bash "$M/p12-sampler.sh" "$p12" & sampler=$!
  started="$(date +%s)"
  bash "$M/run-round.sh" "$round" "$k" > /dev/null 2>&1
  touch "$p12.stop"; wait "$sampler" 2>/dev/null
  identity "after try $k"
  log="$HOME/native-bootstrap-round-$round$suffix.log"
  clean="$(sed $'s/\x1b\\[[0-9;]*m//g' "$log" | tr '\r' '\n')"
  rc="$(echo "$clean" | grep -a '^native_rc=' | tail -1 | cut -d= -f2 | cut -d' ' -f1)"
  stop="$(grep -c 'HARD STOP' "$M/round-$round$suffix.monitor.txt")"
  moving="$(echo "$clean" | grep -aciE 'source-differs|closure\.moving|closure-missing')"
  missing=0; for l in "$TMPDIR"/semio-repo/"$(id -u)"/host-*.log; do [ "$(stat -f %m "$l")" -ge "$started" ] && missing=$((missing + $(grep -ac 'closure-missing' "$l"))); done
  sig="$(echo "$clean" | grep -av '^\[script\] {' | grep -aiE 'unexpected failure|cannot continue|could not|error' | tail -1 | sed -E 's/\(reference [0-9a-f]+\)//; s/[0-9]+/N/g' | cut -c1-200)"
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=$rc hard_stop=$stop moving_lines=$moving closure_missing_lines=$missing sig=[$sig]" >> "$out"
  printf '{"origin":"g1-m07","round":%s,"native_rc":%s,"ready":false,"failed":["workspace:setup"],"try":%s,"at":"%s"}\n' "$round" "${rc:-1}" "$k" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$M/state.json"
  [ "$rc" = "0" ] && break; [ "$rc" = "4" ] && break
  if [ "$stop" = "0" ] && [ "$rc" != "75" ] && [ ! -e "$M/round-$round$suffix.store-wait.txt" ] && [ "$moving" = "0" ] && [ "$missing" = "0" ]; then
    [ -n "$prev" ] && [ "$sig" = "$prev" ] && { echo "two tries in a row with the same other error" >> "$out"; break; }
    prev="$sig"
  else prev=""; fi
  k=$((k + 1)); [ "$k" -le "$last" ] && sleep 180
done
echo "driver ended $(date -u +%Y-%m-%dT%H:%M:%SZ) at try $k" >> "$out"
