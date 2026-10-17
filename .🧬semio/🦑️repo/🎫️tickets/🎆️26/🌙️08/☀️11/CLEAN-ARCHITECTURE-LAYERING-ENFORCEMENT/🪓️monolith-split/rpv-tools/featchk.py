"""🧭 RPV testkit-wiring check: simulates the Cargo (resolver 2) feature propagation of a `cargo test -p <user>` build (the user's dev-dependencies count, those of its dependencies do not) and reports whether each owner crate receives feature `testkit`.
The union oracle cannot see this: it turns `test` on for every crate.
usage: featchk.py <user-pkg> <owner-pkg> [<feature>=testkit] [--lib]   (--lib: dev-dependencies of the user are ignored)"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/../pos-tools")
from poslib import *


def resolve(user, with_dev, mans):
    en = {}
    work = [(user, "default")]
    en[user] = {"default"}
    seen_expand = set()
    while work:
        pkg, feat = work.pop()
        m = mans.manifest(pkg)
        if m is None:
            continue
        tab = mans.features_table(m)
        deps = mans.deps(m)
        if feat == "default" and "default" not in tab:
            pass
        for entry in tab.get(feat, []):
            if entry.startswith("dep:"):
                continue
            if "/" in entry:
                dname, ff = entry.split("/", 1)
                dname = dname.rstrip("?")
                for kind, name, dpkg, feats, opt, renamed in deps:
                    if name == dname and kind in ("dependencies",) + (("dev-dependencies",) if with_dev and pkg == user else ()):
                        if ff not in en.setdefault(dpkg, set()):
                            en[dpkg].add(ff)
                            work.append((dpkg, ff))
            elif entry not in en[pkg]:
                en[pkg].add(entry)
                work.append((pkg, entry))
        if (pkg, "__deps__") in seen_expand:
            continue
        seen_expand.add((pkg, "__deps__"))
        for kind, name, dpkg, feats, opt, renamed in deps:
            if kind == "dependencies" or (kind == "dev-dependencies" and with_dev and pkg == user):
                if opt:
                    continue
                s = en.setdefault(dpkg, set())
                for f in feats:
                    if f not in s:
                        s.add(f)
                        work.append((dpkg, f))
                if dpkg not in en or True:
                    work.append((dpkg, "__deps__"))
    return en


def main():
    a = sys.argv[1:]
    mans = Manifests(Source(None))
    mans.build_index()
    lib = "--lib" in a
    a = [x for x in a if x != "--lib"]
    user, owner = a[0], a[1]
    feat = a[2] if len(a) > 2 else "testkit"
    en = resolve(user, not lib, mans)
    ok = feat in en.get(owner, set())
    print(f"{'OK  ' if ok else 'MISS'} {user.replace('semio-framework-os-renderer-wgpu', '~')} -> {owner.replace('semio-framework-os-renderer-wgpu', '~')} feature {feat}: {'enabled' if ok else 'NOT enabled'} ({'lib' if lib else 'test'} build)")
    sys.exit(0 if ok else 1)


main()
