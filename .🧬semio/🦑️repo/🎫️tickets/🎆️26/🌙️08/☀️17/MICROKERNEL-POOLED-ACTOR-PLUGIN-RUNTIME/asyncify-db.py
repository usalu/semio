#!/usr/bin/env python3
"""🌉️ Compiler-driven finisher for the atomic `db-trait-flip` packet.

Reads `cargo check --message-format=json`, and for the pure-logic db modules only:
  * `E0277` "`?` cannot be applied to `Pin<Box<dyn Future…>>`"  -> insert `.await` before the `?`
  * `E0308` "expected `Result<…>`, found `Pin<Box<…>>`"          -> append `.await` to the tail expr
  * `E0728` "`await` outside of an `async` fn"                   -> make the enclosing `fn` `async`
Runs to a fixpoint. Thread-owning modules (artifact/engine/cli) are excluded by design — they
bridge with `db_actor::block_on` instead, so their threads keep their current shape.
"""
import json, re, subprocess, sys, io, collections

ASYNC_OK = ("📸️snapshot", "📝️wal", "🔢️index", "🗜️compact", "🔄️sync", "🌐️cluster", "📽️projection", "🔍️query", "🧪️testkit")
PKG = ["cargo", "check", "-p", "semio-framework-os-kernel-db", "--message-format=json"] + sys.argv[1:]

def diagnostics():
    out = subprocess.run(PKG, capture_output=True, text=True).stdout
    for line in out.splitlines():
        try: m = json.loads(line)
        except Exception: continue
        d = m.get("message")
        if d and d.get("level") == "error":
            yield d

def collect():
    awaits = collections.defaultdict(list)   # file -> [(line, col, kind)]
    asyncs = collections.defaultdict(set)    # file -> {line}
    for d in diagnostics():
        code = (d.get("code") or {}).get("code")
        msg = d.get("message", "")
        prim = [s for s in d.get("spans", []) if s.get("is_primary")]
        if not prim: continue
        s = prim[0]
        if not any(k in s["file_name"] for k in ASYNC_OK): continue
        label = s.get("label") or ""
        if code == "E0277" and "`?` operator" in msg and ("Future<Output" in label or "future" in label):
            awaits[s["file_name"]].append((s["line_start"], s["column_end"] - 1, "before-?"))
        elif code == "E0308" and ("found `Pin<Box<" in label or label.endswith("found future") or "found `impl Future" in label):
            awaits[s["file_name"]].append((s["line_start"], s["column_end"], "tail"))
        elif code == "E0728":
            asyncs[s["file_name"]].add(s["line_start"])
    return awaits, asyncs

def apply(awaits, asyncs):
    changed = 0
    for path in set(list(awaits) + list(asyncs)):
        lines = io.open(path, encoding="utf-8").read().split("\n")
        for (ln, col, kind) in sorted(set(awaits.get(path, [])), reverse=True):
            row = lines[ln - 1]
            i = col - 1
            if kind == "before-?":
                if i >= len(row) or row[i] != "?":
                    print(f"  ⚠️ skip {path}:{ln}:{col} (expected `?`, saw {row[i:i+1]!r})"); continue
                lines[ln - 1] = row[:i] + ".await" + row[i:]
            else:
                if row[i - 1:i + 1] == ".a" or row[max(0,i-6):i] == ".await": continue
                lines[ln - 1] = row[:i] + ".await" + row[i:]
            changed += 1
        targets = set()
        for ln in asyncs.get(path, ()):
            for probe in range(ln, 0, -1):
                if re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s", lines[probe - 1]):
                    targets.add(probe); break
        for ln in sorted(targets, reverse=True):
            row = lines[ln - 1]
            m = re.match(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?)(fn\s)", row)
            if not m:
                continue
            if "async" in row[:m.end(2)]: continue
            lines[ln - 1] = row[:m.end(2)] + "async " + row[m.end(2):]
            changed += 1
        io.open(path, "w", encoding="utf-8").write("\n".join(lines))
    return changed

for round_no in range(1, 26):
    a, y = collect()
    n = apply(a, y)
    print(f"round {round_no}: {sum(len(set(v)) for v in a.values())} awaits, {sum(len(v) for v in y.values())} async fns -> {n} edits")
    if n == 0:
        break
