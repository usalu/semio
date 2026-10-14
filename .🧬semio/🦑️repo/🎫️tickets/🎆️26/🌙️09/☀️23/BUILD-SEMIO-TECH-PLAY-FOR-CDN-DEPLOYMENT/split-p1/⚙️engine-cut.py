#!/usr/bin/env python3
"""⚙️ Cuts the energy simulation engine (`🔨️modules/⚡️simulation/⚙️engine/<m>`) into one crate per module (lane P1, S1b).

Per module: `📦️packages/🦀️rust/Cargo.toml` (`semio-s-plugin-energy-engine-<m>`, `[lib] path = "../../🦀️.rs"`), and a
crate-root block of `extern crate <sibling> as <m>;` lines so every existing `crate::<m>::Item` path names the owning
crate. Dependencies are derived from the comment-stripped source: sibling crates from `crate::<m>` paths (lib vs test),
framework crates from the path roots the module names (plus the crates its derives expand to).

usage: ⚙️engine-cut.py plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "🕸️engine-graph.py"), encoding="utf-8").read().split("mods = {")[0])
PLUGIN = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
mode = sys.argv[1]
mods = {re.sub(r'^[^a-z_]+', '', d): d for d in sorted(os.listdir(ENGINE)) if os.path.isdir(os.path.join(ENGINE, d))}
def package(m): return "semio-s-plugin-energy-engine-" + m.replace("_", "-")
def ident(m): return package(m).replace("-", "_")
FRAMEWORK = {
    "semio_framework_value": "semio-framework-value", "semio_framework_value_derive": "semio-framework-value-derive",
    "semio_framework_job": "semio-framework-job", "semio_framework_dsl_record": "semio-framework-dsl-record",
    "semio_framework_dsl_record_derive": "semio-framework-dsl-record-derive", "semio_framework_pack_json": "semio-framework-pack-json",
    "semio_framework_plugin": "semio-framework-plugin", "serde": "serde", "serde_json": "serde_json", "geometry": "geometry",
}
IMPLIED = {"semio_framework_dsl_record_derive": ["semio_framework_dsl_record"], "semio_framework_value_derive": ["semio_framework_value"]}
plan = {}
for m, d in mods.items():
    lib_sib, test_sib, lib_ext, test_ext = set(), set(), set(), set()
    for root, _, files in os.walk(os.path.join(ENGINE, d)):
        if "📦️packages" in root: continue
        for f in files:
            if not f.endswith(".rs"): continue
            p = os.path.join(root, f); s = strip(open(p, encoding="utf-8").read()); t = "🧪️tests" in p
            for x in re.findall(r"\bcrate::([a-z_][a-z0-9_]*)", s):
                if x in mods and x != m: (test_sib if t else lib_sib).add(x)
            for x in re.findall(r"(?<![\w:])([a-z_][a-z0-9_]*)::", s):
                if x in FRAMEWORK: (test_ext if t else lib_ext).add(x)
            if re.search(r"CanonicalJsonTree|canonical_json", s): (test_ext if t else lib_ext).add("semio_framework_pack_json")
    for x in list(lib_ext):
        lib_ext.update(IMPLIED.get(x, []))
    for x in list(test_ext):
        test_ext.update(IMPLIED.get(x, []))
    plan[m] = {"dir": d, "lib": sorted(lib_sib), "test": sorted(test_sib - lib_sib), "ext": sorted(lib_ext), "test_ext": sorted(test_ext - lib_ext)}
json.dump(plan, open(os.path.join(HERE, "🗑️generated", "engine-plan.json"), "w"), indent=1)
order, seen = [], set()
def visit(m, stack=()):
    if m in seen: return
    if m in stack: raise SystemExit("cycle: " + " -> ".join(stack + (m,)))
    for n in plan[m]["lib"]: visit(n, stack + (m,))
    seen.add(m); order.append(m)
for m in plan: visit(m)
for m in plan:
    for n in plan[m]["test"]:
        if m in closure if False else False: pass
def lib_closure(m, acc=None):
    acc = set() if acc is None else acc
    for n in plan[m]["lib"]:
        if n not in acc: acc.add(n); lib_closure(n, acc)
    return acc
bad = [(m, n) for m in plan for n in plan[m]["test"] if m in lib_closure(n)]
if bad: print("TEST-CYCLES", bad)
if mode == "plan":
    for m in order: print(m, "lib", plan[m]["lib"], "test", plan[m]["test"], "ext", plan[m]["ext"], "test_ext", plan[m]["test_ext"])
    sys.exit(0)
for m in order:
    d = os.path.join(ENGINE, plan[m]["dir"]); root = os.path.join(d, "🦀️.rs"); src = open(root, encoding="utf-8").read()
    para = []
    for l in src.splitlines():
        if not l.startswith("//!") or (para and not l[3:].strip()): break
        para.append(l[3:].strip())
    first = re.sub(r"^\W+\s*", "", " ".join(para)).replace('"', "'").replace("`", "'")
    first = re.split(r"(?<=[a-z0-9\)'])\.(?:\s|$)|[:;—]\s", first)[0].rstrip(",. ")
    def dep_rows(sib, ext):
        rows = [f"{package(n)} = {{ workspace = true }}" for n in sib]
        for x in ext:
            rows.append(f"{FRAMEWORK[x]} = {{ workspace = true }}")
        return sorted(rows)
    lines = ["[package]", 'workspace = "../../../../../.."', f'name = "{package(m)}"', "version.workspace = true", "edition.workspace = true",
             "rust-version.workspace = true", f'description = "Energy simulation engine `{m}`: {first}"', "", "[package.metadata.semio]", 'role = "library"', "",
             "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "", "[dependencies]", *dep_rows(plan[m]["lib"], plan[m]["ext"])]
    dev = dep_rows(plan[m]["test"], plan[m]["test_ext"])
    if dev: lines += ["", "[dev-dependencies]", *dev]
    os.makedirs(os.path.join(d, "📦️packages", "🦀️rust"), exist_ok=True)
    open(os.path.join(d, "📦️packages", "🦀️rust", "Cargo.toml"), "w", encoding="utf-8").write("\n".join(lines) + "\n")
    if "extern crate semio_s_plugin_energy_engine_" in src: continue
    aliases = [f"extern crate {ident(n)} as {n};" for n in plan[m]["lib"]] + [f"#[cfg(test)]\nextern crate {ident(n)} as {n};" for n in plan[m]["test"]]
    if aliases:
        lines_src = src.split("\n"); i = 0
        while i < len(lines_src) and (lines_src[i].startswith("//!") or lines_src[i].startswith("#![")): i += 1
        block = "\n".join(aliases)
        src = "\n".join(lines_src[:i]) + ("\n\n" if i else "") + block + "\n" + ("\n" if lines_src[i].strip() else "") + "\n".join(lines_src[i:])
        open(root, "w", encoding="utf-8").write(src)
print("crates", len(order)); print(" ".join(order))
