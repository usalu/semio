#!/usr/bin/env python3
"""🌉️ Third fixer shape: a test calling `.unwrap()`/`.is_ok()`/`.expect(..)` straight on a future.

The diagnostic points at the *method*, not the future, so this walks the receiver chain backwards
(balanced parens/brackets, path segments, `.` hops) and wraps exactly that receiver in `block_on`.
"""
import json, re, subprocess, io, collections

IDENT = re.compile(r"[A-Za-z0-9_]")
METHODS = {"unwrap", "expect", "is_ok", "is_err", "unwrap_err", "unwrap_or", "unwrap_or_default", "ok", "err", "map"}

def receiver_start(row, dot):
    pos = dot
    while True:
        j = pos - 1
        while j >= 0 and row[j] == " ": j -= 1
        if j < 0: return pos
        if row[j] in ")]":
            close, open_ = row[j], "(" if row[j] == ")" else "["
            depth = 0
            while j >= 0:
                if row[j] == close: depth += 1
                elif row[j] == open_:
                    depth -= 1
                    if depth == 0: break
                j -= 1
            pos = j
            continue
        if IDENT.match(row[j]) or row[j] == ":":
            while j >= 0 and (IDENT.match(row[j]) or row[j] == ":"): j -= 1
            pos = j + 1
            if j >= 0 and row[j] == ".": pos = j; continue
            return pos
        return pos

def diags():
    out = subprocess.run(["cargo", "check", "-p", "semio-framework-os-kernel-db", "--all-targets",
                          "--message-format=json"], capture_output=True, text=True).stdout
    for line in out.splitlines():
        try: m = json.loads(line)
        except Exception: continue
        d = m.get("message")
        if d and d.get("level") == "error": yield d

def run():
    hits = collections.defaultdict(set)
    for d in diags():
        if (d.get("code") or {}).get("code") != "E0599": continue
        msg = d.get("message", "")
        if "Future<Output" not in msg: continue
        m = re.search(r"no method named `([a-z_]+)`", msg)
        if not m or m.group(1) not in METHODS: continue
        prim = [s for s in d.get("spans", []) if s.get("is_primary")]
        if not prim: continue
        s = prim[0]
        hits[s["file_name"]].add((s["line_start"], s["column_start"]))
    n = 0
    for path, items in hits.items():
        prefix = "db::actor::block_on" if "⌨️cli" in path else "db_actor::block_on"
        lines = io.open(path, encoding="utf-8").read().split("\n")
        for (ln, col) in sorted(items, key=lambda t: (t[0], -t[1])):
            row = lines[ln - 1]
            dot = col - 2                       # column_start is the method name; the `.` precedes it
            while dot > 0 and row[dot] != ".": dot -= 1
            start = receiver_start(row, dot)
            if row[start:start + len(prefix)] == prefix: continue
            lines[ln - 1] = row[:start] + prefix + "(" + row[start:dot] + ")" + row[dot:]
            n += 1
        io.open(path, "w", encoding="utf-8").write("\n".join(lines))
    return n

for r in range(1, 25):
    n = run(); print(f"round {r}: {n} receiver wraps")
    if n == 0: break
