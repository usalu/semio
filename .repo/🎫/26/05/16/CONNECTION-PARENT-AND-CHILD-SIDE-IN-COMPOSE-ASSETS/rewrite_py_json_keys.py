from pathlib import Path
import os
import tempfile

P = Path(r"c:\git\compose\compose\client\lib\py\main.py")
t = P.read_text(encoding="utf-8")
pairs = [
    ('.get("connected"', '.get("parent"'),
    ('.get("connecting"', '.get("child"'),
    ('setdefault("connected"', 'setdefault("parent"'),
    ('setdefault("connecting"', 'setdefault("child"'),
    ('"connected": dict', '"parent": dict'),
    ('"connecting": dict', '"child": dict'),
    ('"connected",', '"parent",'),
    ('"connecting",', '"child",'),
    ('["connecting", "connected"]', '["child", "parent"]'),
    ('["connected", "connecting"]', '["parent", "child"]'),
    ('for side in ["connected", "connecting"]', 'for side in ["parent", "child"]'),
    ('if "connected" not in d or "connecting" not in d', 'if "parent" not in d or "child" not in d'),
    ('w.writeString("connected")', 'w.writeString("parent")'),
    ('_write_diff_hash(w, "connected",', '_write_diff_hash(w, "parent",'),
]
for a, b in pairs:
    t = t.replace(a, b)
fd, tmp = tempfile.mkstemp(suffix=".py")
try:
    with open(fd, "w", encoding="utf-8", newline="\n") as f:
        f.write(t)
    os.replace(tmp, P)
except Exception:
    Path(tmp).unlink(missing_ok=True)
    raise
