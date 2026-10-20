#!/usr/bin/env python3
"""🪓️ gran-value staged carves, second set (run after apply.py groups): anchored, all-or-nothing, dry run by default.

groups, in order:
  facade-tail   the facade stops exporting ordered, snapshot_projection, paged_text, retirement_queue and OriginalAliasBatch; their 37 consumer crates import the owner crates
  hygiene       unused and test-only normal dependency edges into the value crates leave their manifests (each re-proved against the sources at plan time; manifests already edited by facade-tail are skipped)
  rings         crate-boundaries law: ring-0 and ring-1 value crates may not depend on a hotter ring

usage:
  apply2.py [--root DIR] [--group NAME[,NAME]|all] [--consumers FILE] [--diff] [--write] [--status] [--ledger DIR]

Same contract as apply.py: nothing is written unless every selected group plans cleanly; a failed write restores every file from the ledger.
"""
import argparse, difflib, hashlib, importlib.util, json, os, re, subprocess, sys, tempfile, time

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("gv_apply", HERE + "/apply.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
Plan, Refusal, replace_once, dep_remove, section = base.Plan, base.Refusal, base.replace_once, base.dep_remove, base.section
TICKET = os.path.abspath(HERE + "/../..")
GEN = TICKET + "/🗑️generated/gran-value"
V = base.V
FACADE_PKG = base.FACADE
NAMES = ["ordered", "retirement_queue", "snapshot_projection", "paged_text", "OriginalAliasBatch"]
TAIL_DEPS = ["semio-framework-value-ordered", "semio-framework-value-snapshot-projection", "semio-framework-value-paged-text", "semio-framework-value-retirement-queue", "semio-framework-value-original-aliases"]
TAIL_LIBS = f"{V}/🦀️.rs"
TAIL_PKG_LIB = f"{V}/📦️packages/🦀️rust/🦀️.rs"
RINGS = f"{base.FW}/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
RING0 = ["control-contract", "allocation-return", "paged-list", "retirement", "shared-issuance", "native-encoding", "native-decoding", "intrinsic"]
RING1 = ["ordered", "clone-source", "retained-clone", "persistent", "native-owned", "factory", "snapshot-projection", "original-aliases"]
RING2 = ["edit", "registry", "schema", "retained-map", "paged-text", "retirement-queue", "snapshot-read", "dictionary", "derive-expansion"]


def analysis(root):
    """🧰️ Imports the module-tree resolver of the analysis folder bound to the given root."""
    os.environ["GRAN_ROOT"] = root
    sys.path.insert(0, os.path.abspath(HERE + "/../analysis"))
    import tree
    import importlib
    importlib.reload(tree)
    return tree


def manifest_roots(root, rel_dir, tree):
    """🎯 Lib root plus bin, test, example, bench and build roots of one crate directory."""
    text = open(os.path.join(root, rel_dir, "Cargo.toml"), encoding="utf-8").read()
    lib = re.search(r"^\[lib\][^\[]*?^\s*path\s*=\s*\"([^\"]+)\"", text, re.S | re.M)
    libpath = lib.group(1) if lib else "src/lib.rs"
    prod = [os.path.normpath(os.path.join(root, rel_dir, libpath))]
    test = []
    for m in re.finditer(r"^\[\[(bin|test|example|bench)\]\](.*?)(?=^\[|\Z)", text, re.S | re.M):
        pm = re.search(r'^\s*path\s*=\s*"([^"]+)"', m.group(2), re.M)
        if pm:
            (prod if m.group(1) == "bin" else test).append(os.path.normpath(os.path.join(root, rel_dir, pm.group(1))))
    bm = re.search(r'^\s*build\s*=\s*"([^"]+)"', text, re.M)
    if bm:
        prod.append(os.path.normpath(os.path.join(root, rel_dir, bm.group(1))))
    for sub in ("src/main.rs", "build.rs"):
        if os.path.exists(os.path.join(root, rel_dir, sub)):
            prod.append(os.path.join(root, rel_dir, sub))
    return prod, test


IMPLICIT = {"semio-framework-value": ("ToValue", "FromValue", "RetireOwned", "CanonicalJsonTree", "RetainedClone", "FactoryPayloadRetirement", "value_codec", "SnapshotProjection", "NativeReceive", "RetainedEquality"), "semio-framework-value-derive": ("ToValue", "FromValue", "RetireOwned", "CanonicalJsonTree", "RetainedClone", "FactoryPayloadRetirement", "value_codec", "SnapshotProjection", "NativeReceive", "RetainedEquality", "owned_json_file"), "semio-framework-value-native-owned": ("NativeReceive",), "semio-framework-value-persistent": ("NativeReceive", "RetainedEquality")}


def classify(root, rel_dir, dep, tree, cache):
    """🔬 prod, test or none: textual use of the dependency in the crate's module trees (derive-implied roots included)."""
    key = rel_dir
    if key not in cache:
        prod_roots, test_roots = manifest_roots(root, rel_dir, tree)
        pf = set()
        for r in prod_roots:
            pf |= set(tree.module_tree(r))
        tf = set()
        for r in test_roots:
            tf |= set(tree.module_tree(r))
        read = lambda fs: [re.sub(r"//[^\n]*", "", open(f, encoding="utf-8", errors="ignore").read()) for f in sorted(fs)]
        P = "\n".join(t for f, t in zip(sorted(pf), read(pf)) if "🧪️" not in f)
        T = "\n".join(t for f, t in zip(sorted(pf), read(pf)) if "🧪️" in f) + "\n" + "\n".join(read(tf))
        cache[key] = (P, T)
    P, T = cache[key]
    ident = dep.replace("-", "_")
    if re.search(r"\b" + ident + r"\b", P) or any(re.search(r"derive\([^)]*\b" + x + r"\b", P) for x in IMPLICIT.get(dep, ())):
        return "prod"
    if re.search(r"\b" + ident + r"\b", T) or any(re.search(r"derive\([^)]*\b" + x + r"\b", T) for x in IMPLICIT.get(dep, ())):
        return "test"
    return "none"


def features_mention(text, dep):
    sec = re.search(r"^\[features\](.*?)(?=^\[|\Z)", text, re.S | re.M)
    return bool(sec and re.search(r'(dep:)?\b' + re.escape(dep) + r'\b', sec.group(1)))


def group_hygiene(plan, ctx):
    """🧹 Deletes unused and demotes test-only normal edges; skips (and counts) anything not provable or not single-line."""
    tree = analysis(plan.root)
    ledger = json.load(open(HERE + "/../data/hygiene-edges.json", encoding="utf-8"))
    cache, skipped, done = {}, {}, {"unused": 0, "test-only": 0}
    for e in ledger:
        rel = os.path.relpath(e["dir"], "/Users/ueli/Documents/semio") if e["dir"].startswith("/") else e["dir"]
        mf = rel + "/Cargo.toml"
        if not os.path.isfile(plan.abs(mf)):
            skipped.setdefault("missing manifest", []).append(e["crate"]); continue
        want = "none" if e["verdict"] == "unused" else "test"
        if classify(plan.root, rel, e["dependency"], tree, cache) != want:
            skipped.setdefault("verdict drifted", []).append(f'{e["crate"]}->{e["dependency"]}'); continue
        if mf in ctx.get("tail_manifests", set()):
            skipped.setdefault("manifest already edited by facade-tail", []).append(e["crate"]); continue
        text = plan.text(mf)
        if features_mention(text, e["dependency"]):
            skipped.setdefault("named by a feature", []).append(e["crate"]); continue
        lines = text.split("\n")
        try:
            a, b = section(lines, "[dependencies]")
        except Refusal:
            skipped.setdefault("no [dependencies] table", []).append(e["crate"]); continue
        rx = re.compile(r"^" + re.escape(e["dependency"]) + r"(\s*=\s*(\{.*\}|\".*\")|\.workspace\s*=\s*true)\s*$")
        hits = [i for i in range(a, b) if rx.match(lines[i])]
        if len(hits) != 1 or "optional" in lines[hits[0]]:
            skipped.setdefault("not a single plain entry", []).append(f'{e["crate"]}->{e["dependency"]}'); continue
        line = lines[hits[0]]
        del lines[hits[0]]
        if want == "test":
            try:
                da, db = section(lines, "[dev-dependencies]")
                if not any(re.match(r"^" + re.escape(e["dependency"]) + r"(\s*=|\.)", lines[i]) for i in range(da, db)):
                    lines.insert(da, line)
            except Refusal:
                if lines and lines[-1] == "":
                    lines.pop()
                lines += ["", "[dev-dependencies]", line, ""]
        plan.put(mf, "\n".join(lines))
        done[e["verdict"]] += 1
    ctx["hygiene"] = {"done": done, "skipped": {k: len(v) for k, v in skipped.items()}, "skipped_detail": skipped}


def group_facade_tail(plan, ctx):
    """🪢 Facade tail: five re-exports and five manifest edges leave the facade, consumers import the owners."""
    replace_once(plan, TAIL_LIBS, "pub use semio_framework_value_ordered::ordered;\n", "")
    replace_once(plan, TAIL_LIBS, "pub use semio_framework_value_snapshot_projection::snapshot_projection;\n", "")
    replace_once(plan, TAIL_LIBS, "pub use semio_framework_value_paged_text::paged_text;\n", "")
    replace_once(plan, TAIL_PKG_LIB, "pub use semio_framework_value_retirement_queue as retirement_queue;\n", "")
    replace_once(plan, TAIL_PKG_LIB, "pub use semio_framework_value_original_aliases::OriginalAliasBatch;\n", "")
    for d in TAIL_DEPS:
        dep_remove(plan, FACADE_PKG, "[dependencies]", d)
    consumers = json.load(open(ctx["consumers"], encoding="utf-8"))
    default_root = "/Users/ueli/Documents/semio"
    if plan.root != default_root:
        consumers = [{**c, "dir": c["dir"].replace(default_root, plan.root), "files": [f.replace(default_root, plan.root) for f in c["files"]]} for c in consumers]
    with tempfile.TemporaryDirectory() as tmp:
        cj, out = tmp + "/consumers.json", tmp + "/plan.json"
        json.dump(consumers, open(cj, "w", encoding="utf-8"), ensure_ascii=False)
        r = subprocess.run(["bun", HERE + "/repoint/repoint.ts", "--consumers", cj, "--out", out], capture_output=True, text=True, env={**os.environ, "GRAN_ROOT": plan.root})
        if r.returncode != 0:
            raise Refusal("repoint planner failed: " + r.stderr[-400:])
        result = json.load(open(out, encoding="utf-8"))
    if result["issues"]:
        raise Refusal(f'repoint planner issues: {result["issues"][:3]}')
    if result["notes"]:
        raise Refusal(f'repoint planner notes (unresolved paths): {result["notes"][:3]}')
    for item in result["sources"] + result["manifests"]:
        cur = plan.text(item["path"])
        if hashlib.sha256(cur.encode()).hexdigest() != item["before"]:
            raise Refusal(f'{item["path"]}: changed between planning and merge')
        plan.put(item["path"], item["after"])
    rx = re.compile(r"(?:semio_framework_value|\bvalue|__value)\s*::\s*(?:" + "|".join(NAMES) + r")\b")
    left = [c["name"] for c in consumers for f in c["files"] if os.path.relpath(f, plan.root) in plan.new and rx.search(plan.new[os.path.relpath(f, plan.root)])]
    if left:
        raise Refusal(f"residual facade tail references after the rewrite in {sorted(set(left))[:5]}")
    ctx["tail_manifests"] = {m["path"] for m in result["manifests"]}
    ctx["tail"] = {"sources": len(result["sources"]), "manifests": len(result["manifests"]), "rewrites": result["rewrites"]}


def group_rings(plan, ctx):
    """💍 Ring order as a crate-boundaries law: R0 may not depend on R1 or R2, R1 not on R2; entries a crate already declares are not listed (no vacuous row)."""
    text = plan.text(RINGS)
    doc = json.loads(text)
    if json.dumps(doc, indent=2, ensure_ascii=False) + "\n" != text:
        raise Refusal("rings fixture is not canonical json, refusing to reformat")
    full = lambda n: "semio-framework-value-" + n
    forbid = {}
    for n in RING0:
        forbid[full(n)] = [full(x) for x in RING1 + RING2]
    for n in RING1:
        forbid[full(n)] = [full(x) for x in RING2]
    touched = 0
    for row in doc["crates"]:
        add = forbid.get(row["package"])
        if not add:
            continue
        new = [x for x in add if x not in row["forbiddenDependencies"] and x != row["package"]]
        row["forbiddenDependencies"] += new
        touched += 1 if new else 0
    plan.put(RINGS, json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
    ctx["rings"] = {"rows": touched}


GROUPS = {"facade-tail": group_facade_tail, "hygiene": group_hygiene, "rings": group_rings}


def done(root, name):
    """✅️ True when the group's end state is on disk."""
    try:
        if name == "facade-tail":
            return "semio-framework-value-retirement-queue" not in open(os.path.join(root, FACADE_PKG), encoding="utf-8").read()
        if name == "rings":
            return "semio-framework-value-registry" in open(os.path.join(root, RINGS), encoding="utf-8").read().split('"semio-framework-value-control-contract"')[1][:1500]
        return False
    except OSError:
        return False


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=base.DEFAULT_ROOT)
    ap.add_argument("--group", default="all")
    ap.add_argument("--consumers", default=GEN + "/consumers-g5.json")
    ap.add_argument("--diff", action="store_true")
    ap.add_argument("--files", action="store_true")
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--status", action="store_true")
    ap.add_argument("--ledger", default=None)
    a = ap.parse_args()
    names = list(GROUPS) if a.group == "all" else a.group.split(",")
    for n in names:
        if n not in GROUPS:
            sys.exit(f"unknown group {n}")
    if a.status:
        for n in names:
            print(f"{n}: {'applied' if done(a.root, n) else 'pending'}")
        return 0
    plan, ctx = Plan(a.root), {"consumers": a.consumers}
    try:
        for n in names:
            GROUPS[n](plan, ctx)
    except Refusal as e:
        print(f"REFUSED: {e}")
        return 2
    ch = plan.changed()
    if a.diff:
        for rel, text in sorted(ch.items()):
            old = plan.orig[rel]
            print("\n".join(difflib.unified_diff((old or "").splitlines(), text.splitlines(), rel, rel, lineterm="", n=0)))
    if a.files:
        print("\n".join(sorted(ch)))
    kinds = {"new": sum(1 for r in ch if plan.orig[r] is None), "edited": sum(1 for r in ch if plan.orig[r] is not None)}
    print(json.dumps({k: v for k, v in ctx.items() if k in ("tail", "rings")} | ({"hygiene": {k: v for k, v in ctx["hygiene"].items() if k != "skipped_detail"}} if "hygiene" in ctx else {}), ensure_ascii=False))
    print(f"{len(ch)} files {kinds}, groups {names}, {'WRITE' if a.write else 'dry run'}")
    if a.write:
        ledger = a.ledger or os.path.join(a.root, base.TICKET, "🗑️generated", "gran-value-apply2", time.strftime("%Y%m%d-%H%M%S"))
        base.write(plan, ledger)
        print(f"written; ledger {ledger}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
