#!/usr/bin/env python3
"""🧪️ The `#[cfg(test)]` half of the `db-trait-flip` finisher (rule 25: `--lib` green is not the gate).

Every db test module is a plain sync `#[test] fn`, and this crate has no async test runtime of its
own (it names no `tokio`). So test bodies bridge with the family's one sanctioned `block_on` exactly
like the thread-owning modules do — never by making the test `async`. Edits are hard-restricted to
lines at or after each file's `#[cfg(test)]` marker so production code is never touched here.
"""
import json, re, subprocess, sys, io, collections

MATCH  = re.compile(r"^(\s*(?:let\s+.*?=\s*)?match\s+)(.*?)(\s*\{\s*)$")
IFLET  = re.compile(r"^(\s*(?:\}\s*)?if\s+let\s+.*?=\s+)(.*?)(\s*\{\s*)$")
TAILM  = re.compile(r"^(\s*(?:let\s+.*?=\s*)?)(.*?)(\.(?:expect|unwrap|unwrap_err)\(.*)$")

def prefix_for(path):
    return "db::actor::block_on" if "⌨️cli" in path else "db_actor::block_on"

def test_start(path, cache={}):
    if path not in cache:
        lines = io.open(path, encoding="utf-8").read().split("\n")
        cache[path] = next((i + 1 for i, l in enumerate(lines) if l.startswith("#[cfg(test)]")), 10**9)
    return cache[path]

def diags():
    out = subprocess.run(["cargo", "check", "-p", "semio-framework-os-kernel-db",
                          "--all-targets", "--message-format=json"], capture_output=True, text=True).stdout
    for line in out.splitlines():
        try: m = json.loads(line)
        except Exception: continue
        d = m.get("message")
        if d and d.get("level") == "error": yield d

def run():
    spans = collections.defaultdict(list)   # path -> [(line, a, b)]
    scrut = collections.defaultdict(set)    # path -> {line}
    for d in diags():
        code = (d.get("code") or {}).get("code"); msg = d.get("message", "")
        prim = [s for s in d.get("spans", []) if s.get("is_primary")]
        if not prim: continue
        s = prim[0]; path = s["file_name"]
        if "🛢️db" not in path or s["line_start"] < test_start(path): continue
        label = s.get("label") or ""
        if code == "E0277" and "`?` operator" in msg and "Future<Output" in label:
            spans[path].append((s["line_start"], s["column_start"], s["column_end"] - 1))
        elif code == "E0308" and (label.endswith("found future") or "found `impl Future" in label or "found `Pin<Box<" in label):
            spans[path].append((s["line_start"], s["column_start"], s["column_end"]))
        elif code == "E0599" and "opaque type `impl Future" in msg:
            scrut[path].add(s["line_start"])
        elif code == "E0308" and "found `Result" in label and "expected future" in label:
            scrut[path].add(s["line_start"])
    n = 0
    for path in set(list(spans) + list(scrut)):
        prefix = prefix_for(path)
        lines = io.open(path, encoding="utf-8").read().split("\n")
        claimed = collections.defaultdict(list)
        for (ln, a, b) in sorted(set(spans.get(path, [])), key=lambda t: (t[0], -t[1])):
            if any(not (b <= ca or a >= cb) for (ca, cb) in claimed[ln]): continue
            claimed[ln].append((a, b))
        targets = {}
        for ln in scrut.get(path, ()):
            for probe in range(ln, max(test_start(path) - 1, ln - 40), -1):
                row = lines[probe - 1]
                for rx in (MATCH, IFLET, TAILM):
                    m = rx.match(row)
                    if m and m.group(2) and not m.group(2).startswith(prefix) and "=" not in m.group(2):
                        targets[probe] = m; break
                else: continue
                break
        for ln in sorted(set(list(claimed) + list(targets)), reverse=True):
            if ln in claimed:
                for (a, b) in sorted(claimed[ln], reverse=True):
                    row = lines[ln - 1]
                    lines[ln - 1] = row[:a - 1] + prefix + "(" + row[a - 1:b - 1] + ")" + row[b - 1:]
                    n += 1
            elif ln in targets:
                m = targets[ln]
                lines[ln - 1] = m.group(1) + prefix + "(" + m.group(2) + ")" + m.group(3)
                n += 1
        io.open(path, "w", encoding="utf-8").write("\n".join(lines))
    return n

for r in range(1, 31):
    n = run()
    print(f"round {r}: {n} test-side block_on wraps")
    if n == 0: break
