#!/usr/bin/env python3
"""🧮️ H14 t6-queue row 34: the creation catalog complete by construction (MAX_KINDS = the trusted open-target ceiling,
MAX_BYTES derived from the worst-case row, hub const assert + per-row presentability, capacity laws on both twins).
Hunks live in h14-catalog-bound.json ({file, old, new}); --dry-run | --write | --revert, idempotent. Anchored on h14-creation-rule.py."""
import json, os, sys, pathlib
ROOT = pathlib.Path(os.environ.get('H14_ROOT', '/Users/ueli/Documents/semio'))
HUNKS = json.loads((pathlib.Path(__file__).parent / 'h14-catalog-bound.json').read_text())
mode = sys.argv[1] if len(sys.argv) > 1 else '--dry-run'
problems, edits, texts = [], 0, {}
for hunk in HUNKS:
    path = ROOT / hunk['file']
    if hunk.get('create'):
        present = path.exists() and path.read_text() == hunk['new']
        if mode == '--revert':
            if path.exists(): edits += 1; texts[path] = None
        elif not present:
            edits += 1; texts[path] = hunk['new']
        continue
    text = texts.get(path, path.read_text())
    old, new = (hunk['new'], hunk['old']) if mode == '--revert' else (hunk['old'], hunk['new'])
    if text.count(new) == 1 and text.count(old) == new.count(old):
        continue
    if text.count(old) != 1:
        problems.append(f"{hunk['file']}: anchor count {text.count(old)} for {old[:80]!r}"); continue
    texts[path] = text.replace(old, new); edits += 1
print(f"{mode}: {edits} edits, {len(problems)} problems"); [print('  PROBLEM', p) for p in problems]
if problems: sys.exit(1)
if mode in ('--write', '--revert'):
    for path, text in texts.items():
        if text is None: path.unlink()
        else: path.parent.mkdir(parents=True, exist_ok=True); path.write_text(text)
