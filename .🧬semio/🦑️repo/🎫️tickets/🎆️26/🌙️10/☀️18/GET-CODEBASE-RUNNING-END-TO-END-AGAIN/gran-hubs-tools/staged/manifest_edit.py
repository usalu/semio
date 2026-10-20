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


def plan_manifest(text, key, owners_prod, owners_test, ws_declared, rel_of, have_normal=(), have_dev=()):
    """text: manifest. key: dependency key of the hub. owners_*: sorted package names. ws_declared(pkg) -> bool (the owning workspace declares pkg).
    rel_of(pkg) -> relative path string. Returns (new_text, None) or (None, reason)."""
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
    nondev = [h for h in hits if h[1] != "dev-dependencies"]
    if len(nondev) > 1:
        return None, "hub declared in several non-dev tables"
    new_lines = list(lines)
    edits = []
    have = {("dependencies", p) for p in have_normal} | {("dev-dependencies", p) for p in have_dev}
    plans = []
    for name, k, a, b, (idx, val, dotted) in hits:
        flat = (val or "") + " " + " ".join(f"{x}={y}" for x, y in dotted.items())
        if val is not None and not (val.startswith("{") and val.rstrip().endswith("}")) and not dotted:
            return None, "hub entry is not an inline table: " + (val or "")[:40]
        flat = re.sub(r'package\s*=\s*"%s"' % re.escape(key), "", flat)
        if re.search(r"features|optional|default-features|default_features|package\s*=", flat):
            return None, "hub entry has features, optional or rename"
        if dotted and not set(dotted) <= {"workspace", "path"}:
            return None, "dotted hub entry has extra attributes"
        style = "ws" if re.search(r"workspace\s*=\s*true", flat) else "path"
        owners = list(owners_test) if k == "dev-dependencies" else list(owners_prod)
        plans.append((name, k, a, b, idx, style, owners))
    dev_extra = list(owners_test)
    for name, k, a, b, idx, style, owners in plans:
        out = []
        for pkg in owners:
            if (k, pkg) in have or (k == "dev-dependencies" and ("dependencies", pkg) in have):
                continue
            st = "ws" if style == "ws" and ws_declared(pkg) else "path"
            out.append(new_line(pkg, st, rel_of(pkg) if st == "path" else ""))
        edits.append((idx, out))
    if not any(h[1] == "dev-dependencies" for h in hits) and dev_extra:
        edits.append((None, [new_line(p, "ws" if ws_declared(p) and any(x[5] == "ws" for x in plans) else "path", rel_of(p)) for p in dev_extra if ("dev-dependencies", p) not in have and ("dependencies", p) not in have]))
    for idx, out in edits:
        if idx is None:
            continue
        first = idx[0]
        new_lines[first] = "\n".join(out) if out else None
        for i in idx[1:]:
            new_lines[i] = None
    dev_add = [e[1] for e in edits if e[0] is None]
    res = [l for l in new_lines if l is not None]
    if dev_add and dev_add[0]:
        add = dev_add[0]
        secs2 = sections(res)
        dev = [s for s in secs2 if kind_of(s[0]) == "dev-dependencies" and s[0].strip() == "dev-dependencies"]
        if dev:
            _, a, b = dev[0]
            j = b
            while j > a + 1 and res[j - 1].strip() == "":
                j -= 1
            res[j:j] = add
        else:
            if res and res[-1].strip() != "":
                res.append("")
            res += ["[dev-dependencies]"] + add + [""]
    return "\n".join("\n".join(res).split("\n")), None


def line_ops(old, new):
    """Exact-once replacement operations that turn `old` into `new` (line based; pure insertions are anchored on the preceding line)."""
    import difflib
    a, b = old.split("\n"), new.split("\n")
    ops = []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == "equal":
            continue
        if i1 == i2:
            anchor = a[i1 - 1]
            ops.append({"old": anchor, "new": anchor + "\n" + "\n".join(b[j1:j2])})
        elif j1 == j2 and i2 < len(a):
            ops.append({"old": "\n".join(a[i1:i2]) + "\n", "new": ""})
        else:
            ops.append({"old": "\n".join(a[i1:i2]), "new": "\n".join(b[j1:j2])})
    return ops


def apply_ops(old, ops):
    new = old
    for op in ops:
        if new.count(op["old"]) != 1:
            return None
        new = new.replace(op["old"], op["new"])
    return new
