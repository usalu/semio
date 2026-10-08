#!/usr/bin/env bash
cd /c/git/semio
P=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-v1
CARGO_TARGET_DIR=$P CARGO_BUILD_BUILD_DIR=$P/private-build cargo build -p semio-framework-repo-dashboard --bin semio --message-format=short > "$(dirname "$0")/🗑️generated/v1-build.txt" 2>&1
echo "exit $?" >> "$(dirname "$0")/🗑️generated/v1-build.txt"
