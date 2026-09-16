#!/usr/bin/env bash
set -euo pipefail
repo="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$repo"
if [[ "$(uname -s)" == Darwin ]] && ! xcrun --sdk macosx --show-sdk-path >/dev/null 2>&1; then
  clt_root="/Library/Developer/CommandLineTools"
  clt_sdk="${clt_root}/SDKs/MacOSX.sdk"
  if [[ -d "$clt_sdk" ]] && DEVELOPER_DIR="$clt_root" SDKROOT="$clt_sdk" xcrun --sdk macosx --show-sdk-path >/dev/null 2>&1; then
    export DEVELOPER_DIR="$clt_root"
    export SDKROOT="$clt_sdk"
    echo "Using Command Line Tools SDK (Xcode.app license not accepted): $SDKROOT" >&2
  else
    echo "Xcode license required (cc exit 69 on link):" >&2
    echo "  sudo xcodebuild -license" >&2
    echo "Or install Command Line Tools, or run inside the devcontainer after Docker Desktop is up:" >&2
    echo "  docker compose -f .devcontainer/docker-compose.yml run --rm compose bash -lc 'cd /workspaces/semio && .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/GIS-2D-END-TO-END-BUILD/📜️verify-gis2d-envelope-gates.sh'" >&2
    exit 69
  fi
fi
cargo test -p semio-framework-plugin --features artifact-app-testing advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault -- --test-threads=1
cargo test -p semio-s-artifact-gis-gismap --features component-app-assembly gis_map_live_envelope_submit -- --test-threads=1
cargo test -p semio-s-artifact-gis-gismap --features component-app-assembly --lib -- --test-threads=1
bun nx run @semio-tech/framework-os-dev:activate-gis2d-wgpu-dev
cargo test -p semio-framework-plugin --features artifact-app-testing one_reactor_turn_pumps_the_envelope_decode_worker -- --test-threads=1
