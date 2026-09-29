#!/bin/zsh
# 🧪️ Overlay2 proof of `u6-stdio-wire-drift.py` (run inside `o2cargo.sh`): pass 1 runs the four lib suites and lets the
# overlay-only `debug_regenerate_demo_assets` probe rewrite the zip demo assets; pass 2 (the probe removed) recompiles zip and
# proves the regenerated assets honest.
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-overlay2
P=(-p semio-s-artifact-stdio-zip -p semio-s-artifact-stdio-bmp -p semio-s-artifact-stdio-gltf -p semio-s-artifact-stdio-pdf)
echo "PASS1 $(date +%T)"
cargo test --offline --lib --no-fail-fast $P 2>&1 | tee $L/o2-pass1.txt | /usr/bin/grep -E "^test .* FAILED$|^error|^test result|DEBUG" | head -60
python3 - "$O" <<'PY'
import sys, re
from pathlib import Path
p = Path(sys.argv[1]) / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"
t = p.read_text()
start = t.index("    #[semio_framework_async_macros::async_test]\n    async fn debug_regenerate_demo_assets()")
end = t.index("    #[semio_framework_async_macros::async_test]\n    async fn fixture_honesty_law()")
p.write_text(t[:start] + t[end:]); print("probe removed")
PY
echo "PASS2 $(date +%T)"
cargo test --offline --lib --no-fail-fast $P 2>&1 | tee $L/o2-pass2.txt | /usr/bin/grep -E "^test .* FAILED$|^error|^test result" | head -60
echo "DONE $(date +%T)"
