"""🔬️ Stub-layer differential for the RENDERER-TEST-DEPS crates (RESOLUTION-ORACLE.md, frs-tools reused): baseline closure files (rtd-snap) vs the live files, per crate, --test and --lib, external crates stubbed from each side's own manifest so an undeclared crate stays an error.
Usage: RTD_WORK=<scratch dir holding base/ from rtd-snap.ts> python3 rtd-oracle.py <crate-suffix,...|all> [--cfgs test,lib] ; sandboxes live under $RTD_WORK/oracle (outside the repo and the ticket tree: copies of tracked sources exceed the path limit), error files under 🗑️generated/rtd-oracle-*.
@see ../RESULT-RENDERER-TEST-DEPS.md"""
import os, re, sys, json, shutil, collections
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ROOT = "/Users/ueli/Documents/semio"
sys.path.insert(0, F + "/frs-tools")
import frslib, oracle, stubgen

WORK = os.environ["RTD_WORK"]
W2 = WORK + "/oracle"


def infer_arity_fast(alltext, found):
    """Same result as stubgen.infer_arity (max lifetime/type arity of every `Leaf<` use) in one pass over the text instead of one scan per leaf."""
    leaves = set()
    for crate, d in found.items():
        for segs, kind in d.items():
            if segs and re.match(r"^[A-Z]", segs[-1]) and kind != "macro":
                leaves.add(segs[-1])
    out = {}
    for m in re.finditer(r"(?<![A-Za-z0-9_])([A-Za-z_][A-Za-z0-9_]*)\s*(?:::\s*)?<", alltext):
        leaf = m.group(1)
        if leaf not in leaves:
            continue
        r = stubgen.generic_args(alltext, m.end() - 1)
        if r:
            L, T = out.get(leaf, (0, 0))
            out[leaf] = (max(L, r[0]), max(T, r[1]))
    return out


TRAIT_GENERIC = [
    re.compile(r"\bimpl\s+(?:<[^>]*>\s*)?(?:\w+::)*(\w+)(?:<[^>]*>)?\s+for\b"),
    re.compile(r"\bdyn\s+(?:\w+::)*(\w+)\b"),
    re.compile(r"->\s*impl\s+(?:\w+::)*(\w+)\b"),
]
TRAIT_PLUS = re.compile(r"(?:\bdyn|\bimpl)\s+([^;{]*?\+[^;{]*)")


def collect_fast(files, crates):
    """stubgen.collect with the per-leaf trait scans replaced by one scan per pattern (same classification, linear time)."""
    found = {c: {} for c in crates}
    alt = "|".join(sorted(map(re.escape, crates), key=len, reverse=True))
    use_rx = re.compile(r"(?:^|[;{}\s])use\s+(?:::)?((?:" + alt + r")\b[^;]*);", re.S)
    path_rx = re.compile(stubgen.NOT_BEFORE + r"(?:::)?((?:" + alt + r")(?:\s*::\s*" + stubgen.IDENT + r")+)")
    texts = {p: stubgen.mask(stubgen.read(p)) for p in files}
    for p, t in texts.items():
        for m in use_rx.finditer(t):
            for kind, segs in stubgen.expand_use(m.group(1)):
                if segs and segs[0] in found:
                    found[segs[0]].setdefault(tuple(segs[1:]), kind)
        for m in path_rx.finditer(t):
            segs = [x.strip() for x in re.split(r"\s*::\s*", m.group(1))]
            start = m.start(1)
            if "#[" in t[max(0, start - 3):start]:
                continue
            tail = t[m.end(1):m.end(1) + 3]
            kind = "macro" if tail.lstrip().startswith("!") else "item"
            found[segs[0]].setdefault(tuple(segs[1:]), kind)
    alltext = "\n".join(texts.values())
    traitish = set()
    for rx in TRAIT_GENERIC:
        traitish.update(m.group(1) for m in rx.finditer(alltext))
    for m in TRAIT_PLUS.finditer(alltext):
        for part in m.group(1).split("+"):
            mm = re.match(r"\s*(?:\w+::)*(\w+)\b", part)
            if mm:
                traitish.add(mm.group(1))
    for crate, d in found.items():
        for segs in list(d):
            if segs and segs[-1] in traitish and re.match(r"^[A-Z]", segs[-1]) and d[segs] not in ("macro",):
                d[segs] = "trait"
    for crate, d in found.items():
        for segs in list(d):
            if segs and segs[-1] in stubgen.HINT_TRAITS and d[segs] not in ("macro",):
                d[segs] = "trait"
    stubgen.ARITY.clear()
    stubgen.ARITY.update(stubgen.infer_arity(alltext, found))
    return found


if os.environ.get("RTD_FAST_ARITY", "1") == "1":
    stubgen.infer_arity = infer_arity_fast
    stubgen.collect = collect_fast
frslib.W = W2
oracle.W = W2
BASE = WORK + "/base"
index = json.load(open(BASE + "/index.json"))
CFGS = {"test": ["--test"], "lib": []}


def lib_rel_of(manifest_text, manifest_rel):
    m = re.search(r"^\[lib\]\s*$(.*?)(?=^\[|\Z)", manifest_text, re.S | re.M)
    p = "src/lib.rs"
    if m:
        pm = re.search(r'^path\s*=\s*"([^"]+)"', m.group(1), re.M)
        if pm:
            p = pm.group(1)
    return os.path.relpath(os.path.normpath(os.path.join(os.path.dirname(os.path.join(ROOT, manifest_rel)), p)), ROOT)


def dep_idents(manifest_text):
    out = set()
    section = ""
    for line in manifest_text.split("\n"):
        h = re.match(r"^\[([^\]]+)\]\s*$", line)
        if h:
            section = h.group(1)
            continue
        if section.endswith("dependencies"):
            m = re.match(r"^([A-Za-z0-9_\-]+)\s*=", line)
            if m:
                out.add(m.group(1).replace("-", "_"))
    return out


def feature_names(manifest_text):
    m = re.search(r"^\[features\]\s*$(.*?)(?=^\[|\Z)", manifest_text, re.S | re.M)
    return [k for k in re.findall(r"^([A-Za-z0-9_\-]+)\s*=", m.group(1), re.M)] if m else []


PUB_EXTERN = re.compile(r"^(\s*)(pub(?:\([^)]*\))?)\s+extern\s+crate\s+(\w+)(\s+as\s+\w+)?\s*;", re.M)


def pre_externs(root):
    """`pub extern crate X as Y;` becomes `pub use X as Y;` so the stub layer can rewrite X (the lib root of the facade crate re-exports its dependencies this way)."""
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(dp, f)
            oracle.sandbox_under(p)
            t = open(p, encoding="utf8").read()
            t2 = PUB_EXTERN.sub(lambda m: f"{m.group(1)}{m.group(2)} use {m.group(3)}{m.group(4) or ''};" if m.group(3) not in ("std", "core", "alloc") else m.group(0), t)
            if t2 != t:
                open(p, "w", encoding="utf8").write(t2)


def build(crate, which, files, manifest_rel):
    root = f"{W2}/{crate}/{which}"
    oracle.sandbox_under(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    for f in files:
        src = (f"{BASE}/crates/{crate}/{f}") if which == "A" else f"{ROOT}/{f}"
        if not os.path.exists(src):
            continue
        dst = os.path.join(root, f)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(src, dst)
    mtext = open(f"{BASE}/manifests/{manifest_rel}" if which == "A" else f"{ROOT}/{manifest_rel}", encoding="utf8").read()
    return root, mtext


def main():
    want = sys.argv[1]
    cfgs = ["test", "lib"]
    if "--cfgs" in sys.argv:
        cfgs = sys.argv[sys.argv.index("--cfgs") + 1].split(",")
    crates = [c for c in index if want == "all" or any(c.endswith(w) for w in want.split(","))]
    manifests = {}
    for line in open(F + "/🗑️generated/rtd-manifests.txt", encoding="utf8"):
        line = line.strip()
        if not line:
            continue
        t = open(f"{ROOT}/{line}", encoding="utf8").read()
        m = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
        if m:
            manifests[m.group(1)] = line
    total_bad = 0
    summary = {}
    for crate in crates:
        manifest_rel = manifests[crate]
        files = index[crate]
        rootA, mA = build(crate, "A", files, manifest_rel)
        rootB, mB = build(crate, "B", files, manifest_rel)
        lib_rel = lib_rel_of(mB, manifest_rel)
        snake = crate.replace("-", "_")
        for r_ in (rootA, rootB):
            pre_externs(r_)
        pa = oracle.patch_tree(rootA, snake)
        pb = oracle.patch_tree(rootB, snake)
        extA = sorted(dep_idents(mA) - {snake})
        extB = sorted(dep_idents(mB) - {snake} - set(filter(None, os.environ.get("RTD_DROP_EXT", "").split(","))))
        stubgen.apply([rootA], lib_rel, extA)
        stubgen.apply([rootB], lib_rel, extB)
        feats = feature_names(mB)
        runs = [(c, CFGS[c]) for c in cfgs]
        if feats:
            fx = sum([["--cfg", f'feature="{x}"'] for x in feats], [])
            runs += [(c + "+features", CFGS[c] + fx) for c in cfgs]
        for c, extra in runs:
            ea = oracle.run_rustc(rootA, lib_rel, snake, extra).replace(rootA, "<S>")
            eb = oracle.run_rustc(rootB, lib_rel, snake, extra).replace(rootB, "<S>")
            frslib.write(f"{F}/🗑️generated/rtd-oracle-{crate}-{c}-A.err", ea)
            frslib.write(f"{F}/🗑️generated/rtd-oracle-{crate}-{c}-B.err", eb)
            ka, kb = oracle.parse(ea), oracle.parse(eb)
            ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
            cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
            onlyB, onlyA = cb - ca, ca - cb
            status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
            if status != "IDENTICAL":
                total_bad += 1
            print(f"== {crate.replace('semio-framework-os-renderer-', '')} {c}: baseline={len(ka)} current={len(kb)} {status}")
            for (h, m, l), n in sorted(onlyB.items())[:12]:
                print(f"   +current {n}x {h}: {m[:150]} | {l[:70]}")
            for (h, m, l), n in sorted(onlyA.items())[:12]:
                print(f"   -current {n}x {h}: {m[:150]} | {l[:70]}")
            summary[f"{crate}|{c}"] = dict(base=len(ka), cur=len(kb), status=status, new=sum(onlyB.values()), gone=sum(onlyA.values()))
    frslib.write(f"{F}/🗑️generated/rtd-oracle-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", "ALL IDENTICAL" if total_bad == 0 else f"{total_bad} DIFFERENT")


main()
