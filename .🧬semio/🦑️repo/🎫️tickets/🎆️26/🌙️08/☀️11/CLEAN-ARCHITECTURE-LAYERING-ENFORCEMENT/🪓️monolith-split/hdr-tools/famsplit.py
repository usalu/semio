import json, os, re, sys, unicodedata
# 🪓️ famsplit.py <manifest> [--write] [--parts N] : splits a leaf-family crate (a lib root that is only a list of leaf mounts, flat, nested in a wrapper chain, or one inline module per leaf) into the crate and a sibling part crate.
# The old crate re-exports the moved leaf modules (explicit pub use), so consumers keep their paths. Registers the new crate in the artifact workspace (member, dependency table) and the owner rows.
R = "/Users/ueli/Documents/semio"
PKG = "📦️packages/🦀️rust"
POOL = ["🪜", "🧰", "🔩", "🧲", "🪙", "🪛", "🔧", "🪝", "🧵", "🪢", "🧶", "🪡"]


def die(msg):
    print("ABORT:", msg)
    sys.exit(2)


def brace_delta(line):
    s = re.sub(r'"[^"]*"', '""', line)
    s = re.sub(r"//.*", "", s)
    return s.count("{") - s.count("}")


def parse_items(lines):
    items = []
    i = 0
    n = len(lines)
    while i < n:
        line = lines[i]
        st = line.strip()
        if st == "" :
            items.append(dict(kind="blank", raw=[line]))
            i += 1
            continue
        if st.startswith("#![") or st.startswith("extern crate "):
            items.append(dict(kind="inner", raw=[line]))
            i += 1
            continue
        if st.startswith("//"):
            items.append(dict(kind="marker", raw=[line]))
            i += 1
            continue
        attrs = []
        start = i
        while i < n and lines[i].strip().startswith("#["):
            attrs.append(lines[i])
            i += 1
        if i >= n:
            die("dangling attribute")
        head = lines[i]
        depth = brace_delta(head)
        body = None
        close = None
        j = i + 1
        if depth > 0:
            bstart = j
            while j < n and depth > 0:
                depth += brace_delta(lines[j])
                j += 1
            body = lines[bstart:j - 1]
            close = lines[j - 1]
        else:
            if not head.rstrip().endswith(";"):
                die("item head not ending in ; or {: " + head.strip())
        m = re.match(r"\s*(pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", head)
        kind = "mod" if m else ("reexp" if re.match(r"\s*pub use \w+_\d+::", head) else "other")
        items.append(dict(kind=kind, name=m.group(2) if m else None, attrs=attrs, head=head, body=body, close=close, raw=lines[start:j]))
        i = j
    return items


def mods_of(items):
    return [it for it in items if it["kind"] == "mod"]


def find_container(lines):
    wrappers = []
    items = parse_items(lines)
    while True:
        mods = mods_of(items)
        others = [it for it in items if it["kind"] == "other"]
        if len(mods) >= 2:
            return wrappers, items
        if len(mods) == 1 and mods[0]["body"] is not None and not others:
            wrappers.append(mods[0])
            items = parse_items(mods[0]["body"])
            continue
        die(f"no leaf container (mods={len(mods)} others={len(others)})")


PATH_RX = re.compile(r'#\[path\s*=\s*"([^"]*)"\]')


def mounted_files(item, base):
    out = []
    for line in item["raw"]:
        for m in PATH_RX.finditer(line):
            p = m.group(1)
            if p == ".":
                continue
            out.append(os.path.normpath(os.path.join(base, p)))
    return out


def closure(files):
    seen = []
    stack = list(files)
    while stack:
        f = stack.pop()
        if f in seen or not os.path.isfile(f):
            continue
        seen.append(f)
        txt = open(f).read()
        for m in PATH_RX.finditer(txt):
            p = m.group(1)
            if p == ".":
                continue
            stack.append(os.path.normpath(os.path.join(os.path.dirname(f), p)))
    return seen


def snake(name):
    return name.replace("-", "_")


def main():
    args = sys.argv[1:]
    write = "--write" in args
    parts = 2
    sibling = "--sibling" in args
    base_name = args[args.index("--base-name") + 1] if "--base-name" in args else None
    base_id = args[args.index("--base-id") + 1] if "--base-id" in args else None
    no_reexport = "--no-reexport" in args
    first_index = int(args[args.index("--first-index") + 1]) if "--first-index" in args else 2
    if "--parts" in args:
        parts = int(args[args.index("--parts") + 1])
    man = os.path.join(R, [a for a in args if not a.startswith("--") and not a.isdigit() and a != base_name and a != base_id][0])
    mtxt = open(man).read()
    pkg = re.search(r'^name = "([^"]+)"', mtxt, re.M).group(1)
    pkgdir = os.path.dirname(man)
    libm = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mtxt)
    target_name = None
    if not libm:
        tm = re.search(r'\[\[test\]\]\nname = "([^"]+)"\n(?:[^\n\[][^\n]*\n)*?path = "([^"]+)"', mtxt)
        if not tm:
            die("no lib and no test target")
        target_name = tm.group(1)
        libm = re.search(r'path = "([^"]+)"', tm.group(0))
    root = os.path.normpath(os.path.join(pkgdir, libm.group(1)))
    rdir = os.path.dirname(root)
    text = open(root).read()
    lines = text.split("\n")
    first = next((i for i, l in enumerate(lines) if l.lstrip().startswith("#[path") or l.strip().startswith("//#region") or re.match(r"\s*(pub )?mod \w+", l)), None)
    if first is None:
        die("no mounts")
    header = lines[:first]
    wrappers, leafitems = find_container(lines[first:])
    leaves = mods_of(leafitems)
    if any(it["kind"] == "other" for it in leafitems):
        die("non-module items at the leaf level")
    names = [it["name"] for it in leaves]
    n = len(leaves)
    fileset = {idx: closure(mounted_files(it, rdir)) for idx, it in enumerate(leaves)}
    refs = {idx: set() for idx in range(n)}
    for idx in range(n):
        others = [names[i] for i in range(n) if i != idx]
        rx = re.compile(r"(?:::|\bsuper::|\bself::)(%s)\b" % "|".join(map(re.escape, others)))
        for f in fileset[idx]:
            for l in open(f).read().split("\n"):
                if l.strip().startswith("//"):
                    continue
                for m in rx.finditer(l):
                    refs[idx].add(names.index(m.group(1)))
    uf = list(range(n))
    def find(x):
        while uf[x] != x:
            uf[x] = uf[uf[x]]
            x = uf[x]
        return x
    for a_, rs in refs.items():
        for b_ in rs:
            uf[find(a_)] = find(b_)
    comps = {}
    for i in range(n):
        comps.setdefault(find(i), []).append(i)
    fb = {i: (len(fileset[i]), sum(os.path.getsize(f) for f in fileset[i])) for i in range(n)}
    comps = sorted(comps.values(), key=lambda c: (-max(sum(fb[i][0] for i in c) / 32, sum(fb[i][1] for i in c) / 262144), c[0]))
    if len(comps) < parts:
        die("only %d independent leaf groups" % len(comps))
    bins = [[] for _ in range(parts)]
    tot = [[1, 0] for _ in range(parts)]
    def score(t):
        return max(t[0] / 32, t[1] / 262144)
    for c in comps:
        cf, cb = sum(fb[i][0] for i in c), sum(fb[i][1] for i in c)
        k = min(range(parts), key=lambda j: score([tot[j][0] + cf, tot[j][1] + cb]))
        bins[k] += c
        tot[k][0] += cf
        tot[k][1] += cb
    weights = [round(score(t), 3) for t in tot]
    groups = sorted([sorted(b_) for b_ in bins], key=lambda g: g[0])
    if max(weights) > 0.93:
        die("unbalanced parts %s components %s refs %s" % (weights, [[names[i] for i in c] for c in comps], {names[a]: [names[b] for b in v] for a, v in refs.items() if v}))
    keep, moved_groups = groups[0], groups[1:]
    created = []
    for gi, g in enumerate(groups[1:], start=first_index):
        # directory of the part crate: sibling of the leaf directories (parent of mounted leaf dirs)
        allmounted = [f for it in leaves for f in mounted_files(it, rdir)]
        parent = os.path.dirname(rdir) if sibling else os.path.commonpath([os.path.dirname(f) for f in allmounted])
        sibs = os.listdir(parent)
        used = {unicodedata.normalize("NFC", x).replace("\ufe0f", "")[:1] for x in sibs}
        emoji = next(e for e in POOL if e not in used)
        srcname = os.path.basename(rdir) if os.path.basename(rdir) != "🦀️rust" else os.path.basename(os.path.dirname(os.path.dirname(rdir)))
        famname = re.sub(r"^[^A-Za-z0-9]+", "", srcname)
        newname = "%s-%d" % (re.sub(r"-[0-9]+$", "", famname) if base_name else famname, gi)
        newdir = os.path.join(parent, f"{emoji}️{newname}")
        if os.path.exists(newdir):
            die("exists " + newdir)
        newpkgdir = os.path.join(newdir, PKG)
        newroot = os.path.join(newdir, "🦀️.rs")
        # rebased lines
        def rebase(raw_lines):
            out = []
            for l in raw_lines:
                def sub(m):
                    p = m.group(1)
                    if p == ".":
                        return m.group(0)
                    tgt = os.path.normpath(os.path.join(rdir, p))
                    return '#[path = "%s"]' % os.path.relpath(tgt, newdir)
                out.append(PATH_RX.sub(sub, l))
            return out
        hdr = list(header)
        for k, l in enumerate(hdr):
            if l.startswith("//!"):
                hdr[k] = "//! " + emoji + "️ " + re.sub(r"^//!\s*\S+\s*", "", l).rstrip().rstrip(".") + " (part %d)." % gi
                break
        # build body
        def nest(inner_lines):
            out = []
            for w in wrappers:
                out += w["attrs"] + [w["head"]]
            out += inner_lines
            for w in reversed(wrappers):
                out.append(w["close"])
            return out
        sel = [leaves[i] for i in g]
        inner = []
        for it in sel:
            inner += rebase(it["raw"])
        inners = [l for it in leafitems if it["kind"] == "inner" for l in it["raw"]]
        marks = [it["raw"][0] for it in leafitems if it["kind"] == "marker" and it["raw"][0].strip().startswith("//#")]
        if len(marks) == 2:
            inner = [marks[0]] + inners + inner + [marks[1]]
        else:
            inner = inners + inner
        newtext = "\n".join(hdr) .rstrip("\n") + "\n\n" + "\n".join(nest(inner)) + "\n"
        created.append(dict(g=g, gi=gi, newdir=newdir, newpkgdir=newpkgdir, newroot=newroot, newtext=newtext, newname=newname, emoji=emoji))
    # rebuild old root
    keep_items = [leaves[i] for i in keep]
    moved_all = [i for g in groups[1:] for i in g]
    # re-export lines
    chain = []
    for w in wrappers:
        chain.append(w["name"])
    old_inner = []
    for it in leafitems:
        if it["kind"] == "mod":
            if names.index(it["name"]) in moved_all:
                continue
        old_inner += it["raw"]
    pkgs = []
    for c in created:
        newpkg_name = f"{base_name or pkg}-{c['gi']}"
        c["pkg"] = newpkg_name
        ids = ", ".join(names[i] for i in c["g"])
        prefix = (("::".join(chain) + "::") if chain else "")
        indent = re.match(r"\s*", leafitems[0]["raw"][0] if leafitems[0]["raw"] else "").group(0)
        if not no_reexport:
            old_inner += [f"{indent}pub use {snake(newpkg_name)}::{prefix}{{{ids}}};"]
    old_lines = list(header)
    out = []
    for w in wrappers:
        out += w["attrs"] + [w["head"]]
    out += old_inner
    for w in reversed(wrappers):
        out.append(w["close"])
    newold = "\n".join(old_lines).rstrip("\n") + "\n\n" + "\n".join(out) + "\n"
    print(f"{pkg}: {n} leaves (parts {weights}) -> keep {len(keep)}, move {len(moved_all)} into {[c['pkg'] for c in created]}; wrappers {chain}")
    for c in created:
        print("  new dir", os.path.relpath(c["newdir"], R))
    if not write:
        return
    # manifest for parts
    wsrel = re.search(r'^workspace = "([^"]+)"', mtxt, re.M).group(1)
    wsabs = os.path.normpath(os.path.join(pkgdir, wsrel))
    ws_toml = os.path.join(wsabs, "Cargo.toml")
    wtxt = open(ws_toml).read()
    for c in created:
        os.makedirs(c["newpkgdir"])
        def rebase_path(m):
            tgt = os.path.normpath(os.path.join(pkgdir, m.group(1)))
            return 'path = "%s"' % os.path.relpath(tgt, c["newpkgdir"])
        nm = re.sub(r'path = "(\.\./[^"]*)"', rebase_path, mtxt)
        nm = re.sub(r"\n\[\[(?:test|bin|bench|example)\]\]\n(?:[^\n\[][^\n]*\n)+", "\n", nm)
        nm = re.sub(r"\n{3,}", "\n\n", nm).rstrip("\n") + "\n"
        if target_name:
            nm += '\n[[test]]\nname = "%s-%d"\npath = "../../🦀️.rs"\n' % (target_name, c["gi"])
        nm = re.sub(r'^workspace = "[^"]+"', 'workspace = "%s"' % os.path.relpath(wsabs, c["newpkgdir"]), nm, count=1, flags=re.M)
        nm = re.sub(r'^name = "[^"]+"', 'name = "%s"' % c["pkg"], nm, count=1, flags=re.M)
        nm = re.sub(r'^description = "([^"]*)"', lambda m: 'description = "%s (part %d)"' % (m.group(1), c["gi"]), nm, count=1, flags=re.M)
        nm = re.sub(r'(\[lib\][^\[]*?path = )"[^"]+"', r'\1"../../🦀️.rs"', nm, count=1)
        idm = re.search(r'^id = "([^"]+)"', nm, re.M)
        c["fw"] = idm.group(1) if idm else None
        if idm:
            c["id"] = f"{base_id or idm.group(1)}-{c['gi']}"
            nm = nm.replace(idm.group(0), f'id = "{c["id"]}"', 1)
            open(os.path.join(c["newpkgdir"], "📋️project.json"), "w").write('{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n' % c["id"])
        open(os.path.join(c["newpkgdir"], "Cargo.toml"), "w").write(nm)
        open(c["newroot"], "w").write(c["newtext"])
        # members and dependency table
        mem = os.path.relpath(c["newpkgdir"], wsabs)
        mrx = re.search(r"members = \[\n((?:  \"[^\"]+\",\n)+)\]", wtxt)
        if not mrx:
            mrx = re.search(r"members = \[\n((?:\s+\"[^\"]+\",\n)+)\s*\]", wtxt)
        if not mrx:
            die("members block not found in " + ws_toml)
        entries = re.findall(r'"([^"]+)"', mrx.group(1))
        indent_m = re.match(r"(\s*)\"", mrx.group(1)).group(1)
        old_mem = os.path.relpath(pkgdir, wsabs)
        if old_mem in entries:
            entries.insert(entries.index(old_mem) + 1, mem)
        else:
            entries.append(mem)
            entries.sort()
        wtxt = wtxt[:mrx.start(1)] + "".join(f'{indent_m}"{e}",\n' for e in entries) + wtxt[mrx.end(1):]
        dep_rx = re.search(r"\[workspace\.dependencies\.%s\]\npath = \"[^\"]+\"\n" % re.escape(pkg), wtxt)
        has_table = bool(dep_rx)
        if has_table:
            wtxt = wtxt[:dep_rx.end()] + f'\n[workspace.dependencies.{c["pkg"]}]\npath = "{mem}"\n' + wtxt[dep_rx.end():]
        c["has_table"] = has_table
    open(ws_toml, "w").write(wtxt)
    # old crate: dependency on the parts
    for c in created if not no_reexport else []:
        line = f'{c["pkg"]} = {{ workspace = true }}\n' if c["has_table"] else f'{c["pkg"]} = {{ path = "{os.path.relpath(c["newpkgdir"], pkgdir)}" }}\n'
        dm = re.search(r"^\[dependencies\]\n", mtxt, re.M)
        mtxt = mtxt[:dm.end()] + line + mtxt[dm.end():]
        fm = re.search(r"^testkit = \[([^\]]*)\]", mtxt, re.M)
        if fm:
            items = fm.group(1).strip()
            mtxt = mtxt.replace(fm.group(0), "testkit = [%s]" % ((items + ", " if items else "") + '"%s/testkit"' % c["pkg"]), 1)
    open(man, "w").write(mtxt)
    open(root, "w").write(newold)
    # owners
    otxt = open(R + "/📋️build-owners.json").read()
    for c in created:
        mrel = os.path.relpath(os.path.join(c["newpkgdir"], "Cargo.toml"), R)
        rrel = os.path.relpath(c["newpkgdir"], R)
        oname = f"@semio-tech/{c['id']}-rs" if c["fw"] else c["pkg"]
        if c["fw"]:
            mrel = os.path.join(rrel, "📋️project.json")
        row = f'    {{\n      "name": "{oname}",\n      "root": "{rrel}",\n      "manifest": "{mrel}",\n      "packages": []\n    }},\n'
        names_ = [(m.start(), m.group(1)) for m in re.finditer(r'    \{\n      "name": "([^"]+)"', otxt)]
        if f'"{oname}"' in otxt:
            die("owner row exists")
        pos = next((p for p, nme in names_ if nme > oname), None)
        if pos is None:
            die("owner position")
        otxt = otxt[:pos] + row + otxt[pos:]
    json.loads(otxt)
    open(R + "/📋️build-owners.json", "w").write(otxt)
    fwc = [c for c in created if c["fw"]]
    if fwc:
        ridx = os.path.join(os.path.dirname(os.path.abspath(__file__)), "route-index.txt")
        ltxt = open(R + "/.vscode/launch.json").read()
        for c in fwc:
            k = int(open(ridx).read())
            open(ridx, "w").write(str(k + 1))
            a_, b_ = 2 * k - 1, 2 * k
            def route(label, target, conf, group, order):
                return (f'    {{\n      "name": "🦀️ {c["pkg"]} {label}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/{c["id"]}-rs:{target} --configuration={conf} --parallel=1 -- --lib --offline",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{group}",\n        "order": {order}\n      }}\n    }},\n')
            routes = "".join([route("🏗️build 🛠️prototype", "build", "prototype", "4_build", f"900.96{a_:03d}"), route("🏗️build 🚀️release", "build", "release", "4_build", f"900.96{b_:03d}"),
                route("🔎️check 🛠️prototype", "check", "prototype", "4_gate", f"922.96{a_:03d}"), route("🔎️check 🚀️release", "check", "release", "4_gate", f"922.96{b_:03d}"),
                route("🧪️test 🛠️prototype", "test", "prototype", "4_test", f"903.96{a_:03d}"), route("🧪️test 🚀️release", "test", "release", "4_test", f"903.96{b_:03d}")])
            for o in (f"900.96{a_:03d}", f"900.96{b_:03d}"):
                assert f'"order": {o}\n' not in ltxt, ("order used", o)
            last = [m.end() for m in re.finditer(r'"name": "🦀️ %s [^"]+",\n(?:.*\n)*?    \},\n' % re.escape(pkg), ltxt)]
            if not last:
                die("old crate routes not found")
            ltxt = ltxt[:last[-1]] + routes + ltxt[last[-1]:]
        json.loads(ltxt)
        open(R + "/.vscode/launch.json", "w").write(ltxt)
    with open(os.path.dirname(os.path.abspath(__file__)) + "/created.tsv", "a") as f:
        for c in created:
            f.write(f"{pkg}\t{c['pkg']}\t{os.path.relpath(c['newdir'], R)}\t{len(c['g'])}\n")
    print("written")


main()
