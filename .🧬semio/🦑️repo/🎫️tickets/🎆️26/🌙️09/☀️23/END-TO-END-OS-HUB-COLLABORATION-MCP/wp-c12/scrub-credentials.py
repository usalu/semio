"""🧽️ C12: scrubs hub credentials that older collab-e2e runs logged (sign-in POST bodies with passwords, bearer session
capabilities) from text captures IN PLACE; the logs themselves stay. usage: python3 scrub-credentials.py [--write] <dir>…"""
import os, re, sys
PATTERNS = [
    (re.compile(rb'body=\{"schema":"semio\.hub\.auth[^\n]*? \xe2\x80\x94 '), b'body=<redacted> \xe2\x80\x94 '),
    (re.compile(rb'"password"\s*:\s*"[^"]*"'), b'"password":"<redacted>"'),
    (re.compile(rb'session\.v1\.[0-9a-f]{16,}(?:\.[0-9a-f]{16,})?'), b'session.v1.<redacted>'),
]
write = "--write" in sys.argv
roots = [arg for arg in sys.argv[1:] if arg != "--write"]
total = 0
for root in roots:
    for base, _, files in os.walk(root):
        for name in files:
            if not name.endswith((".txt", ".json", ".log", ".jsonl", ".md")):
                continue
            path = os.path.join(base, name)
            with open(path, "rb") as handle:
                data = handle.read()
            hits = 0
            for pattern, replacement in PATTERNS:
                data, count = pattern.subn(replacement, data)
                hits += count
            if hits:
                total += hits
                print(f"{hits:6d} {path}")
                if write:
                    with open(path, "wb") as handle:
                        handle.write(data)
print(f"{'scrubbed' if write else 'would scrub'} {total} occurrence(s)")
