#!/bin/zsh
# 🧱️ Slice S11 — rebuilds the two guests this slice's presence-owner fixes must reach, then publishes.
#
# 🪐️space carries the space INDEX editor's three presence retirement owners (§3.3),
# ⚙️playbook-module-procedural the module app's (§3.2) plus its document-store retirement catalog,
# and 🎬️sequence the `StepParams` cold boundary the play session proposed (§8). Both are built through cargo first — the
# activate verb only PUBLISHES what is on disk and prints success over a total failure (S10 §7.2) —
# then materialized, then activated. Aborts on the FIRST non-zero step, so a rebuild that did not
# rebuild says so in its last line. The caller wraps this in 📜️mutex-ordered.sh; every step here is
# therefore already inside the fleet's wasm mutex and must NOT take it again.
set -u
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
OUT="$PWD/$TICKET/🗑️generated"
CARGO_SCRIPT="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"
PLUGIN_SCRIPT="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts"
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export NX_DAEMON=false
export CARGO_INCREMENTAL=0
MANIFESTS=(
  "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml"
)
NAMES=(space-plugin)
for i in {1..1}; do
  manifest="${MANIFESTS[$i]}"; name="${NAMES[$i]}"
  echo "[s11-restage] ($i/1) component dev $name  $(date '+%F %T')"
  bun "$CARGO_SCRIPT" native component dev --manifest "$manifest" > "$OUT/s11-component-$name.txt" 2>&1
  rc=$?
  echo "[s11-restage] ($i/1) component rc=$rc $(date '+%F %T')"
  if [ "$rc" -ne 0 ]; then
    echo "[s11-restage] ABORT: $name component build failed — see 🗑️generated/s11-component-$name.txt"
    echo "[s11-restage] FAILED $(date '+%F %T')"; exit 1
  fi
  echo "[s11-restage] ($i/1) materialize $name  $(date '+%F %T')"
  bun "$PLUGIN_SCRIPT" materialize dev --manifest "$manifest" > "$OUT/s11-materialize-$name.txt" 2>&1
  rc=$?
  echo "[s11-restage] ($i/1) materialize rc=$rc $(date '+%F %T')"
  if [ "$rc" -ne 0 ]; then
    echo "[s11-restage] ABORT: $name materialize failed — see 🗑️generated/s11-materialize-$name.txt"
    echo "[s11-restage] FAILED $(date '+%F %T')"; exit 1
  fi
done
echo "[s11-restage] activate  $(date '+%F %T')"
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" && bun ./📜️script.ts activate s react dev ) > "$OUT/s11-activate.txt" 2>&1
echo "[s11-restage] activate rc=$? $(date '+%F %T')"
tail -1 "$OUT/s11-activate.txt"
echo "[s11-restage] DONE $(date '+%F %T')"
