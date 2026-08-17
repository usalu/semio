#!/usr/bin/env bash
set -euo pipefail
cd /Users/ueli/Documents/semio
TICKET=$(find .🦑️repo/🎫️tickets -type d -name 'FRAMEWORK-UI-FAMILY-CRATE-CONSOLIDATION' | head -1)
WS="$TICKET/🧪️members-workspace"
DEVELOPER_DIR=/Library/Developer/CommandLineTools \
  CARGO_TARGET_DIR="$PWD/$TICKET/🧪️members-workspace-target" \
  cargo "$@" --manifest-path "$WS/Cargo.toml"
