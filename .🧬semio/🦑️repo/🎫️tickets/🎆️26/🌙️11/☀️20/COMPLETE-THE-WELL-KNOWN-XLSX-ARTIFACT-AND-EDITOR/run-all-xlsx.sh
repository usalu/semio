#!/bin/zsh
cd "$(dirname "$0")/../../../../../../.." || exit 1
export SEMIO_LANGUAGE=en
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/🗑️generated/claude-suites"
X="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.toml"
mkdir -p "$G"
repo cargo --store=shared --configuration=prototype -- test --manifest-path "$X" -p semio-s-artifact-stdio-xlsx-base -p semio-s-artifact-stdio-xlsx-base-mutations -p semio-s-artifact-stdio-xlsx-strict -p semio-s-artifact-stdio-xlsx-strict-mutations -p semio-s-artifact-stdio-xlsx-transitional --lib --no-fail-fast > "$G/all-rust.log" 2>&1
echo "rust-libs exit $?" > "$G/all-done"
repo cargo --store=shared --configuration=prototype -- test --manifest-path "$X" -p semio-s-artifact-stdio-xlsx --test base_io_laws --no-fail-fast > "$G/all-io-laws.log" 2>&1
echo "base-io-laws exit $?" >> "$G/all-done"
repo run @semio-tech/s-dev-composition:check-stdio-format-claims > "$G/all-claims.log" 2>&1
echo "claims exit $?" >> "$G/all-done"
