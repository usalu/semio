#!/usr/bin/env bash
# 🧾️ Runs every unit suite of the course readiness check one after another and prints one result line per suite.
set -u
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS"
O="$T/🗑️generated/final"; mkdir -p "$O"
export SEMIO_LANGUAGE=en PATH="$HOME/.local/share/semio/node/v24.15.0/bin:$HOME/.cargo/bin:$PATH"
suite() { local name=$1 dir=$2 cfg=$3; (cd "$R/$dir" && repo tools exec vitest -- run --configLoader native --config "$cfg") >"$O/$name.txt" 2>&1; echo "$name exit $? $(grep -E 'Tests +[0-9]' "$O/$name.txt" | tail -1)"; }
suite course-core "🧰️framework/🛍️products/📚️course" "🧪️tests/🎚️config/🟦️.ts"
suite course-react "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react" "🧪️tests/🎚️config/🟦️.ts"
suite course-site "🎓️teaching/🏛️architecture/📚️course" "🧪️tests/🎚️config/🟦️.ts"
suite pets-core "🧰️framework/🛍️products/🐾️pets" "🧪️tests/🎚️config/🟦️.ts"
suite pets-react "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react" "🧪️tests/🎚️config/🟦️.ts"
suite ui-react "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript" "$T/perf2-ui-vitest.config.ts"
(cd "$R" && repo cargo --store=shared --configuration=prototype -- test --manifest-path 🎓️teaching/Cargo.toml --workspace) >"$O/proctor.txt" 2>&1
echo "proctor exit $? passed $(grep -oE 'ok\. [0-9]+ passed' "$O/proctor.txt" | awk '{s+=$2} END {print s}') failed $(grep -oE '[0-9]+ failed' "$O/proctor.txt" | awk '{s+=$1} END {print s}')"
(cd "$T" && repo tools exec bun -- test ./fresh-proctor.test.mjs) >"$O/fresh.txt" 2>&1; echo "fresh-proctor exit $?"
