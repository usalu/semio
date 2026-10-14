#!/bin/bash
# Translates the fleet usage `🚦️cargo-slot.sh <agent> -- cargo <args>` into the issued repository Cargo launcher:
#   bun repo cargo --store=<shared|isolated> --configuration=<prototype|release> -- <args>
# The launcher's own host-capacity gate, jobserver and memory broker share the machine, so the former 6-slot mkdir gate is gone.
# Store: isolated when the caller set CARGO_TARGET_DIR (a private target), else the shared fixed store.
# Configuration: SEMIO_BUILD_MODE when set, else release for --release / --profile (wasm-)release, else prototype.
# A relative --manifest-path is resolved against the caller's directory; the launcher runs from the repository root.
# Ticket 26/09/23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT, 📓️2026-10-14-k0-issued-cargo-launcher.md.
set -euo pipefail
AGENT="${1:?usage: 🚦️cargo-slot.sh <agent> -- cargo <args>}"; shift
[ "${1:-}" = "--" ] && shift
[ "${1:-}" = "cargo" ] && shift
ROOT="/Users/ueli/Documents/semio"
STORE="shared"; [ -n "${CARGO_TARGET_DIR:-}" ] && STORE="isolated"
CONFIGURATION="${SEMIO_BUILD_MODE:-}"
ARGS=(); PREVIOUS=""
for ARG in "$@"; do
  if [ "$PREVIOUS" = "--manifest-path" ] && [ "${ARG#/}" = "$ARG" ]; then ARG="$PWD/$ARG"; fi
  case "$ARG" in --manifest-path=*) VALUE="${ARG#--manifest-path=}"; [ "${VALUE#/}" = "$VALUE" ] && ARG="--manifest-path=$PWD/$VALUE";; esac
  if [ -z "${SEMIO_BUILD_MODE:-}" ]; then
    case "$ARG" in --release|--profile=release|--profile=wasm-release) CONFIGURATION="release";; esac
    if [ "$PREVIOUS" = "--profile" ] && { [ "$ARG" = "release" ] || [ "$ARG" = "wasm-release" ]; }; then CONFIGURATION="release"; fi
  fi
  ARGS+=("$ARG"); PREVIOUS="$ARG"
done
CONFIGURATION="${CONFIGURATION:-prototype}"
echo "[cargo-slot] $AGENT $(date +%H:%M:%S) bun repo cargo --store=$STORE --configuration=$CONFIGURATION -- ${ARGS[*]}" >&2
cd "$ROOT"
exec env -u CARGO_TARGET_DIR bun repo cargo --store="$STORE" --configuration="$CONFIGURATION" -- "${ARGS[@]}"
