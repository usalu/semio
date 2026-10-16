import subprocess, os, tempfile, sys
def check(ws, paths):
    bad = []
    for p in paths:
        src = ws.read(p)
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true,unstable_features=true'], input=src, capture_output=True, text=True)
        if r.returncode != 0:
            bad.append((p, r.stderr[:400]))
    return bad
