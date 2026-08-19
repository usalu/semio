#!/usr/bin/env python3
"""🌉️ Companion to `bridge-db.py` for the shapes whose diagnostic span points at a `match` arm or an
`if let` binding rather than at the future itself: scans upward to the scrutinee and wraps that."""
import json, re, subprocess, sys, io, collections

BRIDGE = {"📄️artifact": "db_actor::block_on", "⚙️engine": "db_actor::block_on",
          "⌨️cli": "db::actor::block_on", "🧪️testkit": "db_actor::block_on"}
MATCH = re.compile(r"^(\s*(?:let\s+.*?=\s*)?match\s+)(.*?)(\s*\{\s*)$")
IFLET = re.compile(r"^(\s*(?:\}\s*)?if\s+let\s+.*?=\s+)(.*?)(\s*\{\s*)$")
EXPECT = re.compile(r"^(\s*(?:let\s+.*?=\s*)?)(.*?)(\.expect\(.*)$")

def diags(extra):
    out = subprocess.run(["cargo", "check", "-p", "semio-framework-os-kernel-db", "--message-format=json"] + extra,
                         capture_output=True, text=True).stdout
    for line in out.splitlines():
        try: m = json.loads(line)
        except Exception: continue
        d = m.get("message")
        if d and d.get("level") == "error": yield d

def run(extra):
    want = collections.defaultdict(set)
    for d in diags(extra):
        code = (d.get("code") or {}).get("code"); msg = d.get("message", "")
        prim = [s for s in d.get("spans", []) if s.get("is_primary")]
        if not prim: continue
        s = prim[0]
        prefix = next((v for k, v in BRIDGE.items() if k in s["file_name"]), None)
        if not prefix: continue
        label = s.get("label") or ""
        hit = (code == "E0308" and "found `Result" in label and "expected future" in label) \
           or (code == "E0599" and "opaque type `impl Future" in msg)
        if hit: want[(s["file_name"], prefix)].add(s["line_start"])
    n = 0
    for (path, prefix), rows in want.items():
        lines = io.open(path, encoding="utf-8").read().split("\n")
        targets = {}
        for ln in rows:
            for probe in range(ln, max(0, ln - 40), -1):
                row = lines[probe - 1]
                for rx in (MATCH, IFLET, EXPECT):
                    m = rx.match(row)
                    if m and not m.group(2).startswith(prefix):
                        targets[probe] = (rx, m); break
                else:
                    continue
                break
        for ln in sorted(targets, reverse=True):
            rx, m = targets[ln]
            lines[ln - 1] = m.group(1) + prefix + "(" + m.group(2) + ")" + m.group(3)
            n += 1
        io.open(path, "w", encoding="utf-8").write("\n".join(lines))
    return n

extra = sys.argv[1:]
for r in range(1, 15):
    n = run(extra)
    print(f"round {r}: {n} scrutinee wraps")
    if n == 0: break
