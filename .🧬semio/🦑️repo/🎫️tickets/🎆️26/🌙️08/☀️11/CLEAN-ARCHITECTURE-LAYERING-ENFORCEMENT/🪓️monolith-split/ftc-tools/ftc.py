"""🧪 FTC driver: pre-edit snapshots of crate source closures (sandbox only), the companion-aware union oracle, the test-name multiset and the fixture byte-identity proof.
usage:
  ftc.py snap <key> <pkg>...                 copy the reachable source closure + manifest of each package into scratchpad/ftc-snap/<key>
  ftc.py oracle <family.json> [--cfgs ...]   union differential: base = snapshot, cur = working tree (libNoTest crates are built without cfg(test))
  ftc.py tests <key> <pkg>...                multiset of test fn names in the closure of working tree vs snapshot
  ftc.py hashes <key> <pkg>...               sha256 of every non-rs file reachable in the snapshot vs the working tree (same rel path)
"""
import collections, hashlib, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ftclib import *
import ftcunion as U
from ftcstub import mask

SNAP = SCRATCH + "/ftc-snap"


def snap_root(key):
    return SNAP + "/" + key


def closure(src, mans, pkg):
    man = mans.manifest(pkg, src)
    if man is None:
        return None, {}, set()
    lib_rel = mans.lib_rel(man)
    files, code, missing = U.reach(src, lib_rel, man["dir"])
    return man, files, code


def cmd_snap(key, pkgs):
    mans = Manifests(Source(None))
    mans.build_index()
    root = snap_root(key)
    sb = Sandbox(SANDBOX_BASE)
    if os.path.exists(root):
        rp = os.path.realpath(root)
        assert rp.startswith(os.path.realpath(SCRATCH) + os.sep)
        shutil.rmtree(rp)
    os.makedirs(root, exist_ok=True)
    src = Source(None)
    meta = {}
    for pkg in pkgs:
        man, files, code = closure(src, mans, pkg)
        assert man is not None, pkg
        n = 0
        for rel, data in list(files.items()) + [(man["rel"], src.read(man["rel"]))]:
            dest = os.path.join(root, rel)
            assert os.path.realpath(os.path.dirname(dest) if os.path.exists(os.path.dirname(dest)) else root).startswith(os.path.realpath(SCRATCH))
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            with open(dest, "wb") as f:
                f.write(data)
            n += 1
        meta[pkg] = dict(manifest=man["rel"], files=n, sha={rel: hashlib.sha256(d).hexdigest() for rel, d in files.items() if rel not in code})
        print(f"snapshot {pkg}: {n} files")
    with open(root + "/__meta.json", "w", encoding="utf-8") as f:
        json.dump(meta, f, ensure_ascii=False)


TEST_RX = re.compile(r"#\[\s*(?:[A-Za-z_][\w:]*::)?(?:test|async_test|tokio::test|wasm_bindgen_test)\b[^\]]*\]((?:\s*#\[[^\]]*\])*)\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)")


def test_names(files, code):
    out = collections.Counter()
    for rel in code:
        t = mask(files[rel].decode("utf-8", "replace"))
        raw = files[rel].decode("utf-8", "replace")
        for m in TEST_RX.finditer(t):
            out[m.group(2)] += 1
    return out


def cmd_tests(key, pkgs):
    mans = Manifests(Source(None))
    mans.build_index()
    snap = Source("snap:" + snap_root(key))
    cur = Source(None)
    base_c, cur_c = collections.Counter(), collections.Counter()
    for pkg in json.load(open(snap_root(key) + "/__meta.json", encoding="utf-8")):
        man, files, code = closure(snap, mans, pkg)
        base_c += test_names(files, code)
    for pkg in pkgs:
        man, files, code = closure(cur, mans, pkg)
        assert man is not None, pkg
        cur_c += test_names(files, code)
    only_b, only_c = base_c - cur_c, cur_c - base_c
    print(f"[{key}] baseline tests {sum(base_c.values())} ({len(base_c)} names); current {sum(cur_c.values())} ({len(cur_c)} names)")
    print("IDENTICAL multiset" if not only_b and not only_c else f"DIFFERENT only-baseline={dict(only_b)} only-current={dict(only_c)}")
    return not only_b and not only_c


def cmd_hashes(key, pkgs):
    meta = json.load(open(snap_root(key) + "/__meta.json", encoding="utf-8"))
    bad = 0
    n = 0
    for pkg, m in meta.items():
        for rel, h in m["sha"].items():
            p = os.path.join(ROOT, rel)
            n += 1
            cur = hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else "MISSING"
            if cur != h:
                bad += 1
                print("CHANGED", rel, h[:12], cur[:12])
    print(f"[{key}] checked {n} embedded/raw inputs of the baseline closure; changed or missing: {bad}")
    return bad == 0


def main():
    a = sys.argv[1:]
    if a[0] == "snap":
        cmd_snap(a[1], a[2:])
    elif a[0] == "tests":
        cmd_tests(a[1], a[2:])
    elif a[0] == "files":
        mans = Manifests(Source(None))
        mans.build_index()
        for pkg in a[1:]:
            man, files, code = closure(Source(None), mans, pkg)
            print(",".join(sorted(code)))
    elif a[0] == "hashes":
        cmd_hashes(a[1], a[2:])
    elif a[0] == "oracle":
        import ftcpos
        sys.argv = ["ftcpos.py", "run", *a[1:]]
        ftcpos.main()


if __name__ == "__main__":
    main()
