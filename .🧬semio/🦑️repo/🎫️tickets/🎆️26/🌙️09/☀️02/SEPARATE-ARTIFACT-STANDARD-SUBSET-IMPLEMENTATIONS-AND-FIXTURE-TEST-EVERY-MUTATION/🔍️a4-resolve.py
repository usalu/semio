import json
import os
import re
import unicodedata

TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

territory = json.load(open(f"{TICKET}/🗑️generated/a4-territory-before.json"))
uri_re = re.compile(r"Fixture (asset|shared|local)://(\S+) does not resolve")


def strip_leading_emoji(name):
    # strip leading non-ascii "emoji-ish" chars followed by optional variation selector / ZWJ,
    # stop at first ASCII letter/digit
    i = 0
    while i < len(name):
        ch = name[i]
        if ch.isascii() and (ch.isalnum() or ch in "._-"):
            break
        i += 1
    return name[i:]


def resolve_root(scope, scheme, uri):
    feature_path = scope
    case_dir = os.path.dirname(feature_path)
    tests_dir = os.path.dirname(case_dir)
    owner_dir = os.path.dirname(tests_dir)
    if scheme == "asset":
        # uri itself contains the full relative path from owner; root = owner, but the
        # directory to search is the parent dir named in the uri (minus the filename)
        rel_dir = os.path.dirname(uri)
        root = os.path.join(owner_dir, rel_dir)
        fname = os.path.basename(uri)
        return owner_dir, root, fname, case_dir
    elif scheme == "shared":
        root = os.path.join(owner_dir, "🧫️fixtures")
        return owner_dir, root, uri, case_dir
    elif scheme == "local":
        root = os.path.join(case_dir, "🧫️fixtures")
        return owner_dir, root, uri, case_dir
    return None, None, None, case_dir


results = []
for b in territory:
    scope = b["scope"]
    m = uri_re.search(b["summary"])
    if not m:
        results.append({"scope": scope, "summary": b["summary"], "status": "NO_URI_MATCH"})
        continue
    scheme, uri = m.group(1), m.group(2)
    owner_dir, root, fname, case_dir = resolve_root(scope, scheme, uri)
    stem, ext = os.path.splitext(fname)
    stripped_stem = strip_leading_emoji(stem)

    entry = {
        "scope": scope,
        "scheme": scheme,
        "uri": uri,
        "root": root,
        "fname": fname,
        "stripped_stem": stripped_stem,
        "ext": ext,
    }

    if not root or not os.path.isdir(root):
        entry["status"] = "ROOT_MISSING"
        results.append(entry)
        continue

    # collect all files with matching ext under root (search a bit wider: root's parent too for asset scheme where rel_dir points to exact dir)
    matches = []
    for dirpath, dirnames, filenames in os.walk(root):
        for fn in filenames:
            if fn.lower().endswith(ext.lower()) and ext:
                matches.append(os.path.join(dirpath, fn))

    # score matches
    scored = []
    for mpath in matches:
        parent = os.path.basename(os.path.dirname(mpath))
        parent_stripped = strip_leading_emoji(parent)
        same_dir = os.path.dirname(mpath) == root
        score = 0
        if parent_stripped == stripped_stem:
            score = 100
        elif stripped_stem and stripped_stem in parent_stripped:
            score = 80
        elif same_dir:
            score = 50
        else:
            score = 10
        scored.append((score, mpath))
    scored.sort(key=lambda x: -x[0])

    entry["candidates"] = [{"score": s, "path": p} for s, p in scored[:5]]
    entry["status"] = "OK" if scored else "NO_MATCH"
    results.append(entry)

with open(f"{TICKET}/🗑️generated/a4-resolve.json", "w") as f:
    json.dump(results, f, indent=2, ensure_ascii=False)

# print summary
for r in results:
    cands = r.get("candidates", [])
    top = cands[0] if cands else None
    print(r["scheme"], "|", r["uri"], "->", (top["path"] if top else r["status"]), f"(score={top['score'] if top else '-'}, n={len(cands)})")
