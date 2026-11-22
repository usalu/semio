#!/usr/bin/env bash
# 🔗️ Detached release chain: prerequisites, catalog, generators, site build, then release verify; restarts after host faults (ENFILE, killed shells) up to CHAIN_ATTEMPTS times.
cd "$(dirname "${BASH_SOURCE[0]}")" || exit 1
for chain in $(seq 1 "${CHAIN_ATTEMPTS:-8}"); do
  echo "[chain $chain] $(date)"
  if [ -f "$NATIVE_PLAY_OUT/done/site-build" ] || bash native-play.sh all; then bash native-play.sh verify && { echo "[chain exit 0] $(date)"; exit 0; }; fi
  echo "[chain retry] attempt $chain failed $(date)"; sleep 120
done
echo "[chain exit 1] $(date)"; exit 1
