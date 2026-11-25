#!/bin/zsh
cd "$(dirname "$0")/../../../../../../.." || exit 1
export SEMIO_LANGUAGE=en
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/🗑️generated/claude-suites"
X="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.toml"
mkdir -p "$G"
repo cargo --store=shared --configuration=prototype -- test --manifest-path "$X" -p semio-s-artifact-stdio-xlsx --test base_io_laws --test model_laws --no-fail-fast > "$G/final-io-laws.log" 2>&1
echo "io+model laws exit $?" > "$G/final-done"
repo cargo --store=shared --configuration=prototype -- test --manifest-path "$X" -p semio-s-artifact-stdio-xlsx-strict -p semio-s-artifact-stdio-xlsx-transitional --lib paged_natural_route --no-fail-fast > "$G/final-paged.log" 2>&1
echo "paged exit $?" >> "$G/final-done"
