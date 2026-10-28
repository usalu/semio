#!/bin/sh
# 🧭 usage: journeys-mirror.sh <mirror-root> [test-binary filters…]
# Builds the dashboard PTY journeys crate inside a MIRROR with the pinned nightly (plain cargo, no repo wrapper, offline, private CARGO_HOME copy) and runs it single-threaded like the official route. SEMIO_TEST_CLI (default: the copy of an installed dashboard binary in $TMPDIR/sm/tp-bin/semio) is the binary under test; SEMIO_TEST_TIME_SCALE forces the host scale; SLOW=1 runs the binary behind slow-terminal.py (every frame shown late and apart: the deterministic stand-in for a loaded host).
M="$1"; shift; T="$(cd "$(dirname "$0")" && pwd)"
. "$T/cargo-env.sh"
R="$M/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/📦️packages/🦀️rust"
[ -f "$R/Cargo.lock" ] || cp "$T/../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.lock" "$R/Cargo.lock"
cd "$R" || exit 2
nice -n 10 cargo test --locked --offline --no-run >"${TMPDIR%/}/sm/tp-art/journeys-build.log" 2>&1 || { tail -30 "${TMPDIR%/}/sm/tp-art/journeys-build.log"; exit 3; }
EXE=$(ls -t "$M"/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/d/b/debug/build/semio-dashboard-journeys/*/out/journeys-* | grep -v '\.d$' | head -1)
mkdir -p "${TMPDIR%/}/sm/tp-art/j"
CLI="${SEMIO_TEST_CLI:-${TMPDIR%/}/sm/tp-bin/semio}"
if [ "${SLOW:-}" = 1 ]; then
  printf '#!/bin/sh\nSEMIO_SLOW_TARGET="%s" exec python3 -I "%s/slow-terminal.py" "$@"\n' "$CLI" "$T" >"${TMPDIR%/}/sm/tp-bin/semio-slow"; chmod +x "${TMPDIR%/}/sm/tp-bin/semio-slow"; CLI="${TMPDIR%/}/sm/tp-bin/semio-slow"
fi
SEMIO_TEST_CLI="$CLI" SEMIO_TEST_ARTIFACT_DIR="${TMPDIR%/}/sm/tp-art/j" exec "$EXE" --test-threads=1 "$@"
