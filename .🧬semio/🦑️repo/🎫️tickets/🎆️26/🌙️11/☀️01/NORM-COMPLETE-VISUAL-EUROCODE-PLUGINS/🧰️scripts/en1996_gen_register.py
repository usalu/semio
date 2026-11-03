"""Registers the EN 1996 wave 2 mutation group crates: workspace members/dependencies, dependents, four Cargo.lock files, build-owners and dependencies. Idempotent."""
import os, re, json
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))

NEW = list(CRATES)
BASE = "semio-s-artifact-norm-en1996-mut-openings"
REL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
FULL = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/" + REL


def ws_toml():
    p = f"{ROOT}/Cargo.toml"
    s = rd(p)
    for c in NEW:
        member = f'    "{REL}/{crate_dir(c)}/📦️packages/🦀️rust",\n'
        if member not in s:
            anchor = f'    "{REL}/🧱️walls/📦️packages/🦀️rust",\n'
            s = s.replace(anchor, anchor + member)
        block = f'[workspace.dependencies.{crate_pkg(c)}]\npath = "{REL}/{crate_dir(c)}/📦️packages/🦀️rust"\n\n'
        if f"[workspace.dependencies.{crate_pkg(c)}]" not in s:
            anchor = f'[workspace.dependencies.{BASE}]\npath = "{REL}/🪟️openings/📦️packages/🦀️rust"\n\n'
            assert anchor in s
            s = s.replace(anchor, anchor + block)
    wr(p, s)


def dependents():
    for p in (f"{MUT}/📦️packages/🦀️rust/Cargo.toml", f"{IO}/📦️packages/🦀️rust/Cargo.toml", f"{IO}/📝️text/📦️packages/🦀️rust/Cargo.toml"):
        s = rd(p)
        line = f"{BASE} = {{ workspace = true }}\n"
        assert line in s, p
        add = "".join(f"{crate_pkg(c)} = {{ workspace = true }}\n" for c in NEW if f"{crate_pkg(c)} =" not in s)
        wr(p, s.replace(line, line + add))


def lock(p):
    s = rd(p)
    blocks = re.split(r"(?=\[\[package\]\]\n)", s)
    head, pk = blocks[0], blocks[1:]
    names = [re.search(r'name = "([^"]+)"', b).group(1) for b in pk]
    tmpl = pk[names.index(BASE)]
    out = []
    for b in pk:
        if f' "{BASE}",\n' in b and f'name = "{BASE}"' not in b:
            m = re.search(r"dependencies = \[\n(.*?)\]\n", b, re.S)
            deps = re.findall(r' "([^"]+)",', m.group(1))
            for c in NEW:
                if crate_pkg(c) not in deps:
                    deps.append(crate_pkg(c))
            deps.sort()
            b = b[: m.start(1)] + "".join(f' "{d}",\n' for d in deps) + b[m.end(1):]
        out.append(b)
    def nm(b):
        return re.search(r'name = "([^"]+)"', b).group(1)
    for c in NEW:
        n = crate_pkg(c)
        if n in {nm(b) for b in out}:
            continue
        blk = tmpl.replace(BASE, n)
        if not blk.endswith("\n\n"):
            blk = blk.rstrip("\n") + "\n\n"
        i = next((k for k, b in enumerate(out) if nm(b) > n), len(out))
        out.insert(i, blk)
    wr(p, head + "".join(out))


def owners():
    p = "/home/deck/semio/📋️build-owners.json"
    d = jload(p)
    owners = d["owners"]
    have = {o["name"] for o in owners}
    tmpl = next(o for o in owners if o["name"] == BASE)
    for c in NEW:
        n = crate_pkg(c)
        if n in have:
            continue
        o = copy.deepcopy(tmpl)
        o["name"] = n
        for k in ("root", "manifest"):
            o[k] = o[k].replace("🪟️openings", crate_dir(c))
        idx = next(i for i, x in enumerate(owners) if x["name"] == "semio-s-artifact-norm-en1996-mut-openings")
        owners.insert(idx, o)
    owners.sort(key=lambda o: 0) if False else None
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def deps():
    p = "/home/deck/semio/🔒️dependencies.json"
    s = rd(p)
    line = f'        "{FULL}/🪟️openings/📦️packages/🦀️rust/Cargo.toml",\n'
    assert line in s
    add = ""
    for c in NEW:
        l = f'        "{FULL}/{crate_dir(c)}/📦️packages/🦀️rust/Cargo.toml",\n'
        if l not in s:
            add += l
    wr(p, s.replace(line, line + add))


ws_toml()
dependents()
for lk in (f"{ROOT}/Cargo.lock", "/home/deck/semio/🌎️hub/Cargo.lock", "/home/deck/semio/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock"):
    lock(lk)
owners()
deps()
print("registered", NEW)
