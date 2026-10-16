#!/usr/bin/env python3
# 🔎️ Definition scan: private fields, methods and free items of a carved crate that sources of another crate (or the remaining monolith) still mention.
# Names that are also defined `pub` anywhere else in the tree are skipped (too ambiguous); `--seed` plants a private field and expects the scan to catch it.
import collections, json, os, re, sys
from common import *
import modtree, items
from rs import blank


def all_rs():
    out = []
    for base, dirs, fs in os.walk(E):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "target")]
        out += [os.path.join(base, f) for f in fs if f.endswith(".rs")]
    return out


def private_members(text):
    clean = blank(text)
    fields, methods, free = {}, {}, set()
    for it in items.table(text):
        if it["kind"] in ("struct",) and not it["vis"].startswith("pub(") or it["kind"] == "struct":
            body = clean.split("\n")[it["head"] - 1:it["end"]]
            b = "\n".join(body)
            i = b.find("{")
            if i >= 0:
                for fm in re.finditer(r"(?m)^\s*(pub(?:\([^)]*\))?\s+)?([a-z_][a-z0-9_]*)\s*:", b[i:]):
                    if not fm.group(1):
                        fields.setdefault(it["name"], set()).add(fm.group(2))
        elif it["kind"] == "impl" and " for " not in it["name"]:
            b = "\n".join(clean.split("\n")[it["head"]:it["end"]])
            for mm in re.finditer(r"(?m)^    ((?:pub(?:\([^)]*\))?\s+)?)((?:async |const |unsafe )*fn)\s+([a-z_][a-z0-9_]*)", b):
                if not mm.group(1):
                    methods.setdefault(it["name"], set()).add(mm.group(3))
        if it["kind"] in ("fn", "const", "static", "struct", "enum", "type", "trait") and not it["vis"].startswith("pub") and not it.get("thread_local"):
            free.add(it["name"])
    return fields, methods, free


def main():
    seed = "--seed" in sys.argv
    carved = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/carved.json"))
    sel = [a for a in sys.argv[1:] if not a.startswith("--")] or carved
    files = all_rs()
    texts = {f: read(f) for f in files}
    pub_names = collections.Counter()
    for f, t in texts.items():
        for it in items.table(t):
            if it["vis"].startswith("pub"):
                pub_names[it["name"]] += 1
        for m in re.finditer(r"(?m)^\s*pub\s+(?:\([^)]*\)\s+)?(?:async\s+|const\s+|unsafe\s+)*(?:fn\s+)?([a-z_][a-z0-9_]*)\s*[:(<]", blank(t)):
            pub_names[m.group(1)] += 1
    total = 0
    for c in sel:
        own = set(modtree.tree(T + "/" + CRATES[c] + "/🦀️.rs"))
        own = {f for f in own if "/🧪️tests/" not in f}
        fields, methods, free = {}, {}, set()
        for f in own:
            fl, ml, fr = private_members(texts.get(f) or read(f))
            for k, v in fl.items(): fields.setdefault(k, set()).update(v)
            for k, v in ml.items(): methods.setdefault(k, set()).update(v)
            free |= fr
        if seed and c == "mailbox-core":
            fields.setdefault("BoundedCompletionQueue", set()).add("in_flight")
        names_f = {x for s in fields.values() for x in s if len(x) > 3 and pub_names[x] == 0}
        names_m = {x for s in methods.values() for x in s if len(x) > 4 and pub_names[x] == 0}
        names_r = {x for x in free if len(x) > 4 and pub_names[x] == 0}
        hits = collections.defaultdict(set)
        for f, t in texts.items():
            if f in own:
                continue
            code = blank(t)
            if f.endswith("/tests.rs") or "/🧪️tests/" in f and any(o.split("/")[-2] in f for o in own):
                pass
            for n in names_f:
                if re.search(r"\." + n + r"\b(?!\s*\()", code):
                    hits[("field", n)].add(f.replace(E + "/", "")[:70])
            for n in names_m:
                if re.search(r"\." + n + r"\s*\(", code):
                    hits[("method", n)].add(f.replace(E + "/", "")[:70])
            for n in names_r:
                if re.search(r"(?<![A-Za-z0-9_.$])" + n + r"\b", code):
                    hits[("free", n)].add(f.replace(E + "/", "")[:70])
        print("##", c, "private fields", sum(map(len, fields.values())), "methods", sum(map(len, methods.values())), "free", len(free), "-> mentions outside:", len(hits))
        for (kind, n), fs in sorted(hits.items())[:40]:
            print("  ", kind, n, "in", sorted(fs)[:2], "+%d" % (len(fs) - 2) if len(fs) > 2 else "")
        total += len(hits)
    print("total", total)


if __name__ == "__main__":
    main()
