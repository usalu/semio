"""🔬 Probe: how far does a purely SPEC-DERIVED mutation engine reproduce every committed
`(before, mutation, after)` vector in the 15 norm vocabularies?

Written from `📓️taxonomy.md` (verb table, naming mechanics, addressing convention) and
`📓️derivation-rules.md` (shape rules) only. Scratch driver — not the deliverable.
"""
import json, os, re, sys, glob, copy, collections

ROOT = os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..")
ART = os.path.abspath(os.path.join(ROOT, "✏️s/🔌️plugins/📕️norm/🗿️artifacts"))

def load(p):
    with open(p, encoding="utf-8") as h: return json.load(h)

def norm(s): return re.sub(r"[-_ ]", "", str(s)).lower()

def unwrap(m):
    if isinstance(m, dict) and isinstance(m.get("mutation"), str):
        return m["mutation"], {k: v for k, v in m.items() if k != "mutation"}
    if isinstance(m, dict) and len(m) == 1:
        k = next(iter(m))
        if isinstance(m[k], dict): return k, m[k]
    raise ValueError("unrecognised mutation wire form %r" % (m,))

def resolve(doc, name):
    """🔎 The document key whose normalised spelling is `name`'s."""
    want = norm(name)
    for k in doc:
        if norm(k) == want: return k
    return None

def new_fields(payload):
    return [k for k in payload if norm(k).startswith("new")]

def strip_new(k):
    return re.sub(r"^new[-_]?", "", k, flags=re.I)

def apply_generic(kind, payload, doc):
    """⚙️ Returns (document, rejected_reason|None)."""
    doc = copy.deepcopy(doc)
    verb = kind.split("-")[0]
    news = new_fields(payload)
    addr = {k: v for k, v in payload.items() if k not in news}

    if verb in ("change", "set", "update", "rename", "edit", "replace", "resize"):
        if not news: return doc, "no new-value field in payload"
        if "index" in addr:                       # addressed element of an ordered collection
            coll = collection_for(kind, doc)
            if coll is None: return doc, "no collection for %s" % kind
            items = doc[coll]
            i = addr["index"]
            if not isinstance(items, list) or i >= len(items): return doc, "index out of range"
            for nk in news:
                fk = resolve(items[i], strip_new(nk))
                if fk is None: return doc, "no element field for %s" % nk
                items[i][fk] = payload[nk]
            return doc, None
        for nk in news:
            fk = resolve(doc, strip_new(nk))
            if fk is None: return doc, "no document field for %s" % nk
            doc[fk] = payload[nk]
        return doc, None

    if verb == "insert":
        coll = collection_for(kind, doc)
        if coll is None: return doc, "no collection"
        item = next((payload[k] for k in payload if k != "index"), None)
        i = min(payload.get("index", len(doc[coll])), len(doc[coll]))
        doc[coll].insert(i, item)
        return doc, None

    if verb == "remove":
        coll = collection_for(kind, doc)
        if coll is None: return doc, "no collection"
        i = payload.get("index")
        if not isinstance(doc.get(coll), list) or i is None or i >= len(doc[coll]):
            return doc, "index out of range"
        doc[coll].pop(i)
        return doc, None

    if verb == "reorder":
        coll = collection_for(kind, doc)
        if coll is None: return doc, "no collection"
        items = doc[coll]
        f, t = payload["from"], payload["to"]
        if f >= len(items) or t >= len(items): return doc, "index out of range"
        items.insert(t, items.pop(f))
        return doc, None

    return doc, "unsupported verb %s" % verb

def collection_for(kind, doc):
    noun = "-".join(kind.split("-")[1:])
    for cand in (noun, noun + "s", re.sub(r"s$", "", noun)):
        k = resolve(doc, cand)
        if k is not None and isinstance(doc.get(k), list): return k
    lists = [k for k, v in doc.items() if isinstance(v, list)]
    return lists[0] if len(lists) == 1 else None

def main():
    tally = collections.Counter()
    detail = collections.defaultdict(list)
    for a in sorted(os.listdir(ART)):
        mroot = os.path.join(ART, a, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
        if not os.path.isdir(mroot): continue
        for d in sorted(os.listdir(mroot)):
            t = os.path.join(mroot, d, "🧪️tests")
            if not os.path.isdir(t): continue
            fx = sorted(os.listdir(t))[0]
            base = os.path.join(t, fx)
            kind = re.sub(r"^[^a-z]+", "", d)
            before = load(os.path.join(base, "📸️snapshot/⬅️before/🔣️component.json"))
            after = load(os.path.join(base, "📸️snapshot/➡️after/🔣️component.json"))
            mut = load(os.path.join(base, "🦠️mutation/🔣️component.json"))
            status = load(os.path.join(base, "🎯️outcome/🔣️component.json"))["status"]
            try:
                tag, payload = unwrap(mut)
            except ValueError as e:
                tally[a + " WIRE"] += 1; detail[a].append((kind, str(e))); continue
            got, rejected = apply_generic(kind, payload, before)
            ok = (got == after) and ((rejected is None) == (status == "applied"))
            tally[a + (" ok" if ok else " MISMATCH")] += 1
            if not ok: detail[a].append((kind, "rejected=%s status=%s" % (rejected, status)))
    for a in sorted(set(k.rsplit(" ", 1)[0] for k in tally)):
        ok = tally[a + " ok"]; bad = tally[a + " MISMATCH"] + tally[a + " WIRE"]
        print(f"{a:14s} ok={ok:3d} mismatch={bad:3d}")
        for k, why in detail[a][:8]: print(f"      {k:42s} {why}")
main()
