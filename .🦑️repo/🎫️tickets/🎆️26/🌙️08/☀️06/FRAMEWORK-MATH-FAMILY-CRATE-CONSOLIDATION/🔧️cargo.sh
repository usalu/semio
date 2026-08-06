#!/usr/bin/env bash
# 🧪️ Pre-registrar cargo runner: temporarily makes the consolidated math crate its own workspace
# root (mirroring the repo's workspace lints) so it can be checked before it lands in root Cargo.toml.
set -euo pipefail
cd /Users/ueli/Documents/semio
TICKET='.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/FRAMEWORK-MATH-FAMILY-CRATE-CONSOLIDATION'
MANIFEST='🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/Cargo.toml'
cp "$MANIFEST" "$TICKET/🧪️Cargo.toml.bak"
trap 'cp "$TICKET/🧪️Cargo.toml.bak" "$MANIFEST"' EXIT
cat >> "$MANIFEST" <<'TOML'

[workspace]
members = ["."]

[workspace.lints.rust]
future_incompatible = { level = "warn", priority = -1 }
rust_2018_idioms = { level = "warn", priority = -1 }
unsafe_op_in_unsafe_fn = "warn"
unused_lifetimes = "warn"
unused_qualifications = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
cloned_instead_of_copied = "warn"
inefficient_to_string = "warn"
map_unwrap_or = "warn"
needless_pass_by_value = "warn"
semicolon_if_nothing_returned = "warn"
unnecessary_wraps = "warn"
redundant_clone = "warn"
TOML
DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_TARGET_DIR="$PWD/$TICKET/🧪️target" cargo "$@" --manifest-path "$MANIFEST"
