"""Surgical exact-string helpers for slice L-S4a (no reformatting, count-checked)."""
import json, re, sys, os

ROOT = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"

def p(path):
    return path if path.startswith("/") else os.path.join(ROOT, path)

def read(path):
    with open(p(path), encoding="utf-8") as handle:
        return handle.read()

def write(path, text):
    with open(p(path), "w", encoding="utf-8") as handle:
        handle.write(text)

def sub(path, old, new, count=1):
    text = read(path)
    found = text.count(old)
    if found != count:
        raise SystemExit(f"EDIT REFUSED {path}: expected {count} occurrence(s), found {found}: {old[:120]!r}")
    write(path, text.replace(old, new))
    print(f"edited {path.split('/')[-2]}/{path.split('/')[-1]}: {count}x {old[:70]!r}")

def ctx(path, pattern, before=160, after=320, limit=40):
    text = read(path)
    windows = []
    for match in re.finditer(pattern, text):
        start, end = max(0, match.start() - before), min(len(text), match.end() + after)
        if windows and start <= windows[-1][1]:
            windows[-1][1] = max(windows[-1][1], end)
        else:
            windows.append([start, end])
    for start, end in windows[:limit]:
        print(f"--- L{text.count(chr(10), 0, start) + 1}-L{text.count(chr(10), 0, end) + 1}: {text[start:end]!r}")
    if len(windows) > limit:
        print(f"... {len(windows) - limit} more windows")
    if not windows:
        print(f"no match for {pattern!r} in {path.split('/')[-2]}")

def lines(path, first, last, width=600):
    for number, line in enumerate(read(path).split("\n"), 1):
        if first <= number <= last:
            print(f"{number}: {line[:width]}{' …[+' + str(len(line) - width) + ']' if len(line) > width else ''}")

def check_json(path):
    json.loads(read(path))
    print(f"json ok {path.split('/')[-2]}/{path.split('/')[-1]}")
