#!/usr/bin/env python3
"""⏳ Re-await the RUNTIME harness across the remodel editor/viewer test modules.

`ArtifactEditor` (AUTHORING) is sync — W1 de-asynced it. `ArtifactApp`/`VcsArtifactApp` (RUNTIME) are
NOT: `new_app`, `new_app_with_registry`, `dispatch_typed` and `render` are all async, so every
`testkit::{app, app_with_registry, dispatch, render}` call site inside a `#[…async_test]` body needs
`.await` back. Brace/paren-matched, never regex-over-nesting: the call's closing paren is found by
counting, so a nested `dispatch(&mut app, Cmd(Payload { .. }))` is handled correctly."""
import os, re, sys

ROOT = "/Users/ueli/Documents/semio"
SUB = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
BASE = os.path.join(ROOT, SUB)
TARGET_DIRS = [os.path.join(BASE, d) for d in ("✏️editor", "👁️viewer")]

# name -> takes arguments?
CALLS = {"app": False, "app_with_registry": False, "dispatch": True, "render": True, "render_body": True}
APPLY = "--apply" in sys.argv

def close_paren(text, open_index):
    depth = 0
    i = open_index
    while i < len(text):
        c = text[i]
        if c == '"':
            i += 1
            while i < len(text) and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1

total = 0
touched = []
for d in TARGET_DIRS:
    for dp, _dns, fns in os.walk(d):
        for f in fns:
            if not f.endswith(".rs"):
                continue
            path = os.path.join(dp, f)
            src = open(path, encoding="utf-8").read()
            out = src
            n = 0
            for name in CALLS:
                pattern = re.compile(r"(?<![A-Za-z0-9_:.])" + name + r"\(")
                pos = 0
                while True:
                    m = pattern.search(out, pos)
                    if not m:
                        break
                    start = m.start()
                    end = close_paren(out, m.end() - 1)
                    if end < 0:
                        pos = m.end()
                        continue
                    # skip definitions and already-awaited calls
                    line_start = out.rfind("\n", 0, start) + 1
                    line = out[line_start:start]
                    after = out[end + 1 : end + 7]
                    if "fn " in line or after.startswith(".await"):
                        pos = end + 1
                        continue
                    out = out[: end + 1] + ".await" + out[end + 1 :]
                    n += 1
                    pos = end + 7
            if n:
                touched.append((os.path.relpath(path, BASE), n))
                total += n
                if APPLY:
                    open(path, "w", encoding="utf-8").write(out)

for rel, n in sorted(touched):
    print(f"{n:4d}  {rel}")
print(f"# files: {len(touched)}  awaits added: {total}  applied={APPLY}")
