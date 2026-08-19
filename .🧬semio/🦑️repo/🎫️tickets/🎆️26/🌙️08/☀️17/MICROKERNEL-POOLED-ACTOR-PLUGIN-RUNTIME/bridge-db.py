#!/usr/bin/env python3
"""🌉️ The sync half of the `db-trait-flip` finisher.

`db_artifact`/`db_engine`/`db_cli` own threads (`ArtifactAuthority`'s actor thread, `db_engine`'s
per-submit bridge threads, the CLI's single-shot process). The handover's hard constraint is that
those threads keep their current shape, so these modules do NOT go `async` — they wrap the now-async
call in the family's one sanctioned `block_on`, which is exactly the blocking that used to live
*inside* each storage backend body. Blocking moves outward one level; no thread changes.
"""
import json, subprocess, sys, io, collections

BRIDGE = {"📄️artifact": "db_actor::block_on", "⚙️engine": "db_actor::block_on", "⌨️cli": "db::actor::block_on"}

def diagnostics(extra):
    out = subprocess.run(["cargo", "check", "-p", "semio-framework-os-kernel-db", "--message-format=json"] + extra,
                         capture_output=True, text=True).stdout
    for line in out.splitlines():
        try: m = json.loads(line)
        except Exception: continue
        d = m.get("message")
        if d and d.get("level") == "error":
            yield d

def run(extra):
    edits = collections.defaultdict(list)
    for d in diagnostics(extra):
        code = (d.get("code") or {}).get("code")
        msg = d.get("message", "")
        prim = [s for s in d.get("spans", []) if s.get("is_primary")]
        if not prim: continue
        s = prim[0]
        prefix = next((v for k, v in BRIDGE.items() if k in s["file_name"]), None)
        if not prefix: continue
        label = s.get("label") or ""
        if code == "E0277" and "`?` operator" in msg and "Future<Output" in label:
            edits[s["file_name"]].append((s["line_start"], s["column_start"], s["column_end"] - 1, prefix))
        elif code == "E0308" and (label.endswith("found future") or "found `impl Future" in label or "found `Pin<Box<" in label):
            edits[s["file_name"]].append((s["line_start"], s["column_start"], s["column_end"], prefix))
    n = 0
    for path, items in edits.items():
        lines = io.open(path, encoding="utf-8").read().split("\n")
        claimed = collections.defaultdict(list)
        for (ln, a, b, prefix) in sorted(set(items), key=lambda t: (t[0], -t[1])):
            if any(not (b <= ca or a >= cb) for (ca, cb) in claimed[ln]):
                continue
            claimed[ln].append((a, b))
        for ln in sorted(claimed, reverse=True):
            for (a, b) in sorted(claimed[ln], reverse=True):
                prefix = next(p for (l, s0, e0, p) in items if l == ln and s0 == a)
                row = lines[ln - 1]
                lines[ln - 1] = row[:a - 1] + prefix + "(" + row[a - 1:b - 1] + ")" + row[b - 1:]
                n += 1
        io.open(path, "w", encoding="utf-8").write("\n".join(lines))
    return n

extra = sys.argv[1:]
for r in range(1, 21):
    n = run(extra)
    print(f"round {r}: {n} block_on wraps")
    if n == 0: break
