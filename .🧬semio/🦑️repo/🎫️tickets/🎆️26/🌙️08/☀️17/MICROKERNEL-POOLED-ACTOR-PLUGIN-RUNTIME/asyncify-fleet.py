#!/usr/bin/env python3
"""🌊️ Total async conversion of the plugin fleet — every `fn` becomes an `async fn`.

Owner directive (2026-08-19): "Every single function must have async keyword and be implemented
with async, doesn't matter if it breaks the code." This is the mechanical half; `.await`
propagation at call sites is driven afterwards off compiler diagnostics.

Language facts this has to respect to keep files PARSEABLE (an unparseable file cannot be fixed
by any later pass, so these are not exceptions to the directive — they are the only way to honour it):
  * `const fn` cannot also be `async` -> the `const` is dropped, fn becomes `async`.
  * `extern "abi" fn` cannot be `async` -> the extern ABI is dropped, fn becomes `async`.
Both are counted and reported, never silently changed.
"""
import os, re, sys, json, time

ROOT = "/Users/ueli/Documents/semio"
FN = re.compile(
    r'^(?P<indent>\s*)'
    r'(?P<vis>(?:pub(?:\([^)]*\))?\s+)?)'
    r'(?P<default>(?:default\s+)?)'
    r'(?P<const>(?:const\s+)?)'
    r'(?P<async>(?:async\s+)?)'
    r'(?P<unsafe>(?:unsafe\s+)?)'
    r'(?P<extern>(?:extern\s+"[^"]*"\s+)?)'
    r'fn\s'
)

def convert(path, stats):
    try:
        src = open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError):
        stats["unreadable"] += 1
        return False
    out, changed = [], False
    for line in src.split("\n"):
        m = FN.match(line)
        if not m or m.group("async").strip():
            out.append(line)
            continue
        if m.group("const").strip():
            stats["const_fn_demoted"] += 1
        if m.group("extern").strip():
            stats["extern_fn_demoted"] += 1
        head = m.group("indent") + m.group("vis") + m.group("default") + "async " + m.group("unsafe")
        out.append(head + line[m.end():] if False else head + "fn " + line[m.end():])
        changed = True
        stats["converted"] += 1
    if changed:
        open(path, "w", encoding="utf-8").write("\n".join(out))
        stats["files_changed"] += 1
    return changed

def walk(target, skip_recent_s, stats):
    now = time.time()
    for dp, dn, fs in os.walk(target):
        dn[:] = [d for d in dn if "target" not in d and d != "node_modules"]
        for f in fs:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(dp, f)
            try:
                if skip_recent_s and now - os.path.getmtime(p) < skip_recent_s:
                    stats["skipped_live"] += 1
                    stats["skipped_paths"].append(p)
                    continue
            except OSError:
                continue
            stats["files_seen"] += 1
            convert(p, stats)

if __name__ == "__main__":
    target = sys.argv[1] if len(sys.argv) > 1 else os.path.join(ROOT, "✏️s", "🔌️plugins")
    skip = float(sys.argv[2]) if len(sys.argv) > 2 else 180.0
    stats = dict(files_seen=0, files_changed=0, converted=0, const_fn_demoted=0,
                 extern_fn_demoted=0, unreadable=0, skipped_live=0, skipped_paths=[])
    walk(target, skip, stats)
    paths = stats.pop("skipped_paths")
    print(json.dumps(stats, indent=1))
    if paths:
        print(f"skipped (modified < {skip:.0f}s ago — another session is writing them):")
        for p in paths[:20]:
            print("   ", p[len(ROOT) + 1:])
