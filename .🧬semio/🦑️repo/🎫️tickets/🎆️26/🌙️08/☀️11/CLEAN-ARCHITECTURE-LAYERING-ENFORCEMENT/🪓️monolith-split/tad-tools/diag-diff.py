"""🔬️ Diffs two pos.py runs by their final rustc JSON error streams: keyed multiset (code, message, repo-relative file, span text), line numbers dropped.
usage: diag-diff.py <before-final.err> <after-final.err> [--grep REGEX] [--show N]"""
import collections, json, re, sys

def keys(path):
    out = collections.Counter()
    rows = {}
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") not in ("error", "error: internal compiler error"):
            continue
        msg = d.get("message", "")
        if msg.startswith("aborting due to") or "previous error" in msg or msg.startswith("could not compile"):
            continue
        code = (d.get("code") or {}).get("code") or ""
        prim = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        file_ = re.sub(r"^__m_[A-Za-z0-9_]+/", "", prim["file_name"]) if prim else ""
        if file_.endswith("🪆️graph/🪆️content/🦀️.rs") or file_.endswith("🪆️content/🧿️semio/🦀️.rs"):
            file_ = "<semio-content-binding>"
        text = " ".join(t.get("text", "").strip() for t in (prim or {}).get("text", []))[:160]
        k = (code, re.sub(r"__m_[A-Za-z0-9_]+", "__m_X", msg), file_, text)
        out[k] += 1
        rows.setdefault(k, (prim or {}).get("line_start", 0))
    return out, rows

def main():
    a, b = sys.argv[1], sys.argv[2]
    rx = re.compile(sys.argv[sys.argv.index("--grep") + 1]) if "--grep" in sys.argv else None
    show = int(sys.argv[sys.argv.index("--show") + 1]) if "--show" in sys.argv else 40
    ka, ra = keys(a)
    kb, rb = keys(b)
    gone = ka - kb
    new = kb - ka
    print(f"before {sum(ka.values())} errors, after {sum(kb.values())} errors; only-before {sum(gone.values())}, only-after {sum(new.values())}")
    for title, c, rows in (("ONLY BEFORE (fixed)", gone, ra), ("ONLY AFTER (new)", new, rb)):
        print("==", title)
        shown = 0
        for k, n in sorted(c.items(), key=lambda kv: (kv[0][2], kv[0][0])):
            if rx and not rx.search(" ".join(k)):
                continue
            if "__ext" in k[1] or "__ext" in k[3]:
                continue
            print(f"  x{n} {k[0]} {k[2].split('/')[-4:]!s:.80} L{rows.get(k, 0)}: {k[1][:110]} | {k[3][:90]}")
            shown += 1
            if shown >= show:
                print("  ...")
                break

main()
