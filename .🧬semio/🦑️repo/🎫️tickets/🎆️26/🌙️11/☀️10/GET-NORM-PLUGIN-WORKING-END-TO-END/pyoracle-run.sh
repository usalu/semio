#!/usr/bin/env bash
# 🐍️ Runs the toolchain unit test then provisions the Python environment; output stays under the generated folder.
set -u
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated"
repo tools exec bun -- test "🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🧪️tests/🟦️.ts" > "$T/tc-test.txt" 2>&1; echo "rc=$?" >> "$T/tc-test.txt"
repo run workspace:deps-python > "$T/depspython.txt" 2>&1; echo "rc=$?" >> "$T/depspython.txt"
