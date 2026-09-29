#!/bin/zsh
# ⚖️ LW1 → C13 laws (T3 c13-p1 + c13-p2): replication fold transition laws, os app_builder_tests (viewer manifest), replication TS vitest.
R=/Users/ueli/Documents/semio
cargo test --offline --no-fail-fast -p semio-framework-replication --lib -- transition; echo "LW1-STEP replication-transition rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-os --lib -- app_builder_tests; echo "LW1-STEP os-app_builder_tests rc=$?"
cd "$R/🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript" && bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" --reporter=verbose; echo "LW1-STEP replication-vitest rc=$?"
