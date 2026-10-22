"""🧾 Manifest edit for a facade repoint: the hub dependency line of a crate manifest is replaced by the producer dependency lines (workspace-inherited when the owning workspace declares the producer, relative path otherwise); test-only producers go to [dev-dependencies]. Pure function on manifest text, fail-closed (returns an error string instead of guessing). Fixtures: staged/cases.json."""
import os
import re

SEC = re.compile(r"^\s*\[\[?\s*([^\]]+?)\s*\]\]?\s*(#.*)?$")


def sections(lines):
    out = []
    cur = ("", -1)
    for i, l in enumerate(lines):
        m = SEC.match(l)
        if m and l.lstrip().startswith("["):
            if cur[1] >= 0 or cur[0] == "":
                out.append((cur[0], cur[1], i))
            cur = (m.group(1).strip(), i)
    out.append((cur[0], cur[1], len(lines)))
    return out


def kind_of(name):
    n = name.strip()
    for k in ("dev-dependencies", "build-dependencies", "dependencies"):
        if n == k or n.endswith("." + k):
            return k
    return None


def find_entry(lines, a, b, key):
    """Returns (indices, value_text, dotted_attrs) of the hub entry inside lines[a:b] or None."""
    kq = r'(?:"%s"|%s)' % (re.escape(key), re.escape(key))
    idx, val, dotted = [], None, {}
    for i in range(a, b):
        l = lines[i]
        m = re.match(r"^\s*%s\s*=\s*(.*?)\s*(#.*)?$" % kq, l)
        if m:
            idx.append(i)
            val = m.group(1)
            continue
        m = re.match(r"^\s*%s\.([A-Za-z_-]+)\s*=\s*(.*?)\s*(#.*)?$" % kq, l)
        if m:
            idx.append(i)
            dotted[m.group(1)] = m.group(2)
    return (idx, val, dotted) if idx else None


def new_line(pkg, style, rel):
    if style == "ws":
        return f"{pkg} = {{ workspace = true }}"
    return f'{pkg} = {{ path = "{rel}" }}'


def render(pkg, style, rel, feats):
    f = f", features = [{', '.join(chr(34) + x + chr(34) for x in sorted(feats))}]" if feats else ""
    if style == "ws":
        return f"{pkg} = {{ workspace = true{f} }}"
    return f'{pkg} = {{ path = "{rel}"{f} }}'


def plan_manifest(text, key, owners_prod, owners_test, ws_declared, rel_of, have_normal=(), have_dev=(), feat_map=None):
    """text: manifest. key: dependency key of the hub. owners_*: sorted package names. ws_declared(pkg) -> bool (the owning workspace declares pkg). rel_of(pkg) -> relative path.
    feat_map(features) -> ({pkg: {feature}}, None) | (None, reason): the hub's feature forwarding for an edge that carries `features`; applied only to producers the crate names or already depends on (dev edges become dev entries).
    Returns (new_text, None) or (None, reason)."""
    lines = text.split("\n")
    secs = sections(lines)
    hits = []
    for name, a, b in secs:
        k = kind_of(name)
        if k is None or a < 0:
            continue
        e = find_entry(lines, a + 1, b, key)
        if e:
            hits.append((name, k, a, b, e))
    if not hits:
        return None, "hub entry not found in manifest"
    if any(re.match(r"^\s*dependencies\.%s\s*$" % re.escape(key), n) for n, _, _ in secs):
        return None, "table-form dependency"
    if len([h for h in hits if h[1] != "dev-dependencies"]) > 1:
        return None, "hub declared in several non-dev tables"
    named = set(owners_prod) | set(owners_test)
    existing = set(have_normal) | set(have_dev)
    want = {}
    dev_name = None
    for name, a, b in secs:
        if name.strip() == "dev-dependencies" and a >= 0:
            dev_name = name
    plans = []
    for name, k, a, b, (idx, val, dotted) in hits:
        flat = (val or "") + " " + " ".join(f"{x}={y}" for x, y in dotted.items())
        if val is not None and not (val.startswith("{") and val.rstrip().endswith("}")) and not dotted:
            return None, "hub entry is not an inline table: " + (val or "")[:40]
        flat = re.sub(r'package\s*=\s*"%s"' % re.escape(key), "", flat)
        if re.search(r"optional|default-features|default_features|package\s*=", flat):
            return None, "hub entry is optional, renamed or without default features"
        if dotted and not set(dotted) <= {"workspace", "path"}:
            return None, "dotted hub entry has extra attributes"
        fm = re.search(r"features\s*=\s*\[([^\]]*)\]", flat)
        feats = re.findall(r'"([^"]+)"', fm.group(1)) if fm else []
        if re.search(r"features", flat) and not fm:
            return None, "unparsable features"
        style = "ws" if re.search(r"workspace\s*=\s*true", flat) else "path"
        owners = list(owners_test) if k == "dev-dependencies" else list(owners_prod)
        plans.append((name, k, a, b, idx, style, owners))
        for p in owners:
            want.setdefault((name, p), set())
        if feats:
            if feat_map is None:
                return None, "hub entry has features"
            fmap, err = feat_map(feats)
            if err:
                return None, err
            for p, fs in fmap.items():
                if p not in named and p not in existing:
                    continue
                if k == "dev-dependencies":
                    want.setdefault((name, p), set()).update(fs)
                else:
                    if p in existing:
                        return None, f"feature merge on existing dependency {p}"
                    want.setdefault((name, p), set()).update(fs)
    if not any(h[1] == "dev-dependencies" for h in hits):
        for p in owners_test:
            if p not in owners_prod:
                want.setdefault(("dev-dependencies", p), set())
    new_lines = list(lines)
    style_of = {h[0]: ("ws" if re.search(r"workspace\s*=\s*true", " ".join(lines[i] for i in h[4][0])) else "path") for h in hits}
    anyws = any(v == "ws" for v in style_of.values())
    dev_add = []
    by_sec = {}
    for (sec, p), fs in sorted(want.items()):
        k = "dev-dependencies" if sec == "dev-dependencies" else kind_of(sec)
        have_here = (set(have_dev) if k == "dev-dependencies" else set(have_normal))
        if p in have_here and not fs:
            continue
        if p in have_here and fs:
            return None, f"{p} already declared in that table"
        if k != "dev-dependencies" and p in have_dev and False:
            pass
        st = "ws" if (style_of.get(sec) == "ws" or (sec == "dev-dependencies" and anyws)) and ws_declared(p) else "path"
        if k == "dev-dependencies" and p in set(have_normal) and not fs:
            continue
        by_sec.setdefault(sec, []).append(render(p, st, rel_of(p) if st == "path" else "", fs))
    for name, k, a, b, idx, style, owners in plans:
        out = by_sec.pop(name, [])
        first = idx[0]
        new_lines[first] = "\n".join(out) if out else None
        for i in idx[1:]:
            new_lines[i] = None
    res = [l for l in new_lines if l is not None]
    for sec, add in by_sec.items():
        secs2 = sections(res)
        tgt = [s for s in secs2 if s[0].strip() == sec.strip() and s[1] >= 0]
        if tgt:
            _, a, b = tgt[0]
            j = b
            while j > a + 1 and res[j - 1].strip() == "":
                j -= 1
            res[j:j] = add
        else:
            if res and res[-1].strip() != "":
                res.append("")
            res += [f"[{sec}]"] + add + [""]
    return "\n".join(res), None


def line_ops(old, new):
    """Exact-once replacement operations that turn `old` into `new`: line based; each operation's old text is extended with neighbouring lines until it occurs exactly once in `old`; operations whose extended ranges overlap are merged first."""
    import difflib
    a, b = old.split("\n"), new.split("\n")
    ocs = [o for o in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes() if o[0] != "equal"]
    clusters = [[o] for o in ocs]

    def rng(cl):
        i1, i2 = cl[0][1], cl[-1][2]
        lo, hi = i1, i2
        while True:
            blk = "\n".join(a[lo:hi])
            if blk != "" and old.count(blk) == 1:
                break
            if lo > 0:
                lo -= 1
            elif hi < len(a):
                hi += 1
            else:
                break
        return lo, hi

    while True:
        rs = [rng(c) for c in clusters]
        merged = False
        for k in range(len(clusters) - 1):
            if rs[k][1] > rs[k + 1][0] or (rs[k][1] == rs[k + 1][0] and False):
                clusters[k:k + 2] = [clusters[k] + clusters[k + 1]]
                merged = True
                break
        if not merged:
            break
    ops = []
    for cl, (lo, hi) in zip(clusters, rs):
        j1, j2 = cl[0][3], cl[-1][4]
        i1, i2 = cl[0][1], cl[-1][2]
        nb = b[j1 - (i1 - lo): j2 + (hi - i2)]
        ot = "\n".join(a[lo:hi])
        if not nb:
            if hi < len(a):
                ops.append({"old": ot + "\n", "new": ""})
            else:
                ops.append({"old": "\n" + ot, "new": ""})
            continue
        ops.append({"old": ot, "new": "\n".join(nb)})
    return ops


def apply_ops(old, ops):
    new = old
    for op in ops:
        if new.count(op["old"]) != 1:
            return None
        new = new.replace(op["old"], op["new"])
    return new


def add_deps(text, owners, ws_declared, rel_of, section="dependencies", have=()):
    """Appends producer dependency lines at the end of `[<section>]` (created when absent); returns (new_text, None) or (None, reason)."""
    lines = text.split("\n")
    secs = sections(lines)
    add = [new_line(p, "ws" if ws_declared(p) else "path", rel_of(p)) for p in sorted(set(owners)) if p not in have]
    if not add:
        return text, None
    tgt = [s for s in secs if s[0].strip() == section and s[1] >= 0]
    if tgt:
        _, a, b = tgt[0]
        j = b
        while j > a + 1 and lines[j - 1].strip() == "":
            j -= 1
        lines[j:j] = add
    else:
        if lines and lines[-1].strip() == "":
            lines.pop()
        lines += ["", f"[{section}]"] + add + [""]
    return "\n".join(lines), None


def hub_lines_of(text, key):
    """The original manifest lines of the hub entry (the dev table's when present): (lines, kind) or (None, None)."""
    lines = text.split("\n")
    found = None
    for name, a, b in sections(lines):
        k = kind_of(name)
        if k is None or a < 0:
            continue
        e = find_entry(lines, a + 1, b, key)
        if e and (found is None or k == "dev-dependencies"):
            found = ([lines[i] for i in e[0]], k)
    return found if found else (None, None)


def append_to_section(text, section, add):
    lines = text.split("\n")
    tgt = [s for s in sections(lines) if s[0].strip() == section and s[1] >= 0]
    if tgt:
        _, a, b = tgt[0]
        j = b
        while j > a + 1 and lines[j - 1].strip() == "":
            j -= 1
        lines[j:j] = add
    else:
        if lines and lines[-1].strip() == "":
            lines.pop()
        lines += ["", f"[{section}]"] + add + [""]
    return "\n".join(lines)
