"""Derive overrides.json from unused-import warnings of the staged tree (rustc analysis only, output to /dev/null)."""
import os, re, sys, json, collections, shutil
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *
import oracle, plans, osmeng

os.environ["OSM_LINTS"] = "1"
oracle.LINT = True
KEEP = {("remote:binding_helpers", "CANONICAL_CHECKPOINT_RESOURCE_SCHEMA")}
WARN = re.compile(r"^warning: unused imports?: (.*)$")
NAMES = re.compile(r"`([^`]+)`")


def collect(err, root):
    L = err.split("\n")
    found = []
    for i, line in enumerate(L):
        m = WARN.match(line)
        if not m:
            continue
        names = [n.split("::")[-1] for n in NAMES.findall(m.group(1))]
        loc = None
        for j in range(i + 1, min(i + 6, len(L))):
            mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
            if mm:
                loc = mm.group(1)
                break
        found.append((loc, names))
    return found


def main():
    targets = sys.argv[1].split(",")
    CONFIGS = {"services": ["lib", "test"], "transport": ["lib", "test"], "remote": ["lib", "test", "wasm", "wasmtest"], "bridge": ["lib", "test", "wasm", "wasmtest"]}
    ov = {}
    for name in targets:
        t = TARGETS[name]
        snapkey = "services" if name == "services" else "mcp"
        os.environ["OSM_STAGE"] = "1"
        root, _ = oracle.fresh(snapkey, "B", t["crate"], t["tree"])
        per = collections.defaultdict(lambda: collections.defaultdict(set))
        for c in CONFIGS[name]:
            err = oracle.run_rustc(root, t["lib"], t["crate"], oracle.CFG[c]).replace(root, "<S>")
            write(f"{F}/🗑️generated/osm-prune-{name}-{c}.err", err)
            for loc, names in collect(err, root):
                if loc is None:
                    continue
                for n in names:
                    per[(loc, n)][c].add(1)
        files = plans.GROUPS[name]()
        base_dirs = {}
        for f in files:
            d = os.path.dirname(f.rel)
            for ch in f.children:
                base_dirs[os.path.join(d, ch["dir"], "🦀️.rs")] = f"{f.label}:{ch['mod']}"
            base_dirs[f.rel] = f"{f.label}:<exports>"
        for (loc, n), cfgs in per.items():
            rel = loc.replace("<S>/", "")
            rel = os.path.normpath(rel)
            key = None
            for suffix_rel, k in base_dirs.items():
                if os.path.normpath(rel).endswith(os.path.normpath(suffix_rel)) or os.path.normpath(rel).endswith(os.path.normpath(suffix_rel.split("/", 0)[0])) and False:
                    key = k
            if key is None:
                for suffix_rel, k in base_dirs.items():
                    tail = "/".join(suffix_rel.split("/")[-3:])
                    if rel.endswith(tail):
                        key = k
            if key is None:
                print("UNMAPPED", rel, n)
                continue
            all_cfgs = set(CONFIGS[name])
            flagged = set(cfgs)
            test_cfgs = {c for c in all_cfgs if c.endswith("test")}
            if (key, n) in KEEP:
                print("KEEP", key, n, "(use hidden inside an unresolved external macro)")
                continue
            if flagged == all_cfgs:
                ov.setdefault(key, {}).setdefault("drop", []).append(n)
            elif not (flagged & test_cfgs):
                ov.setdefault(key, {}).setdefault("cfg_test", []).append(n)
            else:
                print("UNDECIDED", key, n, sorted(flagged))
    return ov


if __name__ == "__main__":
    ov = main()
    out = os.path.join(os.path.dirname(__file__), "overrides.json")
    old = json.load(open(out)) if os.path.exists(out) else {}
    for k, v in ov.items():
        for kind, names in v.items():
            cur = set(old.setdefault(k, {}).setdefault(kind, []))
            cur |= set(names)
            old[k][kind] = sorted(cur)
    write(out, json.dumps(old, ensure_ascii=False, indent=1) + "\n")
    print(json.dumps(ov, ensure_ascii=False, indent=1))
