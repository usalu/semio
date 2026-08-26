#!/usr/bin/env python3
"""📚️ Prints one subset's committed mutation vectors compactly: the base snapshot once, then per
vector the payload, the committed outcome, and the record-level difference between the two snapshots.
Authoring aid only."""
import json, os, sys

root = sys.argv[1]
first = None
for slug in sorted(os.listdir(root)):
    tests = os.path.join(root, slug, "🧪️tests")
    if not os.path.isdir(tests):
        continue
    for fx in sorted(os.listdir(tests)):
        p = os.path.join(tests, fx)
        def rd(name):
            q = os.path.join(p, name)
            return json.load(open(q, encoding="utf-8")) if os.path.exists(q) else None
        b, a, m, o = rd("📸️snapshot/⬅️before/🔣️component.json"), rd("📸️snapshot/➡️after/🔣️component.json"), rd("🦠️mutation/🔣️component.json"), rd("🎯️outcome/🔣️component.json")
        if first is None:
            first = b
            print("BASE:", json.dumps(b, ensure_ascii=False))
        print("###", (m or {}).get("mutation"), "|", fx, "|", json.dumps(o, ensure_ascii=False))
        print("  M:", json.dumps(m, ensure_ascii=False)[:600])
        for k in sorted(set(b or {}) | set(a or {})):
            if a is None or (b or {}).get(k) == a.get(k):
                continue
            if k not in b or k not in a:
                print("   %s member %s" % ("+" if k in a else "-", k), json.dumps(a.get(k, b.get(k)), ensure_ascii=False)[:250]); continue
            if isinstance(b[k], list) and isinstance(a[k], list):
                ib = [json.dumps(x, sort_keys=True) for x in b[k]]
                ia = [json.dumps(x, sort_keys=True) for x in a[k]]
                for x in ib:
                    if x not in ia:
                        print("   -", k, x[:300])
                for x in ia:
                    if x not in ib:
                        print("   +", k, x[:300])
                if sorted(ib) == sorted(ia):
                    print("   ~", k, "reordered")
            else:
                print("   B", k, json.dumps(b[k], ensure_ascii=False)[:250])
                print("   A", k, json.dumps(a[k], ensure_ascii=False)[:250])
