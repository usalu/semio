#!/usr/bin/env python3
"""📦️ Wave 2 (split plan 2a): the workspace manifest and the 46 new crate manifests, dependencies derived from the paths
each crate's own sources name; conversion crates carry their format crates inside the generator-owned region."""
import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from ownership import CRATE, rust_name, crate_roots
from use_rewrite import _mask

HERE = __file__.rsplit("/", 1)[0]
own = json.load(open(HERE + "/ownership.json"))
roots = crate_roots()
WS = SEMIO + "/Cargo.toml"
AGG_MANIFEST = SEMIO + "/📦️packages/🦀️rust/Cargo.toml"
mono = toml_loads(head_text(AGG_MANIFEST))
ws_text = read(WS)
ws = toml_loads(ws_text)

PATH_DEPS = {k: v for sec in ("dependencies", "dev-dependencies") for k, v in mono[sec].items() if isinstance(v, dict) and "path" in v}
DEV_ONLY = {k for k in mono["dev-dependencies"] if k not in mono["dependencies"]}
FEATURED_DEV = {k: v["features"] for k, v in mono["dev-dependencies"].items() if isinstance(v, dict) and "features" in v and k in mono["dependencies"]}
ALL_DEPS = set(mono["dependencies"]) | set(mono["dev-dependencies"])
IDENT = {}
for k in ALL_DEPS:
    IDENT[k.replace("-", "_")] = k
for alias, k in (("dsl", "semio-framework-os-kernel"), ("protocol", "semio-framework-os-kernel"), ("store", "semio-framework-os-kernel"), ("framework_schema", "semio-framework-schema"), ("value_derive", "semio-framework-value-derive"), ("graph_core", "semio-framework-graph")):
    IDENT[alias] = k
for key, name in CRATE.items():
    IDENT[name.replace("-", "_")] = name
ALWAYS = {"semio-framework-os-kernel", "semio-framework-schema", "semio-framework-value-derive"}


def pkg_dir(key):
    return os.path.dirname(roots[key]) + "/📦️packages/🦀️rust" if key != "agg" else SEMIO + "/📦️packages/🦀️rust"


TESTS = {}
for t in mono.get("test", []):
    abs_path = os.path.normpath(os.path.join(SEMIO + "/📦️packages/🦀️rust", t["path"]))
    rel = os.path.relpath(abs_path, SUBSETS)
    sub = re.sub(r"^[^a-z]+", "", rel.split("/")[0]) if not rel.startswith("..") else "agg"
    key = "agg" if sub in ("base", "agg") else sub
    TESTS.setdefault(key, []).append((t["name"], abs_path))


def used(files):
    found = set()
    for f in files:
        m = _mask(read(f))
        for ident in re.findall(r"(?<![\w:])([a-z][a-z0-9_]*)(?=::|!)", m):
            if ident in IDENT:
                found.add(IDENT[ident])
        for ident in re.findall(r"extern crate ([a-z][a-z0-9_]*)", m):
            if ident in IDENT:
                found.add(IDENT[ident])
    return found


def is_test_file(f):
    return "/🧪️tests/" in f


def spec(dep, features=None):
    feats = ", features = [%s]" % ", ".join('"%s"' % x for x in features) if features else ""
    return "%s = { workspace = true%s }" % (dep, feats)


FORMAT = re.compile(r"^semio-s-artifact-stdio-(?!semio|contract)")
DESC = {"base": "Foundation of the stdio semio artifact family: shared vocabulary, codec primitives, editor plumbing and conversion registration",
        "agg": "Stdio semio artifact: the envelope union of every semio subset, its members and its declaration",
        "catalog": "Full stdio semio catalog: the semio artifact declared with every subset conversion"}
manifests = {}
for key in roots:
    files = [f for f, k in own.items() if k == key]
    tests = TESTS.get(key, [])
    lib_files = [f for f in files if not is_test_file(f)]
    test_files = [f for f in files if is_test_file(f)] + [p for _, p in tests]
    normal = (used(lib_files) | ALWAYS) - DEV_ONLY - {CRATE[key]}
    dev = (used(test_files) - normal) - {CRATE[key]}
    has_tests = bool(test_files) or any("#[cfg(test)]" in read(f) for f in files)
    formats = sorted(d for d in normal if FORMAT.match(d))
    plain = sorted(d for d in normal if not FORMAT.match(d))
    if key == "catalog":
        conv = sorted(CRATE[k] for k in roots if k.endswith(("-import", "-export")))
        plain = sorted(set(plain) - set(conv))
    lines = []
    if key != "agg":
        rel_ws = os.path.relpath(SEMIO, pkg_dir(key))
        desc = DESC.get(key) or ("Stdio semio `%s` subset: schema, io codecs, editor and viewer" % key if key in NAMES else "Stdio semio `%s` subset %s conversions" % tuple(key.rsplit("-", 1)))
        lines += ["[package]", 'workspace = "%s"' % rel_ws, 'name = "%s"' % CRATE[key], "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", 'description = "%s"' % desc, "", "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "", "[dependencies]"]
        lines += [spec(d) for d in plain]
        if key.endswith(("-import", "-export")):
            lines += ["# 🧩️ Capability Dependencies"] + [spec(d) for d in formats] + ["# /🧩️ Capability Dependencies"]
        else:
            lines += [spec(d) for d in formats]
        if key == "catalog":
            lines += ["# 🔗️ Conversion Dependencies"] + [spec(d) for d in conv] + ["# /🔗️ Conversion Dependencies"]
        devs = sorted(dev)
        fdev = sorted(d for d in FEATURED_DEV if d in normal) if has_tests else []
        if devs or fdev:
            lines += ["", "[dev-dependencies]"] + [spec(d, FEATURED_DEV[d]) for d in fdev] + [spec(d) for d in devs if d not in FEATURED_DEV]
        for name, path in tests:
            lines += ["", "[[test]]", 'name = "%s"' % name, 'path = "%s"' % os.path.relpath(path, pkg_dir(key))]
        manifests[pkg_dir(key) + "/Cargo.toml"] = "\n".join(lines) + "\n"
    else:
        agg_deps = (normal, dev, has_tests, tests)

agg_normal, agg_dev, _, agg_tests = agg_deps
agg = head_text(AGG_MANIFEST)
pre = agg[:agg.index("[features]")]
post_meta = agg[agg.index("[package.metadata.semio.composition]"):agg.index("[[test]]\nname = \"drawing_original_prefix\"")]
lines = [pre.rstrip("\n"), "", "[dependencies]"] + [spec(d) for d in sorted(agg_normal - {"semio-framework-dsl-record-derive"} | ({"semio-framework-dsl-record-derive"} & agg_normal))]
fdev = sorted(d for d in FEATURED_DEV if d in agg_normal)
lines += ["", "[dev-dependencies]"] + [spec(d, FEATURED_DEV[d]) for d in fdev] + [spec(d) for d in sorted(agg_dev) if d not in FEATURED_DEV]
lines += ["", post_meta.rstrip("\n")]
for name, path in agg_tests:
    lines += ["", "[[test]]", 'name = "%s"' % name, 'path = "%s"' % os.path.relpath(path, SEMIO + "/📦️packages/🦀️rust")]
manifests[AGG_MANIFEST] = "\n".join(lines) + "\n"

members = ['    "%s",' % os.path.relpath(pkg_dir(k), SEMIO) for k in ["agg", "base"] + NAMES + [s + "-" + d for s in CONVERSION_SUBSETS for d in ("import", "export")] + ["catalog"]]
ws_new = re.sub(r"members = \[\n(?:.*\n)*?\]\n", "members = [\n" + "\n".join(members) + "\n]\n", ws_text, count=1)
ws_new = re.sub(r'exclude = \[\n    "\*\*/🏅️standards/\*\*",\n    "\*\*/🏭️bridge/\*\*",\n    "\*\*/👽️guest/\*\*",\n    "\*\*/🔮️oracles/\*\*",\n', 'exclude = [\n    "🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge",\n', ws_new, count=1)
ws_new = ws_new.replace('exclude-patterns = [\n  "**/🏅️standards/**",\n  "**/👽️guest/**",\n  "**/🔮️oracles/**",\n  "**/🏭️bridge/**"\n]', 'exclude-patterns = [\n  "**/🏭️generator/**",\n  "**/🔬️probes/**",\n  "**/🏭️bridge/**",\n  "**/👽️guest/**",\n  "**/🔮️oracles/**"\n]')
extra = []
for k, v in sorted(PATH_DEPS.items()):
    p = os.path.relpath(os.path.normpath(os.path.join(SEMIO + "/📦️packages/🦀️rust", v["path"])), SEMIO)
    extra.append('[workspace.dependencies.%s]\npath = "%s"\n' % (k, p))
for k in ["base"] + NAMES + [s + "-" + d for s in CONVERSION_SUBSETS for d in ("import", "export")] + ["agg", "catalog"]:
    extra.append('[workspace.dependencies.%s]\npath = "%s"\n' % (CRATE[k], os.path.relpath(pkg_dir(k), SEMIO)))
for k in ("semio-framework-async-macros",):
    pass
anchor = "[workspace.dependencies.serde]"
ws_new = ws_new.replace(anchor, "\n".join(extra) + "\n" + anchor, 1)
for k in ("parry3d", "num-rational", "resvg", "hayro"):
    v = mono["dev-dependencies"][k]
    ws_new = ws_new.replace(anchor, '[workspace.dependencies.%s]\nversion = "%s"\n\n' % (k, v) + anchor, 1)
manifests[WS] = ws_new
for p, c in manifests.items():
    write(p, c)
graph_root = roots["graph"]
g = read(graph_root)
if "graph_core" not in g.split("extern crate semio_framework_value_derive as value_derive;")[0]:
    write(graph_root, g.replace("extern crate semio_framework_value_derive as value_derive;\n", "extern crate semio_framework_value_derive as value_derive;\nextern crate semio_framework_graph as graph_core;\n", 1))
print("wrote", len(manifests), "manifests")
