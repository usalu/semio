#!/usr/bin/env bash
# 🧪️ Pre-registrar cargo runner for merged semio-framework-ui.
# Always restores from ticket Cargo.toml.final on EXIT so a crashed run cannot leave
# a nested [workspace] overlay on disk.
set -euo pipefail
cd /Users/ueli/Documents/semio
TICKET=$(find .🦑️repo/🎫️tickets -type d -name 'FRAMEWORK-UI-FAMILY-CRATE-CONSOLIDATION' | head -1)
MANIFEST='🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml'
FINAL="$TICKET/Cargo.toml.final"
cp "$FINAL" "$MANIFEST"
trap 'cp "$FINAL" "$MANIFEST"; rm -f "${MANIFEST%/*}/Cargo.lock"' EXIT

if [[ "${UI_KEEP_3D:-0}" != "1" ]]; then
  python3 - <<'PY'
from pathlib import Path
p = Path('🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml')
text = p.read_text()
text = text.replace(
    '    "dep:kernel_3d_scene",\n',
    '    # "dep:kernel_3d_scene",  # stripped pre-registrar\n',
)
old = 'kernel_3d_scene = { path = "../../../../../✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust", package = "semio-s-3d", optional = true }\n'
if old not in text:
    raise SystemExit('kernel_3d_scene dep line not found — refuse to run')
text = text.replace(old, '# kernel_3d_scene stripped pre-registrar (3d→core→ui-wgpu cycle until registrar)\n')
p.write_text(text)
PY
fi

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools \
  CARGO_TARGET_DIR="$PWD/$TICKET/🧪️target" \
  cargo "$@" --manifest-path "$MANIFEST"
