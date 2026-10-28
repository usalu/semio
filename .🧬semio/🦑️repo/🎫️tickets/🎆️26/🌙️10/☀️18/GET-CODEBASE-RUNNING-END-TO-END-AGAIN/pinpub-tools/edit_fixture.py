"""🧫 [pinpub] replays the fixture edits of batch `pinned` onto <root>'s graph fixture (anchored; exit 1 = anchor lost): the ancestor of `exec --projects=app` is an `owners` selection now, a missing caller authority refuses when the reuse opens its Graph reader (no read), and the expansion batch has no concurrency bound."""
import json, sys
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json"
path = f"{sys.argv[1]}/{REL}"
s = open(path, encoding="utf8").read()
def swap(old, new):
    global s
    if s.count(old) != 1: sys.exit(f"anchor lost ({s.count(old)}): {old[:80]}")
    s = s.replace(old, new)
swap('''        "id": "all-to-task",
        "ancestor": [
          "exec",
          "--projects=app",
          "--",
          "bun",
          "repo",
          "run",
          "app:test"
        ],
        "request": [
          "run",
          "app:test"
        ],
        "pin": {
          "kind": "all"
        },''', '''        "id": "owners-to-task",
        "ancestor": [
          "exec",
          "--projects=app",
          "--",
          "bun",
          "repo",
          "run",
          "app:test"
        ],
        "request": [
          "run",
          "app:test"
        ],
        "pin": {
          "kind": "owners",
          "projects": [
            "app"
          ],
          "exclude": []
        },''')
start = s.find('"id": "missing-authority"'); end = s.find('"missingAuthority"', start)
if start < 0 or end < 0: sys.exit("anchor lost: missing-authority")
segment = s[start:end]
if segment.count('''"calls": [
          "requested:reuse"
        ],''') != 1: sys.exit("anchor lost: missing-authority calls")
s = s[:start] + segment.replace('''"calls": [
          "requested:reuse"
        ],''', '"calls": [],') + s[end:]
swap('''      "count": 21,
      "maximumReads": 16,
''', '''      "count": 21,
''')
json.loads(s)
open(path, "w", encoding="utf8").write(s)
print("edited fixture")
