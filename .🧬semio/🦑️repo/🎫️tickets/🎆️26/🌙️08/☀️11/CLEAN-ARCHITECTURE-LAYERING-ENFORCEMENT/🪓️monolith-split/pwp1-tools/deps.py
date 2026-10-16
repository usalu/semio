#!/usr/bin/env python3
# 📦️ External-crate evidence of a carved crate and the manifest `[dependencies]` it implies (specs and paths taken from the facade manifest, re-relativised).
# usage: deps.py <crate> [--apply]
import json, os, re, subprocess, sys
from lib import *
import wp1files
from rs import blank
from crates_wp1 import ROOT

FACADE = PLUG + "/" + PKGDIR + "/Cargo.toml"
ALIASES = {"store": "semio-framework-os-kernel", "protocol": "semio-framework-os-kernel", "dsl": "semio-framework-os-kernel", "vcs": "semio-framework-os-kernel"}


def toml_json(path):
    out = subprocess.run(["bun", TOOLS + "/tomljson.ts", path], capture_output=True, text=True)
    return json.loads(out.stdout)


def facade_specs():
    d = toml_json(FACADE)
    return d["dependencies"], d.get("dev-dependencies", {})


def used_keys(files, specs):
    keys = {}
    for k in specs:
        keys[k.replace("-", "_")] = k
    found = set()
    texts = [blank(read(f)) for f in files]
    allt = "\n".join(texts)
    for ident, k in keys.items():
        if ident.startswith("semio_framework_plugin_"):
            continue
        if re.search(r"(?<![A-Za-z0-9_])" + re.escape(ident) + r"(?:::|!|\b\s*(?:as\b|;|,|\}))", allt) and re.search(r"(?<![A-Za-z0-9_$])" + re.escape(ident) + r"\b", allt):
            found.add(k)
    for alias in ALIASES:
        if re.search(r"(?<![A-Za-z0-9_$.:])" + alias + r"::", allt):
            found.add(ALIASES[alias])
    if re.search(r"extern\s+crate\s+semio_framework_os_kernel\b", allt):
        found.add("semio-framework-os-kernel")
    if re.search(r"(?<![A-Za-z0-9_$.:])pack::", allt):
        found.add("semio-framework-os-kernel")
    return found, allt


def rebase(spec, pkg_dir):
    spec = dict(spec) if isinstance(spec, dict) else spec
    if isinstance(spec, dict) and "path" in spec:
        absolute = os.path.normpath(os.path.join(PLUG + "/" + PKGDIR, spec["path"]))
        spec["path"] = os.path.relpath(absolute, pkg_dir)
    return spec


def fmt(k, spec):
    if isinstance(spec, str):
        return f'{k} = "{spec}"'
    parts = []
    for key in ("path", "workspace", "package", "version", "default-features", "features", "optional"):
        if key in spec:
            v = spec[key]
            if isinstance(v, bool):
                parts.append(f"{key} = {'true' if v else 'false'}")
            elif isinstance(v, list):
                parts.append(f"{key} = [" + ", ".join('"%s"' % x for x in v) + "]")
            else:
                parts.append(f'{key} = "{v}"' if key != "workspace" else "workspace = true")
    return f"{k} = {{ " + ", ".join(parts) + " }"



CRATES = json.load(open(GEN + "/pwp0-crates.json"))


def first_party(allt):
    found = set()
    for m in re.finditer(r"(?<![A-Za-z0-9_])semio_framework_plugin_([a-z0-9_]+)(?=::|\s+as\b|;|\s*\})", allt):
        found.add(m.group(1).replace("_", "-"))
    return found


def analyse(crate, root_override=None, extra_prod=(), extra_test=()):
    specs, devspecs = facade_specs()
    root = root_override or (PLUG + "/" + ROOT[crate] + "/🦀️.rs")
    prod, test = wp1files.tree_split(root)
    prod += list(extra_prod)
    test += list(extra_test)
    pk, pall = used_keys(prod, {**specs, **devspecs})
    tk, tall = used_keys(test, {**specs, **devspecs})
    pf = first_party(pall)
    tf = first_party(tall)
    return prod, test, pk, tk, pf, tf, specs, devspecs


def fp_name(f):
    return CRATES[f]["name"] if f in CRATES else "semio-framework-plugin-" + f


def fp_entry(f, pkg, specs):
    if f in CRATES:
        r = CRATES[f]
        return (r["name"], {"path": os.path.relpath(PLUG + "/" + r["root"] + "/" + PKGDIR, pkg)})
    return ("semio-framework-plugin-" + f, rebase(specs["semio-framework-plugin-" + f], pkg))


def render_manifest(crate, pk, tk, pf, tf, specs, devspecs, keep_dev=()):
    pkg = PLUG + "/" + CRATES[crate]["root"] + "/" + PKGDIR
    lines = ["[dependencies]"]
    items = []
    for k in sorted(pk):
        spec = specs.get(k, devspecs.get(k))
        items.append((k, rebase(spec, pkg)))
    for f in sorted(pf):
        items.append(fp_entry(f, pkg, specs))
    for k, v in sorted(items):
        lines.append(fmt(k, v))
    lines += ["", "[dev-dependencies]"]
    ditems = []
    for k in sorted(tk - pk):
        spec = devspecs.get(k, specs.get(k))
        ditems.append((k, rebase(spec, pkg)))
    for k in sorted(set(pk) & tk):
        pass
    for f in sorted(set(tf) - set(pf)):
        ditems.append(fp_entry(f, pkg, specs))
    for k, v in sorted(ditems):
        lines.append(fmt(k, v))
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    crate = sys.argv[1]
    apply = "--apply" in sys.argv
    prod, test, pk, tk, pf, tf, specs, devspecs = analyse(crate)
    cur = toml_json(PLUG + "/" + CRATES[crate]["root"] + "/" + PKGDIR + "/Cargo.toml")
    have = set(cur.get("dependencies", {}))
    print(crate, "prod files", len(prod), "test files", len(test))
    print(" external used (prod):", sorted(pk))
    print(" external used (test only):", sorted(tk - pk))
    print(" first-party used (prod):", sorted(pf), " (test only):", sorted(set(tf) - set(pf)))
    print(" manifest has:", sorted(have))
    print(" missing in manifest:", sorted(({k for k in pk} | {fp_name(f) for f in pf}) - have))
    print(" unused in manifest:", sorted(have - ({k for k in pk} | {fp_name(f) for f in pf})))
    if apply:
        mp = PLUG + "/" + CRATES[crate]["root"] + "/" + PKGDIR + "/Cargo.toml"
        txt = read(mp)
        head = txt[:txt.index("[dependencies]")]
        write(mp, head + render_manifest(crate, pk, tk, pf, tf, specs, devspecs))
        print("manifest rewritten")
