#!/bin/zsh
# 🧱️ Slice S13 — ONE mutex hold that rebuilds ALL SIXTY guest components of the `s` session.
#
# Why all sixty: S12 §3 moved `ArtifactApp::build_presence_{local_root,peer}_retirement_factory` from
# `None` to a generic `bounded_presence_root_retirement_factory::<Self::Presence>()`, and S13 declared
# the three editable window-KIT verbs' arguments — both live in `semio-framework-plugin`, which EVERY
# guest LINKS STATICALLY. The activate verb's staleness view is keyed on `✏️s/🔌️plugins/**`, so a
# framework change marks nothing stale and `activate` answers `60 completed components (unchanged)`
# in seconds over guests that still carry the old code (measured by S10, `📜️s10-restage4.sh` header).
# Therefore: cargo builds each component (its own dependency tracking DOES see the framework crate),
# then materialize, then ONE activate publishes the lot.
#
# The caller wraps this in 📜️mutex-ordered.sh, so every step here is already inside the fleet wasm
# mutex and must NOT take it again (preamble rule 27a).
set -u
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
OUT="/Users/ueli/Documents/semio/.tmp-ticket/wp-s14/generated"
CARGO_SCRIPT="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"
PLUGIN_SCRIPT="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts"
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export NX_DAEMON=false
export CARGO_INCREMENTAL=0
export CARGO_BUILD_JOBS=4

# 🎯️ The sixty component manifests, derived rather than transcribed: every plugin rust package under
# `✏️s/🔌️plugins` that is not an artifact package and not one of the six library packages the session
# does not list as a component (`📇️registry/🧬️contract`, `🔮️oracles`, `⚙️engine`, `🔨️modules/**`).
MANIFESTS=("${(@f)$(find ✏️s/🔌️plugins -name Cargo.toml -path '*📦️packages/🦀️rust*' \
  | grep -v '🗿️artifacts' | grep -v '📇️registry/🧬️contract' | grep -v '🔮️oracles' \
  | grep -v '⚙️engine' | grep -v '🔨️modules/' | sort)}")
COUNT=${#MANIFESTS[@]}
echo "[s14-restage] $COUNT component manifests  $(date '+%F %T')"
if [ "$COUNT" -ne 60 ]; then
  echo "[s14-restage] ABORT: expected 60 components, found $COUNT — the session roster moved, re-derive before building"
  echo "[s14-restage] FAILED $(date '+%F %T')"; exit 1
fi

i=0
for manifest in "${MANIFESTS[@]}"; do
  i=$((i + 1))
  name=$(echo "$manifest" | sed 's|✏️s/🔌️plugins/||; s|/📦️packages/🦀️rust/Cargo.toml||; s|/🧩️extensions/|-|g; s|/|-|g')
  if [ -f "$OUT/s14-component-$name.ok" ]; then
    echo "[s14-restage] ($i/$COUNT) SKIP $name  $(date '+%F %T')"
    continue
  fi
  echo "[s14-restage] ($i/$COUNT) component dev $name  $(date '+%F %T')"
  bun "$CARGO_SCRIPT" native component dev --manifest "$manifest" > "$OUT/s14-component-$name.txt" 2>&1
  rc=$?
  echo "[s14-restage] ($i/$COUNT) component rc=$rc $(date '+%F %T')"
  # 🛑️ Fail-fast: the activate verb publishes whatever is on disk and cannot fail on a missing build,
  # so a chain that keeps going after a red component ends `DONE` over stale guests (S10 §7.2).
  if [ "$rc" -ne 0 ]; then
    echo "[s14-restage] ABORT: $name component build failed — see 🗑️generated/s14-component-$name.txt"
    tail -5 "$OUT/s14-component-$name.txt"
    echo "[s14-restage] FAILED $(date '+%F %T')"; exit 1
  fi
  echo "[s14-restage] ($i/$COUNT) materialize $name  $(date '+%F %T')"
  bun "$PLUGIN_SCRIPT" materialize dev --manifest "$manifest" > "$OUT/s14-materialize-$name.txt" 2>&1
  rc=$?
  echo "[s14-restage] ($i/$COUNT) materialize rc=$rc $(date '+%F %T')"
  if [ "$rc" -eq 0 ]; then
    touch "$OUT/s14-component-$name.ok"
  fi
  if [ "$rc" -ne 0 ]; then
    echo "[s14-restage] ABORT: $name materialize failed — see 🗑️generated/s14-materialize-$name.txt"
    tail -5 "$OUT/s14-materialize-$name.txt"
    echo "[s14-restage] FAILED $(date '+%F %T')"; exit 1
  fi
done
echo "[s14-restage] activate  $(date '+%F %T')"
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" && bun ./📜️script.ts activate s react dev ) > "$OUT/s14-activate.txt" 2>&1
echo "[s14-restage] activate rc=$? $(date '+%F %T')"
tail -1 "$OUT/s14-activate.txt"
echo "[s14-restage] DONE $(date '+%F %T')"
