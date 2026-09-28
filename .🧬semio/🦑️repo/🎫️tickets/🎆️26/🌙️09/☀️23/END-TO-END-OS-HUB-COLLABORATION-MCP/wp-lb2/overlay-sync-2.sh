#!/bin/zsh
# 🪞️ LB2 session 14b: brings the scratch overlay onto the live tree (clone changed files, top up gitignored sources, prune files the
# live tree no longer has — incl. last night's applied patch files), re-adds the ticket-local `lb2-wal-ops` crate as an overlay-only
# workspace member, then applies the prepared patches p1 (arena budget), p2 (declared-arguments law), p2b (per-plugin calls).
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-overlay"
echo "SYNC $(date '+%H:%M:%S')"
python3 "$W/lb2-overlay.py" "$O" || exit 1
python3 "$W/lb2-overlay-missing.py" "$O" | tail -1
python3 "$W/lb2-overlay-prune.py" "$O" | tail -1
mkdir -p "$O/lb2-tools/wal-ops/src"
cp "$W/wal-ops/Cargo.toml" "$O/lb2-tools/wal-ops/Cargo.toml"
cp "$W/wal-ops/src/main.rs" "$O/lb2-tools/wal-ops/src/main.rs"
python3 - "$O/Cargo.toml" <<'PY'
import sys
from pathlib import Path
path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
if '"lb2-tools/wal-ops",' not in text:
    anchor = "[workspace]\nmembers = [\n"
    assert text.count(anchor) == 1, "workspace members anchor"
    path.write_text(text.replace(anchor, anchor + '    "lb2-tools/wal-ops",\n', 1), encoding="utf-8")
print("wal-ops member present")
PY
echo "APPLY $(date '+%H:%M:%S')"
python3 "$W/lb2-p1-arena-budget.py" --write --root "$O" | /usr/bin/grep -E '^(PROBLEM|root=|written)'; echo "p1 rc=$?"
python3 "$W/lb2-p2-declared-arguments.py" --write --root "$O"; echo "p2 rc=$?"
python3 "$W/lb2-p2-plugin-calls.py" --write --root "$O" | /usr/bin/grep -E '^(PROBLEM|root=|written)'; echo "p2b rc=$?"
echo "DONE $(date '+%H:%M:%S')"
