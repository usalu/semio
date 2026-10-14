#!/usr/bin/env python3
"""⚙️ Writes the manifests of the `🧪️sim` family crates and trims each crate root's alias block to the crates it names
(lane P1, slice S1d). Dependencies come from the comment-stripped sources: `crate::<alias>::` paths and framework roots.
"""
import os, re, json
HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "🕸️engine-graph.py"), encoding="utf-8").read().split("mods = {")[0])
SIM = ENGINE + "/🧪️sim"
FRAMEWORK = {"semio_framework_value": "semio-framework-value", "semio_framework_value_derive": "semio-framework-value-derive", "semio_framework_job": "semio-framework-job",
             "semio_framework_dsl_record": "semio-framework-dsl-record", "semio_framework_pack_json": "semio-framework-pack-json", "serde": "serde", "serde_json": "serde_json"}
CRATES = {"retirement": ("🪜️retirement", "semio-s-plugin-energy-engine-sim-retirement", "Energy simulation engine `sim` retirement: close-turn vocabulary and bounded close ladders of the engine owners", "../../../../../../.."),
          "wire": ("📡️wire", "semio-s-plugin-energy-engine-sim-wire", "Energy simulation engine `sim` retained wire: packets, lease, queue and the numerical admission census", "../../../../../../.."),
          "restore": ("♻️restore", "semio-s-plugin-energy-engine-sim-restore", "Energy simulation engine `sim` checkpoint restore", "../../../../../../.."),
          "sim": ("", "semio-s-plugin-energy-engine-sim", None, "../../../../../..")}
engine_mods = {re.sub(r'^[^a-z_]+', '', d) for d in os.listdir(ENGINE) if os.path.isdir(os.path.join(ENGINE, d))}
def pkg(alias): return CRATES[alias][1] if alias in CRATES else "semio-s-plugin-energy-engine-" + alias.replace("_", "-")
for alias, (sub, name, desc, ws) in CRATES.items():
    d = os.path.join(SIM, sub) if sub else SIM
    lib = strip(open(d + "/🦀️.rs", encoding="utf-8").read())
    tp = d + "/🧪️tests/🔬️unit/🦀️.rs"; test = strip(open(tp, encoding="utf-8").read()) if os.path.exists(tp) else ""
    def sib(text): return {x for x in re.findall(r"(?:\bcrate::|(?<![\w:]))([a-z_][a-z0-9_]*)::", text) if (x in engine_mods or x in CRATES) and x != alias}
    def fw(text): return {FRAMEWORK[x] for x in re.findall(r"(?<![\w:])([a-z_][a-z0-9_]*)::", text) if x in FRAMEWORK} | ({"semio-framework-pack-json"} if "canonical_json" in text else set())
    lib_s, test_s = sib(lib), sib(test) - sib(lib)
    lib_f, test_f = fw(lib) | {"semio-framework-value"} if "Derive" in lib else fw(lib), fw(test) - fw(lib)
    man = os.path.join(d, "📦️packages", "🦀️rust", "Cargo.toml")
    if desc is None:
        desc = re.search(r'^description = "([^"]*)"', open(man, encoding="utf-8").read(), re.M).group(1)
    rows = sorted([f"{pkg(x)} = {{ workspace = true }}" for x in lib_s] + [f"{x} = {{ workspace = true }}" for x in lib_f])
    dev = sorted([f"{pkg(x)} = {{ workspace = true }}" for x in test_s] + [f"{x} = {{ workspace = true }}" for x in test_f])
    text = "\n".join(["[package]", f'workspace = "{ws}"', f'name = "{name}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true",
                      f'description = "{desc}"', "", "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "",
                      "[dependencies]", *rows] + (["", "[dev-dependencies]", *dev] if dev else [])) + "\n"
    os.makedirs(os.path.dirname(man), exist_ok=True); open(man, "w", encoding="utf-8").write(text)
    raw = open(d + "/🦀️.rs", encoding="utf-8").read()
    def keep(m):
        a = m.group(2)
        if a in lib_s: return m.group(0).replace("#[cfg(test)]\n", "")
        if a in test_s: return "#[cfg(test)]\n" + m.group(0).replace("#[cfg(test)]\n", "")
        return ""
    raw = re.sub(r"(?m)^(#\[cfg\(test\)\]\n)?extern crate (semio_s_plugin_energy_engine_[a-z_]+) as ([a-z_]+);\n", lambda m: keep(type("M", (), {"group": lambda self, i: {0: m.group(0), 2: m.group(3)}[i]})()), raw)
    missing = [x for x in sorted(lib_s) if f" as {x};" not in raw] + [x for x in sorted(test_s) if f" as {x};" not in raw]
    if missing:
        i = raw.index("\n\n", raw.index("//!")) + 2 if "extern crate" not in raw else raw.index("extern crate")
        raw = raw[:i] + "".join(("#[cfg(test)]\n" if x in test_s else "") + f"extern crate {pkg(x).replace('-', '_')} as {x};\n" for x in missing) + raw[i:]
    open(d + "/🦀️.rs", "w", encoding="utf-8").write(raw)
    print(alias, "lib", sorted(lib_s), sorted(lib_f), "test", sorted(test_s), sorted(test_f))
