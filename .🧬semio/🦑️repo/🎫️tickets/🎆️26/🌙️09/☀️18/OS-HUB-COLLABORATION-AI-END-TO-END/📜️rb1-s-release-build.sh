#!/bin/zsh
# 📦️ RB1 §4 — `build-s-react-release`, the production distribution bundle of the `s` shell, end to
# end. G15 ranked slice 2: "the button has not been pressed". This is the nx target rather than the
# bare verb because the whole point is its dependency closure — every plugin materialized at the
# RELEASE component profile plus the assets build — which is exactly what `📜️script.ts build s react
# release` on its own would skip.
#
# ONE mutex hold for the whole closure (preamble rule 27a): the closure is wasm32 builds all the way
# down, and taking the lock per inner step would interleave with peers mid-closure.
set -u
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
# 🌐️ Baked into the bundle at Vite `define` time (`import.meta.env.VITE_S_HUB_URL`). Without it
# `hubBootstrapOriginV1()` falls back to the PAGE origin — i.e. the static server, not a hub — and
# the shell "skips identity entirely", so no sign-in is possible from the built bundle.
export S_HUB_URL="${S_HUB_URL:-http://192.168.178.70:7661}"
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
echo "===== build-s-react-release :: start $(date '+%F %T') ====="
S=$(date +%s)
zsh "$T/📜️wasm-build-mutex.sh" rb1 -- bun nx run @semio-tech/framework-os-dev:build-s-react-release
echo "exit=$? seconds=$(( $(date +%s) - S )) at $(date '+%F %T')"
OUT="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/build-s-react-release"
echo "--- $OUT"
ls -la "$OUT" 2>&1 | head -20
du -sh "$OUT" 2>&1
find "$OUT" -type f 2>/dev/null | wc -l | sed 's/^/files: /'
echo "===== RB1 S RELEASE DONE $(date '+%F %T') ====="
