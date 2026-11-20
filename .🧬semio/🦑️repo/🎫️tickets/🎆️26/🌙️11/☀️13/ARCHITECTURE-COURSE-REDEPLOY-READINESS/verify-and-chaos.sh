#!/bin/sh
# 🧯️ Runs the gateway companion suite and the proctor workspace, then repeated dev chaos runs.
T="$(cd "$(dirname "$0")" && pwd)"
R="$(cd "$T/../../../../../../.." && pwd)"
G="$T/🗑️generated/round2"
mkdir -p "$G"
export SEMIO_LANGUAGE=en PATH="$HOME/.local/share/semio/node/v24.15.0/bin:$HOME/.cargo/bin:$PATH"
cd "$R"
repo cargo --store=shared --configuration=prototype -- test --manifest-path "🧰️framework/🛍️products/🖥️server/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml" > "$G/companion.txt" 2>&1; echo "companion exit $? $(grep 'test result' "$G/companion.txt" | tail -1)" >> "$G/summary.log"
repo cargo --store=shared --configuration=prototype -- test --manifest-path 🎓️teaching/Cargo.toml --workspace > "$G/proctor.txt" 2>&1; echo "proctor exit $? $(grep -c '^test .* ok$' "$G/proctor.txt") ok $(grep -c 'FAILED' "$G/proctor.txt") failed" >> "$G/summary.log"
cd "$T" && CHAOS_RUNS="${CHAOS_RUNS:-3}" CHAOS_WORK=🗑️generated/chaos-dev2 PROCTOR_PORT=8802 SITE_PORT=6071 ./chaos-repeat.sh
echo "chaos $(cat "$T/🗑️generated/chaos-dev2/repeat.log" | tr '\n' ' ')" >> "$G/summary.log"
