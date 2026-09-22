#!/bin/zsh
# 🧱️ Slice S10 — ONE mutex hold that rebuilds the guests this slice's root fixes must reach.
#
# ⚠️ The first version of this script called `bun ./📜️script.ts activate s react dev` directly. That
# verb only PUBLISHES already-staged components — it computes a receipt over what is on disk and
# never builds — so it answered `60 completed components (unchanged)` in seconds and the guests still
# carried the old code (measured 2026-09-21 11:56, and confirmed by the sweep reading the identical
# `edits [0,0,0,0]` afterwards). Worse, its staleness view is keyed on `✏️s/🔌️plugins/**`, so a change
# in `semio-framework-plugin` — which every guest LINKS — marks nothing stale at all.
#
# So each component is built through cargo, whose own dependency tracking does see the framework
# crate, then materialized, then published by the activate verb. Every wasm32 step is wrapped in the
# fleet mutex (preamble rule 27a) so a detached chain holds it per step rather than for the hour.
set -u
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
MUTEX="$PWD/$TICKET/📜️wasm-build-mutex.sh"
OUT="$PWD/$TICKET/🗑️generated"
CARGO_SCRIPT="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export NX_DAEMON=false
export CARGO_INCREMENTAL=0
# 🎯️ The six guests this slice's four root fixes must reach: 🪐️space (createStudio's footprint),
# 📖️playbook + its 🌀️procedural extension (the new Migrated tool factory), 🌊️flow / 🎬️sequence /
# 🌀️procedural (the `applied` lanes) and 📕️norm (S9's 14 bridged editors). Every one of them also
# picks up the framework `applied` change by linking the rebuilt crate.
PLUGINS=(🪐️space 📖️playbook 📕️norm)
PROJECTS=(space-plugin playbook-plugin norm-plugin)
for i in {1..3}; do
  plugin="${PLUGINS[$i]}"; project="${PROJECTS[$i]}"
  echo "[s10-restage] ($i/3) component dev $plugin  $(date '+%F %T')"
  zsh "$MUTEX" s10 -- bun "$CARGO_SCRIPT" native component dev \
    --manifest "✏️s/🔌️plugins/$plugin/📦️packages/🦀️rust/Cargo.toml" > "$OUT/s10-component-$project.txt" 2>&1
  rc=$?
  echo "[s10-restage] ($i/3) component rc=$rc $(date '+%F %T')"
  # 🛑️ Stop on the FIRST component failure. The previous run let all six fail (`rc=1`, then
  # `materialize rc=130`) and still ended with `activate rc=0 … DONE`, because the activate verb only
  # publishes what is on disk and cannot fail on a missing build — a false green that read as success.
  # A rebuild that did not rebuild must say so in its last line.
  if [ "$rc" -ne 0 ]; then
    echo "[s10-restage] ABORT: $plugin component build failed — see 🗑️generated/s10-component-$project.txt"
    echo "[s10-restage] FAILED $(date '+%F %T')"
    exit 1
  fi
  echo "[s10-restage] ($i/3) materialize $project  $(date '+%F %T')"
  zsh "$MUTEX" s10 -- bun nx run "@semio-tech/$project:materialize-dev" > "$OUT/s10-materialize-$project.txt" 2>&1
  rc=$?
  echo "[s10-restage] ($i/3) materialize rc=$rc $(date '+%F %T')"
  if [ "$rc" -ne 0 ]; then
    echo "[s10-restage] ABORT: $project materialize failed — see 🗑️generated/s10-materialize-$project.txt"
    echo "[s10-restage] FAILED $(date '+%F %T')"
    exit 1
  fi
done
echo "[s10-restage] activate  $(date '+%F %T')"
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" \
  && zsh "$MUTEX" s10 -- bun ./📜️script.ts activate s react dev ) > "$OUT/s10-activate.txt" 2>&1
echo "[s10-restage] activate rc=$? $(date '+%F %T')"
echo "[s10-restage] DONE $(date '+%F %T')"
